// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_EnrollmentStatusTracking_Setup01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_EnrollmentStatusTracking_Setup01 {

/// 
    #[serde(rename = "HasProvisioningCompleted")]
    pub has_provisioning_completed: Option<bool>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_EnrollmentStatusTracking_Setup01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            has_provisioning_completed: None,
            instance_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of HasProvisioningCompleted
    pub fn set_has_provisioning_completed(&mut self, value: bool) {
        self.has_provisioning_completed = Some(value);
    }

    /// Gets the value of HasProvisioningCompleted
    pub fn get_has_provisioning_completed(&self) -> Option<&bool> {
        self.has_provisioning_completed.as_ref()
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

