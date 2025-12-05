// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DiskIo_V2_TypeGroup3 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiskIo_V2_TypeGroup3 {
    #[serde(flatten)]
    pub base: DiskIo_V2,

/// 
    #[serde(rename = "DiskNumber")]
    pub disk_number: Option<u32>,

/// 
    #[serde(rename = "HighResResponseTime")]
    pub high_res_response_time: Option<u64>,

/// 
    #[serde(rename = "Irp")]
    pub irp: Option<u32>,

/// 
    #[serde(rename = "IrpFlags")]
    pub irp_flags: Option<u32>,
}

impl DiskIo_V2_TypeGroup3 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: DiskIo_V2::new(),
            disk_number: None,
            high_res_response_time: None,
            irp: None,
            irp_flags: None,
        }
    }


    /// Sets the value of DiskNumber
    pub fn set_disk_number(&mut self, value: u32) {
        self.disk_number = Some(value);
    }

    /// Gets the value of DiskNumber
    pub fn get_disk_number(&self) -> Option<&u32> {
        self.disk_number.as_ref()
    }

    /// Sets the value of HighResResponseTime
    pub fn set_high_res_response_time(&mut self, value: u64) {
        self.high_res_response_time = Some(value);
    }

    /// Gets the value of HighResResponseTime
    pub fn get_high_res_response_time(&self) -> Option<&u64> {
        self.high_res_response_time.as_ref()
    }

    /// Sets the value of Irp
    pub fn set_irp(&mut self, value: u32) {
        self.irp = Some(value);
    }

    /// Gets the value of Irp
    pub fn get_irp(&self) -> Option<&u32> {
        self.irp.as_ref()
    }

    /// Sets the value of IrpFlags
    pub fn set_irp_flags(&mut self, value: u32) {
        self.irp_flags = Some(value);
    }

    /// Gets the value of IrpFlags
    pub fn get_irp_flags(&self) -> Option<&u32> {
        self.irp_flags.as_ref()
    }
}

