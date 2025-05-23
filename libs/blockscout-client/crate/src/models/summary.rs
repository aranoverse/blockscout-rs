/*
 * BlockScout API
 *
 * API for BlockScout web app
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: you@your-company.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Summary {
    /// Summary template
    #[serde(rename = "summary_template")]
    pub summary_template: String,
    #[serde(rename = "summary_template_variables")]
    pub summary_template_variables: models::SummaryTemplateVariables,
}

impl Summary {
    pub fn new(
        summary_template: String,
        summary_template_variables: models::SummaryTemplateVariables,
    ) -> Summary {
        Summary {
            summary_template,
            summary_template_variables,
        }
    }
}
