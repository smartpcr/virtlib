// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_MemoryDump02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_MemoryDump02 {

/// 
    #[serde(rename = "AllowCrashDump")]
    pub allow_crash_dump: Option<i32>,

/// 
    #[serde(rename = "AllowLiveDump")]
    pub allow_live_dump: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_Result01_MemoryDump02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_crash_dump: None,
            allow_live_dump: None,
            instance_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of AllowCrashDump
    pub fn set_allow_crash_dump(&mut self, value: i32) {
        self.allow_crash_dump = Some(value);
    }

    /// Gets the value of AllowCrashDump
    pub fn get_allow_crash_dump(&self) -> Option<&i32> {
        self.allow_crash_dump.as_ref()
    }

    /// Sets the value of AllowLiveDump
    pub fn set_allow_live_dump(&mut self, value: i32) {
        self.allow_live_dump = Some(value);
    }

    /// Gets the value of AllowLiveDump
    pub fn get_allow_live_dump(&self) -> Option<&i32> {
        self.allow_live_dump.as_ref()
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

