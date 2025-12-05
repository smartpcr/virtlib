// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PeerTransportSecuritySettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PeerTransportSecuritySettings {

/// The transport credential type of the peer security element.
    #[serde(rename = "CredentialType")]
    pub credential_type: Option<String>,
}

impl PeerTransportSecuritySettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            credential_type: None,
        }
    }


    /// Sets the value of CredentialType
    pub fn set_credential_type(&mut self, value: String) {
        self.credential_type = Some(value);
    }

    /// Gets the value of CredentialType
    pub fn get_credential_type(&self) -> Option<&String> {
        self.credential_type.as_ref()
    }
}

