use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;
use std::collections::HashMap;

#[shopify_function]
fn cart_fulfillment_constraints_generate_run(
    input: schema::cart_fulfillment_constraints_generate_run::Input,
) -> Result<schema::CartFulfillmentConstraintsGenerateRunResult> {
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

    let mut operations = vec![];
    
    // Group deliverable lines by their target location
    let mut location_groups: HashMap<String, Vec<String>> = HashMap::new();
    
    for line in input.cart().deliverable_lines() {
        if let Some(attribute) = line.attribute() {
            if attribute.key() == "_Location ID" {
                if let Some(location_id_str) = attribute.value() {
                    // Try to parse the location ID and find the corresponding handle
                    if let Ok(location_id) = location_id_str.parse::<u64>() {
                        if let Some(handle) = location_id_to_handle.get(&location_id) {
                            location_groups
                                .entry(handle.clone())
                                .or_insert_with(Vec::new)
                                .push(line.id().clone());
                        }
                    }
                }
            }
        }
    }
    
    // Create constraints to force each group to be fulfilled from the same location
    for (_location_handle, line_ids) in location_groups {
        if line_ids.len() > 0 {
            operations.push(schema::Operation::DeliverableLinesMustFulfillFromSameLocationAdd(
                schema::DeliverableLinesMustFulfillFromSameLocationAddOperation {
                    deliverable_line_ids: Some(line_ids),
                }
            ));
        }
    }

    Ok(schema::CartFulfillmentConstraintsGenerateRunResult { operations })
}

// Helper function to extract numeric ID from Shopify GID format
fn extract_numeric_id_from_gid(gid: &str) -> Option<u64> {
    gid.split('/').last()?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use shopify_function::{run_function_with_input, Result};

    #[test]
    fn test_no_location_properties() -> Result<()> {
        let result = run_function_with_input(
            cart_fulfillment_constraints_generate_run,
            r#"
              {
                "cart": {
                  "deliverableLines": [
                    {
                      "id": "gid://shopify/CartLine/1",
                      "attribute": null
                    },
                    {
                      "id": "gid://shopify/CartLine/2", 
                      "attribute": null
                    }
                  ]
                },
                "locations": [
                  {
                    "handle": "108050547019",
                    "id": "gid://shopify/Location/108050547019"
                  }
                ]
              }
            "#,
        )?;

        let expected = schema::CartFulfillmentConstraintsGenerateRunResult { operations: vec![] };

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_force_location_constraint() -> Result<()> {
        let result = run_function_with_input(
            cart_fulfillment_constraints_generate_run,
            r#"
              {
                "cart": {
                  "deliverableLines": [
                    {
                      "id": "gid://shopify/CartLine/1",
                      "attribute": {
                        "key": "_Location ID",
                        "value": "108050547019"
                      }
                    },
                    {
                      "id": "gid://shopify/CartLine/2",
                      "attribute": {
                        "key": "_Location ID", 
                        "value": "108050547019"
                      }
                    }
                  ]
                },
                "locations": [
                  {
                    "handle": "108050547019",
                    "id": "gid://shopify/Location/108050547019"
                  },
                  {
                    "handle": "108050645323",
                    "id": "gid://shopify/Location/108050645323"
                  }
                ]
              }
            "#,
        )?;

        assert_eq!(result.operations.len(), 1);
        Ok(())
    }
}
