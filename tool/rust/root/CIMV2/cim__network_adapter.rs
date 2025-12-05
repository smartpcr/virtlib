// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_NetworkAdapter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_NetworkAdapter {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// 
    #[serde(rename = "AutoSense")]
    pub auto_sense: Option<bool>,

/// 
    #[serde(rename = "MaxSpeed")]
    pub max_speed: Option<u64>,

/// 
    #[serde(rename = "NetworkAddresses")]
    pub network_addresses: Vec<String>,

/// 
    #[serde(rename = "PermanentAddress")]
    pub permanent_address: Option<String>,

/// 
    #[serde(rename = "Speed")]
    pub speed: Option<u64>,
}

impl CIM_NetworkAdapter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            auto_sense: None,
            max_speed: None,
            network_addresses: Vec::new(),
            permanent_address: None,
            speed: None,
        }
    }


    /// Sets the value of AutoSense
    pub fn set_auto_sense(&mut self, value: bool) {
        self.auto_sense = Some(value);
    }

    /// Gets the value of AutoSense
    pub fn get_auto_sense(&self) -> Option<&bool> {
        self.auto_sense.as_ref()
    }

    /// Sets the value of MaxSpeed
    pub fn set_max_speed(&mut self, value: u64) {
        self.max_speed = Some(value);
    }

    /// Gets the value of MaxSpeed
    pub fn get_max_speed(&self) -> Option<&u64> {
        self.max_speed.as_ref()
    }

    /// Sets the value of NetworkAddresses
    pub fn set_network_addresses(&mut self, value: Vec<String>) {
        self.network_addresses = value;
    }

    /// Gets the value of NetworkAddresses
    pub fn get_network_addresses(&self) -> &Vec<String> {
        &self.network_addresses
    }

    /// Sets the value of PermanentAddress
    pub fn set_permanent_address(&mut self, value: String) {
        self.permanent_address = Some(value);
    }

    /// Gets the value of PermanentAddress
    pub fn get_permanent_address(&self) -> Option<&String> {
        self.permanent_address.as_ref()
    }

    /// Sets the value of Speed
    pub fn set_speed(&mut self, value: u64) {
        self.speed = Some(value);
    }

    /// Gets the value of Speed
    pub fn get_speed(&self) -> Option<&u64> {
        self.speed.as_ref()
    }
}

