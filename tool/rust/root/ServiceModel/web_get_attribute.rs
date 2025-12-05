// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WebGetAttribute struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WebGetAttribute {
    #[serde(flatten)]
    pub base: Behavior,

/// Specifies the web message body style.
    #[serde(rename = "BodyStyle")]
    pub body_style: Option<String>,

/// Specifies whether a body style has been explicitly specified.
    #[serde(rename = "IsBodyStyleSetExplicitly")]
    pub is_body_style_set_explicitly: Option<bool>,

/// Specifies whether the outgoing request's web message format has been explicitly specified.
    #[serde(rename = "IsRequestFormatSetExplicitly")]
    pub is_request_format_set_explicitly: Option<bool>,

/// Specifies whether the outgoing response's web message format has been explicitly specified.
    #[serde(rename = "IsResponseFormatSetExplicitly")]
    pub is_response_format_set_explicitly: Option<bool>,

/// Specifies the outgoing request's web message format.
    #[serde(rename = "RequestFormat")]
    pub request_format: Option<String>,

/// Specifies the outgoing response's web message format.
    #[serde(rename = "ResponseFormat")]
    pub response_format: Option<String>,

/// Specifies the URI template for the request.
    #[serde(rename = "UriTemplate")]
    pub uri_template: Option<String>,
}

impl WebGetAttribute {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            body_style: None,
            is_body_style_set_explicitly: None,
            is_request_format_set_explicitly: None,
            is_response_format_set_explicitly: None,
            request_format: None,
            response_format: None,
            uri_template: None,
        }
    }


    /// Sets the value of BodyStyle
    pub fn set_body_style(&mut self, value: String) {
        self.body_style = Some(value);
    }

    /// Gets the value of BodyStyle
    pub fn get_body_style(&self) -> Option<&String> {
        self.body_style.as_ref()
    }

    /// Sets the value of IsBodyStyleSetExplicitly
    pub fn set_is_body_style_set_explicitly(&mut self, value: bool) {
        self.is_body_style_set_explicitly = Some(value);
    }

    /// Gets the value of IsBodyStyleSetExplicitly
    pub fn get_is_body_style_set_explicitly(&self) -> Option<&bool> {
        self.is_body_style_set_explicitly.as_ref()
    }

    /// Sets the value of IsRequestFormatSetExplicitly
    pub fn set_is_request_format_set_explicitly(&mut self, value: bool) {
        self.is_request_format_set_explicitly = Some(value);
    }

    /// Gets the value of IsRequestFormatSetExplicitly
    pub fn get_is_request_format_set_explicitly(&self) -> Option<&bool> {
        self.is_request_format_set_explicitly.as_ref()
    }

    /// Sets the value of IsResponseFormatSetExplicitly
    pub fn set_is_response_format_set_explicitly(&mut self, value: bool) {
        self.is_response_format_set_explicitly = Some(value);
    }

    /// Gets the value of IsResponseFormatSetExplicitly
    pub fn get_is_response_format_set_explicitly(&self) -> Option<&bool> {
        self.is_response_format_set_explicitly.as_ref()
    }

    /// Sets the value of RequestFormat
    pub fn set_request_format(&mut self, value: String) {
        self.request_format = Some(value);
    }

    /// Gets the value of RequestFormat
    pub fn get_request_format(&self) -> Option<&String> {
        self.request_format.as_ref()
    }

    /// Sets the value of ResponseFormat
    pub fn set_response_format(&mut self, value: String) {
        self.response_format = Some(value);
    }

    /// Gets the value of ResponseFormat
    pub fn get_response_format(&self) -> Option<&String> {
        self.response_format.as_ref()
    }

    /// Sets the value of UriTemplate
    pub fn set_uri_template(&mut self, value: String) {
        self.uri_template = Some(value);
    }

    /// Gets the value of UriTemplate
    pub fn get_uri_template(&self) -> Option<&String> {
        self.uri_template.as_ref()
    }
}

