// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_WiFiPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_WiFiPort {
    #[serde(flatten)]
    pub base: CIM_WiFiPort,

/// 
    #[serde(rename = "IsBound")]
    pub is_bound: Option<bool>,
}

impl Msvm_WiFiPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_WiFiPort::new(),
            is_bound: None,
        }
    }


    /// Sets the value of IsBound
    pub fn set_is_bound(&mut self, value: bool) {
        self.is_bound = Some(value);
    }

    /// Gets the value of IsBound
    pub fn get_is_bound(&self) -> Option<&bool> {
        self.is_bound.as_ref()
    }
}

