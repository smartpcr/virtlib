// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Chassis struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Chassis {
    #[serde(flatten)]
    pub base: CIM_PhysicalFrame,

/// 
    #[serde(rename = "ChassisTypes")]
    pub chassis_types: Vec<u16>,

/// 
    #[serde(rename = "CurrentRequiredOrProduced")]
    pub current_required_or_produced: Option<i16>,

/// 
    #[serde(rename = "HeatGeneration")]
    pub heat_generation: Option<u16>,

/// 
    #[serde(rename = "NumberOfPowerCords")]
    pub number_of_power_cords: Option<u16>,

/// 
    #[serde(rename = "TypeDescriptions")]
    pub type_descriptions: Vec<String>,
}

impl CIM_Chassis {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PhysicalFrame::new(),
            chassis_types: Vec::new(),
            current_required_or_produced: None,
            heat_generation: None,
            number_of_power_cords: None,
            type_descriptions: Vec::new(),
        }
    }


    /// Sets the value of ChassisTypes
    pub fn set_chassis_types(&mut self, value: Vec<u16>) {
        self.chassis_types = value;
    }

    /// Gets the value of ChassisTypes
    pub fn get_chassis_types(&self) -> &Vec<u16> {
        &self.chassis_types
    }

    /// Sets the value of CurrentRequiredOrProduced
    pub fn set_current_required_or_produced(&mut self, value: i16) {
        self.current_required_or_produced = Some(value);
    }

    /// Gets the value of CurrentRequiredOrProduced
    pub fn get_current_required_or_produced(&self) -> Option<&i16> {
        self.current_required_or_produced.as_ref()
    }

    /// Sets the value of HeatGeneration
    pub fn set_heat_generation(&mut self, value: u16) {
        self.heat_generation = Some(value);
    }

    /// Gets the value of HeatGeneration
    pub fn get_heat_generation(&self) -> Option<&u16> {
        self.heat_generation.as_ref()
    }

    /// Sets the value of NumberOfPowerCords
    pub fn set_number_of_power_cords(&mut self, value: u16) {
        self.number_of_power_cords = Some(value);
    }

    /// Gets the value of NumberOfPowerCords
    pub fn get_number_of_power_cords(&self) -> Option<&u16> {
        self.number_of_power_cords.as_ref()
    }

    /// Sets the value of TypeDescriptions
    pub fn set_type_descriptions(&mut self, value: Vec<String>) {
        self.type_descriptions = value;
    }

    /// Gets the value of TypeDescriptions
    pub fn get_type_descriptions(&self) -> &Vec<String> {
        &self.type_descriptions
    }
}

