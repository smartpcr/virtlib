// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSI_RADIUSConfig struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSI_RADIUSConfig {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// Fixed address of backup RADIUS server
    #[serde(rename = "BackupRADIUSServer")]
    pub backup_radiusserver: Option<ISCSI_IP_Address>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// Fixed address of primary RADIUS server
    #[serde(rename = "RADIUSServer")]
    pub radiusserver: Option<ISCSI_IP_Address>,

/// Must be zero
    #[serde(rename = "Reserved")]
    pub reserved: Option<u32>,

/// Shared secret for communicating with primary and backup RADIUS servers. This field is a variable length array.
    #[serde(rename = "SharedSecret")]
    pub shared_secret: Vec<u8>,

/// Size in bytes of shared secret used to communicate with RADIUS servers
    #[serde(rename = "SharedSecretSizeInBytes")]
    pub shared_secret_size_in_bytes: Option<u32>,

/// TRUE if adapter should use RADIUS for CHAP authentication
    #[serde(rename = "UseRADIUSForCHAP")]
    pub use_radiusfor_chap: Option<bool>,
}

impl MSiSCSI_RADIUSConfig {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            backup_radiusserver: None,
            instance_name: None,
            radiusserver: None,
            reserved: None,
            shared_secret: Vec::new(),
            shared_secret_size_in_bytes: None,
            use_radiusfor_chap: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of BackupRADIUSServer
    pub fn set_backup_radiusserver(&mut self, value: ISCSI_IP_Address) {
        self.backup_radiusserver = Some(value);
    }

    /// Gets the value of BackupRADIUSServer
    pub fn get_backup_radiusserver(&self) -> Option<&ISCSI_IP_Address> {
        self.backup_radiusserver.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of RADIUSServer
    pub fn set_radiusserver(&mut self, value: ISCSI_IP_Address) {
        self.radiusserver = Some(value);
    }

    /// Gets the value of RADIUSServer
    pub fn get_radiusserver(&self) -> Option<&ISCSI_IP_Address> {
        self.radiusserver.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u32) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u32> {
        self.reserved.as_ref()
    }

    /// Sets the value of SharedSecret
    pub fn set_shared_secret(&mut self, value: Vec<u8>) {
        self.shared_secret = value;
    }

    /// Gets the value of SharedSecret
    pub fn get_shared_secret(&self) -> &Vec<u8> {
        &self.shared_secret
    }

    /// Sets the value of SharedSecretSizeInBytes
    pub fn set_shared_secret_size_in_bytes(&mut self, value: u32) {
        self.shared_secret_size_in_bytes = Some(value);
    }

    /// Gets the value of SharedSecretSizeInBytes
    pub fn get_shared_secret_size_in_bytes(&self) -> Option<&u32> {
        self.shared_secret_size_in_bytes.as_ref()
    }

    /// Sets the value of UseRADIUSForCHAP
    pub fn set_use_radiusfor_chap(&mut self, value: bool) {
        self.use_radiusfor_chap = Some(value);
    }

    /// Gets the value of UseRADIUSForCHAP
    pub fn get_use_radiusfor_chap(&self) -> Option<&bool> {
        self.use_radiusfor_chap.as_ref()
    }
}

