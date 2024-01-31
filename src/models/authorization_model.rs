/*
 * OpenFGA
 *
 * A high performance and flexible authorization/permission engine built for developers and inspired by Google Zanzibar.
 *
 * The version of the OpenAPI document: 0.1
 * Contact: community@openfga.dev
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationModel {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "schema_version")]
    pub schema_version: String,
    #[serde(rename = "type_definitions")]
    pub type_definitions: Vec<crate::models::TypeDefinition>,
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<::std::collections::HashMap<String, crate::models::Condition>>,
}

impl AuthorizationModel {
    pub fn new(id: String, schema_version: String, type_definitions: Vec<crate::models::TypeDefinition>) -> AuthorizationModel {
        AuthorizationModel {
            id,
            schema_version,
            type_definitions,
            conditions: None,
        }
    }
}


