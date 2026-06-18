use std::collections::BTreeMap;

use crate::result::{ResultJson, ValidationMessageT};

/// Compare two [`ResultJson`] objects against each other.
///
/// Prints discovered issues and returns `true` if no comparison errors were found.
///
/// Primary result comparison (by JSON pointer / `instance_path`):
/// - Error present in actual but missing from expected -> printed as an error
/// - Error present in expected but missing from actual -> printed as an error
/// - Both sides have the same path but different messages -> printed as a warning
///
/// Secondary result comparison (one-way, never causes failure):
/// - Each expected error/warning/info that is absent from actual -> printed as a warning
/// - Both sides have the same path but different messages -> printed as an info
pub fn compare_result_jsons(actual: &ResultJson, expected: &ResultJson) -> bool {
    let mut has_errors = false;

    let error_color = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Red.into()));
    let warning_color = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Yellow.into()));
    let info_color = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Blue.into()));

    let empty: &[ValidationMessageT] = &[];

    if actual.overall_valid != expected.overall_valid {
        has_errors = true;
        println!(
            "{error_color}❌ Overall validation result mismatch (actual: {}, expected: {}){error_color:#}",
            actual.overall_valid, expected.overall_valid
        );
    }

    if actual.primary_result.passed != expected.primary_result.passed {
        has_errors = true;
        println!(
            "{error_color}❌ [Primary {}/passed] Primary result field mismatch (actual: {}, expected: {}){error_color:#}",
            actual.primary_result.id, actual.primary_result.passed, expected.primary_result.passed
        );
    }

    // --- Primary result ---
    has_errors |= check_primary_messages(
        actual.primary_result.errors.as_deref().unwrap_or(empty),
        expected.primary_result.errors.as_deref().unwrap_or(empty),
        "error",
        actual.primary_result.id.as_str(),
        error_color,
        warning_color,
    );
    has_errors |= check_primary_messages(
        actual.primary_result.warnings.as_deref().unwrap_or(empty),
        expected.primary_result.warnings.as_deref().unwrap_or(empty),
        "warning",
        actual.primary_result.id.as_str(),
        error_color,
        warning_color,
    );
    has_errors |= check_primary_messages(
        actual.primary_result.infos.as_deref().unwrap_or(empty),
        expected.primary_result.infos.as_deref().unwrap_or(empty),
        "info",
        actual.primary_result.id.as_str(),
        error_color,
        warning_color,
    );

    // --- Secondary results (one-way: expected entries must be present in actual) ---
    for secondary_exp in expected.secondary_results.iter().flatten() {
        let test_id: &str = secondary_exp.id.as_str();
        let actual_secondary = actual.secondary_results.iter().flatten().find(|r| r.id == test_id);
        let (actual_errors, actual_warnings, actual_infos) = match actual_secondary {
            Some(r) => (
                r.errors.as_deref().unwrap_or(empty),
                r.warnings.as_deref().unwrap_or(empty),
                r.infos.as_deref().unwrap_or(empty),
            ),
            None => (empty, empty, empty),
        };

        if let Some(actual_result) = actual_secondary
            && actual_result.passed != secondary_exp.passed
        {
            println!(
                "{warning_color}⚠️ [Secondary {}/passed] Result field mismatch (actual: {}, expected: {}){warning_color:#}",
                test_id, actual_result.passed, secondary_exp.passed
            );
        }

        check_secondary_messages(
            actual_errors,
            secondary_exp.errors.as_deref().unwrap_or(empty),
            "error",
            test_id,
            warning_color,
            info_color,
        );
        check_secondary_messages(
            actual_warnings,
            secondary_exp.warnings.as_deref().unwrap_or(empty),
            "warning",
            test_id,
            warning_color,
            info_color,
        );
        check_secondary_messages(
            actual_infos,
            secondary_exp.infos.as_deref().unwrap_or(empty),
            "info",
            test_id,
            warning_color,
            info_color,
        );
    }

    if has_errors {
        println!("{error_color}❌ Test result comparison failed!{error_color:#}");
    } else {
        println!("✅ Test result comparison passed!");
    }

    !has_errors
}

/// Remove pairs of identical messages (by `message` text) from two groups, returning what remains.
/// Both slices must already be pre-grouped to the same `instance_path`.
fn cancel_identical<'a>(
    actual: &[&'a ValidationMessageT],
    expected: &[&'a ValidationMessageT],
) -> (Vec<&'a ValidationMessageT>, Vec<&'a ValidationMessageT>) {
    let mut exp_used = vec![false; expected.len()];
    let mut remaining_actual = Vec::new();

    'outer: for &a in actual {
        for (i, &e) in expected.iter().enumerate() {
            if !exp_used[i] && e.message == a.message {
                exp_used[i] = true;
                continue 'outer;
            }
        }
        remaining_actual.push(a);
    }

    let remaining_expected = expected
        .iter()
        .enumerate()
        .filter_map(|(i, &e)| if !exp_used[i] { Some(e) } else { None })
        .collect();

    (remaining_actual, remaining_expected)
}

fn check_primary_messages(
    actual_msgs: &[ValidationMessageT],
    exp_msgs: &[ValidationMessageT],
    kind: &str,
    test_id: &str,
    error_color: anstyle::Style,
    warning_color: anstyle::Style,
) -> bool {
    let mut has_errors = false;
    let mut by_path: BTreeMap<&str, (Vec<&ValidationMessageT>, Vec<&ValidationMessageT>)> = BTreeMap::new();
    for msg in actual_msgs {
        by_path.entry(msg.instance_path.as_str()).or_default().0.push(msg);
    }
    for msg in exp_msgs {
        by_path.entry(msg.instance_path.as_str()).or_default().1.push(msg);
    }

    for (path, (actual_at_path, exp_at_path)) in &by_path {
        let (rem_actual, rem_expected) = cancel_identical(actual_at_path, exp_at_path);

        if rem_actual.is_empty() && rem_expected.is_empty() {
            continue;
        }

        if rem_actual.len() == rem_expected.len() {
            // Equal count, different messages -> warning
            println!(
                "{warning_color}⚠️  [Primary {test_id}/{kind}] Message mismatch at '{path}' ({} message(s)):{warning_color:#}",
                rem_actual.len()
            );
            println!("{warning_color}  Actual messages not present in expected:{warning_color:#}");
            for m in &rem_actual {
                println!("{warning_color}    - {}{warning_color:#}", m.message);
            }
            println!("{warning_color}  Expected messages not present in actual:{warning_color:#}");
            for m in &rem_expected {
                println!("{warning_color}    - {}{warning_color:#}", m.message);
            }
        } else {
            // Different counts -> error
            has_errors = true;
            println!(
                "{error_color}❌ [Primary {test_id}/{kind}] Message count mismatch at '{path}' (actual: {}, expected: {}):{error_color:#}",
                actual_at_path.len(),
                exp_at_path.len()
            );
            if !rem_actual.is_empty() {
                println!("{error_color}  Actual messages not present in expected:{error_color:#}");
                for m in &rem_actual {
                    println!("{error_color}    - {}{error_color:#}", m.message);
                }
            }
            if !rem_expected.is_empty() {
                println!("{error_color}  Expected messages not present in actual:{error_color:#}");
                for m in &rem_expected {
                    println!("{error_color}    - {}{error_color:#}", m.message);
                }
            }
        }
    }
    has_errors
}

fn check_secondary_messages(
    actual_msgs: &[ValidationMessageT],
    exp_msgs: &[ValidationMessageT],
    kind: &str,
    test_id: &str,
    warning_color: anstyle::Style,
    info_color: anstyle::Style,
) {
    // One-way: only iterate over paths present in expected.
    let mut exp_by_path: BTreeMap<&str, Vec<&ValidationMessageT>> = BTreeMap::new();
    for msg in exp_msgs {
        exp_by_path.entry(msg.instance_path.as_str()).or_default().push(msg);
    }
    let mut act_by_path: BTreeMap<&str, Vec<&ValidationMessageT>> = BTreeMap::new();
    for msg in actual_msgs {
        act_by_path.entry(msg.instance_path.as_str()).or_default().push(msg);
    }

    for (path, exp_at_path) in &exp_by_path {
        let empty_vec = Vec::new();
        let actual_at_path = act_by_path.get(path).unwrap_or(&empty_vec);
        let (rem_actual, rem_expected) = cancel_identical(actual_at_path, exp_at_path);

        if rem_actual.is_empty() && rem_expected.is_empty() {
            continue;
        }

        if rem_actual.len() == rem_expected.len() {
            // Equal count, different messages -> info
            println!(
                "{info_color}💡 [Secondary {test_id}/{kind}] Message mismatch at '{path}' ({} message(s)):{info_color:#}",
                rem_actual.len()
            );
            println!("{info_color}  Actual messages:{info_color:#}");
            for m in &rem_actual {
                println!("{info_color}    - {}{info_color:#}", m.message);
            }
            println!("{info_color}  Expected messages:{info_color:#}");
            for m in &rem_expected {
                println!("{info_color}    - {}{info_color:#}", m.message);
            }
        } else {
            // Fewer actual than expected -> warning; more actual than expected -> info
            let (style, prefix) = if rem_actual.len() < rem_expected.len() {
                (warning_color, "⚠️  ")
            } else {
                (info_color, "💡 ")
            };
            println!(
                "{style}{prefix}[Secondary {test_id}/{kind}] Message count mismatch at '{path}' (actual: {}, expected: {}):{style:#}",
                actual_at_path.len(),
                exp_at_path.len()
            );
            if !rem_actual.is_empty() {
                println!("{style}  Actual messages not present in expected:{style:#}");
                for m in &rem_actual {
                    println!("{style}    - {}{style:#}", m.message);
                }
            }
            if !rem_expected.is_empty() {
                println!("{style}  Expected messages not present in actual:{style:#}");
                for m in &rem_expected {
                    println!("{style}    - {}{style:#}", m.message);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::result::{RESULT_JSON_SCHEMA, ResultJson, ResultT, ValidationMessageT};

    fn make_msg(path: &str, msg: &str) -> ValidationMessageT {
        ValidationMessageT {
            instance_path: path.to_string(),
            message: msg.to_string(),
        }
    }

    fn simple_result_json(id: &str, passed: bool, errors: Option<Vec<ValidationMessageT>>) -> ResultJson {
        ResultJson {
            schema: RESULT_JSON_SCHEMA.to_string(),
            resultschema_version: "2.1".to_string(),
            overall_valid: passed,
            primary_result: ResultT {
                id: id.to_string(),
                passed,
                errors,
                warnings: None,
                infos: None,
            },
            secondary_results: None,
        }
    }

    // --- cancel_identical ---

    #[test]
    fn cancel_identical_both_empty() {
        let (rem_a, rem_e) = cancel_identical(&[], &[]);
        assert!(rem_a.is_empty());
        assert!(rem_e.is_empty());
    }

    #[test]
    fn cancel_identical_exact_match() {
        let a = make_msg("/foo", "msg");
        let e = make_msg("/foo", "msg");
        let (rem_a, rem_e) = cancel_identical(&[&a], &[&e]);
        assert!(rem_a.is_empty());
        assert!(rem_e.is_empty());
    }

    #[test]
    fn cancel_identical_different_messages() {
        let a = make_msg("/foo", "actual msg");
        let e = make_msg("/foo", "expected msg");
        let (rem_a, rem_e) = cancel_identical(&[&a], &[&e]);
        assert_eq!(rem_a.len(), 1);
        assert_eq!(rem_e.len(), 1);
    }

    #[test]
    fn cancel_identical_extra_in_actual() {
        let a1 = make_msg("/foo", "msg");
        let a2 = make_msg("/foo", "extra");
        let e = make_msg("/foo", "msg");
        let (rem_a, rem_e) = cancel_identical(&[&a1, &a2], &[&e]);
        assert_eq!(rem_a.len(), 1);
        assert_eq!(rem_a[0].message, "extra");
        assert!(rem_e.is_empty());
    }

    // --- compare_result_jsons ---

    #[test]
    fn compare_identical_passing_results() {
        let actual = simple_result_json("6.1.1", true, None);
        let expected = simple_result_json("6.1.1", true, None);
        assert!(compare_result_jsons(&actual, &expected));
    }

    #[test]
    fn compare_matching_errors() {
        let msgs = Some(vec![make_msg("/foo", "problem")]);
        let actual = simple_result_json("6.1.1", false, msgs.clone());
        let expected = simple_result_json("6.1.1", false, msgs);
        assert!(compare_result_jsons(&actual, &expected));
    }

    #[test]
    fn compare_extra_actual_error_causes_failure() {
        let actual = simple_result_json("6.1.1", false, Some(vec![make_msg("/foo", "extra")]));
        let expected = simple_result_json("6.1.1", false, None);
        assert!(!compare_result_jsons(&actual, &expected));
    }

    #[test]
    fn compare_missing_actual_error_causes_failure() {
        let actual = simple_result_json("6.1.1", false, None);
        let expected = simple_result_json("6.1.1", false, Some(vec![make_msg("/foo", "missing")]));
        assert!(!compare_result_jsons(&actual, &expected));
    }
}
