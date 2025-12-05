// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Reboot_Schedule01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Reboot_Schedule01 {

/// 
    #[serde(rename = "DailyRecurrent")]
    pub daily_recurrent: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "Single")]
    pub single: Option<String>,
}

impl MDM_Reboot_Schedule01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            daily_recurrent: None,
            instance_id: None,
            parent_id: None,
            single: None,
        }
    }


    /// Sets the value of DailyRecurrent
    pub fn set_daily_recurrent(&mut self, value: String) {
        self.daily_recurrent = Some(value);
    }

    /// Gets the value of DailyRecurrent
    pub fn get_daily_recurrent(&self) -> Option<&String> {
        self.daily_recurrent.as_ref()
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

    /// Sets the value of Single
    pub fn set_single(&mut self, value: String) {
        self.single = Some(value);
    }

    /// Gets the value of Single
    pub fn get_single(&self) -> Option<&String> {
        self.single.as_ref()
    }
}

