// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ProcessorStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessorStatus {
    #[serde(flatten)]
    pub base: MSProcessorClass,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "CurrentPerfState")]
    pub current_perf_state: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "LastRequestedThrottle")]
    pub last_requested_throttle: Option<u32>,

/// 
    #[serde(rename = "LastTransitionResult")]
    pub last_transition_result: Option<u32>,

/// 
    #[serde(rename = "LowestPerfState")]
    pub lowest_perf_state: Option<u32>,

/// 
    #[serde(rename = "PerfStates")]
    pub perf_states: Option<PerformanceStates>,

/// 
    #[serde(rename = "ThrottleValue")]
    pub throttle_value: Option<u32>,

/// 
    #[serde(rename = "UsingLegacyInterface")]
    pub using_legacy_interface: Option<u32>,
}

impl ProcessorStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSProcessorClass::new(),
            active: None,
            current_perf_state: None,
            instance_name: None,
            last_requested_throttle: None,
            last_transition_result: None,
            lowest_perf_state: None,
            perf_states: None,
            throttle_value: None,
            using_legacy_interface: None,
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

    /// Sets the value of CurrentPerfState
    pub fn set_current_perf_state(&mut self, value: u32) {
        self.current_perf_state = Some(value);
    }

    /// Gets the value of CurrentPerfState
    pub fn get_current_perf_state(&self) -> Option<&u32> {
        self.current_perf_state.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of LastRequestedThrottle
    pub fn set_last_requested_throttle(&mut self, value: u32) {
        self.last_requested_throttle = Some(value);
    }

    /// Gets the value of LastRequestedThrottle
    pub fn get_last_requested_throttle(&self) -> Option<&u32> {
        self.last_requested_throttle.as_ref()
    }

    /// Sets the value of LastTransitionResult
    pub fn set_last_transition_result(&mut self, value: u32) {
        self.last_transition_result = Some(value);
    }

    /// Gets the value of LastTransitionResult
    pub fn get_last_transition_result(&self) -> Option<&u32> {
        self.last_transition_result.as_ref()
    }

    /// Sets the value of LowestPerfState
    pub fn set_lowest_perf_state(&mut self, value: u32) {
        self.lowest_perf_state = Some(value);
    }

    /// Gets the value of LowestPerfState
    pub fn get_lowest_perf_state(&self) -> Option<&u32> {
        self.lowest_perf_state.as_ref()
    }

    /// Sets the value of PerfStates
    pub fn set_perf_states(&mut self, value: PerformanceStates) {
        self.perf_states = Some(value);
    }

    /// Gets the value of PerfStates
    pub fn get_perf_states(&self) -> Option<&PerformanceStates> {
        self.perf_states.as_ref()
    }

    /// Sets the value of ThrottleValue
    pub fn set_throttle_value(&mut self, value: u32) {
        self.throttle_value = Some(value);
    }

    /// Gets the value of ThrottleValue
    pub fn get_throttle_value(&self) -> Option<&u32> {
        self.throttle_value.as_ref()
    }

    /// Sets the value of UsingLegacyInterface
    pub fn set_using_legacy_interface(&mut self, value: u32) {
        self.using_legacy_interface = Some(value);
    }

    /// Gets the value of UsingLegacyInterface
    pub fn get_using_legacy_interface(&self) -> Option<&u32> {
        self.using_legacy_interface.as_ref()
    }
}

