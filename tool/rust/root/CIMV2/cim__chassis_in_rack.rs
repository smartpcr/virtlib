// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ChassisInRack struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ChassisInRack {
    #[serde(flatten)]
    pub base: CIM_Container,

/// 
    #[serde(rename = "BottomU")]
    pub bottom_u: Option<u16>,
}

impl CIM_ChassisInRack {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Container::new(),
            bottom_u: None,
        }
    }


    /// Sets the value of BottomU
    pub fn set_bottom_u(&mut self, value: u16) {
        self.bottom_u = Some(value);
    }

    /// Gets the value of BottomU
    pub fn get_bottom_u(&self) -> Option<&u16> {
        self.bottom_u.as_ref()
    }
}

