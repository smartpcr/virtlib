// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DiskIo_V2_TypeGroup2 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiskIo_V2_TypeGroup2 {
    #[serde(flatten)]
    pub base: DiskIo_V2,

/// 
    #[serde(rename = "Irp")]
    pub irp: Option<u32>,
}

impl DiskIo_V2_TypeGroup2 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: DiskIo_V2::new(),
            irp: None,
        }
    }


    /// Sets the value of Irp
    pub fn set_irp(&mut self, value: u32) {
        self.irp = Some(value);
    }

    /// Gets the value of Irp
    pub fn get_irp(&self) -> Option<&u32> {
        self.irp.as_ref()
    }
}

