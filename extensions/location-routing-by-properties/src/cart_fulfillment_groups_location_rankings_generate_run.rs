use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;
use std::collections::HashMap;

#[shopify_function]
fn cart_fulfillment_groups_location_rankings_generate_run(
    input: schema::cart_fulfillment_groups_location_rankings_generate_run::Input,
) -> Result<schema::CartFulfillmentGroupsLocationRankingsGenerateRunResult> {
    // Create a map of location IDs to location handles for quick lookup
    let mut location_id_to_handle = HashMap::new();
    for location in input.locations() {
        // Extract the numeric ID from the Shopify GID format
        if let Some(numeric_id) = extract_numeric_id_from_gid(location.id()) {
            location_id_to_handle.insert(numeric_id, location.handle().clone());
        }
        
        // Also map the handle directly to itself (in case handle equals ID)
        if let Ok(handle_as_id) = location.handle().parse::<u64>() {
            location_id_to_handle.insert(handle_as_id, location.handle().clone());
        }
    }

    let operations = input
        .fulfillment_groups()
        .iter()
        .map(|group| {
            // Check if any line items in this group have a _Location ID property
            let mut preferred_location_handle: Option<String> = None;
            
            for line in group.lines() {
                if let Some(attribute) = line.attribute() {
                    if attribute.key() == "_Location ID" {
                        if let Some(location_id_str) = attribute.value() {
                            // Try to parse the location ID and find the corresponding handle
                            if let Ok(location_id) = location_id_str.parse::<u64>() {
                                if let Some(handle) = location_id_to_handle.get(&location_id) {
                                    preferred_location_handle = Some(handle.clone());
                                    break; // Use the first matching location ID found
                                }
                            }
                        }
                    }
                }
            }

            let rankings = group
                .inventory_location_handles()
                .iter()
                .map(|location_handle| {
                    let rank = if let Some(ref preferred_handle) = preferred_location_handle {
                        // Prioritize the preferred location (rank 0 = highest priority)
                        if location_handle == preferred_handle {
                            0
                        } else {
                            1 // Lower priority for other locations
                        }
                    } else {
                        0 // No preference, all locations have equal priority
                    };

                    schema::RankedLocation {
                        location_handle: location_handle.clone(),
                        rank,
                    }
                })
                .collect::<Vec<schema::RankedLocation>>();

            schema::Operation::FulfillmentGroupLocationRankingAdd(
                schema::FulfillmentGroupLocationRankingAddOperation {
                    fulfillment_group_handle: group.handle().clone(),
                    rankings,
                },
            )
        })
        .collect();

    Ok(schema::CartFulfillmentGroupsLocationRankingsGenerateRunResult { operations })
}

// Helper function to extract numeric ID from Shopify GID format
// e.g., "gid://shopify/Location/99997811016" -> Some(99997811016)
fn extract_numeric_id_from_gid(gid: &str) -> Option<u64> {
    gid.split('/').last()?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use shopify_function::{run_function_with_input, Result};

    #[test]
    fn test_no_location_preference() -> Result<()> {
        let result = run_function_with_input(
            cart_fulfillment_groups_location_rankings_generate_run,
            r#"
                {
                    "fulfillmentGroups": [{
                        "handle": "123",
                        "inventoryLocationHandles": ["location_handle_1", "location_handle_2"],
                        "lines": [{
                            "id": "gid://shopify/CartLine/1",
                            "attribute": null
                        }]
                    }],
                    "locations": [
                        {
                            "handle": "location_handle_1",
                            "id": "gid://shopify/Location/99997811016"
                        },
                        {
                            "handle": "location_handle_2", 
                            "id": "gid://shopify/Location/99997811017"
                        }
                    ]
                }
            "#,
        )?;
        
        let expected = schema::CartFulfillmentGroupsLocationRankingsGenerateRunResult {
            operations: vec![schema::Operation::FulfillmentGroupLocationRankingAdd(
                schema::FulfillmentGroupLocationRankingAddOperation {
                    fulfillment_group_handle: "123".to_string(),
                    rankings: vec![
                        schema::RankedLocation {
                            location_handle: "location_handle_1".to_string(),
                            rank: 0,
                        },
                        schema::RankedLocation {
                            location_handle: "location_handle_2".to_string(),
                            rank: 0,
                        }
                    ],
                },
            )],
        };

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_location_preference_from_line_item_property() -> Result<()> {
        let result = run_function_with_input(
            cart_fulfillment_groups_location_rankings_generate_run,
            r#"
                {
                    "fulfillmentGroups": [{
                        "handle": "123",
                        "inventoryLocationHandles": ["location_handle_1", "location_handle_2"],
                        "lines": [{
                            "id": "gid://shopify/CartLine/1",
                            "attribute": {
                                "key": "_Location ID",
                                "value": "99997811016"
                            }
                        }]
                    }],
                    "locations": [
                        {
                            "handle": "location_handle_1",
                            "id": "gid://shopify/Location/99997811016"
                        },
                        {
                            "handle": "location_handle_2", 
                            "id": "gid://shopify/Location/99997811017"
                        }
                    ]
                }
            "#,
        )?;
        
        let expected = schema::CartFulfillmentGroupsLocationRankingsGenerateRunResult {
            operations: vec![schema::Operation::FulfillmentGroupLocationRankingAdd(
                schema::FulfillmentGroupLocationRankingAddOperation {
                    fulfillment_group_handle: "123".to_string(),
                    rankings: vec![
                        schema::RankedLocation {
                            location_handle: "location_handle_1".to_string(),
                            rank: 0, // Preferred location gets rank 0
                        },
                        schema::RankedLocation {
                            location_handle: "location_handle_2".to_string(),
                            rank: 1, // Other locations get rank 1
                        }
                    ],
                },
            )],
        };

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_extract_numeric_id_from_gid() {
        assert_eq!(extract_numeric_id_from_gid("gid://shopify/Location/99997811016"), Some(99997811016));
        assert_eq!(extract_numeric_id_from_gid("gid://shopify/Location/123"), Some(123));
        assert_eq!(extract_numeric_id_from_gid("invalid"), None);
        assert_eq!(extract_numeric_id_from_gid("gid://shopify/Location/abc"), None);
    }
}
