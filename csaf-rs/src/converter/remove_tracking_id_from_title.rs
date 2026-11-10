use crate::schema::{csaf2_1, loose};

use super::{Convert, Diagnostics, Settings, Warning};

pub struct RemoveTrackingIdFromTitle;

impl Convert for RemoveTrackingIdFromTitle {
    fn convert(&self, doc: &mut loose::Document, _settings: &Settings, diagnostics: &mut dyn Diagnostics) {
        let Some(id) = doc
            .v20_view()
            .document()
            .and_then(|document| document.tracking())
            .and_then(|tracking| tracking.id())
            .and_then(|id| id.get_str().ok())
            .map(str::to_owned)
        else {
            return;
        };

        let Some(title) = doc
            .v20_view()
            .document()
            .and_then(|document| document.title())
            .and_then(|title| title.get_str().ok())
            .map(str::to_owned)
        else {
            return;
        };

        let Some(new_title) = remove_tracking_id_from_title(&title, &id) else {
            return;
        };
        if new_title.is_empty() {
            // An empty title would be schema-invalid, therefore the original
            // title is kept
            diagnostics.warning(
                "/document/title".into(),
                Warning::EmptyTitleAfterTrackingIdRemoval { id },
            );
            return;
        }
        let Some(title_node) = doc
            .v21_view_mut()
            .document_mut()
            .and_then(|document| document.title_mut())
        else {
            return;
        };
        let Ok(new_title) = csaf2_1::schema::TitleOfThisDocument::try_from(new_title) else {
            return;
        };
        title_node.set_str(new_title);
    }
}

/// Removes every occurrence of `id` from `title` together with the separating
/// characters around it. Returns `None` if `title` does not contain `id`.
fn remove_tracking_id_from_title(title: &str, id: &str) -> Option<String> {
    if !title.contains(id) {
        return None;
    }
    let mut result = title.to_string();
    while let Some(pos) = result.find(id) {
        let next = remove_occurrence(&result, pos, id.len());
        if next == result {
            // Degenerate ids that consist of separating characters only (e.g.
            // a single white space) would be re-inserted as the joining
            // separator of the surrounding content, so stop here
            break;
        }
        result = next;
    }
    Some(result)
}

/// Removes the id occurrence at byte offset `pos` together with all separating
/// characters around it. If content remains on both sides of the removed
/// window, it is joined with a single white space, otherwise the window is
/// removed completely.
///
/// Enclosing separators (brackets and quotes) are removed with the id when
/// they would be left dangling: a supported pair enclosing the id, a
/// non-matching pair with nothing between them but the id, or a single
/// separator without a partner anywhere in the title. Enclosing separators
/// belonging to retained title content are kept: the pair around `(EPMM)` in
/// `RHBA-2019:0024: (EPMM) vulnerability` is left untouched.
fn remove_occurrence(title: &str, pos: usize, len: usize) -> String {
    // The window covers the id occurrence plus any number of enclosing
    // separators around it; freely removable separating characters are
    // trimmed afterwards
    let mut window = (pos, pos + len);
    while let Some(extended) = dangling_extension(title, window) {
        window = extended;
    }
    let left = separator_run_left(title, window.0);
    let right = separator_run_right(title, window.1);
    let mut result = String::with_capacity(title.len());
    result.push_str(&title[..left]);
    if left > 0 && right < title.len() {
        result.push(' ');
    }
    result.push_str(&title[right..]);
    result
}

/// Returns the window extended by enclosing separators that would be left
/// dangling after the removal. The extension is computed from the trimmed
/// separator edges so that the resulting offsets are valid char boundaries.
fn dangling_extension(title: &str, window: (usize, usize)) -> Option<(usize, usize)> {
    let left_edge = separator_run_left(title, window.0);
    let right_edge = separator_run_right(title, window.1);
    let open = if left_edge > 0 {
        title[..left_edge].chars().next_back()
    } else {
        None
    };
    let close = if right_edge < title.len() {
        title[right_edge..].chars().next()
    } else {
        None
    };

    if let (Some(o), Some(c)) = (open, close) {
        // A supported pair encloses the id and is removed with it. Symmetric
        // quotation marks alternate between opening and closing, so they may
        // only enclose the window when an even number of non-word-internal
        // instances precedes them.
        if is_supported_pair(o, c)
            && (o != c || count_delimiter_instances(&title[..left_edge - o.len_utf8()], o).is_multiple_of(2))
        {
            return Some((left_edge - o.len_utf8(), right_edge + c.len_utf8()));
        }
    }
    // Enclosing separators without a partner anywhere would be left dangling
    // around the removed id, so they are removed as well. A word-attached
    // quote directly at the window belongs to the id's word (e.g. the
    // possessive in `RHBA-2019:0024's`) and is dropped with the id.
    let open_dangling = open.filter(|&o| {
        let p = left_edge - o.len_utf8();
        is_pair_separator(o)
            && !has_partner(o, title, p, left_edge, window, true)
            && (!is_word_attached_quote(o, title, p, left_edge, true) || left_edge == window.0)
    });
    let close_extension = close.and_then(|c| {
        let q = right_edge + c.len_utf8();
        let word_attached = is_word_attached_quote(c, title, right_edge, q, false);
        if is_pair_separator(c) && !word_attached && !has_partner(c, title, right_edge, q, window, false) {
            return Some(q);
        }
        // A word-attached apostrophe directly after the id is dropped with it
        // only for a recognized possessive suffix (e.g. the `'s` in
        // `RHBA-2019:0024's rest`). Any other word-attached quote opens a word
        // that belongs to the retained title content and is kept.
        if is_pair_separator(c) && word_attached && right_edge == window.1 {
            let remainder = word_remainder(title, q);
            if is_possessive(c, &title[q..q + remainder]) {
                return Some(q + remainder);
            }
            return None;
        }
        None
    });
    match (open_dangling, close_extension) {
        (Some(o), Some(end)) => Some((left_edge - o.len_utf8(), end)),
        (Some(o), None) => Some((left_edge - o.len_utf8(), window.1)),
        (None, Some(end)) => Some((window.0, end)),
        (None, None) => None,
    }
}

/// Returns the length of the alphanumeric run directly at `pos`, e.g. the `s`
/// of a possessive suffix
fn word_remainder(title: &str, pos: usize) -> usize {
    title[pos..]
        .char_indices()
        .take_while(|&(_, ch)| ch.is_alphanumeric())
        .last()
        .map_or(0, |(i, ch)| i + ch.len_utf8())
}

/// Returns whether the word-attached apostrophe `c` followed by `remainder`
/// forms a possessive suffix, e.g. the `'s` in `RHBA-2019:0024's rest`
fn is_possessive(c: char, remainder: &str) -> bool {
    matches!(c, '\'' | '’' | '＇') && remainder.eq_ignore_ascii_case("s")
}

/// Returns the byte offset where the run of freely removable separating
/// characters directly in front of `pos` starts. Word punctuation (e.g. the
/// dot in `.NET`) belongs to the word and is retained.
fn separator_run_left(title: &str, pos: usize) -> usize {
    let mut start = pos;
    let mut chars = title[..pos].char_indices().rev().peekable();
    while let Some((i, c)) = chars.next() {
        if !is_plain_separating_char(c) {
            break;
        }
        // The far side is the character further away from the window
        if is_word_punctuation(c) && chars.peek().is_some_and(|&(_, ch)| ch.is_alphanumeric()) {
            break;
        }
        start = i;
    }
    start
}

/// Returns the byte offset where the run of freely removable separating
/// characters directly after `pos` ends. Word punctuation (e.g. the dot in
/// `.NET`) belongs to the word and is retained.
fn separator_run_right(title: &str, pos: usize) -> usize {
    let mut end = pos;
    let mut chars = title[pos..].char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        if !is_plain_separating_char(c) {
            break;
        }
        // The far side is the character further away from the window
        if is_word_punctuation(c) && chars.peek().is_some_and(|&(_, ch)| ch.is_alphanumeric()) {
            break;
        }
        end = pos + i + c.len_utf8();
    }
    end
}

/// A separating character separates the tracking id from the rest of the
/// title. White space, colon, dash and punctuation with their common
/// look-alikes can be removed as encountered. The specification asks for
/// separating characters "including but not limited to" these, so common
/// look-alikes are covered as well.
fn is_plain_separating_char(c: char) -> bool {
    is_whitespace_like(c) || is_colon_like(c) || is_dash_like(c) || is_punctuation_like(c)
}

/// White space and invisible characters that do not carry the Unicode white
/// space property
fn is_whitespace_like(c: char) -> bool {
    c.is_whitespace() || matches!(c, '\u{200B}'..='\u{200D}' | '\u{2060}' | '\u{FEFF}')
}

fn is_colon_like(c: char) -> bool {
    matches!(c, ':' | '∶' | '﹕' | '：')
}

fn is_dash_like(c: char) -> bool {
    matches!(
        c,
        '-' | '˗' | '֊' | '־' | '‐' | '‑' | '‒' | '–' | '—' | '―' | '⁃' | '−' | '﹘' | '﹣' | '－'
    )
}

/// Period, comma and semicolon are popular separators after the tracking id,
/// e.g. `RHBA-2019:0024. description`, with CJK and fullwidth look-alikes
fn is_punctuation_like(c: char) -> bool {
    matches!(c, '.' | ',' | ';' | '、' | '。' | '，' | '．' | '；')
}

/// ASCII word punctuation that is popular as a separator but may also belong
/// to a word (e.g. the dot in `.NET`); it is only removed when it does not
/// directly border a word character on its far side
fn is_word_punctuation(c: char) -> bool {
    matches!(c, '.')
}

/// Bracket and quote pairs that are treated as separating characters. They
/// are removed when they enclose the id or when they would be left dangling.
/// Symmetric pairs may open and close with the same character.
const PAIRED_SEPARATORS: &[(char, char)] = &[
    // brackets
    ('(', ')'),
    ('[', ']'),
    ('{', '}'),
    ('〈', '〉'),
    ('《', '》'),
    ('「', '」'),
    ('『', '』'),
    ('【', '】'),
    ('〔', '〕'),
    ('〖', '〗'),
    ('〘', '〙'),
    ('（', '）'),
    ('［', '］'),
    ('｛', '｝'),
    ('｟', '｠'),
    ('｢', '｣'),
    // quotes
    ('"', '"'),
    ('\'', '\''),
    ('＂', '＂'),
    ('＇', '＇'),
    ('«', '»'),
    ('‹', '›'),
    ('‘', '’'),
    ('“', '”'),
    ('‚', '‘'),
    ('„', '‘'),
    ('„', '“'),
];

/// Quote characters. When they belong to a word (e.g. the apostrophe in
/// `Vendor's`) they neither count as partners nor are removed as dangling.
/// Brackets are never part of a word.
const QUOTE_CHARS: &[char] = &[
    '"', '\'', '＂', '＇', '«', '»', '‹', '›', '‘', '’', '“', '”', '‚', '„', '｢', '｣', '「', '」', '『', '』',
];

/// Returns whether `open` and `close` form a supported enclosing pair. Openers
/// with several possible closers (e.g. `„` with `‘` or `“`) require checking
/// the actual combination.
fn is_supported_pair(open: char, close: char) -> bool {
    PAIRED_SEPARATORS.iter().any(|&(o, c)| o == open && c == close)
}

/// Returns whether `c` is one half of a supported enclosing pair
fn is_pair_separator(c: char) -> bool {
    PAIRED_SEPARATORS.iter().any(|&(o, c2)| o == c || c2 == c)
}

/// Returns whether the quote character `c` at `[p, q)` belongs to a word,
/// judged by the word character on its side away from the window
fn is_word_attached_quote(c: char, title: &str, p: usize, q: usize, left_side: bool) -> bool {
    if !QUOTE_CHARS.contains(&c) {
        return false;
    }
    let away_side = if left_side {
        title[..p].chars().next_back()
    } else {
        title[q..].chars().next()
    };
    away_side.is_some_and(|ch| ch.is_alphanumeric())
}

/// Returns whether a partner for `c` exists outside the removed window and
/// outside the candidate itself (`[p, q)`). Word-internal instances (e.g. the
/// apostrophe in `Vendor's`) do not count as partners.
fn has_partner(c: char, title: &str, p: usize, q: usize, window: (usize, usize), left_side: bool) -> bool {
    let (before, after): (&str, &str) = if left_side {
        (&title[..p], &title[window.1..])
    } else {
        (&title[..window.0], &title[q..])
    };
    PAIRED_SEPARATORS.iter().any(|&(o, close)| {
        let partner = if o == c {
            close
        } else if close == c {
            o
        } else {
            return false;
        };
        count_delimiter_instances(before, partner) > 0 || count_delimiter_instances(after, partner) > 0
    })
}

/// Counts instances of `c` in `text` that are not part of a word; the
/// apostrophe in `Vendor's` e.g. belongs to the word and does not count
fn count_delimiter_instances(text: &str, c: char) -> usize {
    let mut count = 0;
    let mut prev: Option<char> = None;
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        let next = chars.peek().copied();
        if ch == c
            && !(QUOTE_CHARS.contains(&c)
                && prev.is_some_and(|p| p.is_alphanumeric())
                && next.is_some_and(|n| n.is_alphanumeric()))
        {
            count += 1;
        }
        prev = Some(ch);
    }
    count
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use serde_json::json;

    use crate::converter::tests::{NoDiagnostics, OnlyWarnings};

    use super::*;

    const ID: &str = "RHBA-2019:0024";

    fn converted_title(title: &str, id: &str) -> String {
        let mut doc = loose::Document::new(json!({
            "document": {
                "title": title,
                "tracking": { "id": id }
            }
        }));
        RemoveTrackingIdFromTitle.convert(&mut doc, &Settings::default(), &mut NoDiagnostics);
        doc.into_raw()["document"]["title"]
            .as_str()
            .expect("title must remain a string")
            .to_string()
    }

    fn converted_title_with_only_warnings(title: &str, id: &str) -> (String, OnlyWarnings) {
        let mut doc = loose::Document::new(json!({
            "document": {
                "title": title,
                "tracking": { "id": id }
            }
        }));
        let mut diagnostics = OnlyWarnings::default();
        RemoveTrackingIdFromTitle.convert(&mut doc, &Settings::default(), &mut diagnostics);
        let title = doc.into_raw()["document"]["title"]
            .as_str()
            .expect("title must remain a string")
            .to_string();
        (title, diagnostics)
    }

    #[test]
    fn removes_prefix_with_colon_and_whitespace() {
        // Conformance testcase 27 from csaf-2.0-to-csaf-2.1
        assert_eq!(
            "does not integrate Ivanti Endpoint Manager Mobile (EPMM)",
            converted_title(
                "testcase-17-isduba-2026-001: does not integrate Ivanti Endpoint Manager Mobile (EPMM)",
                "testcase-17-isduba-2026-001",
            ),
        );
    }

    #[test]
    fn leaves_title_without_id_unchanged() {
        assert_eq!("A regular title", converted_title("A regular title", ID));
    }

    #[test]
    fn leaves_title_unchanged_without_tracking_id() {
        let mut doc = loose::Document::new(json!({
            "document": { "title": "A regular title" }
        }));
        RemoveTrackingIdFromTitle.convert(&mut doc, &Settings::default(), &mut NoDiagnostics);
        assert_eq!(json!({ "document": { "title": "A regular title" } }), doc.into_raw());
    }

    #[test]
    fn collapses_separators_between_remaining_content() {
        assert_eq!(
            "Release notes update",
            converted_title("Release notes RHBA-2019:0024 update", ID),
        );
    }

    #[test]
    fn removes_bracketed_id() {
        assert_eq!(
            "Description more",
            converted_title("Description [RHBA-2019:0024] more", ID),
        );
    }

    #[test]
    fn removes_dashed_id() {
        assert_eq!(
            "Description more",
            converted_title("Description -RHBA-2019:0024- more", ID),
        );
    }

    #[test]
    fn removes_id_at_end_with_parentheses() {
        assert_eq!("Description", converted_title("Description (RHBA-2019:0024)", ID),);
    }

    #[test]
    fn removes_id_at_start_with_dash() {
        assert_eq!(
            "description of the issue",
            converted_title("RHBA-2019:0024 - description of the issue", ID),
        );
    }

    #[test]
    fn removes_unicode_lookalike_separators() {
        assert_eq!("Hinweis mehr", converted_title("Hinweis – (RHBA-2019:0024) mehr", ID),);
        // Matched fullwidth bracket pair enclosing the id
        assert_eq!("Hinweis mehr", converted_title("Hinweis：（RHBA-2019:0024）mehr", ID),);
    }

    #[test]
    fn keeps_brackets_enclosing_retained_content() {
        // Regression for the copilot review of PR #1466: the removal window
        // must not consume brackets that belong to retained title content
        assert_eq!(
            "(EPMM) vulnerability",
            converted_title("RHBA-2019:0024: (EPMM) vulnerability", ID),
        );
        assert_eq!("Update (EPMM)", converted_title("Update (EPMM) (RHBA-2019:0024)", ID),);
    }

    #[test]
    fn removes_quoted_id() {
        assert_eq!("description", converted_title("“RHBA-2019:0024” description", ID),);
        assert_eq!("description", converted_title("«RHBA-2019:0024» description", ID),);
        assert_eq!("description", converted_title("\"RHBA-2019:0024\" description", ID),);
        assert_eq!("description", converted_title("„RHBA-2019:0024“ description", ID),);
        assert_eq!("description", converted_title("‚RHBA-2019:0024‘ description", ID),);
    }

    #[test]
    fn keeps_independent_quoted_phrases() {
        // The closing quote of `"quoted"` and the opening quote of `"rest"`
        // must not be matched with each other across the id
        assert_eq!(
            "\"quoted\" \"rest\"",
            converted_title("\"quoted\" (RHBA-2019:0024) \"rest\"", ID),
        );
    }

    #[test]
    fn removes_punctuation_separators() {
        assert_eq!("description", converted_title("RHBA-2019:0024, description", ID),);
        assert_eq!("description", converted_title("RHBA-2019:0024. description", ID),);
        assert_eq!(
            "描述 description",
            converted_title("描述：RHBA-2019:0024；description", ID),
        );
    }

    #[test]
    fn removes_enclosed_id_with_inner_whitespace() {
        // Regression for copilot review of PR #1466: the window must expand
        // from the trimmed separator edges, not from the id occurrence
        assert_eq!("description", converted_title("(  RHBA-2019:0024) description", ID),);
        // Multibyte whitespace between the opener and the id must neither
        // panic nor leave an unmatched bracket
        assert_eq!(
            "description",
            converted_title("( \u{2003}RHBA-2019:0024) description", ID),
        );
    }

    #[test]
    fn removes_unmatched_brackets() {
        // A closing bracket without its matching opener anywhere would be
        // left dangling around the removed id
        assert_eq!("Hinweis mehr", converted_title("Hinweis：RHBA-2019:0024）mehr", ID),);
    }

    #[test]
    fn removes_dangling_quotes_and_mismatched_pairs() {
        // Dangling opener without closer anywhere
        assert_eq!("description", converted_title("„RHBA-2019:0024 description", ID),);
        // Dangling straight quote
        assert_eq!("description", converted_title("RHBA-2019:0024\" description", ID),);
        // A non-matching pair with nothing between them but the id
        assert_eq!("description", converted_title("„RHBA-2019:0024) description", ID),);
    }

    #[test]
    fn keeps_word_internal_punctuation() {
        // The dot in `.NET` belongs to the word and is retained
        assert_eq!(".NET 6 update", converted_title("RHBA-2019:0024.NET 6 update", ID),);
        // The apostrophe in `Vendor's` belongs to the retained word: it
        // neither counts as a partner nor is it removed as a dangling quote
        assert_eq!(
            "Vendor's 'quoted' 'rest'",
            converted_title("Vendor's 'quoted' (RHBA-2019:0024) 'rest'", ID),
        );
        assert_eq!("Vendor's rest", converted_title("Vendor's RHBA-2019:0024 rest", ID),);
    }

    #[test]
    fn drops_possessive_with_id() {
        // The word-attached quote directly after the id belongs to the id's
        // word (the possessive) and is dropped together with the id
        assert_eq!("rest", converted_title("RHBA-2019:0024's rest", ID),);
        assert_eq!("rest", converted_title("RHBA-2019:0024’s rest", ID),);
    }

    #[test]
    fn keeps_quoted_words_after_the_id() {
        // Regression for copilot review of PR #1466: a non-possessive quote
        // after the id opens a word belonging to the retained content and
        // must be kept, both directly after the id and after an enclosing
        // pair was absorbed
        assert_eq!(
            "\"EPMM\" vulnerability",
            converted_title("RHBA-2019:0024\"EPMM\" vulnerability", ID),
        );
        assert_eq!(
            "“EPMM” vulnerability",
            converted_title("[RHBA-2019:0024]“EPMM” vulnerability", ID),
        );
    }

    #[test]
    fn removes_all_occurrences() {
        assert_eq!("update", converted_title("RHBA-2019:0024 update (RHBA-2019:0024)", ID),);
    }

    #[test]
    fn removes_id_from_compound_word() {
        // The id is matched as a substring, per "if the value contains the id"
        assert_eq!("our notes", converted_title("our-RHBA-2019:0024 notes", ID));
    }

    #[test]
    fn warns_when_title_would_become_empty() {
        // Removing the id would result in an empty, schema-invalid title
        let (result, diagnostics) = converted_title_with_only_warnings(ID, ID);
        assert_eq!(ID, result);
        let expected: &[(Cow<'static, str>, Warning)] = &[(
            Cow::Borrowed("/document/title"),
            Warning::EmptyTitleAfterTrackingIdRemoval { id: ID.to_string() },
        )];
        assert_eq!(expected, diagnostics.warnings());
    }

    #[test]
    fn warns_when_only_id_and_separators_remain() {
        let (result, diagnostics) = converted_title_with_only_warnings("RHBA-2019:0024: ", ID);
        assert_eq!("RHBA-2019:0024: ", result);
        let expected: &[(Cow<'static, str>, Warning)] = &[(
            Cow::Borrowed("/document/title"),
            Warning::EmptyTitleAfterTrackingIdRemoval { id: ID.to_string() },
        )];
        assert_eq!(expected, diagnostics.warnings());
    }
}
