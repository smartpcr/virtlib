// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Battery struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Battery {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// 
    #[serde(rename = "BatteryStatus")]
    pub battery_status: Option<u16>,

/// 
    #[serde(rename = "Chemistry")]
    pub chemistry: Option<u16>,

/// 
    #[serde(rename = "DesignCapacity")]
    pub design_capacity: Option<u32>,

/// 
    #[serde(rename = "DesignVoltage")]
    pub design_voltage: Option<u64>,

/// 
    #[serde(rename = "EstimatedChargeRemaining")]
    pub estimated_charge_remaining: Option<u16>,

/// 
    #[serde(rename = "EstimatedRunTime")]
    pub estimated_run_time: Option<u32>,

/// 
    #[serde(rename = "ExpectedLife")]
    pub expected_life: Option<u32>,

/// 
    #[serde(rename = "FullChargeCapacity")]
    pub full_charge_capacity: Option<u32>,

/// 
    #[serde(rename = "MaxRechargeTime")]
    pub max_recharge_time: Option<u32>,

/// 
    #[serde(rename = "SmartBatteryVersion")]
    pub smart_battery_version: Option<String>,

/// 
    #[serde(rename = "TimeOnBattery")]
    pub time_on_battery: Option<u32>,

/// 
    #[serde(rename = "TimeToFullCharge")]
    pub time_to_full_charge: Option<u32>,
}

impl CIM_Battery {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            battery_status: None,
            chemistry: None,
            design_capacity: None,
            design_voltage: None,
            estimated_charge_remaining: None,
            estimated_run_time: None,
            expected_life: None,
            full_charge_capacity: None,
            max_recharge_time: None,
            smart_battery_version: None,
            time_on_battery: None,
            time_to_full_charge: None,
        }
    }


    /// Sets the value of BatteryStatus
    pub fn set_battery_status(&mut self, value: u16) {
        self.battery_status = Some(value);
    }

    /// Gets the value of BatteryStatus
    pub fn get_battery_status(&self) -> Option<&u16> {
        self.battery_status.as_ref()
    }

    /// Sets the value of Chemistry
    pub fn set_chemistry(&mut self, value: u16) {
        self.chemistry = Some(value);
    }

    /// Gets the value of Chemistry
    pub fn get_chemistry(&self) -> Option<&u16> {
        self.chemistry.as_ref()
    }

    /// Sets the value of DesignCapacity
    pub fn set_design_capacity(&mut self, value: u32) {
        self.design_capacity = Some(value);
    }

    /// Gets the value of DesignCapacity
    pub fn get_design_capacity(&self) -> Option<&u32> {
        self.design_capacity.as_ref()
    }

    /// Sets the value of DesignVoltage
    pub fn set_design_voltage(&mut self, value: u64) {
        self.design_voltage = Some(value);
    }

    /// Gets the value of DesignVoltage
    pub fn get_design_voltage(&self) -> Option<&u64> {
        self.design_voltage.as_ref()
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

    /// Sets the value of ExpectedLife
    pub fn set_expected_life(&mut self, value: u32) {
        self.expected_life = Some(value);
    }

    /// Gets the value of ExpectedLife
    pub fn get_expected_life(&self) -> Option<&u32> {
        self.expected_life.as_ref()
    }

    /// Sets the value of FullChargeCapacity
    pub fn set_full_charge_capacity(&mut self, value: u32) {
        self.full_charge_capacity = Some(value);
    }

    /// Gets the value of FullChargeCapacity
    pub fn get_full_charge_capacity(&self) -> Option<&u32> {
        self.full_charge_capacity.as_ref()
    }

    /// Sets the value of MaxRechargeTime
    pub fn set_max_recharge_time(&mut self, value: u32) {
        self.max_recharge_time = Some(value);
    }

    /// Gets the value of MaxRechargeTime
    pub fn get_max_recharge_time(&self) -> Option<&u32> {
        self.max_recharge_time.as_ref()
    }

    /// Sets the value of SmartBatteryVersion
    pub fn set_smart_battery_version(&mut self, value: String) {
        self.smart_battery_version = Some(value);
    }

    /// Gets the value of SmartBatteryVersion
    pub fn get_smart_battery_version(&self) -> Option<&String> {
        self.smart_battery_version.as_ref()
    }

    /// Sets the value of TimeOnBattery
    pub fn set_time_on_battery(&mut self, value: u32) {
        self.time_on_battery = Some(value);
    }

    /// Gets the value of TimeOnBattery
    pub fn get_time_on_battery(&self) -> Option<&u32> {
        self.time_on_battery.as_ref()
    }

    /// Sets the value of TimeToFullCharge
    pub fn set_time_to_full_charge(&mut self, value: u32) {
        self.time_to_full_charge = Some(value);
    }

    /// Gets the value of TimeToFullCharge
    pub fn get_time_to_full_charge(&self) -> Option<&u32> {
        self.time_to_full_charge.as_ref()
    }
}

