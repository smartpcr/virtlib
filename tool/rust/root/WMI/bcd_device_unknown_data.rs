// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// BcdDeviceUnknownData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BcdDeviceUnknownData {
    #[serde(flatten)]
    pub base: BcdDeviceData,

/// This is the binary data of the unknown device element.
    #[serde(rename = "Data")]
    pub data: Vec<u8>,
}

impl BcdDeviceUnknownData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BcdDeviceData::new(),
            data: Vec::new(),
        }
    }


    /// Sets the value of Data
    pub fn set_data(&mut self, value: Vec<u8>) {
        self.data = value;
    }

    /// Gets the value of Data
    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }
}

