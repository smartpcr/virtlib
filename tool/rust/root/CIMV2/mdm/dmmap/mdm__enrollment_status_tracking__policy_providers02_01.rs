// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_EnrollmentStatusTracking_PolicyProviders02_01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_EnrollmentStatusTracking_PolicyProviders02_01 {

/// 
    #[serde(rename = "InstallationState")]
    pub installation_state: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "LastError")]
    pub last_error: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "Timeout")]
    pub timeout: Option<i32>,
}

impl MDM_EnrollmentStatusTracking_PolicyProviders02_01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            installation_state: None,
            instance_id: None,
            last_error: None,
            parent_id: None,
            timeout: None,
        }
    }


    /// Sets the value of InstallationState
    pub fn set_installation_state(&mut self, value: i32) {
        self.installation_state = Some(value);
    }

    /// Gets the value of InstallationState
    pub fn get_installation_state(&self) -> Option<&i32> {
        self.installation_state.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of LastError
    pub fn set_last_error(&mut self, value: i32) {
        self.last_error = Some(value);
    }

    /// Gets the value of LastError
    pub fn get_last_error(&self) -> Option<&i32> {
        self.last_error.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of Timeout
    pub fn set_timeout(&mut self, value: i32) {
        self.timeout = Some(value);
    }

    /// Gets the value of Timeout
    pub fn get_timeout(&self) -> Option<&i32> {
        self.timeout.as_ref()
    }
}

