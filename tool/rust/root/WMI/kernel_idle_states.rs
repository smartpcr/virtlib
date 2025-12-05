// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// KernelIdleStates struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KernelIdleStates {
    #[serde(flatten)]
    pub base: MSKernelPpmClass,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "Count")]
    pub count: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "OldState")]
    pub old_state: Option<u32>,

/// 
    #[serde(rename = "State")]
    pub state: Vec<KernelIdleState>,

/// 
    #[serde(rename = "TargetProcessors")]
    pub target_processors: Option<u64>,

/// 
    #[serde(rename = "TargetState")]
    pub target_state: Option<u32>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u32>,
}

impl KernelIdleStates {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSKernelPpmClass::new(),
            active: None,
            count: None,
            instance_name: None,
            old_state: None,
            state: Vec::new(),
            target_processors: None,
            target_state: None,
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

    /// Sets the value of Count
    pub fn set_count(&mut self, value: u32) {
        self.count = Some(value);
    }

    /// Gets the value of Count
    pub fn get_count(&self) -> Option<&u32> {
        self.count.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of OldState
    pub fn set_old_state(&mut self, value: u32) {
        self.old_state = Some(value);
    }

    /// Gets the value of OldState
    pub fn get_old_state(&self) -> Option<&u32> {
        self.old_state.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: Vec<KernelIdleState>) {
        self.state = value;
    }

    /// Gets the value of State
    pub fn get_state(&self) -> &Vec<KernelIdleState> {
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

    /// Sets the value of TargetState
    pub fn set_target_state(&mut self, value: u32) {
        self.target_state = Some(value);
    }

    /// Gets the value of TargetState
    pub fn get_target_state(&self) -> Option<&u32> {
        self.target_state.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u32) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u32> {
        self.type.as_ref()
    }
}

