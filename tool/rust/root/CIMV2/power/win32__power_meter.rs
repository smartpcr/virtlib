// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.power
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PowerMeter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PowerMeter {
    #[serde(flatten)]
    pub base: CIM_NumericSensor,

/// 
    #[serde(rename = "AveragingInterval")]
    pub averaging_interval: Option<u32>,

/// 
    #[serde(rename = "BudgetEnabled")]
    pub budget_enabled: Option<bool>,

/// 
    #[serde(rename = "BudgetWriteable")]
    pub budget_writeable: Option<bool>,

/// 
    #[serde(rename = "ConfiguredBudget")]
    pub configured_budget: Option<u32>,

/// 
    #[serde(rename = "MaximumAveragingInterval")]
    pub maximum_averaging_interval: Option<u32>,

/// 
    #[serde(rename = "MaxOperatingBudget")]
    pub max_operating_budget: Option<u32>,

/// 
    #[serde(rename = "MeterType")]
    pub meter_type: Option<u32>,

/// 
    #[serde(rename = "MinimumAveragingInterval")]
    pub minimum_averaging_interval: Option<u32>,

/// 
    #[serde(rename = "MinOperatingBudget")]
    pub min_operating_budget: Option<u32>,

/// 
    #[serde(rename = "SamplingPeriod")]
    pub sampling_period: Option<u32>,

/// 
    #[serde(rename = "SupportCapabilities")]
    pub support_capabilities: Option<u32>,
}

impl Win32_PowerMeter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_NumericSensor::new(),
            averaging_interval: None,
            budget_enabled: None,
            budget_writeable: None,
            configured_budget: None,
            maximum_averaging_interval: None,
            max_operating_budget: None,
            meter_type: None,
            minimum_averaging_interval: None,
            min_operating_budget: None,
            sampling_period: None,
            support_capabilities: None,
        }
    }


    /// Sets the value of AveragingInterval
    pub fn set_averaging_interval(&mut self, value: u32) {
        self.averaging_interval = Some(value);
    }

    /// Gets the value of AveragingInterval
    pub fn get_averaging_interval(&self) -> Option<&u32> {
        self.averaging_interval.as_ref()
    }

    /// Sets the value of BudgetEnabled
    pub fn set_budget_enabled(&mut self, value: bool) {
        self.budget_enabled = Some(value);
    }

    /// Gets the value of BudgetEnabled
    pub fn get_budget_enabled(&self) -> Option<&bool> {
        self.budget_enabled.as_ref()
    }

    /// Sets the value of BudgetWriteable
    pub fn set_budget_writeable(&mut self, value: bool) {
        self.budget_writeable = Some(value);
    }

    /// Gets the value of BudgetWriteable
    pub fn get_budget_writeable(&self) -> Option<&bool> {
        self.budget_writeable.as_ref()
    }

    /// Sets the value of ConfiguredBudget
    pub fn set_configured_budget(&mut self, value: u32) {
        self.configured_budget = Some(value);
    }

    /// Gets the value of ConfiguredBudget
    pub fn get_configured_budget(&self) -> Option<&u32> {
        self.configured_budget.as_ref()
    }

    /// Sets the value of MaximumAveragingInterval
    pub fn set_maximum_averaging_interval(&mut self, value: u32) {
        self.maximum_averaging_interval = Some(value);
    }

    /// Gets the value of MaximumAveragingInterval
    pub fn get_maximum_averaging_interval(&self) -> Option<&u32> {
        self.maximum_averaging_interval.as_ref()
    }

    /// Sets the value of MaxOperatingBudget
    pub fn set_max_operating_budget(&mut self, value: u32) {
        self.max_operating_budget = Some(value);
    }

    /// Gets the value of MaxOperatingBudget
    pub fn get_max_operating_budget(&self) -> Option<&u32> {
        self.max_operating_budget.as_ref()
    }

    /// Sets the value of MeterType
    pub fn set_meter_type(&mut self, value: u32) {
        self.meter_type = Some(value);
    }

    /// Gets the value of MeterType
    pub fn get_meter_type(&self) -> Option<&u32> {
        self.meter_type.as_ref()
    }

    /// Sets the value of MinimumAveragingInterval
    pub fn set_minimum_averaging_interval(&mut self, value: u32) {
        self.minimum_averaging_interval = Some(value);
    }

    /// Gets the value of MinimumAveragingInterval
    pub fn get_minimum_averaging_interval(&self) -> Option<&u32> {
        self.minimum_averaging_interval.as_ref()
    }

    /// Sets the value of MinOperatingBudget
    pub fn set_min_operating_budget(&mut self, value: u32) {
        self.min_operating_budget = Some(value);
    }

    /// Gets the value of MinOperatingBudget
    pub fn get_min_operating_budget(&self) -> Option<&u32> {
        self.min_operating_budget.as_ref()
    }

    /// Sets the value of SamplingPeriod
    pub fn set_sampling_period(&mut self, value: u32) {
        self.sampling_period = Some(value);
    }

    /// Gets the value of SamplingPeriod
    pub fn get_sampling_period(&self) -> Option<&u32> {
        self.sampling_period.as_ref()
    }

    /// Sets the value of SupportCapabilities
    pub fn set_support_capabilities(&mut self, value: u32) {
        self.support_capabilities = Some(value);
    }

    /// Gets the value of SupportCapabilities
    pub fn get_support_capabilities(&self) -> Option<&u32> {
        self.support_capabilities.as_ref()
    }
}

