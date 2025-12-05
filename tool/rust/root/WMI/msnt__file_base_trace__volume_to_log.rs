// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNT_FileBaseTrace_VolumeToLog struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNT_FileBaseTrace_VolumeToLog {
    #[serde(flatten)]
    pub base: EventTrace,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<VolumeToLog_Flags>,
}

impl MSNT_FileBaseTrace_VolumeToLog {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: EventTrace::new(),
            flags: None,
        }
    }


    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: VolumeToLog_Flags) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&VolumeToLog_Flags> {
        self.flags.as_ref()
    }
}

