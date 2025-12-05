// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// FileIo_OpEnd struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileIo_OpEnd {
    #[serde(flatten)]
    pub base: FileIo,

/// 
    #[serde(rename = "ExtraInfo")]
    pub extra_info: Option<u32>,

/// 
    #[serde(rename = "IrpPtr")]
    pub irp_ptr: Option<u32>,

/// 
    #[serde(rename = "NtStatus")]
    pub nt_status: Option<u32>,
}

impl FileIo_OpEnd {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: FileIo::new(),
            extra_info: None,
            irp_ptr: None,
            nt_status: None,
        }
    }


    /// Sets the value of ExtraInfo
    pub fn set_extra_info(&mut self, value: u32) {
        self.extra_info = Some(value);
    }

    /// Gets the value of ExtraInfo
    pub fn get_extra_info(&self) -> Option<&u32> {
        self.extra_info.as_ref()
    }

    /// Sets the value of IrpPtr
    pub fn set_irp_ptr(&mut self, value: u32) {
        self.irp_ptr = Some(value);
    }

    /// Gets the value of IrpPtr
    pub fn get_irp_ptr(&self) -> Option<&u32> {
        self.irp_ptr.as_ref()
    }

    /// Sets the value of NtStatus
    pub fn set_nt_status(&mut self, value: u32) {
        self.nt_status = Some(value);
    }

    /// Gets the value of NtStatus
    pub fn get_nt_status(&self) -> Option<&u32> {
        self.nt_status.as_ref()
    }
}

