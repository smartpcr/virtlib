// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SecurityBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SecurityBindingElement {
    #[serde(flatten)]
    pub base: BindingElement,

/// A boolean value that specifies if security should be enabled with insecure transports.
    #[serde(rename = "AllowInsecureTransport")]
    pub allow_insecure_transport: Option<bool>,

/// Specifies the algorithms to use with the binding.
    #[serde(rename = "DefaultAlgorithmSuite")]
    pub default_algorithm_suite: Option<String>,

/// A boolean value that specifies if the response can be unsecured.
    #[serde(rename = "EnableUnsecuredResponse")]
    pub enable_unsecured_response: Option<bool>,

/// A boolean value that specifies if each message contains a timestamp.
    #[serde(rename = "IncludeTimestamp")]
    pub include_timestamp: Option<bool>,

/// The source of entropy used to create keys.
    #[serde(rename = "KeyEntropyMode")]
    pub key_entropy_mode: Option<String>,

/// The binding specific security properties for the local service.
    #[serde(rename = "LocalServiceSecuritySettings")]
    pub local_service_security_settings: Option<LocalServiceSecuritySettings>,

/// The version used for message security.
    #[serde(rename = "MessageSecurityVersion")]
    pub message_security_version: Option<String>,

/// The order of elements in the security header for this binding.
    #[serde(rename = "SecurityHeaderLayout")]
    pub security_header_layout: Option<String>,
}

impl SecurityBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BindingElement::new(),
            allow_insecure_transport: None,
            default_algorithm_suite: None,
            enable_unsecured_response: None,
            include_timestamp: None,
            key_entropy_mode: None,
            local_service_security_settings: None,
            message_security_version: None,
            security_header_layout: None,
        }
    }


    /// Sets the value of AllowInsecureTransport
    pub fn set_allow_insecure_transport(&mut self, value: bool) {
        self.allow_insecure_transport = Some(value);
    }

    /// Gets the value of AllowInsecureTransport
    pub fn get_allow_insecure_transport(&self) -> Option<&bool> {
        self.allow_insecure_transport.as_ref()
    }

    /// Sets the value of DefaultAlgorithmSuite
    pub fn set_default_algorithm_suite(&mut self, value: String) {
        self.default_algorithm_suite = Some(value);
    }

    /// Gets the value of DefaultAlgorithmSuite
    pub fn get_default_algorithm_suite(&self) -> Option<&String> {
        self.default_algorithm_suite.as_ref()
    }

    /// Sets the value of EnableUnsecuredResponse
    pub fn set_enable_unsecured_response(&mut self, value: bool) {
        self.enable_unsecured_response = Some(value);
    }

    /// Gets the value of EnableUnsecuredResponse
    pub fn get_enable_unsecured_response(&self) -> Option<&bool> {
        self.enable_unsecured_response.as_ref()
    }

    /// Sets the value of IncludeTimestamp
    pub fn set_include_timestamp(&mut self, value: bool) {
        self.include_timestamp = Some(value);
    }

    /// Gets the value of IncludeTimestamp
    pub fn get_include_timestamp(&self) -> Option<&bool> {
        self.include_timestamp.as_ref()
    }

    /// Sets the value of KeyEntropyMode
    pub fn set_key_entropy_mode(&mut self, value: String) {
        self.key_entropy_mode = Some(value);
    }

    /// Gets the value of KeyEntropyMode
    pub fn get_key_entropy_mode(&self) -> Option<&String> {
        self.key_entropy_mode.as_ref()
    }

    /// Sets the value of LocalServiceSecuritySettings
    pub fn set_local_service_security_settings(&mut self, value: LocalServiceSecuritySettings) {
        self.local_service_security_settings = Some(value);
    }

    /// Gets the value of LocalServiceSecuritySettings
    pub fn get_local_service_security_settings(&self) -> Option<&LocalServiceSecuritySettings> {
        self.local_service_security_settings.as_ref()
    }

    /// Sets the value of MessageSecurityVersion
    pub fn set_message_security_version(&mut self, value: String) {
        self.message_security_version = Some(value);
    }

    /// Gets the value of MessageSecurityVersion
    pub fn get_message_security_version(&self) -> Option<&String> {
        self.message_security_version.as_ref()
    }

    /// Sets the value of SecurityHeaderLayout
    pub fn set_security_header_layout(&mut self, value: String) {
        self.security_header_layout = Some(value);
    }

    /// Gets the value of SecurityHeaderLayout
    pub fn get_security_header_layout(&self) -> Option<&String> {
        self.security_header_layout.as_ref()
    }
}

