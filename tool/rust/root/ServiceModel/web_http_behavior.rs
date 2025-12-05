// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WebHttpBehavior struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WebHttpBehavior {
    #[serde(flatten)]
    pub base: Behavior,

/// Specifies the web message body style.
    #[serde(rename = "DefaultBodyStyle")]
    pub default_body_style: Option<String>,

/// Specifies the default web message format for outgoing requests if the operation does not specify one.
    #[serde(rename = "DefaultOutgoingRequestFormat")]
    pub default_outgoing_request_format: Option<String>,

/// Specifies the default web message format for outgoing responses if the operation does not specify one.
    #[serde(rename = "DefaultOutgoingResponseFormat")]
    pub default_outgoing_response_format: Option<String>,
}

impl WebHttpBehavior {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            default_body_style: None,
            default_outgoing_request_format: None,
            default_outgoing_response_format: None,
        }
    }


    /// Sets the value of DefaultBodyStyle
    pub fn set_default_body_style(&mut self, value: String) {
        self.default_body_style = Some(value);
    }

    /// Gets the value of DefaultBodyStyle
    pub fn get_default_body_style(&self) -> Option<&String> {
        self.default_body_style.as_ref()
    }

    /// Sets the value of DefaultOutgoingRequestFormat
    pub fn set_default_outgoing_request_format(&mut self, value: String) {
        self.default_outgoing_request_format = Some(value);
    }

    /// Gets the value of DefaultOutgoingRequestFormat
    pub fn get_default_outgoing_request_format(&self) -> Option<&String> {
        self.default_outgoing_request_format.as_ref()
    }

    /// Sets the value of DefaultOutgoingResponseFormat
    pub fn set_default_outgoing_response_format(&mut self, value: String) {
        self.default_outgoing_response_format = Some(value);
    }

    /// Gets the value of DefaultOutgoingResponseFormat
    pub fn get_default_outgoing_response_format(&self) -> Option<&String> {
        self.default_outgoing_response_format.as_ref()
    }
}

