// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WNFPublish struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WNFPublish {
    #[serde(flatten)]
    pub base: WNFTrace,

/// 
    #[serde(rename = "DataLength")]
    pub data_length: Option<u32>,

/// 
    #[serde(rename = "StateName")]
    pub state_name: Option<u64>,
}

impl WNFPublish {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WNFTrace::new(),
            data_length: None,
            state_name: None,
        }
    }


    /// Sets the value of DataLength
    pub fn set_data_length(&mut self, value: u32) {
        self.data_length = Some(value);
    }

    /// Gets the value of DataLength
    pub fn get_data_length(&self) -> Option<&u32> {
        self.data_length.as_ref()
    }

    /// Sets the value of StateName
    pub fn set_state_name(&mut self, value: u64) {
        self.state_name = Some(value);
    }

    /// Gets the value of StateName
    pub fn get_state_name(&self) -> Option<&u64> {
        self.state_name.as_ref()
    }
}

