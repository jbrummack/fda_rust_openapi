# \FdcApi

All URIs are relative to *https://api.nal.usda.gov/fdc*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_food**](FdcApi.md#get_food) | **GET** /v1/food/{fdcId} | Fetches details for one food item by FDC ID
[**get_foods**](FdcApi.md#get_foods) | **GET** /v1/foods | Fetches details for multiple food items using input FDC IDs
[**get_foods_list**](FdcApi.md#get_foods_list) | **GET** /v1/foods/list | Returns a paged list of foods, in the 'abridged' format
[**get_foods_search**](FdcApi.md#get_foods_search) | **GET** /v1/foods/search | Returns a list of foods that matched search (query) keywords
[**get_json_spec**](FdcApi.md#get_json_spec) | **GET** /v1/json-spec | Returns this documentation in JSON format
[**get_yaml_spec**](FdcApi.md#get_yaml_spec) | **GET** /v1/yaml-spec | Returns this documentation in JSON format
[**post_foods**](FdcApi.md#post_foods) | **POST** /v1/foods | Fetches details for multiple food items using input FDC IDs
[**post_foods_list**](FdcApi.md#post_foods_list) | **POST** /v1/foods/list | Returns a paged list of foods, in the 'abridged' format
[**post_foods_search**](FdcApi.md#post_foods_search) | **POST** /v1/foods/search | Returns a list of foods that matched search (query) keywords



## get_food

> models::InlineResponse200 get_food(fdc_id, format, nutrients)
Fetches details for one food item by FDC ID

Retrieves a single food item by an FDC ID. Optional format and nutrients can be specified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fdc_id** | **String** | FDC id of the food to retrieve | [required] |
**format** | Option<**String**> | Optional. 'abridged' for an abridged set of elements, 'full' for all elements (default). |  |
**nutrients** | Option<[**Vec<i32>**](i32.md)> | Optional. List of up to 25 nutrient numbers. Only the nutrient information for the specified nutrients will be returned. Should be comma separated list (e.g. nutrients=203,204) or repeating parameters (e.g. nutrients=203&nutrients=204). If a food does not have any matching nutrients, the food will be returned with an empty foodNutrients element. |  |

### Return type

[**models::InlineResponse200**](inline_response_200.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_foods

> Vec<models::GetFoods200ResponseInner> get_foods(fdc_ids, format, nutrients)
Fetches details for multiple food items using input FDC IDs

Retrieves a list of food items by a list of up to 20 FDC IDs. Optional format and nutrients can be specified. Invalid FDC ID's or ones that are not found are omitted and an empty set is returned if there are no matches.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fdc_ids** | [**Vec<String>**](String.md) | List of multiple FDC ID's. Should be comma separated list (e.g. fdcIds=534358,373052) or repeating parameters (e.g. fdcIds=534358&fdcIds=373052). | [required] |
**format** | Option<**String**> | Optional. 'abridged' for an abridged set of elements, 'full' for all elements (default). |  |
**nutrients** | Option<[**Vec<i32>**](i32.md)> | Optional. List of up to 25 nutrient numbers. Only the nutrient information for the specified nutrients will be returned. Should be comma separated list (e.g. nutrients=203,204) or repeating parameters (e.g. nutrients=203&nutrients=204). If a food does not have any matching nutrients, the food will be returned with an empty foodNutrients element. |  |

### Return type

[**Vec<models::GetFoods200ResponseInner>**](getFoods_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_foods_list

> Vec<models::AbridgedFoodItem> get_foods_list(data_type, page_size, page_number, sort_by, sort_order)
Returns a paged list of foods, in the 'abridged' format

Retrieves a paged list of foods. Use the pageNumber parameter to page through the entire result set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_type** | Option<[**Vec<String>**](String.md)> | Optional. Filter on a specific data type; specify one or more values in an array. |  |
**page_size** | Option<**i32**> | Optional. Maximum number of results to return for the current page. Default is 50. |  |
**page_number** | Option<**i32**> | Optional. Page number to retrieve. The offset into the overall result set is expressed as (pageNumber * pageSize) |  |
**sort_by** | Option<**String**> | Optional. Specify one of the possible values to sort by that field. Note, dataType.keyword will be dataType and lowercaseDescription.keyword will be description in future releases. |  |
**sort_order** | Option<**String**> | Optional. The sort direction for the results. Only applicable if sortBy is specified. |  |

### Return type

[**Vec<models::AbridgedFoodItem>**](AbridgedFoodItem.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_foods_search

> models::SearchResult get_foods_search(query, data_type, page_size, page_number, sort_by, sort_order, brand_owner)
Returns a list of foods that matched search (query) keywords

Search for foods using keywords. Results can be filtered by dataType and there are options for result page sizes or sorting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | One or more search terms.  The string may include [search operators](https://fdc.nal.usda.gov/help.html#bkmk-2) | [required] |
**data_type** | Option<[**Vec<String>**](String.md)> | Optional. Filter on a specific data type; specify one or more values in an array. |  |
**page_size** | Option<**i32**> | Optional. Maximum number of results to return for the current page. Default is 50. |  |
**page_number** | Option<**i32**> | Optional. Page number to retrieve. The offset into the overall result set is expressed as (pageNumber * pageSize) |  |
**sort_by** | Option<**String**> | Optional. Specify one of the possible values to sort by that field. Note, dataType.keyword will be dataType and lowercaseDescription.keyword will be description in future releases. |  |
**sort_order** | Option<**String**> | Optional. The sort direction for the results. Only applicable if sortBy is specified. |  |
**brand_owner** | Option<**String**> | Optional. Filter results based on the brand owner of the food. Only applies to Branded Foods |  |

### Return type

[**models::SearchResult**](SearchResult.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_json_spec

> get_json_spec()
Returns this documentation in JSON format

The OpenAPI 3.0 specification for the FDC API rendered as JSON (JavaScript Object Notation)

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_yaml_spec

> get_yaml_spec()
Returns this documentation in JSON format

The OpenAPI 3.0 specification for the FDC API rendered as YAML (YAML Ain't Markup Language)

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_foods

> Vec<models::GetFoods200ResponseInner> post_foods(foods_criteria)
Fetches details for multiple food items using input FDC IDs

Retrieves a list of food items by a list of up to 20 FDC IDs. Optional format and nutrients can be specified. Invalid FDC ID's or ones that are not found are omitted and an empty set is returned if there are no matches.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**foods_criteria** | [**FoodsCriteria**](FoodsCriteria.md) |  | [required] |

### Return type

[**Vec<models::GetFoods200ResponseInner>**](getFoods_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_foods_list

> Vec<models::AbridgedFoodItem> post_foods_list(food_list_criteria)
Returns a paged list of foods, in the 'abridged' format

Retrieves a paged list of foods. Use the pageNumber parameter to page through the entire result set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**food_list_criteria** | [**FoodListCriteria**](FoodListCriteria.md) |  | [required] |

### Return type

[**Vec<models::AbridgedFoodItem>**](AbridgedFoodItem.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_foods_search

> models::SearchResult post_foods_search(food_search_criteria)
Returns a list of foods that matched search (query) keywords

Search for foods using keywords. Results can be filtered by dataType and there are options for result page sizes or sorting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**food_search_criteria** | [**FoodSearchCriteria**](FoodSearchCriteria.md) | The query string may also include standard [search operators](https://fdc.nal.usda.gov/help.html#bkmk-2) | [required] |

### Return type

[**models::SearchResult**](SearchResult.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

