// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MobTabPerfTraceProvider struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MobTabPerfTraceProvider {
    #[serde(flatten)]
    pub base: EventTrace,

/// Enable Flags
    #[serde(rename = "Flags")]
    pub flags: Option<MobTabPerfTraceProvider_Flags>,
}

impl MobTabPerfTraceProvider {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: EventTrace::new(),
            flags: None,
        }
    }


    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: MobTabPerfTraceProvider_Flags) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&MobTabPerfTraceProvider_Flags> {
        self.flags.as_ref()
    }
}

