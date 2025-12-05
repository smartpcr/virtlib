// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CscFastSync struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CscFastSync {
    #[serde(flatten)]
    pub base: EventTrace,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,
}

impl CscFastSync {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: EventTrace::new(),
            flags: None,
        }
    }


    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }
}

