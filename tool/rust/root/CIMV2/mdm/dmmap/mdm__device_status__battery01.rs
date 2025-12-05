// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_DeviceStatus_Battery01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_DeviceStatus_Battery01 {

/// 
    #[serde(rename = "EstimatedChargeRemaining")]
    pub estimated_charge_remaining: Option<i32>,

/// 
    #[serde(rename = "EstimatedRuntime")]
    pub estimated_runtime: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<i32>,
}

impl MDM_DeviceStatus_Battery01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            estimated_charge_remaining: None,
            estimated_runtime: None,
            instance_id: None,
            parent_id: None,
            status: None,
        }
    }


    /// Sets the value of EstimatedChargeRemaining
    pub fn set_estimated_charge_remaining(&mut self, value: i32) {
        self.estimated_charge_remaining = Some(value);
    }

    /// Gets the value of EstimatedChargeRemaining
    pub fn get_estimated_charge_remaining(&self) -> Option<&i32> {
        self.estimated_charge_remaining.as_ref()
    }

    /// Sets the value of EstimatedRuntime
    pub fn set_estimated_runtime(&mut self, value: i32) {
        self.estimated_runtime = Some(value);
    }

    /// Gets the value of EstimatedRuntime
    pub fn get_estimated_runtime(&self) -> Option<&i32> {
        self.estimated_runtime.as_ref()
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

    /// Sets the value of Status
    pub fn set_status(&mut self, value: i32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&i32> {
        self.status.as_ref()
    }
}

