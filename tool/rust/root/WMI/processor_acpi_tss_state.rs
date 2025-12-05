// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ProcessorAcpiTssState struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessorAcpiTssState {

/// 
    #[serde(rename = "Control")]
    pub control: Option<u32>,

/// 
    #[serde(rename = "FreqPercentage")]
    pub freq_percentage: Option<u32>,

/// 
    #[serde(rename = "Power")]
    pub power: Option<u32>,

/// 
    #[serde(rename = "Reserved1")]
    pub reserved1: Option<u64>,

/// 
    #[serde(rename = "Reserved2")]
    pub reserved2: Option<u64>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<u32>,

/// 
    #[serde(rename = "TransitionLatency")]
    pub transition_latency: Option<u32>,
}

impl ProcessorAcpiTssState {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            control: None,
            freq_percentage: None,
            power: None,
            reserved1: None,
            reserved2: None,
            status: None,
            transition_latency: None,
        }
    }


    /// Sets the value of Control
    pub fn set_control(&mut self, value: u32) {
        self.control = Some(value);
    }

    /// Gets the value of Control
    pub fn get_control(&self) -> Option<&u32> {
        self.control.as_ref()
    }

    /// Sets the value of FreqPercentage
    pub fn set_freq_percentage(&mut self, value: u32) {
        self.freq_percentage = Some(value);
    }

    /// Gets the value of FreqPercentage
    pub fn get_freq_percentage(&self) -> Option<&u32> {
        self.freq_percentage.as_ref()
    }

    /// Sets the value of Power
    pub fn set_power(&mut self, value: u32) {
        self.power = Some(value);
    }

    /// Gets the value of Power
    pub fn get_power(&self) -> Option<&u32> {
        self.power.as_ref()
    }

    /// Sets the value of Reserved1
    pub fn set_reserved1(&mut self, value: u64) {
        self.reserved1 = Some(value);
    }

    /// Gets the value of Reserved1
    pub fn get_reserved1(&self) -> Option<&u64> {
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

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u32> {
        self.status.as_ref()
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

