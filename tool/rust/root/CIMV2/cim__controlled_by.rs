// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ControlledBy struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ControlledBy {
    #[serde(flatten)]
    pub base: CIM_DeviceConnection,

/// 
    #[serde(rename = "AccessState")]
    pub access_state: Option<u16>,

/// 
    #[serde(rename = "NumberOfHardResets")]
    pub number_of_hard_resets: Option<u32>,

/// 
    #[serde(rename = "NumberOfSoftResets")]
    pub number_of_soft_resets: Option<u32>,
}

impl CIM_ControlledBy {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_DeviceConnection::new(),
            access_state: None,
            number_of_hard_resets: None,
            number_of_soft_resets: None,
        }
    }


    /// Sets the value of AccessState
    pub fn set_access_state(&mut self, value: u16) {
        self.access_state = Some(value);
    }

    /// Gets the value of AccessState
    pub fn get_access_state(&self) -> Option<&u16> {
        self.access_state.as_ref()
    }

    /// Sets the value of NumberOfHardResets
    pub fn set_number_of_hard_resets(&mut self, value: u32) {
        self.number_of_hard_resets = Some(value);
    }

    /// Gets the value of NumberOfHardResets
    pub fn get_number_of_hard_resets(&self) -> Option<&u32> {
        self.number_of_hard_resets.as_ref()
    }

    /// Sets the value of NumberOfSoftResets
    pub fn set_number_of_soft_resets(&mut self, value: u32) {
        self.number_of_soft_resets = Some(value);
    }

    /// Gets the value of NumberOfSoftResets
    pub fn get_number_of_soft_resets(&self) -> Option<&u32> {
        self.number_of_soft_resets.as_ref()
    }
}

