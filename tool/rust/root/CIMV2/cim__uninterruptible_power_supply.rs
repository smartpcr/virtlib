// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_UninterruptiblePowerSupply struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_UninterruptiblePowerSupply {
    #[serde(flatten)]
    pub base: CIM_PowerSupply,

/// 
    #[serde(rename = "EstimatedChargeRemaining")]
    pub estimated_charge_remaining: Option<u16>,

/// 
    #[serde(rename = "EstimatedRunTime")]
    pub estimated_run_time: Option<u32>,

/// 
    #[serde(rename = "RemainingCapacityStatus")]
    pub remaining_capacity_status: Option<u16>,

/// 
    #[serde(rename = "TimeOnBackup")]
    pub time_on_backup: Option<u32>,
}

impl CIM_UninterruptiblePowerSupply {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PowerSupply::new(),
            estimated_charge_remaining: None,
            estimated_run_time: None,
            remaining_capacity_status: None,
            time_on_backup: None,
        }
    }


    /// Sets the value of EstimatedChargeRemaining
    pub fn set_estimated_charge_remaining(&mut self, value: u16) {
        self.estimated_charge_remaining = Some(value);
    }

    /// Gets the value of EstimatedChargeRemaining
    pub fn get_estimated_charge_remaining(&self) -> Option<&u16> {
        self.estimated_charge_remaining.as_ref()
    }

    /// Sets the value of EstimatedRunTime
    pub fn set_estimated_run_time(&mut self, value: u32) {
        self.estimated_run_time = Some(value);
    }

    /// Gets the value of EstimatedRunTime
    pub fn get_estimated_run_time(&self) -> Option<&u32> {
        self.estimated_run_time.as_ref()
    }

    /// Sets the value of RemainingCapacityStatus
    pub fn set_remaining_capacity_status(&mut self, value: u16) {
        self.remaining_capacity_status = Some(value);
    }

    /// Gets the value of RemainingCapacityStatus
    pub fn get_remaining_capacity_status(&self) -> Option<&u16> {
        self.remaining_capacity_status.as_ref()
    }

    /// Sets the value of TimeOnBackup
    pub fn set_time_on_backup(&mut self, value: u32) {
        self.time_on_backup = Some(value);
    }

    /// Gets the value of TimeOnBackup
    pub fn get_time_on_backup(&self) -> Option<&u32> {
        self.time_on_backup.as_ref()
    }
}

