// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_ReplicationAuthorizationSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_ReplicationAuthorizationSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "AllowedPrimaryHostSystem")]
    pub allowed_primary_host_system: Option<String>,

/// 
    #[serde(rename = "ReplicaStorageLocation")]
    pub replica_storage_location: Option<String>,

/// 
    #[serde(rename = "TrustGroup")]
    pub trust_group: Option<String>,
}

impl Msvm_ReplicationAuthorizationSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            allowed_primary_host_system: None,
            replica_storage_location: None,
            trust_group: None,
        }
    }


    /// Sets the value of AllowedPrimaryHostSystem
    pub fn set_allowed_primary_host_system(&mut self, value: String) {
        self.allowed_primary_host_system = Some(value);
    }

    /// Gets the value of AllowedPrimaryHostSystem
    pub fn get_allowed_primary_host_system(&self) -> Option<&String> {
        self.allowed_primary_host_system.as_ref()
    }

    /// Sets the value of ReplicaStorageLocation
    pub fn set_replica_storage_location(&mut self, value: String) {
        self.replica_storage_location = Some(value);
    }

    /// Gets the value of ReplicaStorageLocation
    pub fn get_replica_storage_location(&self) -> Option<&String> {
        self.replica_storage_location.as_ref()
    }

    /// Sets the value of TrustGroup
    pub fn set_trust_group(&mut self, value: String) {
        self.trust_group = Some(value);
    }

    /// Gets the value of TrustGroup
    pub fn get_trust_group(&self) -> Option<&String> {
        self.trust_group.as_ref()
    }
}

