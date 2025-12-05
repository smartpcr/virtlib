// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfiguration
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_WebReportManager struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_WebReportManager {
    #[serde(flatten)]
    pub base: OMI_ReportManager,

/// 
    #[serde(rename = "AllowUnsecureConnection")]
    pub allow_unsecure_connection: Option<bool>,

/// 
    #[serde(rename = "CertificateID")]
    pub certificate_id: Option<String>,

/// 
    #[serde(rename = "ProxyCredential")]
    pub proxy_credential: Option<MSFT_Credential>,

/// 
    #[serde(rename = "ProxyURL")]
    pub proxy_url: Option<String>,

/// 
    #[serde(rename = "RegistrationKey")]
    pub registration_key: Option<String>,

/// 
    #[serde(rename = "ServerURL")]
    pub server_url: Option<String>,
}

impl MSFT_WebReportManager {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: OMI_ReportManager::new(),
            allow_unsecure_connection: None,
            certificate_id: None,
            proxy_credential: None,
            proxy_url: None,
            registration_key: None,
            server_url: None,
        }
    }


    /// Sets the value of AllowUnsecureConnection
    pub fn set_allow_unsecure_connection(&mut self, value: bool) {
        self.allow_unsecure_connection = Some(value);
    }

    /// Gets the value of AllowUnsecureConnection
    pub fn get_allow_unsecure_connection(&self) -> Option<&bool> {
        self.allow_unsecure_connection.as_ref()
    }

    /// Sets the value of CertificateID
    pub fn set_certificate_id(&mut self, value: String) {
        self.certificate_id = Some(value);
    }

    /// Gets the value of CertificateID
    pub fn get_certificate_id(&self) -> Option<&String> {
        self.certificate_id.as_ref()
    }

    /// Sets the value of ProxyCredential
    pub fn set_proxy_credential(&mut self, value: MSFT_Credential) {
        self.proxy_credential = Some(value);
    }

    /// Gets the value of ProxyCredential
    pub fn get_proxy_credential(&self) -> Option<&MSFT_Credential> {
        self.proxy_credential.as_ref()
    }

    /// Sets the value of ProxyURL
    pub fn set_proxy_url(&mut self, value: String) {
        self.proxy_url = Some(value);
    }

    /// Gets the value of ProxyURL
    pub fn get_proxy_url(&self) -> Option<&String> {
        self.proxy_url.as_ref()
    }

    /// Sets the value of RegistrationKey
    pub fn set_registration_key(&mut self, value: String) {
        self.registration_key = Some(value);
    }

    /// Gets the value of RegistrationKey
    pub fn get_registration_key(&self) -> Option<&String> {
        self.registration_key.as_ref()
    }

    /// Sets the value of ServerURL
    pub fn set_server_url(&mut self, value: String) {
        self.server_url = Some(value);
    }

    /// Gets the value of ServerURL
    pub fn get_server_url(&self) -> Option<&String> {
        self.server_url.as_ref()
    }
}

