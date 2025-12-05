// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_BinarySensor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_BinarySensor {
    #[serde(flatten)]
    pub base: CIM_Sensor,

/// 
    #[serde(rename = "CurrentReading")]
    pub current_reading: Option<bool>,

/// 
    #[serde(rename = "ExpectedReading")]
    pub expected_reading: Option<bool>,

/// 
    #[serde(rename = "InterpretationOfFalse")]
    pub interpretation_of_false: Option<String>,

/// 
    #[serde(rename = "InterpretationOfTrue")]
    pub interpretation_of_true: Option<String>,
}

impl CIM_BinarySensor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Sensor::new(),
            current_reading: None,
            expected_reading: None,
            interpretation_of_false: None,
            interpretation_of_true: None,
        }
    }


    /// Sets the value of CurrentReading
    pub fn set_current_reading(&mut self, value: bool) {
        self.current_reading = Some(value);
    }

    /// Gets the value of CurrentReading
    pub fn get_current_reading(&self) -> Option<&bool> {
        self.current_reading.as_ref()
    }

    /// Sets the value of ExpectedReading
    pub fn set_expected_reading(&mut self, value: bool) {
        self.expected_reading = Some(value);
    }

    /// Gets the value of ExpectedReading
    pub fn get_expected_reading(&self) -> Option<&bool> {
        self.expected_reading.as_ref()
    }

    /// Sets the value of InterpretationOfFalse
    pub fn set_interpretation_of_false(&mut self, value: String) {
        self.interpretation_of_false = Some(value);
    }

    /// Gets the value of InterpretationOfFalse
    pub fn get_interpretation_of_false(&self) -> Option<&String> {
        self.interpretation_of_false.as_ref()
    }

    /// Sets the value of InterpretationOfTrue
    pub fn set_interpretation_of_true(&mut self, value: String) {
        self.interpretation_of_true = Some(value);
    }

    /// Gets the value of InterpretationOfTrue
    pub fn get_interpretation_of_true(&self) -> Option<&String> {
        self.interpretation_of_true.as_ref()
    }
}

