// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ServiceDebugBehavior struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceDebugBehavior {
    #[serde(flatten)]
    pub base: Behavior,

/// Controls the binding for metadata retrieval using HTTP.
    #[serde(rename = "HttpHelpPageBinding")]
    pub http_help_page_binding: Option<Binding>,

/// Controls whether the service publishes its WSDL at the address controlled by the HttpGetUrl attribute.
    #[serde(rename = "HttpHelpPageEnabled")]
    pub http_help_page_enabled: Option<bool>,

/// Sets the location at which the service WSDL is published for retrieval using HTTP.
    #[serde(rename = "HttpHelpPageUrl")]
    pub http_help_page_url: Option<String>,

/// Controls the binding for metadata retrieval using HTTPS.
    #[serde(rename = "HttpsHelpPageBinding")]
    pub https_help_page_binding: Option<Binding>,

/// Controls whether the service publishes its WSDL over HTTPS at the address controlled by the HttpsGetUrl attribute.
    #[serde(rename = "HttpsHelpPageEnabled")]
    pub https_help_page_enabled: Option<bool>,

/// Sets the location at which the service WSDL is published for retrieval using HTTPS.
    #[serde(rename = "HttpsHelpPageUrl")]
    pub https_help_page_url: Option<String>,

/// Specifies whether to include managed exception information in the detail of SOAP faults returned to the clients for debugging purposes.
    #[serde(rename = "IncludeExceptionDetailInFaults")]
    pub include_exception_detail_in_faults: Option<bool>,
}

impl ServiceDebugBehavior {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            http_help_page_binding: None,
            http_help_page_enabled: None,
            http_help_page_url: None,
            https_help_page_binding: None,
            https_help_page_enabled: None,
            https_help_page_url: None,
            include_exception_detail_in_faults: None,
        }
    }


    /// Sets the value of HttpHelpPageBinding
    pub fn set_http_help_page_binding(&mut self, value: Binding) {
        self.http_help_page_binding = Some(value);
    }

    /// Gets the value of HttpHelpPageBinding
    pub fn get_http_help_page_binding(&self) -> Option<&Binding> {
        self.http_help_page_binding.as_ref()
    }

    /// Sets the value of HttpHelpPageEnabled
    pub fn set_http_help_page_enabled(&mut self, value: bool) {
        self.http_help_page_enabled = Some(value);
    }

    /// Gets the value of HttpHelpPageEnabled
    pub fn get_http_help_page_enabled(&self) -> Option<&bool> {
        self.http_help_page_enabled.as_ref()
    }

    /// Sets the value of HttpHelpPageUrl
    pub fn set_http_help_page_url(&mut self, value: String) {
        self.http_help_page_url = Some(value);
    }

    /// Gets the value of HttpHelpPageUrl
    pub fn get_http_help_page_url(&self) -> Option<&String> {
        self.http_help_page_url.as_ref()
    }

    /// Sets the value of HttpsHelpPageBinding
    pub fn set_https_help_page_binding(&mut self, value: Binding) {
        self.https_help_page_binding = Some(value);
    }

    /// Gets the value of HttpsHelpPageBinding
    pub fn get_https_help_page_binding(&self) -> Option<&Binding> {
        self.https_help_page_binding.as_ref()
    }

    /// Sets the value of HttpsHelpPageEnabled
    pub fn set_https_help_page_enabled(&mut self, value: bool) {
        self.https_help_page_enabled = Some(value);
    }

    /// Gets the value of HttpsHelpPageEnabled
    pub fn get_https_help_page_enabled(&self) -> Option<&bool> {
        self.https_help_page_enabled.as_ref()
    }

    /// Sets the value of HttpsHelpPageUrl
    pub fn set_https_help_page_url(&mut self, value: String) {
        self.https_help_page_url = Some(value);
    }

    /// Gets the value of HttpsHelpPageUrl
    pub fn get_https_help_page_url(&self) -> Option<&String> {
        self.https_help_page_url.as_ref()
    }

    /// Sets the value of IncludeExceptionDetailInFaults
    pub fn set_include_exception_detail_in_faults(&mut self, value: bool) {
        self.include_exception_detail_in_faults = Some(value);
    }

    /// Gets the value of IncludeExceptionDetailInFaults
    pub fn get_include_exception_detail_in_faults(&self) -> Option<&bool> {
        self.include_exception_detail_in_faults.as_ref()
    }
}

