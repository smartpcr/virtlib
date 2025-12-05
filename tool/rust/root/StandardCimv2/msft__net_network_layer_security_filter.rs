// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetNetworkLayerSecurityFilter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetNetworkLayerSecurityFilter {
    #[serde(flatten)]
    pub base: CIM_FilterEntryBase,

/// 
    #[serde(rename = "Authentication")]
    pub authentication: Option<u16>,

/// 
    #[serde(rename = "Encryption")]
    pub encryption: Option<u16>,

/// 
    #[serde(rename = "LocalUsers")]
    pub local_users: Option<String>,

/// 
    #[serde(rename = "OverrideBlockRules")]
    pub override_block_rules: Option<bool>,

/// 
    #[serde(rename = "RemoteMachines")]
    pub remote_machines: Option<String>,

/// 
    #[serde(rename = "RemoteUsers")]
    pub remote_users: Option<String>,
}

impl MSFT_NetNetworkLayerSecurityFilter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_FilterEntryBase::new(),
            authentication: None,
            encryption: None,
            local_users: None,
            override_block_rules: None,
            remote_machines: None,
            remote_users: None,
        }
    }


    /// Sets the value of Authentication
    pub fn set_authentication(&mut self, value: u16) {
        self.authentication = Some(value);
    }

    /// Gets the value of Authentication
    pub fn get_authentication(&self) -> Option<&u16> {
        self.authentication.as_ref()
    }

    /// Sets the value of Encryption
    pub fn set_encryption(&mut self, value: u16) {
        self.encryption = Some(value);
    }

    /// Gets the value of Encryption
    pub fn get_encryption(&self) -> Option<&u16> {
        self.encryption.as_ref()
    }

    /// Sets the value of LocalUsers
    pub fn set_local_users(&mut self, value: String) {
        self.local_users = Some(value);
    }

    /// Gets the value of LocalUsers
    pub fn get_local_users(&self) -> Option<&String> {
        self.local_users.as_ref()
    }

    /// Sets the value of OverrideBlockRules
    pub fn set_override_block_rules(&mut self, value: bool) {
        self.override_block_rules = Some(value);
    }

    /// Gets the value of OverrideBlockRules
    pub fn get_override_block_rules(&self) -> Option<&bool> {
        self.override_block_rules.as_ref()
    }

    /// Sets the value of RemoteMachines
    pub fn set_remote_machines(&mut self, value: String) {
        self.remote_machines = Some(value);
    }

    /// Gets the value of RemoteMachines
    pub fn get_remote_machines(&self) -> Option<&String> {
        self.remote_machines.as_ref()
    }

    /// Sets the value of RemoteUsers
    pub fn set_remote_users(&mut self, value: String) {
        self.remote_users = Some(value);
    }

    /// Gets the value of RemoteUsers
    pub fn get_remote_users(&self) -> Option<&String> {
        self.remote_users.as_ref()
    }
}

