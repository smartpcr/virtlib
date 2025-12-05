// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_DmaGuard02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_DmaGuard02 {

/// 
    #[serde(rename = "DeviceEnumerationPolicy")]
    pub device_enumeration_policy: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_Config01_DmaGuard02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            device_enumeration_policy: None,
            instance_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of DeviceEnumerationPolicy
    pub fn set_device_enumeration_policy(&mut self, value: i32) {
        self.device_enumeration_policy = Some(value);
    }

    /// Gets the value of DeviceEnumerationPolicy
    pub fn get_device_enumeration_policy(&self) -> Option<&i32> {
        self.device_enumeration_policy.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }
}

