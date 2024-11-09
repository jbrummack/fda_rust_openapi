# FoodListCriteria

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data_type** | Option<**Vec<String>**> | Optional. Filter on a specific data type; specify one or more values in an array. | [optional]
**page_size** | Option<**i32**> | Optional. Maximum number of results to return for the current page. Default is 50. | [optional]
**page_number** | Option<**i32**> | Optional. Page number to retrieve. The offset into the overall result set is expressed as (pageNumber * pageSize) | [optional]
**sort_by** | Option<**String**> | Optional. Specify one of the possible values to sort by that field. Note, dataType.keyword will be dataType and lowercaseDescription.keyword will be description in future releases. | [optional]
**sort_order** | Option<**String**> | Optional. The sort direction for the results. Only applicable if sortBy is specified. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


