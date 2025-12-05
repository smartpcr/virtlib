// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DAConnectionStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DAConnectionStatus {
    #[serde(flatten)]
    pub base: MSFT_NetSettingData,

/// 
    #[serde(rename = "Status")]
    pub status: Option<u32>,

/// 
    #[serde(rename = "Substatus")]
    pub substatus: Option<u32>,
}

impl MSFT_DAConnectionStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSettingData::new(),
            status: None,
            substatus: None,
        }
    }


    /// Sets the value of Status
    pub fn set_status(&mut self, value: u32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u32> {
        self.status.as_ref()
    }

    /// Sets the value of Substatus
    pub fn set_substatus(&mut self, value: u32) {
        self.substatus = Some(value);
    }

    /// Gets the value of Substatus
    pub fn get_substatus(&self) -> Option<&u32> {
        self.substatus.as_ref()
    }
}

