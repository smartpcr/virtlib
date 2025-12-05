// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// LoaderBaseEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoaderBaseEvent {
    #[serde(flatten)]
    pub base: Image_V2,

/// 
    #[serde(rename = "BaseAddress")]
    pub base_address: Option<u64>,
}

impl LoaderBaseEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Image_V2::new(),
            base_address: None,
        }
    }


    /// Sets the value of BaseAddress
    pub fn set_base_address(&mut self, value: u64) {
        self.base_address = Some(value);
    }

    /// Gets the value of BaseAddress
    pub fn get_base_address(&self) -> Option<&u64> {
        self.base_address.as_ref()
    }
}

