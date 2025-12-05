// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ReserveCost struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ReserveCost {
    #[serde(flatten)]
    pub base: CIM_Check,

/// 
    #[serde(rename = "ReserveFolder")]
    pub reserve_folder: Option<String>,

/// 
    #[serde(rename = "ReserveKey")]
    pub reserve_key: Option<String>,

/// 
    #[serde(rename = "ReserveLocal")]
    pub reserve_local: Option<u32>,

/// 
    #[serde(rename = "ReserveSource")]
    pub reserve_source: Option<u32>,
}

impl Win32_ReserveCost {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Check::new(),
            reserve_folder: None,
            reserve_key: None,
            reserve_local: None,
            reserve_source: None,
        }
    }


    /// Sets the value of ReserveFolder
    pub fn set_reserve_folder(&mut self, value: String) {
        self.reserve_folder = Some(value);
    }

    /// Gets the value of ReserveFolder
    pub fn get_reserve_folder(&self) -> Option<&String> {
        self.reserve_folder.as_ref()
    }

    /// Sets the value of ReserveKey
    pub fn set_reserve_key(&mut self, value: String) {
        self.reserve_key = Some(value);
    }

    /// Gets the value of ReserveKey
    pub fn get_reserve_key(&self) -> Option<&String> {
        self.reserve_key.as_ref()
    }

    /// Sets the value of ReserveLocal
    pub fn set_reserve_local(&mut self, value: u32) {
        self.reserve_local = Some(value);
    }

    /// Gets the value of ReserveLocal
    pub fn get_reserve_local(&self) -> Option<&u32> {
        self.reserve_local.as_ref()
    }

    /// Sets the value of ReserveSource
    pub fn set_reserve_source(&mut self, value: u32) {
        self.reserve_source = Some(value);
    }

    /// Gets the value of ReserveSource
    pub fn get_reserve_source(&self) -> Option<&u32> {
        self.reserve_source.as_ref()
    }
}

