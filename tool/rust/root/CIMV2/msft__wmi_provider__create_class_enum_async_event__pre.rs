// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msft_WmiProvider_CreateClassEnumAsyncEvent_Pre struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msft_WmiProvider_CreateClassEnumAsyncEvent_Pre {
    #[serde(flatten)]
    pub base: Msft_WmiProvider_OperationEvent_Pre,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "SuperclassName")]
    pub superclass_name: Option<String>,
}

impl Msft_WmiProvider_CreateClassEnumAsyncEvent_Pre {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msft_WmiProvider_OperationEvent_Pre::new(),
            flags: None,
            superclass_name: None,
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

    /// Sets the value of SuperclassName
    pub fn set_superclass_name(&mut self, value: String) {
        self.superclass_name = Some(value);
    }

    /// Gets the value of SuperclassName
    pub fn get_superclass_name(&self) -> Option<&String> {
        self.superclass_name.as_ref()
    }
}

