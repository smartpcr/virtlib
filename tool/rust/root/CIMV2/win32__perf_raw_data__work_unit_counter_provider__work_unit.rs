// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_WorkUnitCounterProvider_WorkUnit struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_WorkUnitCounterProvider_WorkUnit {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "AppOwnerProcessID")]
    pub app_owner_process_id: Option<u32>,

/// 
    #[serde(rename = "HostProcessID")]
    pub host_process_id: Option<u32>,
}

impl Win32_PerfRawData_WorkUnitCounterProvider_WorkUnit {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            app_owner_process_id: None,
            host_process_id: None,
        }
    }


    /// Sets the value of AppOwnerProcessID
    pub fn set_app_owner_process_id(&mut self, value: u32) {
        self.app_owner_process_id = Some(value);
    }

    /// Gets the value of AppOwnerProcessID
    pub fn get_app_owner_process_id(&self) -> Option<&u32> {
        self.app_owner_process_id.as_ref()
    }

    /// Sets the value of HostProcessID
    pub fn set_host_process_id(&mut self, value: u32) {
        self.host_process_id = Some(value);
    }

    /// Gets the value of HostProcessID
    pub fn get_host_process_id(&self) -> Option<&u32> {
        self.host_process_id.as_ref()
    }
}

