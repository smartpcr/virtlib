// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_DeviceMemoryAddress struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_DeviceMemoryAddress {
    #[serde(flatten)]
    pub base: Win32_SystemMemoryResource,

/// 
    #[serde(rename = "MemoryType")]
    pub memory_type: Option<String>,
}

impl Win32_DeviceMemoryAddress {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_SystemMemoryResource::new(),
            memory_type: None,
        }
    }


    /// Sets the value of MemoryType
    pub fn set_memory_type(&mut self, value: String) {
        self.memory_type = Some(value);
    }

    /// Gets the value of MemoryType
    pub fn get_memory_type(&self) -> Option<&String> {
        self.memory_type.as_ref()
    }
}

