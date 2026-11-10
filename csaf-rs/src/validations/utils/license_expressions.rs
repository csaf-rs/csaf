use spdx::error::Reason;
use spdx::lexer::{Lexer, LexerToken, Token};
use spdx::{Expression, ParseError};
use std::ops::Range;

use crate::helpers::SCANCODE_LICENSEDB;
use crate::schema::csaf2_1::schema::LicenseExpression;

pub(crate) const CSAF_PARSE_MODE: spdx::ParseMode = spdx::ParseMode {
    allow_slash_as_or_operator: false,
    allow_imprecise_license_names: false,
    allow_postfix_plus_on_gpl: true,
    allow_deprecated: true,
    allow_unknown: true,
};

/// Parses the given license expression using the SPDX parser with specific options that align with the requirements of CSAF.
/// For example, unknown SPDX identifiers should not fail test 6.1.54, whereas expressions with DocumentRef are not allowed.
pub(crate) fn parse_csaf_license_expression(license: &LicenseExpression) -> Result<Expression, spdx::ParseError> {
    let expression = Expression::parse_mode(license.as_str(), CSAF_PARSE_MODE)?;
    expression
        .requirements()
        .filter_map(|requirement| {
            if let spdx::LicenseItem::Other(license_ref) = &requirement.req.license
                && license_ref.doc_ref.is_some()
            {
                Some(spdx::ParseError {
                    original: license.to_string(),
                    span: requirement.span.start as usize..requirement.span.end as usize,
                    reason: spdx::error::Reason::Unexpected(&["LicenseRef"]),
                })
            } else if let Some(spdx::AdditionItem::Other(addition_ref)) = &requirement.req.addition
                && addition_ref.doc_ref.is_some()
            {
                Some(spdx::ParseError {
                    original: license.to_string(),
                    span: requirement.span.start as usize..requirement.span.end as usize,
                    reason: spdx::error::Reason::Unexpected(&["AdditionRef"]),
                })
            } else {
                None
            }
        })
        .next()
        .map_or(Ok(()), Err)?;
    Ok(expression)
}

/// Returns `Ok(false)` if all license identifiers and exceptions in the expression are listed by SPDX or AboutCode's "ScanCode LicenseDB".
/// Returns `Ok(true)` as soon as an unlisted license identifier or exception is found.
/// Returns an error if a lexical error is encountered before any unlisted license identifier or exception.
pub(crate) fn try_contains_unlisted_license_identifier_or_exception(
    license_expression: &LicenseExpression,
) -> Result<bool, spdx::ParseError> {
    for lexer_result in Lexer::new_mode(license_expression.as_str(), CSAF_PARSE_MODE) {
        // The current `spdx` 0.13.4 lexer does not advance past a lexical error.
        // Continuing after such an error would require custom recovery/lexer logic.
        let token = lexer_result?.token;

        // TODO #1170: SPDX 3.0.1 requires license and exception identifiers to be matched case-insensitively,
        // while `spdx` 0.13.4 currently performs case-sensitive identifier lookup.
        // Update `spdx` once case-insensitive lookup is supported and adjust this handling accordingly.
        let is_listed = match token {
            Token::Unknown(_) => false,

            Token::LicenseRef { lic_ref, .. } => SCANCODE_LICENSEDB
                .get(&lic_ref.to_lowercase())
                .is_some_and(|info| !info.is_exception),

            Token::AdditionRef { add_ref, .. } => SCANCODE_LICENSEDB
                .get(&add_ref.to_lowercase())
                .is_some_and(|info| info.is_exception),

            // Known SPDX licenses/exceptions, operators, parentheses, etc.
            _ => true,
        };

        if !is_listed {
            return Ok(true);
        }
        // If the token is listed, continue to the next token
    }
    Ok(false)
}

const AND: &str = "AND";
const OR: &str = "OR";
const WITH: &str = "WITH";

/// Operators that have to be spelled in uppercase.
const OPERATORS: [&str; 3] = [AND, OR, WITH];

/// Identifiers that denote the absence of license information instead of
/// naming a license. SPDX treats them as `simple-expression`s, but only
/// `AND` may be used in direct conjunction with them.
const SPECIAL_IDENTIFIERS: [&str; 2] = ["NONE", "NOASSERTION"];

/// Prefixes that are reserved for references.
const REFERENCE_PREFIXES: [&str; 3] = ["DocumentRef-", "LicenseRef-", "AdditionRef-"];

/// Validates a CSAF license expression.
///
/// Validation order:
/// 1. Lexical validation
/// 2. Normalization of permitted unknown identifier '+' suffixes
/// 3. SPDX license expression parsing
/// 4. Structural restrictions for NONE and NOASSERTION
pub(crate) fn validate_license_expression(original: &str) -> Result<(), ParseError> {
    // Step 1 + 2:
    // Validate the original tokens and create a normalized copy.
    let (tokens, normalized) = validate_and_normalize(original)?;

    // Step 3:
    // Run the expression parser, including CSAF's DocumentRef
    // restrictions. We only need it for syntax validation.
    if let Err(mut error) = parse_csaf_license_expression(&normalized) {
        // Normalization preserves string length and byte offsets.
        // Replace the parser's normalized input with the original
        // expression so error message shows what the user supplied.
        error.original = original.to_string();

        return Err(error);
    }

    // Step 4:
    // Validate NONE/NOASSERTION with operator precedence
    // and parentheses taken into account.
    validate_special_identifier_structure(original, &tokens)
}

/// Creates an SPDX-compatible parsing error referring to the original
/// license expression and the original byte position.
fn license_expression_error(original: &str, span: Range<usize>, reason: Reason) -> ParseError {
    ParseError {
        original: original.to_string(),
        span,
        reason,
    }
}

/// Returns the canonical spelling of an operator token, or `None` for any other token.
fn canonical_operator(token: &Token<'_>) -> Option<&'static str> {
    match token {
        Token::And => Some(AND),
        Token::Or => Some(OR),
        Token::With => Some(WITH),
        _ => None,
    }
}

/// Checks the SPDX ABNF idstring requirement:
/// only ASCII letters, digits, '-' and '.', with at least one alphanumeric character.
fn is_valid_idstring(value: &str) -> bool {
    value.bytes().any(|byte| byte.is_ascii_alphanumeric())
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'.')
}

/// Returns whether a token represents NONE or NOASSERTION.
///
/// The SPDX lexer may classify these special identifiers as Unknown.
/// Therefore, we inspect the original token text instead.
///
/// Lexical validation rejects non-canonical casing before any caller runs,
/// so an exact match is sufficient here.
fn is_special_identifier(original: &str, token: &LexerToken<'_>) -> bool {
    let text = &original[token.span.clone()];

    SPECIAL_IDENTIFIERS.contains(&text)
}

/// Checks an unknown identifier for reserved words and prefixes.
///
/// Unknown ordinary license identifiers are intentionally permitted:
/// their existence is checked separately by test 6.2.45.
fn validate_unknown_identifier(original: &str, token: &LexerToken<'_>, identifier: &str) -> Result<(), ParseError> {
    // Operators and special identifiers must use their canonical
    // spelling under the strict casing policy.
    for reserved in OPERATORS.into_iter().chain(SPECIAL_IDENTIFIERS) {
        if identifier.eq_ignore_ascii_case(reserved) {
            if identifier != reserved {
                return Err(license_expression_error(
                    original,
                    token.span.clone(),
                    Reason::Unexpected(&["canonical identifier casing"]),
                ));
            }

            // Correctly spelled NONE and NOASSERTION are accepted.
            // AND, OR and WITH should normally have been recognized
            // by the lexer as operator tokens.
            return Ok(());
        }
    }

    // A token beginning with a reserved reference prefix must not
    // be silently interpreted as an ordinary unknown license ID.
    for prefix in REFERENCE_PREFIXES {
        if let Some(actual_prefix) = identifier.get(..prefix.len())
            && actual_prefix.eq_ignore_ascii_case(prefix)
        {
            // DocumentRef is forbidden for this CSAF field.
            //
            // Valid LicenseRef and AdditionRef tokens should already
            // have been recognized by the lexer. Reaching this branch
            // means the token is malformed or uses unsupported casing.
            return Err(license_expression_error(
                original,
                token.span.clone(),
                Reason::Unexpected(&["correctly cased reference identifier"]),
            ));
        }
    }

    // The lexer may accept characters that are not allowed in an
    // SPDX license identifier, so validate the idstring explicitly.
    if !is_valid_idstring(identifier) {
        return Err(license_expression_error(
            original,
            token.span.clone(),
            Reason::Unexpected(&["valid SPDX identifier"]),
        ));
    }
    Ok(())
}

/// Validates the casing of an operator token and the single separating
/// space that is required on both of its sides.
fn validate_operator(
    original: &str,
    tokens: &[LexerToken<'_>],
    index: usize,
    expected: &str,
) -> Result<(), ParseError> {
    let token = &tokens[index];

    // Enforce uppercase operators under the chosen strict casing policy.
    if &original[token.span.clone()] != expected {
        return Err(license_expression_error(
            original,
            token.span.clone(),
            Reason::Unexpected(&["uppercase operator"]),
        ));
    }

    // Operators require a separating space on both sides.
    // The previous whitespace check already guarantees
    // that such a gap cannot contain multiple spaces.
    let has_space_before = index > 0 && original.get(tokens[index - 1].span.end..token.span.start) == Some(" ");

    let has_space_after = tokens
        .get(index + 1)
        .is_some_and(|next| original.get(token.span.end..next.span.start) == Some(" "));

    if has_space_before && has_space_after {
        Ok(())
    } else {
        Err(license_expression_error(
            original,
            token.span.clone(),
            Reason::Unexpected(&["space on both sides of operator"]),
        ))
    }
}

/// Performs lexical checks on the original expression and creates
/// a normalized copy suitable for the SPDX expression parser.
///
/// The original expression is never modified. Normalization only
/// replaces permitted '+' characters with single spaces, preserving
/// the original byte offsets.
///
/// Returns the tokens of the original expression together with the
/// normalized copy as a `LicenseExpression`, so that later validation steps do not have to lex again.
fn validate_and_normalize(original: &str) -> Result<(Vec<LexerToken<'_>>, LicenseExpression), ParseError> {
    // Reject leading and trailing spaces.
    if original.starts_with(' ') || original.ends_with(' ') {
        let position = if original.starts_with(' ') {
            0
        } else {
            original.len() - 1
        };

        return Err(license_expression_error(
            original,
            position..position + 1,
            Reason::Unexpected(&["no surrounding whitespace"]),
        ));
    }
    // First tokenize the ORIGINAL expression so every token span
    // refers to the original input.
    let tokens: Vec<LexerToken<'_>> = Lexer::new_mode(original, CSAF_PARSE_MODE).collect::<Result<_, _>>()?;

    // SPDX expressions must use ordinary spaces, not tabs,
    // line breaks or other whitespace characters.
    if let Some((position, character)) = original
        .char_indices()
        .find(|(_, character)| character.is_whitespace() && *character != ' ')
    {
        return Err(license_expression_error(
            original,
            position..position + character.len_utf8(),
            Reason::InvalidCharacters,
        ));
    }

    // Reject consecutive spaces BETWEEN tokens.
    for pair in tokens.windows(2) {
        let first = &pair[0];
        let second = &pair[1];

        let gap = &original[first.span.end..second.span.start];

        if gap.len() > 1 {
            return Err(license_expression_error(
                original,
                first.span.end + 1..second.span.start,
                Reason::Unexpected(&["single space"]),
            ));
        }
    }

    // Collects the positions of '+' characters that need normalization.
    let mut plus_positions = Vec::new();

    for (index, token) in tokens.iter().enumerate() {
        // Operators are handled first so their canonical spelling can be bound directly.
        if let Some(expected) = canonical_operator(&token.token) {
            validate_operator(original, &tokens, index, expected)?;

            continue;
        }

        match &token.token {
            Token::Unknown(identifier) => {
                // Prevent incorrectly cased reserved words and
                // malformed reference identifiers from being
                // accepted as arbitrary unknown licenses.
                validate_unknown_identifier(original, token, identifier)?;

                // Check whether this unknown license identifier is
                // immediately followed by '+'.
                let Some(next) = tokens.get(index + 1) else {
                    continue;
                };

                if !matches!(&next.token, Token::Plus) {
                    continue;
                }

                // A '+' suffix cannot be applied to an exception
                // identifier following WITH.
                if index > 0 && matches!(&tokens[index - 1].token, Token::With) {
                    return Err(license_expression_error(
                        original,
                        next.span.clone(),
                        Reason::Unexpected(&["exception identifier without +"]),
                    ));
                }

                // An unknown license identifier may have only one '+' suffix.
                // Reject any additional '+' before normalization so the error
                // points to the original invalid character.
                if let Some(extra_plus) = tokens.get(index + 2)
                    && matches!(&extra_plus.token, Token::Plus)
                {
                    return Err(license_expression_error(
                        original,
                        extra_plus.span.clone(),
                        Reason::Unexpected(&["single '+' suffix"]),
                    ));
                }

                // NONE and NOASSERTION are not license IDs and therefore must never
                // be normalized. The dedicated `Token::Plus` arm below rejects them.
                if !is_special_identifier(original, token) {
                    // This unknown license ID may legally have '+'.
                    // Collect the position for later normalization.
                    plus_positions.push(next.span.start);
                }
            },

            Token::LicenseRef { lic_ref, .. } => {
                // Ensure the LicenseRef identifier satisfies the SPDX
                // idstring requirements, which the lexer does not fully enforce.
                if !is_valid_idstring(lic_ref) {
                    return Err(license_expression_error(
                        original,
                        token.span.clone(),
                        Reason::Unexpected(&["valid LicenseRef identifier"]),
                    ));
                }
            },

            Token::AdditionRef { add_ref, .. } => {
                // Apply the same idstring validation to AdditionRef identifiers.
                if !is_valid_idstring(add_ref) {
                    return Err(license_expression_error(
                        original,
                        token.span.clone(),
                        Reason::Unexpected(&["valid AdditionRef identifier"]),
                    ));
                }
            },

            // NONE and NOASSERTION are special identifiers,
            // not license IDs, so they cannot have a '+' suffix.
            Token::Plus if index > 0 && is_special_identifier(original, &tokens[index - 1]) => {
                return Err(license_expression_error(
                    original,
                    token.span.clone(),
                    Reason::Unexpected(&["license identifier without +"]),
                ));
            },

            _ => {},
        }
    }

    // Normalize only after ALL lexical checks have completed.
    let mut normalized = original.to_string();

    // Replacing a one-byte '+' with a one-byte space keeps every
    // original position unchanged.
    for position in plus_positions {
        normalized.replace_range(position..position + 1, " ");
    }

    let normalized = LicenseExpression::try_from(normalized).map_err(|_| {
        license_expression_error(
            original,
            0..original.len(),
            Reason::Unexpected(&["non-empty license expression"]),
        )
    })?;

    Ok((tokens, normalized))
}

/// Operators needed for structural validation.
///
/// WITH and + are handled separately because they do not combine
/// two independent license expressions.
#[derive(Clone, Copy, PartialEq, Eq)]
enum StructuralOperator {
    OpenParen,
    And,
    Or,
}

impl StructuralOperator {
    fn precedence(self) -> u8 {
        match self {
            Self::OpenParen => 0,
            Self::Or => 1,
            Self::And => 2,
        }
    }
}

/// Applies one binary operator to the two most recent expression values.
///
/// Each value indicates whether the corresponding operand is NONE or
/// NOASSERTION. Parentheses are transparent, because `( NONE )` denotes
/// the same thing as `NONE`.
fn apply_structural_operator(
    original: &str,
    operator: StructuralOperator,
    span: Range<usize>,
    values: &mut Vec<bool>,
) -> Result<(), ParseError> {
    // Syntax has already been checked by the main SPDX parser.
    // Therefore, a binary operator should have two operands.
    let right = values
        .pop()
        .ok_or_else(|| license_expression_error(original, span.clone(), Reason::Unexpected(&["right operand"])))?;

    let left = values
        .pop()
        .ok_or_else(|| license_expression_error(original, span.clone(), Reason::Unexpected(&["left operand"])))?;

    // SPDX: AND is the only operator that may be used in conjunction
    // with NONE or NOASSERTION. Only the direct operands count.
    if operator == StructuralOperator::Or && (left || right) {
        return Err(license_expression_error(
            original,
            span,
            Reason::Unexpected(&["OR operands without NONE or NOASSERTION"]),
        ));
    }

    // The result is a compound expression, never a special identifier,
    // so an enclosing OR is not combined with NONE or NOASSERTION.
    //
    // Example:
    //   NONE AND MIT          => false, `(NONE AND MIT) OR MIT` is valid
    //   (NONE)                => true,  `(NONE) OR MIT` is not valid
    values.push(false);

    Ok(())
}

/// Validates additional structural constraints not enforced by the existing SPDX parser.
///
/// The "AND" operator is the only operator that can be used in conjunction
/// with the special identifiers "NONE" or "NOASSERTION".
///
/// The SPDX parser does not enforce it, because it lexes both identifiers as
/// ordinary unknown license identifiers.
///
/// Operates on the tokens produced by `validate_and_normalize`, which
/// already guarantees that the expression lexes without errors.
fn validate_special_identifier_structure(original: &str, tokens: &[LexerToken<'_>]) -> Result<(), ParseError> {
    let mut operators: Vec<(StructuralOperator, Range<usize>)> = Vec::new();

    // Each value indicates whether an operand is directly NONE or NOASSERTION.
    // Compound expressions are represented by false, even if they contain
    // special identifiers.
    let mut values: Vec<bool> = Vec::new();

    // The identifier immediately following WITH is an exception/addition,
    // not an independent license-expression operand.
    let mut expect_with_addition = false;

    for (index, token) in tokens.iter().enumerate() {
        match &token.token {
            Token::OpenParen => {
                operators.push((StructuralOperator::OpenParen, token.span.clone()));
            },

            Token::CloseParen => {
                // Evaluate everything belonging to the current
                // parenthesized expression.
                while let Some((operator, span)) = operators.pop() {
                    if operator == StructuralOperator::OpenParen {
                        break;
                    }

                    apply_structural_operator(original, operator, span, &mut values)?;
                }
            },

            Token::And | Token::Or => {
                let current = if matches!(&token.token, Token::And) {
                    StructuralOperator::And
                } else {
                    StructuralOperator::Or
                };

                // AND has higher precedence than OR.
                //
                // Evaluate preceding operators only if they have
                // equal or higher precedence, stopping at '('.
                while let Some((top, span)) = operators.pop() {
                    if top == StructuralOperator::OpenParen || top.precedence() < current.precedence() {
                        // The operator binds less tightly, so it stays on the stack.
                        operators.push((top, span));

                        break;
                    }

                    apply_structural_operator(original, top, span, &mut values)?;
                }

                operators.push((current, token.span.clone()));
            },

            Token::With => {
                // Reject a second WITH applied to the same license requirement.
                // The SPDX parser may accept it after an unknown addition.
                if index >= 2 && matches!(&tokens[index - 2].token, Token::With) {
                    return Err(license_expression_error(
                        original,
                        token.span.clone(),
                        Reason::Unexpected(&["AND", "OR", ")"]),
                    ));
                }

                // WITH may not use NONE or NOASSERTION as either
                // its license operand or its addition operand.
                let left_is_special = values.last().copied().unwrap_or(false);

                let right_is_special = tokens
                    .get(index + 1)
                    .is_some_and(|next| is_special_identifier(original, next));

                if left_is_special || right_is_special {
                    return Err(license_expression_error(
                        original,
                        token.span.clone(),
                        Reason::Unexpected(&["WITH operands without NONE or NOASSERTION"]),
                    ));
                }

                expect_with_addition = true;
            },

            Token::Plus => {
                // '+' is a postfix modifier, not a separate operand.
                // Its validity has already been checked by the lexer
                // and main parser.
            },

            Token::Spdx(_)
            | Token::LicenseRef { .. }
            | Token::AdditionRef { .. }
            | Token::Exception(_)
            | Token::Unknown(_) => {
                if expect_with_addition {
                    // The addition belongs to the preceding license
                    // requirement. Do not push it onto the values stack.
                    expect_with_addition = false;
                } else {
                    // Store whether this license-expression operand
                    // is one of the two special identifiers.
                    values.push(is_special_identifier(original, token));
                }
            },
        }
    }

    // Apply any remaining operators after all tokens have been read.
    while let Some((operator, span)) = operators.pop() {
        if operator == StructuralOperator::OpenParen {
            return Err(license_expression_error(original, span, Reason::UnclosedParens));
        }

        apply_structural_operator(original, operator, span, &mut values)?;
    }

    Ok(())
}
