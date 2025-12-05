// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// KernelPerfStates struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KernelPerfStates {
    #[serde(flatten)]
    pub base: MSKernelPpmClass,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "BusyAdjThreshold")]
    pub busy_adj_threshold: Option<u8>,

/// 
    #[serde(rename = "Count")]
    pub count: Option<u32>,

/// 
    #[serde(rename = "CurrentState")]
    pub current_state: Option<u32>,

/// 
    #[serde(rename = "FeedbackHandler")]
    pub feedback_handler: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "LowestPerfState")]
    pub lowest_perf_state: Option<u32>,

/// 
    #[serde(rename = "MaxFrequency")]
    pub max_frequency: Option<u32>,

/// 
    #[serde(rename = "MaxPerfState")]
    pub max_perf_state: Option<u32>,

/// 
    #[serde(rename = "MinPerfState")]
    pub min_perf_state: Option<u32>,

/// 
    #[serde(rename = "PolicyType")]
    pub policy_type: Option<u8>,

/// 
    #[serde(rename = "PStateContext")]
    pub pstate_context: Option<u32>,

/// 
    #[serde(rename = "PStateHandler")]
    pub pstate_handler: Option<u32>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<u8>,

/// 
    #[serde(rename = "Reserved1")]
    pub reserved1: Option<u32>,

/// 
    #[serde(rename = "Reserved2")]
    pub reserved2: Option<u64>,

/// 
    #[serde(rename = "State")]
    pub state: Vec<KernelPerfState>,

/// 
    #[serde(rename = "TargetProcessors")]
    pub target_processors: Option<u64>,

/// 
    #[serde(rename = "ThermalConstraint")]
    pub thermal_constraint: Option<u32>,

/// 
    #[serde(rename = "TimerInterval")]
    pub timer_interval: Option<u32>,

/// 
    #[serde(rename = "TStateContext")]
    pub tstate_context: Option<u32>,

/// 
    #[serde(rename = "TStateHandler")]
    pub tstate_handler: Option<u32>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u8>,
}

impl KernelPerfStates {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSKernelPpmClass::new(),
            active: None,
            busy_adj_threshold: None,
            count: None,
            current_state: None,
            feedback_handler: None,
            instance_name: None,
            lowest_perf_state: None,
            max_frequency: None,
            max_perf_state: None,
            min_perf_state: None,
            policy_type: None,
            pstate_context: None,
            pstate_handler: None,
            reserved: None,
            reserved1: None,
            reserved2: None,
            state: Vec::new(),
            target_processors: None,
            thermal_constraint: None,
            timer_interval: None,
            tstate_context: None,
            tstate_handler: None,
            type: None,
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

    /// Sets the value of BusyAdjThreshold
    pub fn set_busy_adj_threshold(&mut self, value: u8) {
        self.busy_adj_threshold = Some(value);
    }

    /// Gets the value of BusyAdjThreshold
    pub fn get_busy_adj_threshold(&self) -> Option<&u8> {
        self.busy_adj_threshold.as_ref()
    }

    /// Sets the value of Count
    pub fn set_count(&mut self, value: u32) {
        self.count = Some(value);
    }

    /// Gets the value of Count
    pub fn get_count(&self) -> Option<&u32> {
        self.count.as_ref()
    }

    /// Sets the value of CurrentState
    pub fn set_current_state(&mut self, value: u32) {
        self.current_state = Some(value);
    }

    /// Gets the value of CurrentState
    pub fn get_current_state(&self) -> Option<&u32> {
        self.current_state.as_ref()
    }

    /// Sets the value of FeedbackHandler
    pub fn set_feedback_handler(&mut self, value: u32) {
        self.feedback_handler = Some(value);
    }

    /// Gets the value of FeedbackHandler
    pub fn get_feedback_handler(&self) -> Option<&u32> {
        self.feedback_handler.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of LowestPerfState
    pub fn set_lowest_perf_state(&mut self, value: u32) {
        self.lowest_perf_state = Some(value);
    }

    /// Gets the value of LowestPerfState
    pub fn get_lowest_perf_state(&self) -> Option<&u32> {
        self.lowest_perf_state.as_ref()
    }

    /// Sets the value of MaxFrequency
    pub fn set_max_frequency(&mut self, value: u32) {
        self.max_frequency = Some(value);
    }

    /// Gets the value of MaxFrequency
    pub fn get_max_frequency(&self) -> Option<&u32> {
        self.max_frequency.as_ref()
    }

    /// Sets the value of MaxPerfState
    pub fn set_max_perf_state(&mut self, value: u32) {
        self.max_perf_state = Some(value);
    }

    /// Gets the value of MaxPerfState
    pub fn get_max_perf_state(&self) -> Option<&u32> {
        self.max_perf_state.as_ref()
    }

    /// Sets the value of MinPerfState
    pub fn set_min_perf_state(&mut self, value: u32) {
        self.min_perf_state = Some(value);
    }

    /// Gets the value of MinPerfState
    pub fn get_min_perf_state(&self) -> Option<&u32> {
        self.min_perf_state.as_ref()
    }

    /// Sets the value of PolicyType
    pub fn set_policy_type(&mut self, value: u8) {
        self.policy_type = Some(value);
    }

    /// Gets the value of PolicyType
    pub fn get_policy_type(&self) -> Option<&u8> {
        self.policy_type.as_ref()
    }

    /// Sets the value of PStateContext
    pub fn set_pstate_context(&mut self, value: u32) {
        self.pstate_context = Some(value);
    }

    /// Gets the value of PStateContext
    pub fn get_pstate_context(&self) -> Option<&u32> {
        self.pstate_context.as_ref()
    }

    /// Sets the value of PStateHandler
    pub fn set_pstate_handler(&mut self, value: u32) {
        self.pstate_handler = Some(value);
    }

    /// Gets the value of PStateHandler
    pub fn get_pstate_handler(&self) -> Option<&u32> {
        self.pstate_handler.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u8) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u8> {
        self.reserved.as_ref()
    }

    /// Sets the value of Reserved1
    pub fn set_reserved1(&mut self, value: u32) {
        self.reserved1 = Some(value);
    }

    /// Gets the value of Reserved1
    pub fn get_reserved1(&self) -> Option<&u32> {
        self.reserved1.as_ref()
    }

    /// Sets the value of Reserved2
    pub fn set_reserved2(&mut self, value: u64) {
        self.reserved2 = Some(value);
    }

    /// Gets the value of Reserved2
    pub fn get_reserved2(&self) -> Option<&u64> {
        self.reserved2.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: Vec<KernelPerfState>) {
        self.state = value;
    }

    /// Gets the value of State
    pub fn get_state(&self) -> &Vec<KernelPerfState> {
        &self.state
    }

    /// Sets the value of TargetProcessors
    pub fn set_target_processors(&mut self, value: u64) {
        self.target_processors = Some(value);
    }

    /// Gets the value of TargetProcessors
    pub fn get_target_processors(&self) -> Option<&u64> {
        self.target_processors.as_ref()
    }

    /// Sets the value of ThermalConstraint
    pub fn set_thermal_constraint(&mut self, value: u32) {
        self.thermal_constraint = Some(value);
    }

    /// Gets the value of ThermalConstraint
    pub fn get_thermal_constraint(&self) -> Option<&u32> {
        self.thermal_constraint.as_ref()
    }

    /// Sets the value of TimerInterval
    pub fn set_timer_interval(&mut self, value: u32) {
        self.timer_interval = Some(value);
    }

    /// Gets the value of TimerInterval
    pub fn get_timer_interval(&self) -> Option<&u32> {
        self.timer_interval.as_ref()
    }

    /// Sets the value of TStateContext
    pub fn set_tstate_context(&mut self, value: u32) {
        self.tstate_context = Some(value);
    }

    /// Gets the value of TStateContext
    pub fn get_tstate_context(&self) -> Option<&u32> {
        self.tstate_context.as_ref()
    }

    /// Sets the value of TStateHandler
    pub fn set_tstate_handler(&mut self, value: u32) {
        self.tstate_handler = Some(value);
    }

    /// Gets the value of TStateHandler
    pub fn get_tstate_handler(&self) -> Option<&u32> {
        self.tstate_handler.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u8) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u8> {
        self.type.as_ref()
    }
}

