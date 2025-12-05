// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_DeviceConnection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_DeviceConnection {
    #[serde(flatten)]
    pub base: CIM_Dependency,

/// 
    #[serde(rename = "NegotiatedDataWidth")]
    pub negotiated_data_width: Option<u32>,

/// 
    #[serde(rename = "NegotiatedSpeed")]
    pub negotiated_speed: Option<u64>,
}

impl CIM_DeviceConnection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Dependency::new(),
            negotiated_data_width: None,
            negotiated_speed: None,
        }
    }


    /// Sets the value of NegotiatedDataWidth
    pub fn set_negotiated_data_width(&mut self, value: u32) {
        self.negotiated_data_width = Some(value);
    }

    /// Gets the value of NegotiatedDataWidth
    pub fn get_negotiated_data_width(&self) -> Option<&u32> {
        self.negotiated_data_width.as_ref()
    }

    /// Sets the value of NegotiatedSpeed
    pub fn set_negotiated_speed(&mut self, value: u64) {
        self.negotiated_speed = Some(value);
    }

    /// Gets the value of NegotiatedSpeed
    pub fn get_negotiated_speed(&self) -> Option<&u64> {
        self.negotiated_speed.as_ref()
    }
}

