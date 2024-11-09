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

/// FoodSearchCriteria : A copy of the criteria that were used in the search.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FoodSearchCriteria {
    /// Search terms to use in the search. The string may also include standard [search operators](https://fdc.nal.usda.gov/help.html#bkmk-2)
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Optional. Filter on a specific data type; specify one or more values in an array.
    #[serde(rename = "dataType", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<Vec<DataType>>,
    /// Optional. Maximum number of results to return for the current page. Default is 50.
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// Optional. Page number to retrieve. The offset into the overall result set is expressed as (pageNumber * pageSize)
    #[serde(rename = "pageNumber", skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// Optional. Specify one of the possible values to sort by that field. Note, dataType.keyword will be dataType and description.keyword will be description in future releases.
    #[serde(rename = "sortBy", skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<SortBy>,
    /// Optional. The sort direction for the results. Only applicable if sortBy is specified.
    #[serde(rename = "sortOrder", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
    /// Optional. Filter results based on the brand owner of the food. Only applies to Branded Foods.
    #[serde(rename = "brandOwner", skip_serializing_if = "Option::is_none")]
    pub brand_owner: Option<String>,
    /// Optional. Filter foods containing any of the specified trade channels.
    #[serde(rename = "tradeChannel", skip_serializing_if = "Option::is_none")]
    pub trade_channel: Option<Vec<TradeChannel>>,
    /// Filter foods published on or after this date. Format: YYYY-MM-DD
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Filter foods published on or before this date. Format: YYYY-MM-DD
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

impl FoodSearchCriteria {
    /// A copy of the criteria that were used in the search.
    pub fn new() -> FoodSearchCriteria {
        FoodSearchCriteria {
            query: None,
            data_type: None,
            page_size: None,
            page_number: None,
            sort_by: None,
            sort_order: None,
            brand_owner: None,
            trade_channel: None,
            start_date: None,
            end_date: None,
        }
    }
}
/// Optional. Filter on a specific data type; specify one or more values in an array.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DataType {
    #[serde(rename = "Branded")]
    Branded,
    #[serde(rename = "Foundation")]
    Foundation,
    #[serde(rename = "Survey (FNDDS)")]
    SurveyLeftParenthesisFnddsRightParenthesis,
    #[serde(rename = "SR Legacy")]
    SrLegacy,
}

impl Default for DataType {
    fn default() -> DataType {
        Self::Branded
    }
}
/// Optional. Specify one of the possible values to sort by that field. Note, dataType.keyword will be dataType and description.keyword will be description in future releases.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SortBy {
    #[serde(rename = "dataType.keyword")]
    DataTypePeriodKeyword,
    #[serde(rename = "lowercaseDescription.keyword")]
    LowercaseDescriptionPeriodKeyword,
    #[serde(rename = "fdcId")]
    FdcId,
    #[serde(rename = "publishedDate")]
    PublishedDate,
}

impl Default for SortBy {
    fn default() -> SortBy {
        Self::DataTypePeriodKeyword
    }
}
/// Optional. The sort direction for the results. Only applicable if sortBy is specified.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

impl Default for SortOrder {
    fn default() -> SortOrder {
        Self::Asc
    }
}
/// Optional. Filter foods containing any of the specified trade channels.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TradeChannel {
    #[serde(rename = "CHILD_NUTRITION_FOOD_PROGRAMS")]
    ChildNutritionFoodPrograms,
    #[serde(rename = "DRUG")]
    Drug,
    #[serde(rename = "FOOD_SERVICE")]
    FoodService,
    #[serde(rename = "GROCERY")]
    Grocery,
    #[serde(rename = "MASS_MERCHANDISING")]
    MassMerchandising,
    #[serde(rename = "MILITARY")]
    Military,
    #[serde(rename = "ONLINE")]
    Online,
    #[serde(rename = "VENDING")]
    Vending,
}

impl Default for TradeChannel {
    fn default() -> TradeChannel {
        Self::ChildNutritionFoodPrograms
    }
}

