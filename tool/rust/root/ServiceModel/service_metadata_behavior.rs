// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ServiceMetadataBehavior struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceMetadataBehavior {
    #[serde(flatten)]
    pub base: Behavior,

/// Sets the location to which the service redirects metadata requests.
    #[serde(rename = "ExternalMetadataLocation")]
    pub external_metadata_location: Option<String>,

/// Controls the binding for metadata retrieval using HTTP.
    #[serde(rename = "HttpGetBinding")]
    pub http_get_binding: Option<Binding>,

/// Controls whether the service publishes its WSDL at the address controlled by the HttpGetUrl attribute.
    #[serde(rename = "HttpGetEnabled")]
    pub http_get_enabled: Option<bool>,

/// Sets the location at which the service WSDL is published for retrieval using HTTP.
    #[serde(rename = "HttpGetUrl")]
    pub http_get_url: Option<String>,

/// Controls the binding for metadata retrieval using HTTPS.
    #[serde(rename = "HttpsGetBinding")]
    pub https_get_binding: Option<Binding>,

/// Controls whether the service publishes its WSDL over HTTPS at the address controlled by the HttpsGetUrl attribute.
    #[serde(rename = "HttpsGetEnabled")]
    pub https_get_enabled: Option<bool>,

/// Sets the location at which the service WSDL is published for retrieval using HTTPS.
    #[serde(rename = "HttpsGetUrl")]
    pub https_get_url: Option<String>,

/// The component responsible for metadata generation associated with this service.
    #[serde(rename = "MetadataExportInfo")]
    pub metadata_export_info: Option<MetadataExporter>,
}

impl ServiceMetadataBehavior {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            external_metadata_location: None,
            http_get_binding: None,
            http_get_enabled: None,
            http_get_url: None,
            https_get_binding: None,
            https_get_enabled: None,
            https_get_url: None,
            metadata_export_info: None,
        }
    }


    /// Sets the value of ExternalMetadataLocation
    pub fn set_external_metadata_location(&mut self, value: String) {
        self.external_metadata_location = Some(value);
    }

    /// Gets the value of ExternalMetadataLocation
    pub fn get_external_metadata_location(&self) -> Option<&String> {
        self.external_metadata_location.as_ref()
    }

    /// Sets the value of HttpGetBinding
    pub fn set_http_get_binding(&mut self, value: Binding) {
        self.http_get_binding = Some(value);
    }

    /// Gets the value of HttpGetBinding
    pub fn get_http_get_binding(&self) -> Option<&Binding> {
        self.http_get_binding.as_ref()
    }

    /// Sets the value of HttpGetEnabled
    pub fn set_http_get_enabled(&mut self, value: bool) {
        self.http_get_enabled = Some(value);
    }

    /// Gets the value of HttpGetEnabled
    pub fn get_http_get_enabled(&self) -> Option<&bool> {
        self.http_get_enabled.as_ref()
    }

    /// Sets the value of HttpGetUrl
    pub fn set_http_get_url(&mut self, value: String) {
        self.http_get_url = Some(value);
    }

    /// Gets the value of HttpGetUrl
    pub fn get_http_get_url(&self) -> Option<&String> {
        self.http_get_url.as_ref()
    }

    /// Sets the value of HttpsGetBinding
    pub fn set_https_get_binding(&mut self, value: Binding) {
        self.https_get_binding = Some(value);
    }

    /// Gets the value of HttpsGetBinding
    pub fn get_https_get_binding(&self) -> Option<&Binding> {
        self.https_get_binding.as_ref()
    }

    /// Sets the value of HttpsGetEnabled
    pub fn set_https_get_enabled(&mut self, value: bool) {
        self.https_get_enabled = Some(value);
    }

    /// Gets the value of HttpsGetEnabled
    pub fn get_https_get_enabled(&self) -> Option<&bool> {
        self.https_get_enabled.as_ref()
    }

    /// Sets the value of HttpsGetUrl
    pub fn set_https_get_url(&mut self, value: String) {
        self.https_get_url = Some(value);
    }

    /// Gets the value of HttpsGetUrl
    pub fn get_https_get_url(&self) -> Option<&String> {
        self.https_get_url.as_ref()
    }

    /// Sets the value of MetadataExportInfo
    pub fn set_metadata_export_info(&mut self, value: MetadataExporter) {
        self.metadata_export_info = Some(value);
    }

    /// Gets the value of MetadataExportInfo
    pub fn get_metadata_export_info(&self) -> Option<&MetadataExporter> {
        self.metadata_export_info.as_ref()
    }
}

