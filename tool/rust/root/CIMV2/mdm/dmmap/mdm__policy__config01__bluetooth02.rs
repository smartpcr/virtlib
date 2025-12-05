// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_Bluetooth02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_Bluetooth02 {

/// 
    #[serde(rename = "AllowAdvertising")]
    pub allow_advertising: Option<i32>,

/// 
    #[serde(rename = "AllowDiscoverableMode")]
    pub allow_discoverable_mode: Option<i32>,

/// 
    #[serde(rename = "AllowPrepairing")]
    pub allow_prepairing: Option<i32>,

/// 
    #[serde(rename = "AllowPromptedProximalConnections")]
    pub allow_prompted_proximal_connections: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "LocalDeviceName")]
    pub local_device_name: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "ServicesAllowedList")]
    pub services_allowed_list: Option<String>,

/// 
    #[serde(rename = "SetMinimumEncryptionKeySize")]
    pub set_minimum_encryption_key_size: Option<i32>,
}

impl MDM_Policy_Config01_Bluetooth02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_advertising: None,
            allow_discoverable_mode: None,
            allow_prepairing: None,
            allow_prompted_proximal_connections: None,
            instance_id: None,
            local_device_name: None,
            parent_id: None,
            services_allowed_list: None,
            set_minimum_encryption_key_size: None,
        }
    }


    /// Sets the value of AllowAdvertising
    pub fn set_allow_advertising(&mut self, value: i32) {
        self.allow_advertising = Some(value);
    }

    /// Gets the value of AllowAdvertising
    pub fn get_allow_advertising(&self) -> Option<&i32> {
        self.allow_advertising.as_ref()
    }

    /// Sets the value of AllowDiscoverableMode
    pub fn set_allow_discoverable_mode(&mut self, value: i32) {
        self.allow_discoverable_mode = Some(value);
    }

    /// Gets the value of AllowDiscoverableMode
    pub fn get_allow_discoverable_mode(&self) -> Option<&i32> {
        self.allow_discoverable_mode.as_ref()
    }

    /// Sets the value of AllowPrepairing
    pub fn set_allow_prepairing(&mut self, value: i32) {
        self.allow_prepairing = Some(value);
    }

    /// Gets the value of AllowPrepairing
    pub fn get_allow_prepairing(&self) -> Option<&i32> {
        self.allow_prepairing.as_ref()
    }

    /// Sets the value of AllowPromptedProximalConnections
    pub fn set_allow_prompted_proximal_connections(&mut self, value: i32) {
        self.allow_prompted_proximal_connections = Some(value);
    }

    /// Gets the value of AllowPromptedProximalConnections
    pub fn get_allow_prompted_proximal_connections(&self) -> Option<&i32> {
        self.allow_prompted_proximal_connections.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of LocalDeviceName
    pub fn set_local_device_name(&mut self, value: String) {
        self.local_device_name = Some(value);
    }

    /// Gets the value of LocalDeviceName
    pub fn get_local_device_name(&self) -> Option<&String> {
        self.local_device_name.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of ServicesAllowedList
    pub fn set_services_allowed_list(&mut self, value: String) {
        self.services_allowed_list = Some(value);
    }

    /// Gets the value of ServicesAllowedList
    pub fn get_services_allowed_list(&self) -> Option<&String> {
        self.services_allowed_list.as_ref()
    }

    /// Sets the value of SetMinimumEncryptionKeySize
    pub fn set_set_minimum_encryption_key_size(&mut self, value: i32) {
        self.set_minimum_encryption_key_size = Some(value);
    }

    /// Gets the value of SetMinimumEncryptionKeySize
    pub fn get_set_minimum_encryption_key_size(&self) -> Option<&i32> {
        self.set_minimum_encryption_key_size.as_ref()
    }
}

