// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetBootSystemDriversFailed struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetBootSystemDriversFailed {
    #[serde(flatten)]
    pub base: MSFT_SCMEventLogEvent,

/// 
    #[serde(rename = "DriverList")]
    pub driver_list: Option<String>,
}

impl MSFT_NetBootSystemDriversFailed {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_SCMEventLogEvent::new(),
            driver_list: None,
        }
    }


    /// Sets the value of DriverList
    pub fn set_driver_list(&mut self, value: String) {
        self.driver_list = Some(value);
    }

    /// Gets the value of DriverList
    pub fn get_driver_list(&self) -> Option<&String> {
        self.driver_list.as_ref()
    }
}

