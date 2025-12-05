// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSSmBios_Sysid1394 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSSmBios_Sysid1394 {
    #[serde(flatten)]
    pub base: MS_SmBios,

/// 
    #[serde(rename = "x1394")]
    pub x1394: Vec<u8>,
}

impl MSSmBios_Sysid1394 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MS_SmBios::new(),
            x1394: Vec::new(),
        }
    }


    /// Sets the value of x1394
    pub fn set_x1394(&mut self, value: Vec<u8>) {
        self.x1394 = value;
    }

    /// Gets the value of x1394
    pub fn get_x1394(&self) -> &Vec<u8> {
        &self.x1394
    }
}

