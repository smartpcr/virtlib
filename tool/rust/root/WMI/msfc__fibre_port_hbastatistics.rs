// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFC_FibrePortHBAStatistics struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFC_FibrePortHBAStatistics {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "HBAStatus")]
    pub hbastatus: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "Statistics")]
    pub statistics: Option<MSFC_HBAPortStatistics>,

/// 
    #[serde(rename = "UniquePortId")]
    pub unique_port_id: Option<u64>,
}

impl MSFC_FibrePortHBAStatistics {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            hbastatus: None,
            instance_name: None,
            statistics: None,
            unique_port_id: None,
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

    /// Sets the value of HBAStatus
    pub fn set_hbastatus(&mut self, value: u32) {
        self.hbastatus = Some(value);
    }

    /// Gets the value of HBAStatus
    pub fn get_hbastatus(&self) -> Option<&u32> {
        self.hbastatus.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of Statistics
    pub fn set_statistics(&mut self, value: MSFC_HBAPortStatistics) {
        self.statistics = Some(value);
    }

    /// Gets the value of Statistics
    pub fn get_statistics(&self) -> Option<&MSFC_HBAPortStatistics> {
        self.statistics.as_ref()
    }

    /// Sets the value of UniquePortId
    pub fn set_unique_port_id(&mut self, value: u64) {
        self.unique_port_id = Some(value);
    }

    /// Gets the value of UniquePortId
    pub fn get_unique_port_id(&self) -> Option<&u64> {
        self.unique_port_id.as_ref()
    }
}

