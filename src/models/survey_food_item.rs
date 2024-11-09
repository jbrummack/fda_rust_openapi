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
pub struct SurveyFoodItem {
    #[serde(rename = "fdcId")]
    pub fdc_id: i32,
    #[serde(rename = "datatype", skip_serializing_if = "Option::is_none")]
    pub datatype: Option<String>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "foodClass", skip_serializing_if = "Option::is_none")]
    pub food_class: Option<String>,
    #[serde(rename = "foodCode", skip_serializing_if = "Option::is_none")]
    pub food_code: Option<String>,
    #[serde(rename = "publicationDate", skip_serializing_if = "Option::is_none")]
    pub publication_date: Option<String>,
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "foodAttributes", skip_serializing_if = "Option::is_none")]
    pub food_attributes: Option<Vec<models::FoodAttribute>>,
    #[serde(rename = "foodPortions", skip_serializing_if = "Option::is_none")]
    pub food_portions: Option<Vec<models::FoodPortion>>,
    #[serde(rename = "inputFoods", skip_serializing_if = "Option::is_none")]
    pub input_foods: Option<Vec<models::InputFoodSurvey>>,
    #[serde(rename = "wweiaFoodCategory", skip_serializing_if = "Option::is_none")]
    pub wweia_food_category: Option<Box<models::WweiaFoodCategory>>,
}

impl SurveyFoodItem {
    pub fn new(fdc_id: i32, description: String) -> SurveyFoodItem {
        SurveyFoodItem {
            fdc_id,
            datatype: None,
            description,
            end_date: None,
            food_class: None,
            food_code: None,
            publication_date: None,
            start_date: None,
            food_attributes: None,
            food_portions: None,
            input_foods: None,
            wweia_food_category: None,
        }
    }
}

