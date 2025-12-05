// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RNDISMPStatisticsOID struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RNDISMPStatisticsOID {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// Number of array elements.
    #[serde(rename = "NumberElements")]
    pub number_elements: Option<u32>,

/// Query this array to get statistics.
    #[serde(rename = "StatsArray")]
    pub stats_array: Vec<u32>,
}

impl RNDISMPStatisticsOID {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            instance_name: None,
            number_elements: None,
            stats_array: Vec::new(),
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

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of NumberElements
    pub fn set_number_elements(&mut self, value: u32) {
        self.number_elements = Some(value);
    }

    /// Gets the value of NumberElements
    pub fn get_number_elements(&self) -> Option<&u32> {
        self.number_elements.as_ref()
    }

    /// Sets the value of StatsArray
    pub fn set_stats_array(&mut self, value: Vec<u32>) {
        self.stats_array = value;
    }

    /// Gets the value of StatsArray
    pub fn get_stats_array(&self) -> &Vec<u32> {
        &self.stats_array
    }
}

