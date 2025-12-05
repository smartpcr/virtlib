// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PageFault_MemReset struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PageFault_MemReset {
    #[serde(flatten)]
    pub base: PageFault_V2,

/// 
    #[serde(rename = "BaseAddress")]
    pub base_address: Option<u32>,

/// 
    #[serde(rename = "SizeInBytes")]
    pub size_in_bytes: Option<serde_json::Value>,
}

impl PageFault_MemReset {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PageFault_V2::new(),
            base_address: None,
            size_in_bytes: None,
        }
    }


    /// Sets the value of BaseAddress
    pub fn set_base_address(&mut self, value: u32) {
        self.base_address = Some(value);
    }

    /// Gets the value of BaseAddress
    pub fn get_base_address(&self) -> Option<&u32> {
        self.base_address.as_ref()
    }

    /// Sets the value of SizeInBytes
    pub fn set_size_in_bytes(&mut self, value: serde_json::Value) {
        self.size_in_bytes = Some(value);
    }

    /// Gets the value of SizeInBytes
    pub fn get_size_in_bytes(&self) -> Option<&serde_json::Value> {
        self.size_in_bytes.as_ref()
    }
}

