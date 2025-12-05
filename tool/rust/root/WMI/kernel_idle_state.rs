// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// KernelIdleState struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KernelIdleState {

/// 
    #[serde(rename = "Context")]
    pub context: Option<u32>,

/// 
    #[serde(rename = "DemotePercent")]
    pub demote_percent: Option<u8>,

/// 
    #[serde(rename = "IdleHandler")]
    pub idle_handler: Option<u32>,

/// 
    #[serde(rename = "Latency")]
    pub latency: Option<u32>,

/// 
    #[serde(rename = "Power")]
    pub power: Option<u32>,

/// 
    #[serde(rename = "PromotePercent")]
    pub promote_percent: Option<u8>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<u8>,

/// 
    #[serde(rename = "Reserved1")]
    pub reserved1: Option<u32>,

/// 
    #[serde(rename = "StateFlags")]
    pub state_flags: Option<u32>,

/// 
    #[serde(rename = "StateType")]
    pub state_type: Option<u8>,

/// 
    #[serde(rename = "TimeCheck")]
    pub time_check: Option<u32>,
}

impl KernelIdleState {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            context: None,
            demote_percent: None,
            idle_handler: None,
            latency: None,
            power: None,
            promote_percent: None,
            reserved: None,
            reserved1: None,
            state_flags: None,
            state_type: None,
            time_check: None,
        }
    }


    /// Sets the value of Context
    pub fn set_context(&mut self, value: u32) {
        self.context = Some(value);
    }

    /// Gets the value of Context
    pub fn get_context(&self) -> Option<&u32> {
        self.context.as_ref()
    }

    /// Sets the value of DemotePercent
    pub fn set_demote_percent(&mut self, value: u8) {
        self.demote_percent = Some(value);
    }

    /// Gets the value of DemotePercent
    pub fn get_demote_percent(&self) -> Option<&u8> {
        self.demote_percent.as_ref()
    }

    /// Sets the value of IdleHandler
    pub fn set_idle_handler(&mut self, value: u32) {
        self.idle_handler = Some(value);
    }

    /// Gets the value of IdleHandler
    pub fn get_idle_handler(&self) -> Option<&u32> {
        self.idle_handler.as_ref()
    }

    /// Sets the value of Latency
    pub fn set_latency(&mut self, value: u32) {
        self.latency = Some(value);
    }

    /// Gets the value of Latency
    pub fn get_latency(&self) -> Option<&u32> {
        self.latency.as_ref()
    }

    /// Sets the value of Power
    pub fn set_power(&mut self, value: u32) {
        self.power = Some(value);
    }

    /// Gets the value of Power
    pub fn get_power(&self) -> Option<&u32> {
        self.power.as_ref()
    }

    /// Sets the value of PromotePercent
    pub fn set_promote_percent(&mut self, value: u8) {
        self.promote_percent = Some(value);
    }

    /// Gets the value of PromotePercent
    pub fn get_promote_percent(&self) -> Option<&u8> {
        self.promote_percent.as_ref()
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

    /// Sets the value of StateFlags
    pub fn set_state_flags(&mut self, value: u32) {
        self.state_flags = Some(value);
    }

    /// Gets the value of StateFlags
    pub fn get_state_flags(&self) -> Option<&u32> {
        self.state_flags.as_ref()
    }

    /// Sets the value of StateType
    pub fn set_state_type(&mut self, value: u8) {
        self.state_type = Some(value);
    }

    /// Gets the value of StateType
    pub fn get_state_type(&self) -> Option<&u8> {
        self.state_type.as_ref()
    }

    /// Sets the value of TimeCheck
    pub fn set_time_check(&mut self, value: u32) {
        self.time_check = Some(value);
    }

    /// Gets the value of TimeCheck
    pub fn get_time_check(&self) -> Option<&u32> {
        self.time_check.as_ref()
    }
}

