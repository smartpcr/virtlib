// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// UsbstorGuid struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UsbstorGuid {
    #[serde(flatten)]
    pub base: EventTrace,

/// Enable Flags
    #[serde(rename = "Flags")]
    pub flags: Option<UsbstorGuid_Flags>,

/// Levels
    #[serde(rename = "Level")]
    pub level: Option<UsbstorGuid_Level>,
}

impl UsbstorGuid {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: EventTrace::new(),
            flags: None,
            level: None,
        }
    }


    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: UsbstorGuid_Flags) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&UsbstorGuid_Flags> {
        self.flags.as_ref()
    }

    /// Sets the value of Level
    pub fn set_level(&mut self, value: UsbstorGuid_Level) {
        self.level = Some(value);
    }

    /// Gets the value of Level
    pub fn get_level(&self) -> Option<&UsbstorGuid_Level> {
        self.level.as_ref()
    }
}

