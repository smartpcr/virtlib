// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ProcessorPerformance struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessorPerformance {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "frequency")]
    pub frequency: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "percentage")]
    pub percentage: Option<u32>,

/// 
    #[serde(rename = "power")]
    pub power: Option<u32>,
}

impl ProcessorPerformance {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            active: None,
            frequency: None,
            instance_name: None,
            percentage: None,
            power: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of frequency
    pub fn set_frequency(&mut self, value: u32) {
        self.frequency = Some(value);
    }

    /// Gets the value of frequency
    pub fn get_frequency(&self) -> Option<&u32> {
        self.frequency.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of percentage
    pub fn set_percentage(&mut self, value: u32) {
        self.percentage = Some(value);
    }

    /// Gets the value of percentage
    pub fn get_percentage(&self) -> Option<&u32> {
        self.percentage.as_ref()
    }

    /// Sets the value of power
    pub fn set_power(&mut self, value: u32) {
        self.power = Some(value);
    }

    /// Gets the value of power
    pub fn get_power(&self) -> Option<&u32> {
        self.power.as_ref()
    }
}

