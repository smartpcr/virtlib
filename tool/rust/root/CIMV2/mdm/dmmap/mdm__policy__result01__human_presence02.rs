// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_HumanPresence02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_HumanPresence02 {

/// 
    #[serde(rename = "ForceInstantLock")]
    pub force_instant_lock: Option<i32>,

/// 
    #[serde(rename = "ForceInstantWake")]
    pub force_instant_wake: Option<i32>,

/// 
    #[serde(rename = "ForceLockTimeout")]
    pub force_lock_timeout: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_Result01_HumanPresence02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            force_instant_lock: None,
            force_instant_wake: None,
            force_lock_timeout: None,
            instance_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of ForceInstantLock
    pub fn set_force_instant_lock(&mut self, value: i32) {
        self.force_instant_lock = Some(value);
    }

    /// Gets the value of ForceInstantLock
    pub fn get_force_instant_lock(&self) -> Option<&i32> {
        self.force_instant_lock.as_ref()
    }

    /// Sets the value of ForceInstantWake
    pub fn set_force_instant_wake(&mut self, value: i32) {
        self.force_instant_wake = Some(value);
    }

    /// Gets the value of ForceInstantWake
    pub fn get_force_instant_wake(&self) -> Option<&i32> {
        self.force_instant_wake.as_ref()
    }

    /// Sets the value of ForceLockTimeout
    pub fn set_force_lock_timeout(&mut self, value: i32) {
        self.force_lock_timeout = Some(value);
    }

    /// Gets the value of ForceLockTimeout
    pub fn get_force_lock_timeout(&self) -> Option<&i32> {
        self.force_lock_timeout.as_ref()
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

