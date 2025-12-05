// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SysCallEnter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SysCallEnter {
    #[serde(flatten)]
    pub base: PerfInfo_V2,

/// 
    #[serde(rename = "SysCallAddress")]
    pub sys_call_address: Option<u32>,
}

impl SysCallEnter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PerfInfo_V2::new(),
            sys_call_address: None,
        }
    }


    /// Sets the value of SysCallAddress
    pub fn set_sys_call_address(&mut self, value: u32) {
        self.sys_call_address = Some(value);
    }

    /// Gets the value of SysCallAddress
    pub fn get_sys_call_address(&self) -> Option<&u32> {
        self.sys_call_address.as_ref()
    }
}

