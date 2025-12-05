// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSWmi_Guid struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSWmi_Guid {

/// 
    #[serde(rename = "Guid")]
    pub guid: Vec<u8>,
}

impl MSWmi_Guid {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            guid: Vec::new(),
        }
    }


    /// Sets the value of Guid
    pub fn set_guid(&mut self, value: Vec<u8>) {
        self.guid = value;
    }

    /// Gets the value of Guid
    pub fn get_guid(&self) -> &Vec<u8> {
        &self.guid
    }
}

