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
/// Brackets are only removed as matched pairs that enclose the id, so brackets
/// belonging to retained title content are kept and unmatched brackets are not
/// introduced: the pair around `(EPMM)` in `RHBA-2019:0024: (EPMM) vulnerability`
/// is left untouched.
fn remove_occurrence(title: &str, pos: usize, len: usize) -> String {
    // The window covers the id occurrence plus any number of bracket pairs
    // enclosing it; freely removable separating characters are trimmed
    // afterwards
    let mut window = (pos, pos + len);
    while let Some((open, close)) = enclosing_bracket_pair(title, window) {
        window = (window.0 - open.len_utf8(), window.1 + close.len_utf8());
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

/// Returns the bracket pair that directly encloses the window: an opener in
/// front of the separating characters left of the window whose matching closer
/// follows those right of it.
fn enclosing_bracket_pair(title: &str, window: (usize, usize)) -> Option<(char, char)> {
    let left_edge = separator_run_left(title, window.0);
    let right_edge = separator_run_right(title, window.1);
    if left_edge == 0 || right_edge == title.len() {
        return None;
    }
    let open = title[..left_edge].chars().next_back()?;
    let close = title[right_edge..].chars().next()?;
    let expected_close = matching_closer(open)?;
    (close == expected_close).then_some((open, close))
}

/// Returns the byte offset where the run of freely removable separating
/// characters directly in front of `pos` starts
fn separator_run_left(title: &str, pos: usize) -> usize {
    title[..pos].trim_end_matches(is_plain_separating_char).len()
}

/// Returns the byte offset where the run of freely removable separating
/// characters directly after `pos` ends
fn separator_run_right(title: &str, pos: usize) -> usize {
    title.len() - title[pos..].trim_start_matches(is_plain_separating_char).len()
}

/// A separating character separates the tracking id from the rest of the
/// title. White space, colon and dash with their common look-alikes can be
/// removed as encountered. The specification asks for separating characters
/// "including but not limited to" these, so common look-alikes are covered as
/// well.
fn is_plain_separating_char(c: char) -> bool {
    is_whitespace_like(c) || is_colon_like(c) || is_dash_like(c)
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

/// Bracket pairs that are treated as separating characters. They are only
/// removed when they enclose the id, so they are kept as matching pairs.
const BRACKET_PAIRS: &[(char, char)] = &[
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
];

/// Returns the matching closing bracket for an opening bracket, if any
fn matching_closer(open: char) -> Option<char> {
    BRACKET_PAIRS.iter().find(|(o, _)| *o == open).map(|(_, c)| *c)
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
    fn keeps_unmatched_brackets() {
        // A closing bracket without its matching opener might enclose retained
        // content, therefore it is kept
        assert_eq!("Hinweis ）mehr", converted_title("Hinweis：RHBA-2019:0024）mehr", ID),);
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
