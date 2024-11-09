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
pub struct FoundationFoodItem {
    #[serde(rename = "fdcId")]
    pub fdc_id: i32,
    #[serde(rename = "dataType")]
    pub data_type: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "foodClass", skip_serializing_if = "Option::is_none")]
    pub food_class: Option<String>,
    #[serde(rename = "footNote", skip_serializing_if = "Option::is_none")]
    pub foot_note: Option<String>,
    #[serde(rename = "isHistoricalReference", skip_serializing_if = "Option::is_none")]
    pub is_historical_reference: Option<bool>,
    #[serde(rename = "ndbNumber", skip_serializing_if = "Option::is_none")]
    pub ndb_number: Option<i32>,
    #[serde(rename = "publicationDate", skip_serializing_if = "Option::is_none")]
    pub publication_date: Option<String>,
    #[serde(rename = "scientificName", skip_serializing_if = "Option::is_none")]
    pub scientific_name: Option<String>,
    #[serde(rename = "foodCategory", skip_serializing_if = "Option::is_none")]
    pub food_category: Option<Box<models::FoodCategory>>,
    #[serde(rename = "foodComponents", skip_serializing_if = "Option::is_none")]
    pub food_components: Option<Vec<models::FoodComponent>>,
    #[serde(rename = "foodNutrients", skip_serializing_if = "Option::is_none")]
    pub food_nutrients: Option<Vec<models::FoodNutrient>>,
    #[serde(rename = "foodPortions", skip_serializing_if = "Option::is_none")]
    pub food_portions: Option<Vec<models::FoodPortion>>,
    #[serde(rename = "inputFoods", skip_serializing_if = "Option::is_none")]
    pub input_foods: Option<Vec<models::InputFoodFoundation>>,
    #[serde(rename = "nutrientConversionFactors", skip_serializing_if = "Option::is_none")]
    pub nutrient_conversion_factors: Option<Vec<models::NutrientConversionFactors>>,
}

impl FoundationFoodItem {
    pub fn new(fdc_id: i32, data_type: String, description: String) -> FoundationFoodItem {
        FoundationFoodItem {
            fdc_id,
            data_type,
            description,
            food_class: None,
            foot_note: None,
            is_historical_reference: None,
            ndb_number: None,
            publication_date: None,
            scientific_name: None,
            food_category: None,
            food_components: None,
            food_nutrients: None,
            food_portions: None,
            input_foods: None,
            nutrient_conversion_factors: None,
        }
    }
}

