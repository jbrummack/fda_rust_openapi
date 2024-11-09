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
pub struct BrandedFoodItemLabelNutrientsSugars {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f32>,
}

impl BrandedFoodItemLabelNutrientsSugars {
    pub fn new() -> BrandedFoodItemLabelNutrientsSugars {
        BrandedFoodItemLabelNutrientsSugars {
            value: None,
        }
    }
}

