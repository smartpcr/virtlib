// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HttpsTransportBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HttpsTransportBindingElement {
    #[serde(flatten)]
    pub base: HttpTransportBindingElement,

/// A value that indicates whether SSL client authentication is required.
    #[serde(rename = "RequireClientCertificate")]
    pub require_client_certificate: Option<bool>,
}

impl HttpsTransportBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: HttpTransportBindingElement::new(),
            require_client_certificate: None,
        }
    }


    /// Sets the value of RequireClientCertificate
    pub fn set_require_client_certificate(&mut self, value: bool) {
        self.require_client_certificate = Some(value);
    }

    /// Gets the value of RequireClientCertificate
    pub fn get_require_client_certificate(&self) -> Option<&bool> {
        self.require_client_certificate.as_ref()
    }
}

