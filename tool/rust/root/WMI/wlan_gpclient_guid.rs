// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WlanGPClientGuid struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WlanGPClientGuid {
    #[serde(flatten)]
    pub base: EventTrace,

/// Enable Flags
    #[serde(rename = "Flags")]
    pub flags: Option<WlanGPClientGuid_Flags>,
}

impl WlanGPClientGuid {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: EventTrace::new(),
            flags: None,
        }
    }


    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: WlanGPClientGuid_Flags) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&WlanGPClientGuid_Flags> {
        self.flags.as_ref()
    }
}

