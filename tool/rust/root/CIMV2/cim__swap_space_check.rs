// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SwapSpaceCheck struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SwapSpaceCheck {
    #[serde(flatten)]
    pub base: CIM_Check,

/// 
    #[serde(rename = "SwapSpaceSize")]
    pub swap_space_size: Option<u64>,
}

impl CIM_SwapSpaceCheck {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Check::new(),
            swap_space_size: None,
        }
    }


    /// Sets the value of SwapSpaceSize
    pub fn set_swap_space_size(&mut self, value: u64) {
        self.swap_space_size = Some(value);
    }

    /// Gets the value of SwapSpaceSize
    pub fn get_swap_space_size(&self) -> Option<&u64> {
        self.swap_space_size.as_ref()
    }
}

