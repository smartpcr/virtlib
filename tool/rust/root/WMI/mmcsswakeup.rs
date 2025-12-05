// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MMCSSWakeup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MMCSSWakeup {
    #[serde(flatten)]
    pub base: MMCSSTrace,

/// 
    #[serde(rename = "Reason")]
    pub reason: Option<u32>,
}

impl MMCSSWakeup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MMCSSTrace::new(),
            reason: None,
        }
    }


    /// Sets the value of Reason
    pub fn set_reason(&mut self, value: u32) {
        self.reason = Some(value);
    }

    /// Gets the value of Reason
    pub fn get_reason(&self) -> Option<&u32> {
        self.reason.as_ref()
    }
}

