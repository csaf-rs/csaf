use std::ops::Range;

use spdx::error::{ParseError, Reason};
use spdx::lexer::{Lexer, LexerToken, Token};

use crate::csaf_traits::CsafTrait;
use crate::schema::csaf2_1::schema::LicenseExpression;
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::license_expressions::{CSAF_PARSE_MODE, parse_csaf_license_expression};

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

fn create_invalid_license_expression_error(license_expression: &str, error: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Invalid license expression '{license_expression}': {error}."),
        instance_path: "/document/license_expression".to_string(),
    })
}

/// 6.1.54 License Expression
///
/// It SHALL be tested that the license expression is valid.
/// To implement this test, it is deemed sufficient to check for the ABNF defined
/// in annex B of [SPDX](https://spdx.github.io/spdx-spec/v3.1-dev/) and the restriction
/// on the `DocumentRef` part and case sensitivity given in 3.2.2.8.
/// Although a match against the SPDX License List (`license-id`)
/// or a specific `LicenseRef` or `AdditionalRef` list is not in scope for this test,
/// it checks all other constraints given in the ABNF.
pub fn test_6_1_54_invalid_license_expression(
    doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
) -> Result<(), Vec<TestFinding>> {
    let document = doc.get_document();

    document
        .license_expression
        .as_ref()
        // Unknown SPDX identifiers should not fail test 6.1.54
        .map(|license| match validate_license_expression(license) {
            Ok(_) => Ok(()),
            Err(error) => Err(vec![create_invalid_license_expression_error(
                license.as_str(),
                format!("Error at position {}: {}", error.span.start, error.reason).as_str(),
            )]),
        })
        .unwrap_or(Ok(())) // TODO: this may become passed - not data #409
}

/// Validates a CSAF license expression.
///
/// Validation order:
/// 1. Lexical validation
/// 2. Normalization of permitted unknown identifier '+' suffixes
/// 3. SPDX license expression parsing
/// 4. Structural restrictions for NONE and NOASSERTION
fn validate_license_expression(original: &str) -> Result<(), ParseError> {
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

/// Validates the only restriction on NONE and NOASSERTION that the SPDX
/// grammar cannot express:
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

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_54, test_6_1_54_invalid_license_expression);

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_54 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[rstest]
    fn valid_license_expressions(
        #[values(
            // Regular SPDX expressions.
            "MIT",
            "MIT AND Apache-2.0",
            "MIT OR Apache-2.0",
            "MIT WITH LLVM-exception",
            // Special identifiers.
            "NONE",
            "NOASSERTION",
            "NONE AND MIT",
            "MIT AND NONE",
            "NOASSERTION AND Apache-2.0",
            "NONE AND NOASSERTION",
            // Parentheses and operator precedence.
            "NONE AND (MIT OR Apache-2.0)",
            "MIT OR (Apache-2.0 AND BSD-3-Clause)",
            "(NONE AND MIT) AND Apache-2.0",
            "MIT AND (NOASSERTION AND Apache-2.0)",
            "( MIT )",
            // Unknown license identifiers.
            "Unknown-License",
            "Unknown-License+",
            "Unknown-A+ AND Unknown-B+",
            "Unknown-A+ OR MIT",
            "Unknown-License+ WITH Unknown-Exception",
            "Unknown-licence+",
            "mit",
            "apache-2.0",
            "MIT WITH llvm-exception",
            // Valid user-defined references.
            "LicenseRef-Custom",
            "MIT AND LicenseRef-Custom",
            "LicenseRef-..Custom--",
            "MIT WITH AdditionRef-Custom",
            // Special identifiers are only restricted as direct operands.
            "NONE AND MIT OR Apache-2.0",
            "((NONE AND MIT)) OR Apache-2.0",
            "(NONE AND MIT) OR (NOASSERTION AND Apache-2.0)",
            "(MIT OR Apache-2.0) AND NONE",
            "NONE AND MIT AND NOASSERTION",
            "Apache-2.0 OR (MIT AND NOASSERTION)"
        )]
        expression: &str,
    ) {
        assert!(
            validate_license_expression(expression).is_ok(),
            "Expected valid expression: {expression}"
        );
    }

    #[rstest]
    fn invalid_license_expressions(
        #[values(
            // Invalid combinations with special identifiers.
            "NONE OR MIT",
            "MIT OR NONE",
            "NOASSERTION OR Apache-2.0",
            "MIT OR NOASSERTION",
            "NONE WITH LLVM-exception",
            "MIT WITH NOASSERTION",
            // Invalid combinations with parentheses.
            "MIT AND (NONE OR Apache-2.0)",
            "NONE AND (MIT OR NOASSERTION)",
            "((NONE)) OR MIT",
            "MIT OR (NONE)",
            // Invalid '+' usage.
            "NONE+",
            "NOASSERTION+",
            "LicenseRef-Custom+",
            "MIT WITH Unknown-Exception+",
            // Case sensitivity under our strict policy.
            "MIT or Apache-2.0",
            "MIT and Apache-2.0",
            "MIT with LLVM-exception",
            "none",
            "noassertion",
            "licenseref-Custom",
            "additionref-Custom",
            // DocumentRef restrictions.
            "DocumentRef-Example:LicenseRef-Custom",
            // Invalid whitespace.
            "MIT AND  Apache-2.0",
            "MIT  OR Apache-2.0",
            "MIT\tAND Apache-2.0",
            "MIT AND\nApache-2.0",
            "MIT AND(Apache-2.0)",
            "MIT OR(Apache-2.0)",
            // Invalid syntax.
            "MIT AND",
            "MIT OR",
            "MIT AND OR Apache-2.0",
            "(MIT AND Apache-2.0",
            "MIT AND Apache-2.0)",
            // Invalid reference identifiers.
            "LicenseRef-",
            "LicenseRef-.",
            "MIT WITH AdditionRef-",
            "MIT WITH AdditionRef-.",
            // Invalid unknown identifiers.
            "Foo:Bar",
            "---",
            // Invalid surrounding whitespace.
            " MIT",
            "MIT ",
            // Multiple '+' suffixes after a license identifier.
            "Unknown-License++",
            "Unknown-License+++",
            "Unknown-A++ AND MIT",
            "MIT AND Unknown-B++",
            // '+' suffixes after a special identifier.
            "NONE++",
            "NOASSERTION++",
            "NOASSERTION+++",
            "MIT AND NOASSERTION+",
            "NOASSERTION+ AND MIT",
            // Invalid: '+' must immediately follow the license identifier.
            "Unknown-License +",
            "Unknown-License + AND MIT",
            "MIT WITH Unknown-Exception +",
            // Multiple WITH clauses on the same license requirement.
            "MIT WITH Unknown-Exception WITH LLVM-exception",
            "Unknown-License+ WITH Unknown-Exception WITH LLVM-exception",
            "MIT WITH Unknown-Exception WITH Another-Exception"
        )]
        expression: &str,
    ) {
        assert!(
            validate_license_expression(expression).is_err(),
            "Expected invalid expression: {expression}"
        );
    }

    #[test]
    fn parser_error_preserves_original_expression() {
        let expression = "Unknown-License+ AND )";

        let error = validate_license_expression(expression).unwrap_err();

        // The error must refer to the original string, not the
        // normalized version where '+' has been replaced.
        assert_eq!(error.original, expression);
    }

    #[test]
    fn structural_validation_respects_operator_precedence() {
        // OR is entirely within the right operand of AND.
        assert!(validate_license_expression("NONE AND (MIT OR Apache-2.0)").is_ok());

        // AND binds tighter, so NONE is an operand of AND, not of OR.
        assert!(validate_license_expression("MIT OR NONE AND Apache-2.0").is_ok());

        // Parentheses are transparent: NONE is still a direct OR operand.
        assert!(validate_license_expression("(NONE) OR Apache-2.0").is_err());
    }

    #[test]
    fn test_test_6_1_54() {
        TESTS_2_1.test_6_1_54.expect(ExpectedResults {
            case_01: Err(vec![create_invalid_license_expression_error(
                "This is a license text that should not be here.",
                "Error at position 5: expected one of `AND`, `OR`, `WITH`, `)`, `+` here",
            )]),
            case_02: Err(vec![create_invalid_license_expression_error(
                "DocumentRef-some-document-reference:LicenseRef-www.example.org-Example-CSAF-License-2.0",
                "Error at position 0: expected a `LicenseRef` here",
            )]),
            case_03: Err(vec![create_invalid_license_expression_error(
                "LicenseRef-www.example.org-Example-CSAF-License-3.0+",
                "Error at position 51: expected one of `AND`, `OR`, `WITH`, `)` here",
            )]),
            case_04: Err(vec![create_invalid_license_expression_error(
                "LicenseRef-www.example.org/Example-CSAF-License-3.0",
                "Error at position 26: invalid character(s)",
            )]),
            case_05: Err(vec![create_invalid_license_expression_error(
                "LicenseRef-www.example.org%20Example-CSAF-License-3.0",
                "Error at position 26: invalid character(s)",
            )]),
            case_06: Err(vec![create_invalid_license_expression_error(
                "LicenseRef-www.example.org#Example-CSAF-License-3.0",
                "Error at position 26: invalid character(s)",
            )]),
            case_07: Err(vec![create_invalid_license_expression_error(
                "licenseref-www.example.org-Example-CSAF-License-3.0",
                "Error at position 0: expected a `correctly cased reference identifier` here",
            )]),
            case_08: Err(vec![create_invalid_license_expression_error(
                "licenseREF-www.example.org-Example-CSAF-License-3.0",
                "Error at position 0: expected a `correctly cased reference identifier` here",
            )]),
            case_09: Err(vec![create_invalid_license_expression_error(
                "LicenseRef-www.example.org-Example-CSAF-License-3.0 WITH additionref-www.example.org-Example-exception-0.9",
                "Error at position 57: expected a `correctly cased reference identifier` here",
            )]),
            case_21: Err(vec![create_invalid_license_expression_error(
                "LicenseRef-www.example.org-Example-CSAF-License-3.0 WITH  AdditionRef-www.example.org-Example-exception-0.9",
                "Error at position 57: expected a `single space` here",
            )]),
            case_22: Err(vec![create_invalid_license_expression_error(
                "MIT OR (any-OSI and NONE)",
                "Error at position 16: expected a `uppercase operator` here",
            )]),
            case_23: Err(vec![create_invalid_license_expression_error(
                "MIT or )any-OSI WITH NONE)",
                "Error at position 4: expected a `uppercase operator` here",
            )]),

            // Case 11: "CC-BY-4.0"
            // Case 12: "LicenseRef-www.example.org-Example-CSAF-License-2.0"
            // Case 13: "LicenseRef-www.example.org-Example-CSAF-License-3.0"
            // Case 14: "A.SPDX-License-ID-that.technically.could.exist-but-obviously-does-not"
            // Case 15: "A.SPDX-License-ID-that.technically.could.exist-but-obviously-does-not.including+"
            // Case 16: "LicenseRef-www.example.org-Example-CSAF-License-3.0 WITH AdditionRef-www.example.org-Example-exception-0.9"
            // Case 17: "LicenseRef-www.example.org-Example-CSAF-License-3.0 WITH Some.SPDX-License-Exception-ID-that.technically.could.exist-but-obviously-does-not"
            // Case 18: "MIT OR any-OSI"
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            case_14: Ok(()),
            case_15: Ok(()),
            case_16: Ok(()),
            case_17: Ok(()),
            case_18: Ok(()),
        });
    }
}
