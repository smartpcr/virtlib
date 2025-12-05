// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.power
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Sensor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Sensor {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// 
    #[serde(rename = "CurrentState")]
    pub current_state: Option<String>,

/// 
    #[serde(rename = "OtherSensorTypeDescription")]
    pub other_sensor_type_description: Option<String>,

/// 
    #[serde(rename = "PollingInterval")]
    pub polling_interval: Option<u64>,

/// 
    #[serde(rename = "PossibleStates")]
    pub possible_states: Vec<String>,

/// 
    #[serde(rename = "SensorType")]
    pub sensor_type: Option<u16>,
}

impl CIM_Sensor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            current_state: None,
            other_sensor_type_description: None,
            polling_interval: None,
            possible_states: Vec::new(),
            sensor_type: None,
        }
    }


    /// Sets the value of CurrentState
    pub fn set_current_state(&mut self, value: String) {
        self.current_state = Some(value);
    }

    /// Gets the value of CurrentState
    pub fn get_current_state(&self) -> Option<&String> {
        self.current_state.as_ref()
    }

    /// Sets the value of OtherSensorTypeDescription
    pub fn set_other_sensor_type_description(&mut self, value: String) {
        self.other_sensor_type_description = Some(value);
    }

    /// Gets the value of OtherSensorTypeDescription
    pub fn get_other_sensor_type_description(&self) -> Option<&String> {
        self.other_sensor_type_description.as_ref()
    }

    /// Sets the value of PollingInterval
    pub fn set_polling_interval(&mut self, value: u64) {
        self.polling_interval = Some(value);
    }

    /// Gets the value of PollingInterval
    pub fn get_polling_interval(&self) -> Option<&u64> {
        self.polling_interval.as_ref()
    }

    /// Sets the value of PossibleStates
    pub fn set_possible_states(&mut self, value: Vec<String>) {
        self.possible_states = value;
    }

    /// Gets the value of PossibleStates
    pub fn get_possible_states(&self) -> &Vec<String> {
        &self.possible_states
    }

    /// Sets the value of SensorType
    pub fn set_sensor_type(&mut self, value: u16) {
        self.sensor_type = Some(value);
    }

    /// Gets the value of SensorType
    pub fn get_sensor_type(&self) -> Option<&u16> {
        self.sensor_type.as_ref()
    }
}

