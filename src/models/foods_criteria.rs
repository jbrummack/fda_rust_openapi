/*
 * Food Data Central API
 *
 * The FoodData Central API provides REST access to FoodData Central (FDC). It is intended primarily to assist application developers wishing to incorporate nutrient data into their applications or websites.   To take full advantage of the API, developers should familiarize themselves with the database by reading the database documentation available via links on [Data Type Documentation](https://fdc.nal.usda.gov/data-documentation.html). This documentation provides the detailed definitions and descriptions needed to understand the data elements referenced in the API documentation.      Additional details about the API including rate limits, access, and licensing are available on the [FDC website](https://fdc.nal.usda.gov/api-guide.html)
 *
 * The version of the OpenAPI document: 1.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// FoodsCriteria : JSON for request body of 'foods' POST request. Retrieves a list of food items by a list of up to 20 FDC IDs. Optional format and nutrients can be specified. Invalid FDC ID's or ones that are not found are omitted and an empty set is returned if there are no matches.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FoodsCriteria {
    /// List of multiple FDC ID's
    #[serde(rename = "fdcIds", skip_serializing_if = "Option::is_none")]
    pub fdc_ids: Option<Vec<i32>>,
    /// Optional. 'abridged' for an abridged set of elements, 'full' for all elements (default).
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<Format>,
    /// Optional. List of up to 25 nutrient numbers. Only the nutrient information for the specified nutrients will be returned.  If a food does not have any matching nutrients, the food will be returned with an empty foodNutrients element.
    #[serde(rename = "nutrients", skip_serializing_if = "Option::is_none")]
    pub nutrients: Option<Vec<i32>>,
}

impl FoodsCriteria {
    /// JSON for request body of 'foods' POST request. Retrieves a list of food items by a list of up to 20 FDC IDs. Optional format and nutrients can be specified. Invalid FDC ID's or ones that are not found are omitted and an empty set is returned if there are no matches.
    pub fn new() -> FoodsCriteria {
        FoodsCriteria {
            fdc_ids: None,
            format: None,
            nutrients: None,
        }
    }
}
/// Optional. 'abridged' for an abridged set of elements, 'full' for all elements (default).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "abridged")]
    Abridged,
    #[serde(rename = "full")]
    Full,
}

impl Default for Format {
    fn default() -> Format {
        Self::Abridged
    }
}

