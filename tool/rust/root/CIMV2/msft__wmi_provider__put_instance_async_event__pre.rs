// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msft_WmiProvider_PutInstanceAsyncEvent_Pre struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msft_WmiProvider_PutInstanceAsyncEvent_Pre {
    #[serde(flatten)]
    pub base: Msft_WmiProvider_OperationEvent_Pre,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "InstanceObject")]
    pub instance_object: Option<serde_json::Value>,
}

impl Msft_WmiProvider_PutInstanceAsyncEvent_Pre {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msft_WmiProvider_OperationEvent_Pre::new(),
            flags: None,
            instance_object: None,
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

    /// Sets the value of InstanceObject
    pub fn set_instance_object(&mut self, value: serde_json::Value) {
        self.instance_object = Some(value);
    }

    /// Gets the value of InstanceObject
    pub fn get_instance_object(&self) -> Option<&serde_json::Value> {
        self.instance_object.as_ref()
    }
}

