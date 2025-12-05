// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WmiMonitorListedSupportedSourceModes struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WmiMonitorListedSupportedSourceModes {
    #[serde(flatten)]
    pub base: MSMonitorClass,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "MonitorSourceModes")]
    pub monitor_source_modes: Vec<VideoModeDescriptor>,

/// 
    #[serde(rename = "NumOfMonitorSourceModes")]
    pub num_of_monitor_source_modes: Option<u16>,

/// 
    #[serde(rename = "PreferredMonitorSourceModeIndex")]
    pub preferred_monitor_source_mode_index: Option<u16>,
}

impl WmiMonitorListedSupportedSourceModes {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSMonitorClass::new(),
            active: None,
            instance_name: None,
            monitor_source_modes: Vec::new(),
            num_of_monitor_source_modes: None,
            preferred_monitor_source_mode_index: None,
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

    /// Sets the value of MonitorSourceModes
    pub fn set_monitor_source_modes(&mut self, value: Vec<VideoModeDescriptor>) {
        self.monitor_source_modes = value;
    }

    /// Gets the value of MonitorSourceModes
    pub fn get_monitor_source_modes(&self) -> &Vec<VideoModeDescriptor> {
        &self.monitor_source_modes
    }

    /// Sets the value of NumOfMonitorSourceModes
    pub fn set_num_of_monitor_source_modes(&mut self, value: u16) {
        self.num_of_monitor_source_modes = Some(value);
    }

    /// Gets the value of NumOfMonitorSourceModes
    pub fn get_num_of_monitor_source_modes(&self) -> Option<&u16> {
        self.num_of_monitor_source_modes.as_ref()
    }

    /// Sets the value of PreferredMonitorSourceModeIndex
    pub fn set_preferred_monitor_source_mode_index(&mut self, value: u16) {
        self.preferred_monitor_source_mode_index = Some(value);
    }

    /// Gets the value of PreferredMonitorSourceModeIndex
    pub fn get_preferred_monitor_source_mode_index(&self) -> Option<&u16> {
        self.preferred_monitor_source_mode_index.as_ref()
    }
}

