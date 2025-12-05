// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_SecurityPerProcessStatistics struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_SecurityPerProcessStatistics {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "ContextHandles")]
    pub context_handles: Option<u32>,

/// 
    #[serde(rename = "CredentialHandles")]
    pub credential_handles: Option<u32>,
}

impl Win32_PerfRawData_Counters_SecurityPerProcessStatistics {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            context_handles: None,
            credential_handles: None,
        }
    }


    /// Sets the value of ContextHandles
    pub fn set_context_handles(&mut self, value: u32) {
        self.context_handles = Some(value);
    }

    /// Gets the value of ContextHandles
    pub fn get_context_handles(&self) -> Option<&u32> {
        self.context_handles.as_ref()
    }

    /// Sets the value of CredentialHandles
    pub fn set_credential_handles(&mut self, value: u32) {
        self.credential_handles = Some(value);
    }

    /// Gets the value of CredentialHandles
    pub fn get_credential_handles(&self) -> Option<&u32> {
        self.credential_handles.as_ref()
    }
}

