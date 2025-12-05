// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Reporting_SecurityAuditing01_RetrieveByTimeRange02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Reporting_SecurityAuditing01_RetrieveByTimeRange02 {

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "Logs")]
    pub logs: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "StartTime")]
    pub start_time: Option<String>,

/// 
    #[serde(rename = "StopTime")]
    pub stop_time: Option<String>,
}

impl MDM_Reporting_SecurityAuditing01_RetrieveByTimeRange02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            logs: None,
            parent_id: None,
            start_time: None,
            stop_time: None,
        }
    }


    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of Logs
    pub fn set_logs(&mut self, value: String) {
        self.logs = Some(value);
    }

    /// Gets the value of Logs
    pub fn get_logs(&self) -> Option<&String> {
        self.logs.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of StartTime
    pub fn set_start_time(&mut self, value: String) {
        self.start_time = Some(value);
    }

    /// Gets the value of StartTime
    pub fn get_start_time(&self) -> Option<&String> {
        self.start_time.as_ref()
    }

    /// Sets the value of StopTime
    pub fn set_stop_time(&mut self, value: String) {
        self.stop_time = Some(value);
    }

    /// Gets the value of StopTime
    pub fn get_stop_time(&self) -> Option<&String> {
        self.stop_time.as_ref()
    }
}

