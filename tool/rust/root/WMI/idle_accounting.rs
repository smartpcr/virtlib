// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// IdleAccounting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IdleAccounting {
    #[serde(flatten)]
    pub base: WMIEvent,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "ResetCount")]
    pub reset_count: Option<u32>,

/// 
    #[serde(rename = "StartTime")]
    pub start_time: Option<u64>,

/// 
    #[serde(rename = "State")]
    pub state: Vec<IdleStateAccounting>,

/// 
    #[serde(rename = "StateCount")]
    pub state_count: Option<u32>,

/// 
    #[serde(rename = "TotalTransitions")]
    pub total_transitions: Option<u32>,
}

impl IdleAccounting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WMIEvent::new(),
            active: None,
            instance_name: None,
            reset_count: None,
            start_time: None,
            state: Vec::new(),
            state_count: None,
            total_transitions: None,
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

    /// Sets the value of ResetCount
    pub fn set_reset_count(&mut self, value: u32) {
        self.reset_count = Some(value);
    }

    /// Gets the value of ResetCount
    pub fn get_reset_count(&self) -> Option<&u32> {
        self.reset_count.as_ref()
    }

    /// Sets the value of StartTime
    pub fn set_start_time(&mut self, value: u64) {
        self.start_time = Some(value);
    }

    /// Gets the value of StartTime
    pub fn get_start_time(&self) -> Option<&u64> {
        self.start_time.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: Vec<IdleStateAccounting>) {
        self.state = value;
    }

    /// Gets the value of State
    pub fn get_state(&self) -> &Vec<IdleStateAccounting> {
        &self.state
    }

    /// Sets the value of StateCount
    pub fn set_state_count(&mut self, value: u32) {
        self.state_count = Some(value);
    }

    /// Gets the value of StateCount
    pub fn get_state_count(&self) -> Option<&u32> {
        self.state_count.as_ref()
    }

    /// Sets the value of TotalTransitions
    pub fn set_total_transitions(&mut self, value: u32) {
        self.total_transitions = Some(value);
    }

    /// Gets the value of TotalTransitions
    pub fn get_total_transitions(&self) -> Option<&u32> {
        self.total_transitions.as_ref()
    }
}

