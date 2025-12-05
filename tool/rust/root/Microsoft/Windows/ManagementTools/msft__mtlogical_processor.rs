// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ManagementTools
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MTLogicalProcessor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MTLogicalProcessor {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "CpuId")]
    pub cpu_id: Option<u16>,

/// 
    #[serde(rename = "CurrentIndex")]
    pub current_index: Option<u16>,

/// 
    #[serde(rename = "IntervalSeconds")]
    pub interval_seconds: Option<u16>,

/// 
    #[serde(rename = "NodeId")]
    pub node_id: Option<u16>,

/// 
    #[serde(rename = "Parking")]
    pub parking: Option<bool>,

/// 
    #[serde(rename = "Privileged")]
    pub privileged: Vec<f32>,

/// 
    #[serde(rename = "Utilization")]
    pub utilization: Vec<f32>,
}

impl MSFT_MTLogicalProcessor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            cpu_id: None,
            current_index: None,
            interval_seconds: None,
            node_id: None,
            parking: None,
            privileged: Vec::new(),
            utilization: Vec::new(),
        }
    }


    /// Sets the value of CpuId
    pub fn set_cpu_id(&mut self, value: u16) {
        self.cpu_id = Some(value);
    }

    /// Gets the value of CpuId
    pub fn get_cpu_id(&self) -> Option<&u16> {
        self.cpu_id.as_ref()
    }

    /// Sets the value of CurrentIndex
    pub fn set_current_index(&mut self, value: u16) {
        self.current_index = Some(value);
    }

    /// Gets the value of CurrentIndex
    pub fn get_current_index(&self) -> Option<&u16> {
        self.current_index.as_ref()
    }

    /// Sets the value of IntervalSeconds
    pub fn set_interval_seconds(&mut self, value: u16) {
        self.interval_seconds = Some(value);
    }

    /// Gets the value of IntervalSeconds
    pub fn get_interval_seconds(&self) -> Option<&u16> {
        self.interval_seconds.as_ref()
    }

    /// Sets the value of NodeId
    pub fn set_node_id(&mut self, value: u16) {
        self.node_id = Some(value);
    }

    /// Gets the value of NodeId
    pub fn get_node_id(&self) -> Option<&u16> {
        self.node_id.as_ref()
    }

    /// Sets the value of Parking
    pub fn set_parking(&mut self, value: bool) {
        self.parking = Some(value);
    }

    /// Gets the value of Parking
    pub fn get_parking(&self) -> Option<&bool> {
        self.parking.as_ref()
    }

    /// Sets the value of Privileged
    pub fn set_privileged(&mut self, value: Vec<f32>) {
        self.privileged = value;
    }

    /// Gets the value of Privileged
    pub fn get_privileged(&self) -> &Vec<f32> {
        &self.privileged
    }

    /// Sets the value of Utilization
    pub fn set_utilization(&mut self, value: Vec<f32>) {
        self.utilization = value;
    }

    /// Gets the value of Utilization
    pub fn get_utilization(&self) -> &Vec<f32> {
        &self.utilization
    }
}

