// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ProcessorAcpiCstState struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessorAcpiCstState {

/// 
    #[serde(rename = "Latency")]
    pub latency: Option<u16>,

/// 
    #[serde(rename = "PowerConsumption")]
    pub power_consumption: Option<u32>,

/// 
    #[serde(rename = "Register")]
    pub register: Option<AcpiGenAddr>,

/// 
    #[serde(rename = "StateType")]
    pub state_type: Option<u8>,
}

impl ProcessorAcpiCstState {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            latency: None,
            power_consumption: None,
            register: None,
            state_type: None,
        }
    }


    /// Sets the value of Latency
    pub fn set_latency(&mut self, value: u16) {
        self.latency = Some(value);
    }

    /// Gets the value of Latency
    pub fn get_latency(&self) -> Option<&u16> {
        self.latency.as_ref()
    }

    /// Sets the value of PowerConsumption
    pub fn set_power_consumption(&mut self, value: u32) {
        self.power_consumption = Some(value);
    }

    /// Gets the value of PowerConsumption
    pub fn get_power_consumption(&self) -> Option<&u32> {
        self.power_consumption.as_ref()
    }

    /// Sets the value of Register
    pub fn set_register(&mut self, value: AcpiGenAddr) {
        self.register = Some(value);
    }

    /// Gets the value of Register
    pub fn get_register(&self) -> Option<&AcpiGenAddr> {
        self.register.as_ref()
    }

    /// Sets the value of StateType
    pub fn set_state_type(&mut self, value: u8) {
        self.state_type = Some(value);
    }

    /// Gets the value of StateType
    pub fn get_state_type(&self) -> Option<&u8> {
        self.state_type.as_ref()
    }
}

