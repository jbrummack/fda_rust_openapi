# SearchResultFood

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fdc_id** | **i32** | Unique ID of the food. | 
**data_type** | Option<**String**> | The type of the food data. | [optional]
**description** | **String** | The description of the food. | 
**food_code** | Option<**String**> | Any A unique ID identifying the food within FNDDS. | [optional]
**food_nutrients** | Option<[**Vec<models::AbridgedFoodNutrient>**](AbridgedFoodNutrient.md)> |  | [optional]
**publication_date** | Option<**String**> | Date the item was published to FDC. | [optional]
**scientific_name** | Option<**String**> | The scientific name of the food. | [optional]
**brand_owner** | Option<**String**> | Brand owner for the food. Only applies to Branded Foods. | [optional]
**gtin_upc** | Option<**String**> | GTIN or UPC code identifying the food. Only applies to Branded Foods. | [optional]
**ingredients** | Option<**String**> | The list of ingredients (as it appears on the product label). Only applies to Branded Foods. | [optional]
**ndb_number** | Option<**i32**> | Unique number assigned for foundation foods. Only applies to Foundation and SRLegacy Foods. | [optional]
**additional_descriptions** | Option<**String**> | Any additional descriptions of the food. | [optional]
**all_highlight_fields** | Option<**String**> | allHighlightFields | [optional]
**score** | Option<**f32**> | Relative score indicating how well the food matches the search criteria. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


