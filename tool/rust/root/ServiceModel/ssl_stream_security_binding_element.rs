// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SslStreamSecurityBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SslStreamSecurityBindingElement {
    #[serde(flatten)]
    pub base: BindingElement,

/// Specifies if a client certificate is required for this binding.
    #[serde(rename = "RequireClientCertificate")]
    pub require_client_certificate: Option<bool>,
}

impl SslStreamSecurityBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BindingElement::new(),
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

