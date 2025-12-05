// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetReadfileTimeout struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetReadfileTimeout {
    #[serde(flatten)]
    pub base: MSFT_SCMEventLogEvent,

/// 
    #[serde(rename = "Milliseconds")]
    pub milliseconds: Option<u32>,
}

impl MSFT_NetReadfileTimeout {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_SCMEventLogEvent::new(),
            milliseconds: None,
        }
    }


    /// Sets the value of Milliseconds
    pub fn set_milliseconds(&mut self, value: u32) {
        self.milliseconds = Some(value);
    }

    /// Gets the value of Milliseconds
    pub fn get_milliseconds(&self) -> Option<&u32> {
        self.milliseconds.as_ref()
    }
}

