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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchResultFood {
    /// Unique ID of the food.
    #[serde(rename = "fdcId")]
    pub fdc_id: i32,
    /// The type of the food data.
    #[serde(rename = "dataType", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    /// The description of the food.
    #[serde(rename = "description")]
    pub description: String,
    /// Any A unique ID identifying the food within FNDDS.
    #[serde(rename = "foodCode", skip_serializing_if = "Option::is_none")]
    pub food_code: Option<String>,
    #[serde(rename = "foodNutrients", skip_serializing_if = "Option::is_none")]
    pub food_nutrients: Option<Vec<models::AbridgedFoodNutrient>>,
    /// Date the item was published to FDC.
    #[serde(rename = "publicationDate", skip_serializing_if = "Option::is_none")]
    pub publication_date: Option<String>,
    /// The scientific name of the food.
    #[serde(rename = "scientificName", skip_serializing_if = "Option::is_none")]
    pub scientific_name: Option<String>,
    /// Brand owner for the food. Only applies to Branded Foods.
    #[serde(rename = "brandOwner", skip_serializing_if = "Option::is_none")]
    pub brand_owner: Option<String>,
    /// GTIN or UPC code identifying the food. Only applies to Branded Foods.
    #[serde(rename = "gtinUpc", skip_serializing_if = "Option::is_none")]
    pub gtin_upc: Option<String>,
    /// The list of ingredients (as it appears on the product label). Only applies to Branded Foods.
    #[serde(rename = "ingredients", skip_serializing_if = "Option::is_none")]
    pub ingredients: Option<String>,
    /// Unique number assigned for foundation foods. Only applies to Foundation and SRLegacy Foods.
    #[serde(rename = "ndbNumber", skip_serializing_if = "Option::is_none")]
    pub ndb_number: Option<i32>,
    /// Any additional descriptions of the food.
    #[serde(rename = "additionalDescriptions", skip_serializing_if = "Option::is_none")]
    pub additional_descriptions: Option<String>,
    /// allHighlightFields
    #[serde(rename = "allHighlightFields", skip_serializing_if = "Option::is_none")]
    pub all_highlight_fields: Option<String>,
    /// Relative score indicating how well the food matches the search criteria.
    #[serde(rename = "score", skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
}

impl SearchResultFood {
    pub fn new(fdc_id: i32, description: String) -> SearchResultFood {
        SearchResultFood {
            fdc_id,
            data_type: None,
            description,
            food_code: None,
            food_nutrients: None,
            publication_date: None,
            scientific_name: None,
            brand_owner: None,
            gtin_upc: None,
            ingredients: None,
            ndb_number: None,
            additional_descriptions: None,
            all_highlight_fields: None,
            score: None,
        }
    }
}

