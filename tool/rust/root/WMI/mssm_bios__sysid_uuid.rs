// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSSmBios_SysidUUID struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSSmBios_SysidUUID {
    #[serde(flatten)]
    pub base: MS_SmBios,

/// 
    #[serde(rename = "Uuid")]
    pub uuid: Vec<u8>,
}

impl MSSmBios_SysidUUID {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MS_SmBios::new(),
            uuid: Vec::new(),
        }
    }


    /// Sets the value of Uuid
    pub fn set_uuid(&mut self, value: Vec<u8>) {
        self.uuid = value;
    }

    /// Gets the value of Uuid
    pub fn get_uuid(&self) -> &Vec<u8> {
        &self.uuid
    }
}

