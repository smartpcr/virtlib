// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_USBHub struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_USBHub {
    #[serde(flatten)]
    pub base: CIM_USBDevice,

/// 
    #[serde(rename = "GangSwitched")]
    pub gang_switched: Option<bool>,

/// 
    #[serde(rename = "NumberOfPorts")]
    pub number_of_ports: Option<u8>,
}

impl CIM_USBHub {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_USBDevice::new(),
            gang_switched: None,
            number_of_ports: None,
        }
    }


    /// Sets the value of GangSwitched
    pub fn set_gang_switched(&mut self, value: bool) {
        self.gang_switched = Some(value);
    }

    /// Gets the value of GangSwitched
    pub fn get_gang_switched(&self) -> Option<&bool> {
        self.gang_switched.as_ref()
    }

    /// Sets the value of NumberOfPorts
    pub fn set_number_of_ports(&mut self, value: u8) {
        self.number_of_ports = Some(value);
    }

    /// Gets the value of NumberOfPorts
    pub fn get_number_of_ports(&self) -> Option<&u8> {
        self.number_of_ports.as_ref()
    }
}

