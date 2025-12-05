// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Update struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Update {

/// 
    #[serde(rename = "DeferUpgrade")]
    pub defer_upgrade: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "LastSuccessfulScanTime")]
    pub last_successful_scan_time: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Update {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            defer_upgrade: None,
            instance_id: None,
            last_successful_scan_time: None,
            parent_id: None,
        }
    }


    /// Sets the value of DeferUpgrade
    pub fn set_defer_upgrade(&mut self, value: i32) {
        self.defer_upgrade = Some(value);
    }

    /// Gets the value of DeferUpgrade
    pub fn get_defer_upgrade(&self) -> Option<&i32> {
        self.defer_upgrade.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of LastSuccessfulScanTime
    pub fn set_last_successful_scan_time(&mut self, value: String) {
        self.last_successful_scan_time = Some(value);
    }

    /// Gets the value of LastSuccessfulScanTime
    pub fn get_last_successful_scan_time(&self) -> Option<&String> {
        self.last_successful_scan_time.as_ref()
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

