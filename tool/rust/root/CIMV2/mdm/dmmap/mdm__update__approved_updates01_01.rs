// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Update_ApprovedUpdates01_01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Update_ApprovedUpdates01_01 {

/// 
    #[serde(rename = "ApprovedTime")]
    pub approved_time: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Update_ApprovedUpdates01_01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            approved_time: None,
            instance_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of ApprovedTime
    pub fn set_approved_time(&mut self, value: String) {
        self.approved_time = Some(value);
    }

    /// Gets the value of ApprovedTime
    pub fn get_approved_time(&self) -> Option<&String> {
        self.approved_time.as_ref()
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

