// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_DiscreteSensor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_DiscreteSensor {
    #[serde(flatten)]
    pub base: CIM_Sensor,

/// 
    #[serde(rename = "AcceptableValues")]
    pub acceptable_values: Vec<String>,

/// 
    #[serde(rename = "CurrentReading")]
    pub current_reading: Option<String>,

/// 
    #[serde(rename = "PossibleValues")]
    pub possible_values: Vec<String>,
}

impl CIM_DiscreteSensor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Sensor::new(),
            acceptable_values: Vec::new(),
            current_reading: None,
            possible_values: Vec::new(),
        }
    }


    /// Sets the value of AcceptableValues
    pub fn set_acceptable_values(&mut self, value: Vec<String>) {
        self.acceptable_values = value;
    }

    /// Gets the value of AcceptableValues
    pub fn get_acceptable_values(&self) -> &Vec<String> {
        &self.acceptable_values
    }

    /// Sets the value of CurrentReading
    pub fn set_current_reading(&mut self, value: String) {
        self.current_reading = Some(value);
    }

    /// Gets the value of CurrentReading
    pub fn get_current_reading(&self) -> Option<&String> {
        self.current_reading.as_ref()
    }

    /// Sets the value of PossibleValues
    pub fn set_possible_values(&mut self, value: Vec<String>) {
        self.possible_values = value;
    }

    /// Gets the value of PossibleValues
    pub fn get_possible_values(&self) -> &Vec<String> {
        &self.possible_values
    }
}

