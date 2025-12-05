// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_DeviceManageability_Provider01_01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_DeviceManageability_Provider01_01 {

/// 
    #[serde(rename = "ConfigInfo")]
    pub config_info: Option<String>,

/// 
    #[serde(rename = "EnrollmentInfo")]
    pub enrollment_info: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_DeviceManageability_Provider01_01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            config_info: None,
            enrollment_info: None,
            instance_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of ConfigInfo
    pub fn set_config_info(&mut self, value: String) {
        self.config_info = Some(value);
    }

    /// Gets the value of ConfigInfo
    pub fn get_config_info(&self) -> Option<&String> {
        self.config_info.as_ref()
    }

    /// Sets the value of EnrollmentInfo
    pub fn set_enrollment_info(&mut self, value: String) {
        self.enrollment_info = Some(value);
    }

    /// Gets the value of EnrollmentInfo
    pub fn get_enrollment_info(&self) -> Option<&String> {
        self.enrollment_info.as_ref()
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

