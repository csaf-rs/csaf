use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf::types::language::CsafLanguage;
use crate::csaf_traits::{
    AcknowledgmentTrait, AggregateSeverityTrait, BranchTrait, CsafTrait, CsafVersion, DistributionTrait,
    DocumentTrait, EngineTrait, GeneratorTrait, InvolvementTrait, NoteTrait, ProductGroupTrait, ProductPathTrait,
    ProductTrait, ProductTreeTrait, PublisherTrait, ReferenceTrait, RemediationTrait, RestartRequiredTrait,
    RevisionTrait, ThreatTrait, TrackingTrait, VulnerabilityTrait,
};
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::text_check::{TextCheckKind, check_text};

fn create_misspelling_finding_info(word: &str, instance_path: &str) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: format!("Misspelled word: '{word}'"),
        instance_path: instance_path.to_string(),
    })
}

/// 6.3.8 Spell Check
///
/// If the document language is given it MUST be tested that a spell check for the given language
/// does not find any mistakes. The test is skipped if the document language is not set. It fails
/// if the given language is not supported (only English is currently supported).
pub fn test_6_3_8_spell_check(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let document = doc.get_document();

    // Skip this test if language is not set
    let lang = match document.get_lang() {
        None => return Ok(()), // #409 skipped
        Some(lang) => lang,
    };

    // Check if the language is supported
    // TODO: currently, only english is supported, this will be delegated to the text_check module
    // matching in the future
    let lang = match &lang {
        CsafLanguage::Valid(valid_lang) if valid_lang.is_english() => valid_lang,
        _ => {
            return Err(vec![TestFinding::Information(TestFindingData {
                message: format!("Spell check does not support language '{lang}'"),
                instance_path: "/document/lang".to_string(),
            })]);
        },
    };

    let mut errors: Option<Vec<TestFinding>> = None;

    // Runs the spell-check for a single piece of text and appends any resulting findings.
    let mut check = |text: &str, instance_path: String| {
        for finding in check_text(TextCheckKind::Spell, text, lang) {
            errors
                .get_or_insert_default()
                .push(create_misspelling_finding_info(&finding.word, &instance_path));
        }
    };

    // Check all text fields listed in the spec
    if let Some(acknowledgments) = document.get_acknowledgments() {
        for (a_i, ack) in acknowledgments.iter().enumerate() {
            for (n_i, name) in ack.get_names().into_iter().enumerate() {
                check(name, format!("/document/acknowledgments/{a_i}/names/{n_i}"));
            }
            if let Some(organization) = ack.get_organization() {
                check(organization, format!("/document/acknowledgments/{a_i}/organization"));
            }
            if let Some(summary) = ack.get_summary() {
                check(summary, format!("/document/acknowledgments/{a_i}/summary"));
            }
        }
    }

    if let Some(aggregate_severity) = document.get_aggregate_severity() {
        check(aggregate_severity.get_text(), "/document/aggregate_severity/text".to_string());
    }

    // Only the "freeform" / "not matched to enum variant" category strings are natural-language text; the
    // well-known literal do not need to be checked
    let category = document.get_category();
    if let CsafDocumentCategory::CsafBaseOther(category_text) = &category {
        check(category_text, "/document/category".to_string());
    }

    let distribution_text = match document.get_csaf_version() {
        CsafVersion::X20 => document.get_distribution_20().and_then(|d| d.get_text()),
        CsafVersion::X21 => document.get_distribution_21().ok().and_then(|d| d.get_text()),
    };
    if let Some(text) = distribution_text {
        check(text, "/document/distribution/text".to_string());
    }

    if let Some(notes) = document.get_notes() {
        for (n_i, note) in notes.iter().enumerate() {
            if let Some(audience) = note.get_audience() {
                check(audience, format!("/document/notes/{n_i}/audience"));
            }
            check(note.get_text(), format!("/document/notes/{n_i}/text"));
            if let Some(title) = note.get_title() {
                check(title, format!("/document/notes/{n_i}/title"));
            }
        }
    }

    let publisher = document.get_publisher();
    if let Some(issuing_authority) = publisher.get_issuing_authority() {
        check(issuing_authority, "/document/publisher/issuing_authority".to_string());
    }
    check(publisher.get_name(), "/document/publisher/name".to_string());

    if let Some(references) = document.get_references() {
        for (r_i, reference) in references.iter().enumerate() {
            check(reference.get_summary(), format!("/document/references/{r_i}/summary"));
        }
    }

    check(document.get_title(), "/document/title".to_string());

    let tracking = document.get_tracking();
    if let Some(aliases) = tracking.get_aliases() {
        for (a_i, alias) in aliases.into_iter().enumerate() {
            check(alias, format!("/document/tracking/aliases/{a_i}"));
        }
    }
    if let Some(generator) = tracking.get_generator() {
        check(
            generator.get_engine().get_name(),
            "/document/tracking/generator/engine/name".to_string(),
        );
    }
    for (r_i, revision) in tracking.get_revision_history().iter().enumerate() {
        check(
            revision.get_summary(),
            format!("/document/tracking/revision_history/{r_i}/summary"),
        );
    }

    if let Some(product_tree) = doc.get_product_tree() {
        product_tree.visit_all_branches(&mut |branch, path| {
            check(branch.get_name(), format!("{path}/name"));
            if let Some(product) = branch.get_product() {
                check(product.get_name(), format!("{path}/product/name"));
            }
        });

        for (fpn_i, fpn) in product_tree.get_full_product_names().iter().enumerate() {
            check(fpn.get_name(), format!("/product_tree/full_product_names/{fpn_i}/name"));
        }

        for (pg_i, product_group) in product_tree.get_product_groups().iter().enumerate() {
            if let Some(summary) = product_group.get_summary() {
                check(summary, format!("/product_tree/product_groups/{pg_i}/summary"));
            }
        }

        let prefix = product_tree.get_product_path_prefix();
        for (p_i, path_item) in product_tree.get_product_paths().iter().enumerate() {
            check(
                path_item.get_full_product_name().get_name(),
                format!("{prefix}/{p_i}/full_product_name/name"),
            );
        }
    }

    for (v_i, vuln) in doc.get_vulnerabilities().iter().enumerate() {
        let vuln_prefix = format!("/vulnerabilities/{v_i}");

        if let Some(acknowledgments) = vuln.get_acknowledgments() {
            for (a_i, ack) in acknowledgments.iter().enumerate() {
                for (n_i, name) in ack.get_names().into_iter().enumerate() {
                    check(name, format!("{vuln_prefix}/acknowledgments/{a_i}/names/{n_i}"));
                }
                if let Some(organization) = ack.get_organization() {
                    check(organization, format!("{vuln_prefix}/acknowledgments/{a_i}/organization"));
                }
                if let Some(summary) = ack.get_summary() {
                    check(summary, format!("{vuln_prefix}/acknowledgments/{a_i}/summary"));
                }
            }
        }

        for (i_i, involvement) in vuln.get_involvements().iter().flat_map(|v| v.iter()).enumerate() {
            if let Some(summary) = involvement.get_summary() {
                check(summary, format!("{vuln_prefix}/involvements/{i_i}/summary"));
            }
        }

        if let Some(notes) = vuln.get_notes() {
            for (n_i, note) in notes.iter().enumerate() {
                if let Some(audience) = note.get_audience() {
                    check(audience, format!("{vuln_prefix}/notes/{n_i}/audience"));
                }
                check(note.get_text(), format!("{vuln_prefix}/notes/{n_i}/text"));
                if let Some(title) = note.get_title() {
                    check(title, format!("{vuln_prefix}/notes/{n_i}/title"));
                }
            }
        }

        if let Some(references) = vuln.get_references() {
            for (r_i, reference) in references.iter().enumerate() {
                check(reference.get_summary(), format!("{vuln_prefix}/references/{r_i}/summary"));
            }
        }

        for (r_i, remediation) in vuln.get_remediations().iter().enumerate() {
            check(remediation.get_details(), format!("{vuln_prefix}/remediations/{r_i}/details"));
            for (e_i, entitlement) in remediation.get_entitlements().into_iter().enumerate() {
                check(
                    entitlement,
                    format!("{vuln_prefix}/remediations/{r_i}/entitlements/{e_i}"),
                );
            }
            if let Some(restart_required) = remediation.get_restart_required()
                && let Some(details) = restart_required.get_details()
            {
                check(
                    details,
                    format!("{vuln_prefix}/remediations/{r_i}/restart_required/details"),
                );
            }
        }

        for (t_i, threat) in vuln.get_threats().iter().enumerate() {
            check(threat.get_details(), format!("{vuln_prefix}/threats/{t_i}/details"));
        }

        if let Some(title) = vuln.get_title() {
            check(title, format!("{vuln_prefix}/title"));
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(ValidatorForTest6_3_8, test_6_3_8_spell_check);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_3_8 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_3_8 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;
    
    #[test]
    fn test_test_6_3_8() {
        let case_01_single_error = Err(vec![create_misspelling_finding_info("Secruity", "/document/notes/0/text")]);
        let case_02_multiple_csaf_20 = Err(vec![
            create_misspelling_finding_info("Pruduct", "/product_tree/branches/0/branches/0/name"),
            create_misspelling_finding_info(
                "Pruduct",
                "/product_tree/branches/0/branches/0/branches/0/product/name",
            ),
            create_misspelling_finding_info("vulnerapility", "/vulnerabilities/0/notes/0/text"),
            create_misspelling_finding_info("undisclused", "/vulnerabilities/0/notes/0/text"),
            create_misspelling_finding_info("addacker", "/vulnerabilities/0/notes/0/text"),
            create_misspelling_finding_info("priviledges", "/vulnerabilities/0/notes/0/text"),
            create_misspelling_finding_info("Vu1nerability", "/vulnerabilities/0/notes/0/title"),
            create_misspelling_finding_info("sommary", "/vulnerabilities/0/notes/0/title"),
        ]);
        let case_02_multiple_csaf_21 = Err(vec![
            create_misspelling_finding_info("Produkt", "/product_tree/branches/0/branches/0/name"),
            create_misspelling_finding_info(
                "Produkt",
                "/product_tree/branches/0/branches/0/branches/0/product/name",
            ),
            create_misspelling_finding_info("combonent", "/vulnerabilities/0/notes/0/text"),
            create_misspelling_finding_info("renote", "/vulnerabilities/0/notes/0/text"),
            create_misspelling_finding_info("Vuknerability", "/vulnerabilities/0/notes/0/title"),
            create_misspelling_finding_info("sumsary", "/vulnerabilities/0/notes/0/title"),
        ]);

        TESTS_2_0.test_6_3_8.expect(ExpectedResults_2_0 {
            case_01: case_01_single_error.clone(),
            case_02: case_02_multiple_csaf_20,
            case_11: Ok(()),
            case_12: Ok(()),
        });
        TESTS_2_1.test_6_3_8.expect(ExpectedResults_2_1 {
            case_01: case_01_single_error,
            case_02: case_02_multiple_csaf_21,
            case_11: Ok(()),
            case_12: Ok(()),
        });
    }
}
