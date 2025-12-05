// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ServiceCredentials struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceCredentials {
    #[serde(flatten)]
    pub base: Behavior,

/// The client certificate authentication and provisioning settings for this service.
    #[serde(rename = "ClientCertificate")]
    pub client_certificate: Option<String>,

/// The current issued token authentication settings for this service.
    #[serde(rename = "IssuedTokenAuthentication")]
    pub issued_token_authentication: Option<String>,

/// The current credential authentication and provisioning settings to be used by peer transport endpoints.
    #[serde(rename = "Peer")]
    pub peer: Option<String>,

/// Specifies the current secure conversation settings.
    #[serde(rename = "SecureConversationAuthentication")]
    pub secure_conversation_authentication: Option<String>,

/// The certificate associated with this service.
    #[serde(rename = "ServiceCertificate")]
    pub service_certificate: Option<String>,

/// The username/password settings for this service.
    #[serde(rename = "UserNameAuthentication")]
    pub user_name_authentication: Option<String>,

/// The Windows authentication settings for this service.
    #[serde(rename = "WindowsAuthentication")]
    pub windows_authentication: Option<String>,
}

impl ServiceCredentials {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            client_certificate: None,
            issued_token_authentication: None,
            peer: None,
            secure_conversation_authentication: None,
            service_certificate: None,
            user_name_authentication: None,
            windows_authentication: None,
        }
    }


    /// Sets the value of ClientCertificate
    pub fn set_client_certificate(&mut self, value: String) {
        self.client_certificate = Some(value);
    }

    /// Gets the value of ClientCertificate
    pub fn get_client_certificate(&self) -> Option<&String> {
        self.client_certificate.as_ref()
    }

    /// Sets the value of IssuedTokenAuthentication
    pub fn set_issued_token_authentication(&mut self, value: String) {
        self.issued_token_authentication = Some(value);
    }

    /// Gets the value of IssuedTokenAuthentication
    pub fn get_issued_token_authentication(&self) -> Option<&String> {
        self.issued_token_authentication.as_ref()
    }

    /// Sets the value of Peer
    pub fn set_peer(&mut self, value: String) {
        self.peer = Some(value);
    }

    /// Gets the value of Peer
    pub fn get_peer(&self) -> Option<&String> {
        self.peer.as_ref()
    }

    /// Sets the value of SecureConversationAuthentication
    pub fn set_secure_conversation_authentication(&mut self, value: String) {
        self.secure_conversation_authentication = Some(value);
    }

    /// Gets the value of SecureConversationAuthentication
    pub fn get_secure_conversation_authentication(&self) -> Option<&String> {
        self.secure_conversation_authentication.as_ref()
    }

    /// Sets the value of ServiceCertificate
    pub fn set_service_certificate(&mut self, value: String) {
        self.service_certificate = Some(value);
    }

    /// Gets the value of ServiceCertificate
    pub fn get_service_certificate(&self) -> Option<&String> {
        self.service_certificate.as_ref()
    }

    /// Sets the value of UserNameAuthentication
    pub fn set_user_name_authentication(&mut self, value: String) {
        self.user_name_authentication = Some(value);
    }

    /// Gets the value of UserNameAuthentication
    pub fn get_user_name_authentication(&self) -> Option<&String> {
        self.user_name_authentication.as_ref()
    }

    /// Sets the value of WindowsAuthentication
    pub fn set_windows_authentication(&mut self, value: String) {
        self.windows_authentication = Some(value);
    }

    /// Gets the value of WindowsAuthentication
    pub fn get_windows_authentication(&self) -> Option<&String> {
        self.windows_authentication.as_ref()
    }
}

