use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    str::FromStr,
};

use serde_json::json;

use crate::{
    converter::{Convert, Warning},
    csaf_traits::TrackingTrait,
    schema::{
        csaf2_0::schema::PartyStatus,
        csaf2_1::schema::{ActionCategory, ActionStatus, DlVulnIdT, Entity, EntityCategory, NameOfTheEntity},
    },
};

pub struct RefactorInvolvement;

impl Convert for RefactorInvolvement {
    /// `$.document.involvement`:
    ///   - If the user passes an `involvement` element to the CSAF 2.0 to CSAF 2.1 Converter, the tool SHALL insert that element into
    ///     `$.document.involvement` and omit the conversion of all elements in `$.vulnerabilities[*].involvements`.
    ///   - If any vulnerability item contains an `involvements` element and the user provided an `involvement` element,
    ///     the CSAF 2.0 to CSAF 2.1 Converter SHOULD issue a warning that all elements in `$.vulnerabilities[*].involvements` omitted as
    ///     the user provided `involvement` takes precedence.
    ///   - If any vulnerability item contains an `involvements` element and the user did not provide an `involvement` element,
    ///     the CSAF 2.0 to CSAF 2.1 Converter SHALL try to convert the data into a valid CSAF Document by apply the following process:
    ///
    ///     [steps 1 to 8]
    ///
    ///     During the conversion, the CSAF 2.0 to CSAF 2.1 Converter SHALL NOT invent data or set values in any other way than described above.
    ///     If the CSAF 2.0 to CSAF 2.1 Converter is unable to create a valid CSAF 2.1 Document, it SHALL remove the `involvement`
    ///     from the CSAF Document and issue a warning that `$.vulnerabilities[*].involvements` could not be converted automatically
    ///     and therefore have been removed.
    fn convert(
        &self,
        scope: &mut crate::schema::loose::Document,
        settings: &super::Settings,
        diagnostics: &mut dyn super::Diagnostics,
    ) {
        // Add user-provided involvements
        if let Some(serde_json::Value::Object(source)) = &settings.involvement
            && let Ok(target) = scope
                .v21_view_mut()
                .caution_get_or_insert_document()
                .caution_get_or_insert_involvement()
                .caution_as_object_mut()
        {
            *target = source.clone();
        }

        // Collect existing involvements
        let involvements = collect_involvements(scope);

        if !involvements.is_empty() {
            // Warn if conversion of existing vulnerability involvements was skipped
            if settings.involvement.is_some() {
                diagnostics.warning(
                    format!("/vulnerabilities/{}/involvements", involvements[0].0).into(),
                    Warning::VulnerabililtyInvolvementsIgnored,
                );
            } else {
                // Convert involvements
                let conversion_result = convert_involvements(scope, settings, diagnostics, &involvements);
                if let Err(error) = conversion_result {
                    scope
                        .v21_view_mut()
                        .document_mut()
                        .and_then(|doc| doc.caution_as_object_mut().ok())
                        .and_then(|doc| doc.remove("involvement"));
                    diagnostics.warning(
                        "/vulnerabilities".into(),
                        Warning::VulnerabililtyInvolvementsConversionFailed(error),
                    );
                }
            }
        }
        remove_vulnerability_involvements(scope);
    }
}

type ErrorString = Cow<'static, str>;

/// Collect all valid CSAF 2.0 vulnerability involvements, including their JSON pointer index in the document
fn collect_involvements(
    scope: &mut crate::schema::loose::Document,
) -> Vec<(usize, usize, crate::schema::csaf2_0::schema::Involvement)> {
    scope
        .v20_view()
        .vulnerabilities()
        .map(|vs| {
            vs.as_slice()
                .unwrap_or_default()
                .iter()
                .enumerate()
                .filter_map(|(v_id, v)| v.involvements().map(|i| (v_id, i)))
                .flat_map(|(v_id, i)| {
                    i.as_slice()
                        .unwrap_or_default()
                        .iter()
                        .enumerate()
                        .filter_map(move |(i_id, i)| i.get_struct().ok().map(|i| (v_id, i_id, i)))
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn convert_involvements(
    scope: &mut crate::schema::loose::Document,
    settings: &super::Settings,
    diagnostics: &mut dyn super::Diagnostics,
    involvements: &[(usize, usize, crate::schema::csaf2_0::schema::Involvement)],
) -> Result<(), ErrorString> {
    let vuln_ids = assign_vulnerability_ids(scope)?;
    let conversion_ids = assign_conversion_ids(scope, involvements, &vuln_ids)?;

    let entity_ids = create_entitites(scope, settings, diagnostics, involvements, &conversion_ids)?;
    apply_entity_mapping(scope, settings)?;

    for (v_id, i_id, involvement) in involvements {
        convert_involvement(
            scope,
            (v_id, i_id, involvement),
            &vuln_ids,
            &conversion_ids,
            &entity_ids,
            settings,
            diagnostics,
        )?;
    }

    collapse_entities(scope, settings, diagnostics)?;
    collapse_actions(scope, settings, diagnostics)?;

    if scope
        .v21_view()
        .document()
        .and_then(|d| d.involvement())
        .is_some_and(|i| {
            i.actions().is_none_or(|a| a.as_slice().is_ok_and(|a| a.is_empty()))
                || i.entities().is_none_or(|e| e.as_slice().is_ok_and(|e| e.is_empty()))
        })
    {
        return Err("Involvement conversion yielded no actions or entities".into());
    }

    Ok(())
}

///     1. The CSAF 2.0 to CSAF 2.1 Converter assigns each vulnerability containing an `involvements` element a document-local vuln ID with the
///        prefix `VULN-`, followed by the current value of counter for the vulnerabilities containing an `involvements` element.
///        The counter starts with `1` and is zero-padded to four digits before appended to the prefix.
///
///        > If the counter is at `1000` no additional padding is needed anymore.
fn assign_vulnerability_ids(
    scope: &mut crate::schema::loose::Document,
) -> Result<HashMap<usize, String>, Cow<'static, str>> {
    let mut vuln_ids: HashMap<usize, String> = HashMap::new();
    let mut counter = 1;
    for (v_id, vuln) in scope
        .v20_view_mut()
        .vulnerabilities_mut()
        .and_then(|v| v.as_slice_mut().ok())
        .unwrap_or_default()
        .iter_mut()
        .enumerate()
    {
        if vuln.involvements().is_some() {
            let vuln_id = format!("VULN-{counter:04}");
            vuln_ids.insert(v_id, vuln_id.clone());

            vuln.caution_as_v21_mut()
                .caution_get_or_insert_dl_vuln_id()
                .set_str(DlVulnIdT::from_str(&vuln_id).map_err(|_| "Failed to create dl_vuln_id")?);
            counter += 1;
        }
    }
    Ok(vuln_ids)
}

///     2. The CSAF 2.0 to CSAF 2.1 Converter SHALL create for each `involvements` item a conversion ID which is constructed as follows:
///        - The filename is computed from the `$.document.tracking.id` according to the rules of [sec](#filename).
///          Remaining `+` in that string are replaced by `_`.
///          The file extension `.json` is removed from the string.
///        - This string is extended by appending the `dl_vuln_id` of the vulnerability the `involvements` item is found in followed
///          by the item number inside the current `involvements`.
///          The counter for the item number starts with `1` and is zero-padded to four digits.
///
///        This conversion ID identifies an involvement item with in a CSAF document locally
///        and provides collision resistance if the filenames of the processed files are sufficiently different.
fn assign_conversion_ids(
    scope: &mut crate::schema::loose::Document,
    involvements: &[(usize, usize, crate::schema::csaf2_0::schema::Involvement)],
    vuln_ids: &HashMap<usize, String>,
) -> Result<HashMap<(usize, usize), String>, ErrorString> {
    let canonical_filename = &scope
        .v20_view()
        .document()
        .and_then(|d| d.tracking())
        .and_then(|t| t.get_struct().ok())
        .map(|tracking| tracking.get_canonical_filename())
        .ok_or("Could not get document tracking ID")?;

    let mut conversion_ids: HashMap<(usize, usize), String> = HashMap::new();
    let base_filename = canonical_filename
        .replace('+', "_")
        .trim_end_matches(".json")
        .to_string();
    for (v_id, item_number, _) in involvements {
        conversion_ids.insert(
            (*v_id, *item_number),
            format!(
                "{base_filename}-{}-{:04}",
                vuln_ids
                    .get(v_id)
                    .ok_or("Could not get document local vulnerability id")?,
                item_number + 1
            ),
        );
    }
    Ok(conversion_ids)
}

///     3. The CSAF 2.0 to CSAF 2.1 Converter SHALL create for each `party` mentioned in the `involvements` a new entry in `entities`
///        and assign a `entity_id` by prepending the mandatory prefix `EID-` to the conversion ID of that `involvements` item.
///
///        > This create a new entry in `$.document.involvement.entities` per `involvements` item.
///
///        The CSAF 2.0 to CSAF 2.1 Converter SHALL convert the value of `party` into the value of `category`.
///        If the value of `party` is `other`, the CSAF 2.0 to CSAF 2.1 Converter SHOULD issue a warning that some parties have been
///        regrouped into new category values.
///        An option to suppress this warning SHALL exist.
///        In addition, an option SHOULD be provided to set one of the new values directly.
///
///        > Entities can be corrected through a mapping in a later step.
///
///     4. The CSAF 2.0 to CSAF 2.1 Converter SHALL create an entity for the publisher of the CSAF 2.0 document with the same `name`
///        and `category` as stated in the `$.document.publisher` object.
///        The `entity_id` SHALL be `EID-Publisher`.
fn create_entitites(
    scope: &mut crate::schema::loose::Document,
    settings: &super::Settings,
    diagnostics: &mut dyn super::Diagnostics,
    involvements: &[(usize, usize, crate::schema::csaf2_0::schema::Involvement)],
    conversion_ids: &HashMap<(usize, usize), String>,
) -> Result<HashMap<(usize, usize), String>, ErrorString> {
    let mut entity_ids: HashMap<(usize, usize), String> = HashMap::new();

    let publisher = scope.v20_view().document().and_then(|d| d.publisher());
    let publisher_name = publisher
        .and_then(|p| p.name())
        .and_then(|n| n.get_str().ok())
        .ok_or("Invalid name of publisher")?
        .to_string();
    let publisher_category = match publisher
        .and_then(|p| p.category())
        .and_then(|c| c.get_enum().ok())
        .ok_or("Invalid category of publisher")?
    {
        crate::schema::csaf2_0::schema::CategoryOfPublisher::Coordinator => EntityCategory::Coordinator,
        crate::schema::csaf2_0::schema::CategoryOfPublisher::Discoverer => EntityCategory::Discoverer,
        crate::schema::csaf2_0::schema::CategoryOfPublisher::Other => EntityCategory::Other,
        // TODO: What kind of mapping should be used for Translator? Currently mapped to Other.
        crate::schema::csaf2_0::schema::CategoryOfPublisher::Translator => EntityCategory::Other,
        crate::schema::csaf2_0::schema::CategoryOfPublisher::User => EntityCategory::User,
        crate::schema::csaf2_0::schema::CategoryOfPublisher::Vendor => EntityCategory::Vendor,
    };

    let entities = scope
        .v21_view_mut()
        .caution_get_or_insert_document()
        .caution_get_or_insert_involvement()
        .caution_get_or_insert_entities()
        .caution_as_vec_mut()
        .map_err(|_| "Failed to insert list of involvement entitites")?;

    let entity: Entity = Entity::builder()
        .entity_id("EID-Publisher".to_string())
        .category(publisher_category)
        .name(Some(
            NameOfTheEntity::try_from(publisher_name).map_err(|_| "Failed to create publisher entity.")?,
        ))
        .try_into()
        .map_err(|_| "Failed to create publisher entity.")?;

    entities.push(serde_json::to_value(entity).map_err(|_| "Failed to serialize publisher entity.")?);

    let mut should_warn_about_other_category =
        !settings.disable_involvement_category_other_warning && settings.involvement_category_other.is_none();

    for (v_id, i_id, involvement) in involvements {
        let eid = format!(
            "EID-{}",
            conversion_ids
                .get(&(*v_id, *i_id))
                .ok_or("Could not get conversion id")?
        );
        entity_ids.insert((*v_id, *i_id), eid.clone());

        let category = match involvement.party {
            crate::schema::csaf2_0::schema::PartyCategory::Coordinator => EntityCategory::Coordinator,
            crate::schema::csaf2_0::schema::PartyCategory::Discoverer => EntityCategory::Discoverer,
            crate::schema::csaf2_0::schema::PartyCategory::Other => {
                if should_warn_about_other_category {
                    diagnostics.warning(
                        format!("/document/vulnerabilities/{v_id}/involvements/{i_id}").into(),
                        Warning::InvolvementCategoryOtherUsed,
                    );
                    should_warn_about_other_category = false;
                }
                settings.involvement_category_other.unwrap_or(EntityCategory::Other)
            },
            crate::schema::csaf2_0::schema::PartyCategory::User => EntityCategory::User,
            crate::schema::csaf2_0::schema::PartyCategory::Vendor => EntityCategory::Vendor,
        };

        let entity: Entity = Entity::builder()
            .entity_id(eid)
            .category(category)
            .try_into()
            .map_err(|_| "Failed to create involvement entity.")?;
        entities.push(serde_json::to_value(entity).map_err(|_| "Failed to serialize publisher entity.")?);
    }

    Ok(entity_ids)
}

///     5. If the user provides a mapping which contains `entities` to CSAF 2.0 to CSAF 2.1 Converter,
///        the tool SHALL replace entries that contain a matching `entity_id` with the corresponding entry from the mapping.
///        The CSAF 2.0 to CSAF 2.1 Converter SHALL skip this step, if not mapping was provided by the user.
fn apply_entity_mapping(
    scope: &mut crate::schema::loose::Document,
    settings: &super::Settings,
) -> Result<(), ErrorString> {
    let entities = scope
        .v21_view_mut()
        .caution_get_or_insert_document()
        .caution_get_or_insert_involvement()
        .caution_get_or_insert_entities()
        .caution_as_vec_mut()
        .map_err(|_| "Failed to get list of involvement entitites")?;

    for entity in entities {
        if let Some(entity_id) = entity
            .get("entity_id")
            .and_then(|eid| eid.as_str())
            .map(|eid| eid.to_string())
            && let Some(serde_json::Value::Array(entities)) = &settings.entities
            && let Some(replacement_entity) = entities.iter().find(|entity| entity["entity_id"] == entity_id)
            && let Some(replacement_data) = replacement_entity.as_object()
            && let Some(entity) = entity.as_object_mut()
        {
            *entity = replacement_data.clone();
        }
    }
    Ok(())
}

///     6. Each item of the `involvements` array inside the current vulnerability is converted using the following subprocess:
///        - The CSAF 2.0 to CSAF 2.1 Converter SHALL create a new `action` and add the document-local vuln ID of the current vulnerability
///          into `dl_vuln_ids` of the action.
///        - The `action_id` is constructed by prepending the prefix `ACN-` to the conversion ID of that `involvements` item.
///        - If the `summary` was given, the CSAF 2.0 to CSAF 2.1 Converter SHALL convert its value as value into the `summary` element.
///        - If the `date` was given, the CSAF 2.0 to CSAF 2.1 Converter SHALL convert its value as value into the `date` element.
///          If the `date` is missing, the CSAF 2.0 to CSAF 2.1 Converter SHOULD stop the subprocess and discard the `action` unless the user
///          instructed the tool to add the element with an empty value.
///          In the first case, the CSAF 2.0 to CSAF 2.1 Converter SHALL issue a warning that an involvement without date was detected and
///          removed.
///          If the CSAF 2.0 to CSAF 2.1 Converter was instructed to add an empty date, it SHALL issue a warning that an involvement without
///          date was detected and converted into an invalid action with empty `date`.
///
///          > In such case, the user is expected to add missing dates from other sources.
///
///        - The CSAF 2.0 to CSAF 2.1 Converter SHALL use the [tab](#tab:involvement-status-and-category-mapping-from-csaf-2-0) to set the
///          `category` and `status`.
///        - The CSAF 2.0 to CSAF 2.1 Converter SHOULD have an option to extract additional details about the entity from the `summary`,
///          if applicable.
///          If that option is given, the CSAF 2.0 to CSAF 2.1 Converter SHALL insert the additionally extracted details in the corresponding
///          fields and issue a warning that additional details were found and used to enrich the `entity`.
///          Such warning SHOULD provide the full enriched `entity` element and the `summary` that the content was extracted from.
///        - The CSAF 2.0 to CSAF 2.1 Converter SHALL set the `acting_entity_refs` and `receiving_entity_refs` according to the content
///          of the current `involvements` item using the corresponding entry from the list of entities.
///          As default, the entity that corresponds to the `party` is referenced in the `acting_entity_refs`.
///          If the `summary` shows that the `party` is the receiver, the entity that corresponds to the `party` is referenced in
///          the `receiving_entity_refs`.
///        - To determine the entity that is not given in the CSAF 2.0 document, the CSAF 2.0 to CSAF 2.1 Converter SHALL apply the
///          following steps in this order:
///          1. If the missing entity can be derived from the context, e.g. through the content of the `summary`,
///             the respective entity SHALL be used.
///             In most cases, this will lead to inserting `EID-Publisher` into the `receiving_entity_refs` respectively `acting_entity_refs`.
///             The CSAF 2.0 to CSAF 2.1 Converter SHALL NOT make up an entity not given in the `summary`.
///          2. If the context suggests that the `involvements` item matches one of the categories that do not need a recipient,
///             the respective `category` SHALL be used.
///             The CSAF 2.0 to CSAF 2.1 Converter SHALL reference the corresponding action `referenced_action_ids`
///             if required by the `category` value.
///             If the value cannot be determined with certainty, this step is unsuccessful.
///          3. If the steps mentioned above are unsuccessful or inconclusive, the CSAF 2.0 to CSAF 2.1 Converter SHALL stop the subprocess
///             and discard the `action` unless the user instructed the tool to add the action with partial data.
///             In the first case, the CSAF 2.0 to CSAF 2.1 Converter SHALL issue a warning that an involvement without sufficient data was
///             detected, could not be converted automatically and therefore was removed.
///             If the CSAF 2.0 to CSAF 2.1 Converter was instructed to add the action with partial data, it SHALL issue a warning that an
///             involvement without sufficient data was detected and only partially converted.
fn convert_involvement(
    scope: &mut crate::schema::loose::Document,
    (v_id, i_id, involvement): (&usize, &usize, &crate::schema::csaf2_0::schema::Involvement),
    vuln_ids: &HashMap<usize, String>,
    conversion_ids: &HashMap<(usize, usize), String>,
    entity_ids: &HashMap<(usize, usize), String>,
    settings: &super::Settings,
    diagnostics: &mut dyn super::Diagnostics,
) -> Result<(), ErrorString> {
    if involvement.date.is_none() && !settings.create_incomplete_actions {
        diagnostics.warning(
            format!("/vulnerabilitites/{v_id}/involvements/{i_id}").into(),
            super::Warning::InvolvementWithoutDateSkipped,
        );
        return Ok(());
    }
    let (status, category) = map_involvement_status(involvement.status);

    let (acting_entity_refs, receiving_entity_refs, referenced_action_ids) = match find_referenced_entities_and_actions(
        (v_id, i_id, involvement),
        &category,
        entity_ids.get(&(*v_id, *i_id)).ok_or("Entity ID not found")?,
        settings.create_incomplete_actions,
        diagnostics,
    ) {
        Ok(referenced) => referenced,
        Err(message) => {
            diagnostics.warning(
                format!("/vulnerabilitites/{v_id}/involvements/{i_id}").into(),
                super::Warning::InvolvementActionParticipantsIndeterminableSkipped(message),
            );
            return Ok(());
        },
    };

    let mut action = json!(
        {
            "dl_vuln_ids": [vuln_ids.get(v_id).ok_or("Vulnerability ID not found")?],
            "action_id": format!("ACN-{}", conversion_ids.get(&(*v_id, *i_id)).ok_or("Conversion ID not found")?),
            "date": involvement.date.clone().unwrap_or_else(|| {
                diagnostics.warning(
                    format!("/vulnerabilitites/{v_id}/involvements/{i_id}").into(),
                    Warning::InvolvementWithoutDateOutputAsActionWithEmptyDate,
                );
                "".to_string()
            }),
            "status": status.to_string(),
            "category": category.to_string(),
            "acting_entity_refs": acting_entity_refs,
        }
    );
    if let Some(summary) = &involvement.summary {
        action["summary"] = json!(summary.as_str());
    }
    if let Some(refs) = receiving_entity_refs {
        action["receiving_entity_refs"] = json!(refs);
    }
    if let Some(refs) = referenced_action_ids {
        action["referenced_action_ids"] = json!(refs);
    }

    scope
        .v21_view_mut()
        .caution_get_or_insert_document()
        .caution_get_or_insert_involvement()
        .caution_get_or_insert_actions()
        .caution_as_vec_mut()
        .map_err(|_| "Failed to insert list of involvement actions")?
        .push(action);

    extract_involvement_entity_details(scope, (v_id, i_id, involvement), settings, diagnostics);
    Ok(())
}

/// Mapping of involvements `status`:
///
/// | CSAF 2.0 `status`   | CSAF 2.1 `status` | CSAF 2.1 Default `category` |
/// |:--------------------|:------------------|:----------------------------|
/// | `completed`         | `completed`       | `coordination`              |
/// | `contact_attempted` | `attempted`       | `notification`              |
/// | `disputed`          | `completed`       | `dispute`                   |
/// | `in_progress`       | `in_progress`     | `coordination`              |
/// | `not_contacted`     | `deferred`        | `notification`              |
/// | `open`              | `outstanding`     | `coordination`              |
fn map_involvement_status(status: PartyStatus) -> (ActionStatus, ActionCategory) {
    match status {
        PartyStatus::Completed => (ActionStatus::Completed, ActionCategory::Coordination),
        PartyStatus::ContactAttempted => (ActionStatus::Attempted, ActionCategory::Notification),
        PartyStatus::Disputed => (ActionStatus::Completed, ActionCategory::Dispute),
        PartyStatus::InProgress => (ActionStatus::InProgress, ActionCategory::Coordination),
        PartyStatus::NotContacted => (ActionStatus::Deferred, ActionCategory::Notification),
        PartyStatus::Open => (ActionStatus::Outstanding, ActionCategory::Coordination),
    }
}

type ReferencedEntitiesAndActions = (Vec<String>, Option<Vec<String>>, Option<Vec<String>>);
///        - The CSAF 2.0 to CSAF 2.1 Converter SHALL set the `acting_entity_refs` and `receiving_entity_refs` according to the content
///          of the current `involvements` item using the corresponding entry from the list of entities.
///          As default, the entity that corresponds to the `party` is referenced in the `acting_entity_refs`.
///          If the `summary` shows that the `party` is the receiver, the entity that corresponds to the `party` is referenced in
///          the `receiving_entity_refs`.
///        - To determine the entity that is not given in the CSAF 2.0 document, the CSAF 2.0 to CSAF 2.1 Converter SHALL apply the
///          following steps in this order:
///          1. If the missing entity can be derived from the context, e.g. through the content of the `summary`,
///             the respective entity SHALL be used.
///             In most cases, this will lead to inserting `EID-Publisher` into the `receiving_entity_refs` respectively `acting_entity_refs`.
///             The CSAF 2.0 to CSAF 2.1 Converter SHALL NOT make up an entity not given in the `summary`.
///          2. If the context suggests that the `involvements` item matches one of the categories that do not need a recipient,
///             the respective `category` SHALL be used.
///             The CSAF 2.0 to CSAF 2.1 Converter SHALL reference the corresponding action `referenced_action_ids`
///             if required by the `category` value.
///             If the value cannot be determined with certainty, this step is unsuccessful.
///          3. If the steps mentioned above are unsuccessful or inconclusive, the CSAF 2.0 to CSAF 2.1 Converter SHALL stop the subprocess
///             and discard the `action` unless the user instructed the tool to add the action with partial data.
///             In the first case, the CSAF 2.0 to CSAF 2.1 Converter SHALL issue a warning that an involvement without sufficient data was
///             detected, could not be converted automatically and therefore was removed.
///             If the CSAF 2.0 to CSAF 2.1 Converter was instructed to add the action with partial data, it SHALL issue a warning that an involvement without sufficient data was detected and only partially converted.
fn find_referenced_entities_and_actions(
    (v_id, i_id, _involvement): (&usize, &usize, &crate::schema::csaf2_0::schema::Involvement),
    category: &ActionCategory,
    entity_id: &str,
    create_incomplete_actions: bool,
    diagnostics: &mut dyn super::Diagnostics,
) -> Result<ReferencedEntitiesAndActions, ErrorString> {
    match category {
        // TODO: Determine receiving entity and referenced action
        ActionCategory::Confirmation | ActionCategory::Dispute => {
            if create_incomplete_actions {
                diagnostics.warning(
                    format!("/vulnerabilitites/{v_id}/involvements/{i_id}").into(),
                    super::Warning::InvolvementActionParticipantsIndeterminableOutputEmpty,
                );
                Ok((
                    vec![entity_id.to_string()],
                    Some(vec!["".to_string()]),
                    Some(vec!["".to_string()]),
                ))
            } else {
                Err("Could not determine receiving entity and referenced action".into())
            }
        },

        // TODO: Determine receiving entity
        ActionCategory::Coordination | ActionCategory::Notification | ActionCategory::Embargo => {
            if create_incomplete_actions {
                diagnostics.warning(
                    format!("/vulnerabilitites/{v_id}/involvements/{i_id}").into(),
                    super::Warning::InvolvementActionParticipantsIndeterminableOutputEmpty,
                );
                Ok((vec![entity_id.to_string()], Some(vec!["".to_string()]), None))
            } else {
                Err("Could not determine receiving entity".into())
            }
        },

        // These categories do not need additional receiving entities or actions.
        ActionCategory::Assessment
        | ActionCategory::Discovery
        | ActionCategory::Exploitation
        | ActionCategory::FixDeployment
        | ActionCategory::FixRelease
        | ActionCategory::Observation => Ok((vec![entity_id.to_string()], None, None)),
    }
}

///        - The CSAF 2.0 to CSAF 2.1 Converter SHOULD have an option to extract additional details about the entity from the `summary`,
///          if applicable.
///          If that option is given, the CSAF 2.0 to CSAF 2.1 Converter SHALL insert the additionally extracted details in the corresponding
///          fields and issue a warning that additional details were found and used to enrich the `entity`.
///          Such warning SHOULD provide the full enriched `entity` element and the `summary` that the content was extracted from.
fn extract_involvement_entity_details(
    _scope: &mut crate::schema::loose::Document,
    (_v_id, _i_id, _involvement): (&usize, &usize, &crate::schema::csaf2_0::schema::Involvement),
    settings: &super::Settings,
    _diagnostics: &mut dyn super::Diagnostics,
) {
    if settings.involvement_extract_entity_details {
        // TODO: implement heuristics
    }
}

///     7. The CSAF 2.0 to CSAF 2.1 Converter SHALL provide an option to collapse entities.
///        If that option is selected, the CSAF 2.0 to CSAF 2.1 Converter SHALL map all items from `entities` into groups where each group
///        contains only members that differ only in their `entity_id`.
///        The first one of each group which becomes the representative of that group.
///        References to other elements of the group are replaced with a reference to the representative of the group.
///        Then, the CSAF 2.0 to CSAF 2.1 Converter SHALL remove all items from `entities` except for the representative of the groups.
fn collapse_entities(
    scope: &mut crate::schema::loose::Document,
    settings: &super::Settings,
    _diagnostics: &mut dyn super::Diagnostics,
) -> Result<(), ErrorString> {
    if !settings.collapse_entities {
        return Ok(());
    }
    let entities = scope
        .v21_view_mut()
        .caution_get_or_insert_document()
        .caution_get_or_insert_involvement()
        .caution_get_or_insert_entities()
        .caution_as_vec_mut()
        .map_err(|_| "Failed to get entities")?;
    let mut representative_entity_by_content: HashMap<serde_json::Map<String, serde_json::Value>, String> =
        HashMap::new();
    let mut replacement_entities: HashMap<String, String> = HashMap::new();
    entities.retain(|entity| {
        if let serde_json::Value::Object(properties) = entity {
            let mut entity_relevant_properties = properties.clone();
            let entity_id = entity_relevant_properties
                .remove("entity_id")
                .and_then(|v| v.as_str().map(|s| s.to_string()))
                .unwrap_or_default();
            if let Some(existing) = representative_entity_by_content.get(&entity_relevant_properties) {
                replacement_entities.insert(entity_id, existing.clone());
                return false;
            } else {
                representative_entity_by_content.insert(entity_relevant_properties, entity_id);
                return true;
            }
        }
        true
    });

    let actions = scope
        .v21_view_mut()
        .caution_get_or_insert_document()
        .caution_get_or_insert_involvement()
        .caution_get_or_insert_actions()
        .as_slice_mut()
        .map_err(|_| "Failed to get actions")?;
    actions.iter_mut().for_each(|action| {
        if let Some(refs) = action.acting_entity_refs_mut()
            && let Ok(ref_list) = refs.caution_as_vec_mut()
        {
            ref_list.iter_mut().for_each(|entity_ref| {
                if let Some(replacement) = replacement_entities.get(entity_ref.as_str().unwrap_or_default()) {
                    *entity_ref = serde_json::Value::String(replacement.clone());
                }
            });
        }
        if let Some(refs) = action.receiving_entity_refs_mut()
            && let Ok(ref_list) = refs.caution_as_vec_mut()
        {
            ref_list.iter_mut().for_each(|entity_ref| {
                if let Some(replacement) = replacement_entities.get(entity_ref.as_str().unwrap_or_default()) {
                    *entity_ref = serde_json::Value::String(replacement.clone());
                }
            });
        }
    });

    Ok(())
}

///     8. The CSAF 2.0 to CSAF 2.1 Converter SHALL provide an option to collapse actions.
///        If that option is selected, the CSAF 2.0 to CSAF 2.1 Converter SHALL map all items from `actions` which are not referenced in
///        any `referenced_action_ids` into groups where each group contains only members that differ only in their `dl_vuln_ids`.
///        The first one of each group which becomes the representative of that group.
///        The CSAF 2.0 to CSAF 2.1 Converter SHALL collect all values from `dl_vuln_ids` within one group and insert the values into
///        `dl_vuln_ids` of the representative of that group.
///        Then, the CSAF 2.0 to CSAF 2.1 Converter SHALL remove all items from `actions` except for the representative of the groups.
fn collapse_actions(
    scope: &mut crate::schema::loose::Document,
    settings: &super::Settings,
    _diagnostics: &mut dyn super::Diagnostics,
) -> Result<(), ErrorString> {
    if !settings.collapse_actions {
        return Ok(());
    }
    let actions = scope
        .v21_view_mut()
        .caution_get_or_insert_document()
        .caution_get_or_insert_involvement()
        .caution_get_or_insert_actions()
        .as_slice_mut()
        .map_err(|_| "Failed to get actions")?;

    let referenced_actions = actions
        .iter()
        .flat_map(|action| {
            action
                .referenced_action_ids()
                .and_then(|action| action.as_slice().ok())
                .map(|actions| {
                    actions
                        .iter()
                        .flat_map(|action| action.get_str().map(|action| action.to_string()))
                })
        })
        .flatten()
        .collect::<HashSet<_>>();

    let mut action_vuln_ids_by_content: HashMap<serde_json::Map<String, serde_json::Value>, String> = HashMap::new();
    let mut combined_vuln_ids: HashMap<String, Vec<serde_json::Value>> = HashMap::new();

    let actions = scope
        .v21_view_mut()
        .caution_get_or_insert_document()
        .caution_get_or_insert_involvement()
        .caution_get_or_insert_actions()
        .caution_as_vec_mut()
        .map_err(|_| "Failed to get actions")?;

    actions.retain_mut(|action| {
        if let serde_json::Value::Object(action_content) = action {
            let mut action_relevant_properties = action_content.clone();
            let action_id = action_relevant_properties
                .remove("action_id")
                .map(|id| id.as_str().map(|id| id.to_string()))
                .unwrap_or_default()
                .unwrap_or_default();
            let vuln_ids = action_relevant_properties
                .remove("dl_vuln_ids")
                .and_then(|ids| ids.as_array().cloned())
                .unwrap_or_default();
            if referenced_actions.contains(&action_id) {
                return true;
            }
            // don't collapse actions without date
            if action_content
                .get("date")
                .is_some_and(|date| date.as_str().unwrap_or_default().is_empty())
            {
                return true;
            }
            if let Some(replacement) = action_vuln_ids_by_content.get(&action_relevant_properties) {
                if let Some(ids) = combined_vuln_ids.get_mut(replacement) {
                    for vuln_id in vuln_ids {
                        if !ids.contains(&vuln_id) {
                            ids.push(vuln_id);
                        }
                    }
                }
                return false;
            } else {
                combined_vuln_ids.insert(action_id.clone(), vuln_ids);
                action_vuln_ids_by_content.insert(action_relevant_properties, action_id);
                return true;
            }
        }
        true
    });

    actions.iter_mut().for_each(|action| {
        let action_id = action.get("action_id").and_then(|id| id.as_str()).unwrap_or_default();
        if let Some(vuln_ids) = combined_vuln_ids.remove(action_id)
            && let Some(ids) = action.get_mut("dl_vuln_ids")
        {
            *ids = serde_json::Value::Array(vuln_ids);
        }
    });
    Ok(())
}

/// Remove the involvements part of the CSAF 2.0 vulnerability.
/// (Not specified by the standard, because it assumes re-creation instead of in-place modification.)
fn remove_vulnerability_involvements(scope: &mut crate::schema::loose::Document) {
    for vulnerability in scope
        .v20_view_mut()
        .vulnerabilities_mut()
        .and_then(|v| v.as_slice_mut().ok())
        .unwrap_or_default()
    {
        if let Ok(vuln_obj) = vulnerability.caution_as_object_mut() {
            vuln_obj.remove("involvements");
        }
    }
}
