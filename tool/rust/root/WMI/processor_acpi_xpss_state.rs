// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ProcessorAcpiXpssState struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessorAcpiXpssState {

/// 
    #[serde(rename = "BmLatency")]
    pub bm_latency: Option<u32>,

/// 
    #[serde(rename = "Control")]
    pub control: Option<u64>,

/// 
    #[serde(rename = "ControlMask")]
    pub control_mask: Option<u64>,

/// 
    #[serde(rename = "Frequency")]
    pub frequency: Option<u32>,

/// 
    #[serde(rename = "Latency")]
    pub latency: Option<u32>,

/// 
    #[serde(rename = "Power")]
    pub power: Option<u32>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<u64>,

/// 
    #[serde(rename = "StatusMask")]
    pub status_mask: Option<u64>,
}

impl ProcessorAcpiXpssState {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            bm_latency: None,
            control: None,
            control_mask: None,
            frequency: None,
            latency: None,
            power: None,
            status: None,
            status_mask: None,
        }
    }


    /// Sets the value of BmLatency
    pub fn set_bm_latency(&mut self, value: u32) {
        self.bm_latency = Some(value);
    }

    /// Gets the value of BmLatency
    pub fn get_bm_latency(&self) -> Option<&u32> {
        self.bm_latency.as_ref()
    }

    /// Sets the value of Control
    pub fn set_control(&mut self, value: u64) {
        self.control = Some(value);
    }

    /// Gets the value of Control
    pub fn get_control(&self) -> Option<&u64> {
        self.control.as_ref()
    }

    /// Sets the value of ControlMask
    pub fn set_control_mask(&mut self, value: u64) {
        self.control_mask = Some(value);
    }

    /// Gets the value of ControlMask
    pub fn get_control_mask(&self) -> Option<&u64> {
        self.control_mask.as_ref()
    }

    /// Sets the value of Frequency
    pub fn set_frequency(&mut self, value: u32) {
        self.frequency = Some(value);
    }

    /// Gets the value of Frequency
    pub fn get_frequency(&self) -> Option<&u32> {
        self.frequency.as_ref()
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

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u64) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u64> {
        self.status.as_ref()
    }

    /// Sets the value of StatusMask
    pub fn set_status_mask(&mut self, value: u64) {
        self.status_mask = Some(value);
    }

    /// Gets the value of StatusMask
    pub fn get_status_mask(&self) -> Option<&u64> {
        self.status_mask.as_ref()
    }
}

