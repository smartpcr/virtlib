// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// KernelPerfState struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KernelPerfState {

/// 
    #[serde(rename = "Control")]
    pub control: Option<u64>,

/// 
    #[serde(rename = "DecreaseLevel")]
    pub decrease_level: Option<u8>,

/// 
    #[serde(rename = "DecreaseTime")]
    pub decrease_time: Option<u32>,

/// 
    #[serde(rename = "Frequency")]
    pub frequency: Option<u32>,

/// 
    #[serde(rename = "HitCount")]
    pub hit_count: Option<u32>,

/// 
    #[serde(rename = "IncreaseLevel")]
    pub increase_level: Option<u8>,

/// 
    #[serde(rename = "IncreaseTime")]
    pub increase_time: Option<u32>,

/// 
    #[serde(rename = "PercentFrequency")]
    pub percent_frequency: Option<u8>,

/// 
    #[serde(rename = "Power")]
    pub power: Option<u32>,

/// 
    #[serde(rename = "Reserved1")]
    pub reserved1: Option<u32>,

/// 
    #[serde(rename = "Reserved2")]
    pub reserved2: Option<u64>,

/// 
    #[serde(rename = "Reserved3")]
    pub reserved3: Option<u64>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<u64>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u8>,
}

impl KernelPerfState {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            control: None,
            decrease_level: None,
            decrease_time: None,
            frequency: None,
            hit_count: None,
            increase_level: None,
            increase_time: None,
            percent_frequency: None,
            power: None,
            reserved1: None,
            reserved2: None,
            reserved3: None,
            status: None,
            type: None,
        }
    }


    /// Sets the value of Control
    pub fn set_control(&mut self, value: u64) {
        self.control = Some(value);
    }

    /// Gets the value of Control
    pub fn get_control(&self) -> Option<&u64> {
        self.control.as_ref()
    }

    /// Sets the value of DecreaseLevel
    pub fn set_decrease_level(&mut self, value: u8) {
        self.decrease_level = Some(value);
    }

    /// Gets the value of DecreaseLevel
    pub fn get_decrease_level(&self) -> Option<&u8> {
        self.decrease_level.as_ref()
    }

    /// Sets the value of DecreaseTime
    pub fn set_decrease_time(&mut self, value: u32) {
        self.decrease_time = Some(value);
    }

    /// Gets the value of DecreaseTime
    pub fn get_decrease_time(&self) -> Option<&u32> {
        self.decrease_time.as_ref()
    }

    /// Sets the value of Frequency
    pub fn set_frequency(&mut self, value: u32) {
        self.frequency = Some(value);
    }

    /// Gets the value of Frequency
    pub fn get_frequency(&self) -> Option<&u32> {
        self.frequency.as_ref()
    }

    /// Sets the value of HitCount
    pub fn set_hit_count(&mut self, value: u32) {
        self.hit_count = Some(value);
    }

    /// Gets the value of HitCount
    pub fn get_hit_count(&self) -> Option<&u32> {
        self.hit_count.as_ref()
    }

    /// Sets the value of IncreaseLevel
    pub fn set_increase_level(&mut self, value: u8) {
        self.increase_level = Some(value);
    }

    /// Gets the value of IncreaseLevel
    pub fn get_increase_level(&self) -> Option<&u8> {
        self.increase_level.as_ref()
    }

    /// Sets the value of IncreaseTime
    pub fn set_increase_time(&mut self, value: u32) {
        self.increase_time = Some(value);
    }

    /// Gets the value of IncreaseTime
    pub fn get_increase_time(&self) -> Option<&u32> {
        self.increase_time.as_ref()
    }

    /// Sets the value of PercentFrequency
    pub fn set_percent_frequency(&mut self, value: u8) {
        self.percent_frequency = Some(value);
    }

    /// Gets the value of PercentFrequency
    pub fn get_percent_frequency(&self) -> Option<&u8> {
        self.percent_frequency.as_ref()
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

    /// Sets the value of Reserved3
    pub fn set_reserved3(&mut self, value: u64) {
        self.reserved3 = Some(value);
    }

    /// Gets the value of Reserved3
    pub fn get_reserved3(&self) -> Option<&u64> {
        self.reserved3.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u64) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u64> {
        self.status.as_ref()
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

