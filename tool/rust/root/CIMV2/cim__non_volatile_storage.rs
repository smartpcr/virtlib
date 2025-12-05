// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_NonVolatileStorage struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_NonVolatileStorage {
    #[serde(flatten)]
    pub base: CIM_Memory,

/// 
    #[serde(rename = "IsWriteable")]
    pub is_writeable: Option<bool>,
}

impl CIM_NonVolatileStorage {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Memory::new(),
            is_writeable: None,
        }
    }


    /// Sets the value of IsWriteable
    pub fn set_is_writeable(&mut self, value: bool) {
        self.is_writeable = Some(value);
    }

    /// Gets the value of IsWriteable
    pub fn get_is_writeable(&self) -> Option<&bool> {
        self.is_writeable.as_ref()
    }
}

