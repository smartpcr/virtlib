// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PerformanceStates struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PerformanceStates {

/// 
    #[serde(rename = "Count")]
    pub count: Option<u32>,

/// 
    #[serde(rename = "Current")]
    pub current: Option<u32>,

/// 
    #[serde(rename = "State")]
    pub state: Vec<PerformanceState>,

/// 
    #[serde(rename = "TransitionFunction")]
    pub transition_function: Option<u32>,

/// 
    #[serde(rename = "TransitionLatency")]
    pub transition_latency: Option<u32>,
}

impl PerformanceStates {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            count: None,
            current: None,
            state: Vec::new(),
            transition_function: None,
            transition_latency: None,
        }
    }


    /// Sets the value of Count
    pub fn set_count(&mut self, value: u32) {
        self.count = Some(value);
    }

    /// Gets the value of Count
    pub fn get_count(&self) -> Option<&u32> {
        self.count.as_ref()
    }

    /// Sets the value of Current
    pub fn set_current(&mut self, value: u32) {
        self.current = Some(value);
    }

    /// Gets the value of Current
    pub fn get_current(&self) -> Option<&u32> {
        self.current.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: Vec<PerformanceState>) {
        self.state = value;
    }

    /// Gets the value of State
    pub fn get_state(&self) -> &Vec<PerformanceState> {
        &self.state
    }

    /// Sets the value of TransitionFunction
    pub fn set_transition_function(&mut self, value: u32) {
        self.transition_function = Some(value);
    }

    /// Gets the value of TransitionFunction
    pub fn get_transition_function(&self) -> Option<&u32> {
        self.transition_function.as_ref()
    }

    /// Sets the value of TransitionLatency
    pub fn set_transition_latency(&mut self, value: u32) {
        self.transition_latency = Some(value);
    }

    /// Gets the value of TransitionLatency
    pub fn get_transition_latency(&self) -> Option<&u32> {
        self.transition_latency.as_ref()
    }
}

