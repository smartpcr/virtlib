// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_PlannedComputerSystem struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_PlannedComputerSystem {
    #[serde(flatten)]
    pub base: CIM_ComputerSystem,

/// 
    #[serde(rename = "AssignedNumaNodeList")]
    pub assigned_numa_node_list: Vec<u16>,

/// 
    #[serde(rename = "OnTimeInMilliseconds")]
    pub on_time_in_milliseconds: Option<u64>,

/// 
    #[serde(rename = "ProcessID")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "TimeOfLastConfigurationChange")]
    pub time_of_last_configuration_change: Option<String>,
}

impl Msvm_PlannedComputerSystem {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ComputerSystem::new(),
            assigned_numa_node_list: Vec::new(),
            on_time_in_milliseconds: None,
            process_id: None,
            time_of_last_configuration_change: None,
        }
    }


    /// Sets the value of AssignedNumaNodeList
    pub fn set_assigned_numa_node_list(&mut self, value: Vec<u16>) {
        self.assigned_numa_node_list = value;
    }

    /// Gets the value of AssignedNumaNodeList
    pub fn get_assigned_numa_node_list(&self) -> &Vec<u16> {
        &self.assigned_numa_node_list
    }

    /// Sets the value of OnTimeInMilliseconds
    pub fn set_on_time_in_milliseconds(&mut self, value: u64) {
        self.on_time_in_milliseconds = Some(value);
    }

    /// Gets the value of OnTimeInMilliseconds
    pub fn get_on_time_in_milliseconds(&self) -> Option<&u64> {
        self.on_time_in_milliseconds.as_ref()
    }

    /// Sets the value of ProcessID
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessID
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }

    /// Sets the value of TimeOfLastConfigurationChange
    pub fn set_time_of_last_configuration_change(&mut self, value: String) {
        self.time_of_last_configuration_change = Some(value);
    }

    /// Gets the value of TimeOfLastConfigurationChange
    pub fn get_time_of_last_configuration_change(&self) -> Option<&String> {
        self.time_of_last_configuration_change.as_ref()
    }
}

