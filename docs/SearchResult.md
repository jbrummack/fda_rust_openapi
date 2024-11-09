# SearchResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**food_search_criteria** | Option<[**models::FoodSearchCriteria**](FoodSearchCriteria.md)> |  | [optional]
**total_hits** | Option<**i32**> | The total number of foods found matching the search criteria. | [optional]
**current_page** | Option<**i32**> | The current page of results being returned. | [optional]
**total_pages** | Option<**i32**> | The total number of pages found matching the search criteria. | [optional]
**foods** | Option<[**Vec<models::SearchResultFood>**](SearchResultFood.md)> | The list of foods found matching the search criteria. See Food Fields below. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


