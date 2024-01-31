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
pub struct WriteAssertionsRequest {
    #[serde(rename = "assertions")]
    pub assertions: Vec<crate::models::Assertion>,
}

impl WriteAssertionsRequest {
    pub fn new(assertions: Vec<crate::models::Assertion>) -> WriteAssertionsRequest {
        WriteAssertionsRequest {
            assertions,
        }
    }
}


