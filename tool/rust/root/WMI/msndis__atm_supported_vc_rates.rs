// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_AtmSupportedVcRates struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_AtmSupportedVcRates {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "MaxCellRate")]
    pub max_cell_rate: Option<u32>,

/// 
    #[serde(rename = "MinCellRate")]
    pub min_cell_rate: Option<u32>,
}

impl MSNdis_AtmSupportedVcRates {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            active: None,
            instance_name: None,
            max_cell_rate: None,
            min_cell_rate: None,
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

    /// Sets the value of MaxCellRate
    pub fn set_max_cell_rate(&mut self, value: u32) {
        self.max_cell_rate = Some(value);
    }

    /// Gets the value of MaxCellRate
    pub fn get_max_cell_rate(&self) -> Option<&u32> {
        self.max_cell_rate.as_ref()
    }

    /// Sets the value of MinCellRate
    pub fn set_min_cell_rate(&mut self, value: u32) {
        self.min_cell_rate = Some(value);
    }

    /// Gets the value of MinCellRate
    pub fn get_min_cell_rate(&self) -> Option<&u32> {
        self.min_cell_rate.as_ref()
    }
}

