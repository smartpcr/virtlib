// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WmiMonitorListedFrequencyRanges struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WmiMonitorListedFrequencyRanges {
    #[serde(flatten)]
    pub base: MSMonitorClass,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "MonitorFreqRanges")]
    pub monitor_freq_ranges: Vec<FrequencyRangeDescriptor>,

/// 
    #[serde(rename = "NumOfMonitorFreqRanges")]
    pub num_of_monitor_freq_ranges: Option<u16>,
}

impl WmiMonitorListedFrequencyRanges {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSMonitorClass::new(),
            active: None,
            instance_name: None,
            monitor_freq_ranges: Vec::new(),
            num_of_monitor_freq_ranges: None,
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

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of MonitorFreqRanges
    pub fn set_monitor_freq_ranges(&mut self, value: Vec<FrequencyRangeDescriptor>) {
        self.monitor_freq_ranges = value;
    }

    /// Gets the value of MonitorFreqRanges
    pub fn get_monitor_freq_ranges(&self) -> &Vec<FrequencyRangeDescriptor> {
        &self.monitor_freq_ranges
    }

    /// Sets the value of NumOfMonitorFreqRanges
    pub fn set_num_of_monitor_freq_ranges(&mut self, value: u16) {
        self.num_of_monitor_freq_ranges = Some(value);
    }

    /// Gets the value of NumOfMonitorFreqRanges
    pub fn get_num_of_monitor_freq_ranges(&self) -> Option<&u16> {
        self.num_of_monitor_freq_ranges.as_ref()
    }
}

