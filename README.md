# Rust API client for openapi

A high performance and flexible authorization/permission engine built for developers and inspired by Google Zanzibar.

For more information, please visit [https://openfga.dev](https://openfga.dev)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.1
- Package version: 0.1
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AssertionsApi* | [**read_assertions**](docs/AssertionsApi.md#read_assertions) | **GET** /stores/{store_id}/assertions/{authorization_model_id} | Read assertions for an authorization model ID
*AssertionsApi* | [**write_assertions**](docs/AssertionsApi.md#write_assertions) | **PUT** /stores/{store_id}/assertions/{authorization_model_id} | Upsert assertions for an authorization model ID
*AuthorizationModelsApi* | [**read_authorization_model**](docs/AuthorizationModelsApi.md#read_authorization_model) | **GET** /stores/{store_id}/authorization-models/{id} | Return a particular version of an authorization model
*AuthorizationModelsApi* | [**read_authorization_models**](docs/AuthorizationModelsApi.md#read_authorization_models) | **GET** /stores/{store_id}/authorization-models | Return all the authorization models for a particular store
*AuthorizationModelsApi* | [**write_authorization_model**](docs/AuthorizationModelsApi.md#write_authorization_model) | **POST** /stores/{store_id}/authorization-models | Create a new authorization model
*RelationshipQueriesApi* | [**check**](docs/RelationshipQueriesApi.md#check) | **POST** /stores/{store_id}/check | Check whether a user is authorized to access an object
*RelationshipQueriesApi* | [**expand**](docs/RelationshipQueriesApi.md#expand) | **POST** /stores/{store_id}/expand | Expand all relationships in userset tree format, and following userset rewrite rules.  Useful to reason about and debug a certain relationship
*RelationshipQueriesApi* | [**list_objects**](docs/RelationshipQueriesApi.md#list_objects) | **POST** /stores/{store_id}/list-objects | List all objects of the given type that the user has a relation with
*RelationshipQueriesApi* | [**streamed_list_objects**](docs/RelationshipQueriesApi.md#streamed_list_objects) | **POST** /stores/{store_id}/streamed-list-objects | [EXPERIMENTAL] Stream all objects of the given type that the user has a relation with
*RelationshipTuplesApi* | [**read**](docs/RelationshipTuplesApi.md#read) | **POST** /stores/{store_id}/read | Get tuples from the store that matches a query, without following userset rewrite rules
*RelationshipTuplesApi* | [**read_changes**](docs/RelationshipTuplesApi.md#read_changes) | **GET** /stores/{store_id}/changes | Return a list of all the tuple changes
*RelationshipTuplesApi* | [**write**](docs/RelationshipTuplesApi.md#write) | **POST** /stores/{store_id}/write | Add or delete tuples from the store
*StoresApi* | [**create_store**](docs/StoresApi.md#create_store) | **POST** /stores | Create a store
*StoresApi* | [**delete_store**](docs/StoresApi.md#delete_store) | **DELETE** /stores/{store_id} | Delete a store
*StoresApi* | [**get_store**](docs/StoresApi.md#get_store) | **GET** /stores/{store_id} | Get a store
*StoresApi* | [**list_stores**](docs/StoresApi.md#list_stores) | **GET** /stores | List all stores


## Documentation For Models

 - [AbortedMessageResponse](docs/AbortedMessageResponse.md)
 - [Any](docs/Any.md)
 - [Assertion](docs/Assertion.md)
 - [AssertionTupleKey](docs/AssertionTupleKey.md)
 - [AuthorizationModel](docs/AuthorizationModel.md)
 - [CheckRequest](docs/CheckRequest.md)
 - [CheckRequestTupleKey](docs/CheckRequestTupleKey.md)
 - [CheckResponse](docs/CheckResponse.md)
 - [Computed](docs/Computed.md)
 - [Condition](docs/Condition.md)
 - [ConditionParamTypeRef](docs/ConditionParamTypeRef.md)
 - [ContextualTupleKeys](docs/ContextualTupleKeys.md)
 - [CreateStoreRequest](docs/CreateStoreRequest.md)
 - [CreateStoreResponse](docs/CreateStoreResponse.md)
 - [ErrorCode](docs/ErrorCode.md)
 - [ExpandRequest](docs/ExpandRequest.md)
 - [ExpandRequestTupleKey](docs/ExpandRequestTupleKey.md)
 - [ExpandResponse](docs/ExpandResponse.md)
 - [GetStoreResponse](docs/GetStoreResponse.md)
 - [InternalErrorCode](docs/InternalErrorCode.md)
 - [InternalErrorMessageResponse](docs/InternalErrorMessageResponse.md)
 - [Leaf](docs/Leaf.md)
 - [ListObjectsRequest](docs/ListObjectsRequest.md)
 - [ListObjectsResponse](docs/ListObjectsResponse.md)
 - [ListStoresResponse](docs/ListStoresResponse.md)
 - [Metadata](docs/Metadata.md)
 - [Node](docs/Node.md)
 - [Nodes](docs/Nodes.md)
 - [NotFoundErrorCode](docs/NotFoundErrorCode.md)
 - [NullValue](docs/NullValue.md)
 - [ObjectRelation](docs/ObjectRelation.md)
 - [PathUnknownErrorMessageResponse](docs/PathUnknownErrorMessageResponse.md)
 - [ReadAssertionsResponse](docs/ReadAssertionsResponse.md)
 - [ReadAuthorizationModelResponse](docs/ReadAuthorizationModelResponse.md)
 - [ReadAuthorizationModelsResponse](docs/ReadAuthorizationModelsResponse.md)
 - [ReadChangesResponse](docs/ReadChangesResponse.md)
 - [ReadRequest](docs/ReadRequest.md)
 - [ReadRequestTupleKey](docs/ReadRequestTupleKey.md)
 - [ReadResponse](docs/ReadResponse.md)
 - [RelationMetadata](docs/RelationMetadata.md)
 - [RelationReference](docs/RelationReference.md)
 - [RelationshipCondition](docs/RelationshipCondition.md)
 - [Status](docs/Status.md)
 - [Store](docs/Store.md)
 - [StreamResultOfStreamedListObjectsResponse](docs/StreamResultOfStreamedListObjectsResponse.md)
 - [StreamedListObjectsResponse](docs/StreamedListObjectsResponse.md)
 - [Tuple](docs/Tuple.md)
 - [TupleChange](docs/TupleChange.md)
 - [TupleKey](docs/TupleKey.md)
 - [TupleKeyWithoutCondition](docs/TupleKeyWithoutCondition.md)
 - [TupleOperation](docs/TupleOperation.md)
 - [TypeDefinition](docs/TypeDefinition.md)
 - [TypeName](docs/TypeName.md)
 - [Users](docs/Users.md)
 - [Userset](docs/Userset.md)
 - [UsersetTree](docs/UsersetTree.md)
 - [UsersetTreePeriodDifference](docs/UsersetTreePeriodDifference.md)
 - [UsersetTreePeriodTupleToUserset](docs/UsersetTreePeriodTupleToUserset.md)
 - [Usersets](docs/Usersets.md)
 - [V1PeriodDifference](docs/V1PeriodDifference.md)
 - [V1PeriodTupleToUserset](docs/V1PeriodTupleToUserset.md)
 - [ValidationErrorMessageResponse](docs/ValidationErrorMessageResponse.md)
 - [WriteAssertionsRequest](docs/WriteAssertionsRequest.md)
 - [WriteAuthorizationModelRequest](docs/WriteAuthorizationModelRequest.md)
 - [WriteAuthorizationModelResponse](docs/WriteAuthorizationModelResponse.md)
 - [WriteRequest](docs/WriteRequest.md)
 - [WriteRequestDeletes](docs/WriteRequestDeletes.md)
 - [WriteRequestWrites](docs/WriteRequestWrites.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

community@openfga.dev

