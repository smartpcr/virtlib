// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SysCallExit struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SysCallExit {
    #[serde(flatten)]
    pub base: PerfInfo_V2,

/// 
    #[serde(rename = "SysCallNtStatus")]
    pub sys_call_nt_status: Option<u32>,
}

impl SysCallExit {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PerfInfo_V2::new(),
            sys_call_nt_status: None,
        }
    }


    /// Sets the value of SysCallNtStatus
    pub fn set_sys_call_nt_status(&mut self, value: u32) {
        self.sys_call_nt_status = Some(value);
    }

    /// Gets the value of SysCallNtStatus
    pub fn get_sys_call_nt_status(&self) -> Option<&u32> {
        self.sys_call_nt_status.as_ref()
    }
}

