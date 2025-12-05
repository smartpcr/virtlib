// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_NamedJobObject struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_NamedJobObject {
    #[serde(flatten)]
    pub base: CIM_CollectionOfMSEs,

/// 
    #[serde(rename = "BasicUIRestrictions")]
    pub basic_uirestrictions: Option<u32>,
}

impl Win32_NamedJobObject {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_CollectionOfMSEs::new(),
            basic_uirestrictions: None,
        }
    }


    /// Sets the value of BasicUIRestrictions
    pub fn set_basic_uirestrictions(&mut self, value: u32) {
        self.basic_uirestrictions = Some(value);
    }

    /// Gets the value of BasicUIRestrictions
    pub fn get_basic_uirestrictions(&self) -> Option<&u32> {
        self.basic_uirestrictions.as_ref()
    }
}

