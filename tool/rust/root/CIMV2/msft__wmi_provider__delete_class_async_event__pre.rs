// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msft_WmiProvider_DeleteClassAsyncEvent_Pre struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msft_WmiProvider_DeleteClassAsyncEvent_Pre {
    #[serde(flatten)]
    pub base: Msft_WmiProvider_OperationEvent_Pre,

/// 
    #[serde(rename = "ClassName")]
    pub class_name: Option<String>,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,
}

impl Msft_WmiProvider_DeleteClassAsyncEvent_Pre {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msft_WmiProvider_OperationEvent_Pre::new(),
            class_name: None,
            flags: None,
        }
    }


    /// Sets the value of ClassName
    pub fn set_class_name(&mut self, value: String) {
        self.class_name = Some(value);
    }

    /// Gets the value of ClassName
    pub fn get_class_name(&self) -> Option<&String> {
        self.class_name.as_ref()
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

