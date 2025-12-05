// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msft_WmiProvider_DeleteInstanceAsyncEvent_Pre struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msft_WmiProvider_DeleteInstanceAsyncEvent_Pre {
    #[serde(flatten)]
    pub base: Msft_WmiProvider_OperationEvent_Pre,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "ObjectPath")]
    pub object_path: Option<String>,
}

impl Msft_WmiProvider_DeleteInstanceAsyncEvent_Pre {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msft_WmiProvider_OperationEvent_Pre::new(),
            flags: None,
            object_path: None,
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

    /// Sets the value of ObjectPath
    pub fn set_object_path(&mut self, value: String) {
        self.object_path = Some(value);
    }

    /// Gets the value of ObjectPath
    pub fn get_object_path(&self) -> Option<&String> {
        self.object_path.as_ref()
    }
}

