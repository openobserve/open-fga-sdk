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
pub struct CheckRequest {
    #[serde(rename = "tuple_key")]
    pub tuple_key: Box<crate::models::CheckRequestTupleKey>,
    #[serde(rename = "contextual_tuples", skip_serializing_if = "Option::is_none")]
    pub contextual_tuples: Option<Box<crate::models::ContextualTupleKeys>>,
    #[serde(rename = "authorization_model_id", skip_serializing_if = "Option::is_none")]
    pub authorization_model_id: Option<String>,
    /// Defaults to false. Making it true has performance implications.
    #[serde(rename = "trace", skip_serializing_if = "Option::is_none")]
    pub trace: Option<bool>,
    /// Additional request context that will be used to evaluate any ABAC conditions encountered in the query evaluation.
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
}

impl CheckRequest {
    pub fn new(tuple_key: crate::models::CheckRequestTupleKey) -> CheckRequest {
        CheckRequest {
            tuple_key: Box::new(tuple_key),
            contextual_tuples: None,
            authorization_model_id: None,
            trace: None,
            context: None,
        }
    }
}


