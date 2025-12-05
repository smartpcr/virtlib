// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetFirstLogonFailed struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetFirstLogonFailed {
    #[serde(flatten)]
    pub base: MSFT_SCMEventLogEvent,

/// 
    #[serde(rename = "Error")]
    pub error: Option<u32>,
}

impl MSFT_NetFirstLogonFailed {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_SCMEventLogEvent::new(),
            error: None,
        }
    }


    /// Sets the value of Error
    pub fn set_error(&mut self, value: u32) {
        self.error = Some(value);
    }

    /// Gets the value of Error
    pub fn get_error(&self) -> Option<&u32> {
        self.error.as_ref()
    }
}

