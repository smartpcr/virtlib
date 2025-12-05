// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ClientCredentials struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClientCredentials {
    #[serde(flatten)]
    pub base: Behavior,

/// The X.509 certificate the client uses to authenticate to the service.
    #[serde(rename = "ClientCertificate")]
    pub client_certificate: Option<String>,

/// The current Http Digest credential.
    #[serde(rename = "HttpDigest")]
    pub http_digest: Option<String>,

/// The endpoint address and binding used to contact the locat security token service.
    #[serde(rename = "IssuedToken")]
    pub issued_token: Option<String>,

/// The credentials peer node uses to authenticate itself to other nodes in the mesh.
    #[serde(rename = "Peer")]
    pub peer: Option<String>,

/// The service's x.509 certificate.
    #[serde(rename = "ServiceCertificate")]
    pub service_certificate: Option<String>,

/// A Boolean value that specifies if the credential supports interactive negotiation.
    #[serde(rename = "SupportInteractive")]
    pub support_interactive: Option<bool>,

/// The username and password the client will use to authenticate itself to the service.
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,

/// The windows credentials the client will use to authenticate itself to the service.
    #[serde(rename = "Windows")]
    pub windows: Option<String>,
}

impl ClientCredentials {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            client_certificate: None,
            http_digest: None,
            issued_token: None,
            peer: None,
            service_certificate: None,
            support_interactive: None,
            user_name: None,
            windows: None,
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

    /// Sets the value of HttpDigest
    pub fn set_http_digest(&mut self, value: String) {
        self.http_digest = Some(value);
    }

    /// Gets the value of HttpDigest
    pub fn get_http_digest(&self) -> Option<&String> {
        self.http_digest.as_ref()
    }

    /// Sets the value of IssuedToken
    pub fn set_issued_token(&mut self, value: String) {
        self.issued_token = Some(value);
    }

    /// Gets the value of IssuedToken
    pub fn get_issued_token(&self) -> Option<&String> {
        self.issued_token.as_ref()
    }

    /// Sets the value of Peer
    pub fn set_peer(&mut self, value: String) {
        self.peer = Some(value);
    }

    /// Gets the value of Peer
    pub fn get_peer(&self) -> Option<&String> {
        self.peer.as_ref()
    }

    /// Sets the value of ServiceCertificate
    pub fn set_service_certificate(&mut self, value: String) {
        self.service_certificate = Some(value);
    }

    /// Gets the value of ServiceCertificate
    pub fn get_service_certificate(&self) -> Option<&String> {
        self.service_certificate.as_ref()
    }

    /// Sets the value of SupportInteractive
    pub fn set_support_interactive(&mut self, value: bool) {
        self.support_interactive = Some(value);
    }

    /// Gets the value of SupportInteractive
    pub fn get_support_interactive(&self) -> Option<&bool> {
        self.support_interactive.as_ref()
    }

    /// Sets the value of UserName
    pub fn set_user_name(&mut self, value: String) {
        self.user_name = Some(value);
    }

    /// Gets the value of UserName
    pub fn get_user_name(&self) -> Option<&String> {
        self.user_name.as_ref()
    }

    /// Sets the value of Windows
    pub fn set_windows(&mut self, value: String) {
        self.windows = Some(value);
    }

    /// Gets the value of Windows
    pub fn get_windows(&self) -> Option<&String> {
        self.windows.as_ref()
    }
}

