// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.power
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_PowerSupply struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_PowerSupply {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// 
    #[serde(rename = "ActiveInputVoltage")]
    pub active_input_voltage: Option<u16>,

/// 
    #[serde(rename = "IsSwitchingSupply")]
    pub is_switching_supply: Option<bool>,

/// 
    #[serde(rename = "Range1InputFrequencyHigh")]
    pub range1_input_frequency_high: Option<u32>,

/// 
    #[serde(rename = "Range1InputFrequencyLow")]
    pub range1_input_frequency_low: Option<u32>,

/// 
    #[serde(rename = "Range1InputVoltageHigh")]
    pub range1_input_voltage_high: Option<u32>,

/// 
    #[serde(rename = "Range1InputVoltageLow")]
    pub range1_input_voltage_low: Option<u32>,

/// 
    #[serde(rename = "Range2InputFrequencyHigh")]
    pub range2_input_frequency_high: Option<u32>,

/// 
    #[serde(rename = "Range2InputFrequencyLow")]
    pub range2_input_frequency_low: Option<u32>,

/// 
    #[serde(rename = "Range2InputVoltageHigh")]
    pub range2_input_voltage_high: Option<u32>,

/// 
    #[serde(rename = "Range2InputVoltageLow")]
    pub range2_input_voltage_low: Option<u32>,

/// 
    #[serde(rename = "TotalOutputPower")]
    pub total_output_power: Option<u32>,

/// 
    #[serde(rename = "TypeOfRangeSwitching")]
    pub type_of_range_switching: Option<u16>,
}

impl CIM_PowerSupply {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            active_input_voltage: None,
            is_switching_supply: None,
            range1_input_frequency_high: None,
            range1_input_frequency_low: None,
            range1_input_voltage_high: None,
            range1_input_voltage_low: None,
            range2_input_frequency_high: None,
            range2_input_frequency_low: None,
            range2_input_voltage_high: None,
            range2_input_voltage_low: None,
            total_output_power: None,
            type_of_range_switching: None,
        }
    }


    /// Sets the value of ActiveInputVoltage
    pub fn set_active_input_voltage(&mut self, value: u16) {
        self.active_input_voltage = Some(value);
    }

    /// Gets the value of ActiveInputVoltage
    pub fn get_active_input_voltage(&self) -> Option<&u16> {
        self.active_input_voltage.as_ref()
    }

    /// Sets the value of IsSwitchingSupply
    pub fn set_is_switching_supply(&mut self, value: bool) {
        self.is_switching_supply = Some(value);
    }

    /// Gets the value of IsSwitchingSupply
    pub fn get_is_switching_supply(&self) -> Option<&bool> {
        self.is_switching_supply.as_ref()
    }

    /// Sets the value of Range1InputFrequencyHigh
    pub fn set_range1_input_frequency_high(&mut self, value: u32) {
        self.range1_input_frequency_high = Some(value);
    }

    /// Gets the value of Range1InputFrequencyHigh
    pub fn get_range1_input_frequency_high(&self) -> Option<&u32> {
        self.range1_input_frequency_high.as_ref()
    }

    /// Sets the value of Range1InputFrequencyLow
    pub fn set_range1_input_frequency_low(&mut self, value: u32) {
        self.range1_input_frequency_low = Some(value);
    }

    /// Gets the value of Range1InputFrequencyLow
    pub fn get_range1_input_frequency_low(&self) -> Option<&u32> {
        self.range1_input_frequency_low.as_ref()
    }

    /// Sets the value of Range1InputVoltageHigh
    pub fn set_range1_input_voltage_high(&mut self, value: u32) {
        self.range1_input_voltage_high = Some(value);
    }

    /// Gets the value of Range1InputVoltageHigh
    pub fn get_range1_input_voltage_high(&self) -> Option<&u32> {
        self.range1_input_voltage_high.as_ref()
    }

    /// Sets the value of Range1InputVoltageLow
    pub fn set_range1_input_voltage_low(&mut self, value: u32) {
        self.range1_input_voltage_low = Some(value);
    }

    /// Gets the value of Range1InputVoltageLow
    pub fn get_range1_input_voltage_low(&self) -> Option<&u32> {
        self.range1_input_voltage_low.as_ref()
    }

    /// Sets the value of Range2InputFrequencyHigh
    pub fn set_range2_input_frequency_high(&mut self, value: u32) {
        self.range2_input_frequency_high = Some(value);
    }

    /// Gets the value of Range2InputFrequencyHigh
    pub fn get_range2_input_frequency_high(&self) -> Option<&u32> {
        self.range2_input_frequency_high.as_ref()
    }

    /// Sets the value of Range2InputFrequencyLow
    pub fn set_range2_input_frequency_low(&mut self, value: u32) {
        self.range2_input_frequency_low = Some(value);
    }

    /// Gets the value of Range2InputFrequencyLow
    pub fn get_range2_input_frequency_low(&self) -> Option<&u32> {
        self.range2_input_frequency_low.as_ref()
    }

    /// Sets the value of Range2InputVoltageHigh
    pub fn set_range2_input_voltage_high(&mut self, value: u32) {
        self.range2_input_voltage_high = Some(value);
    }

    /// Gets the value of Range2InputVoltageHigh
    pub fn get_range2_input_voltage_high(&self) -> Option<&u32> {
        self.range2_input_voltage_high.as_ref()
    }

    /// Sets the value of Range2InputVoltageLow
    pub fn set_range2_input_voltage_low(&mut self, value: u32) {
        self.range2_input_voltage_low = Some(value);
    }

    /// Gets the value of Range2InputVoltageLow
    pub fn get_range2_input_voltage_low(&self) -> Option<&u32> {
        self.range2_input_voltage_low.as_ref()
    }

    /// Sets the value of TotalOutputPower
    pub fn set_total_output_power(&mut self, value: u32) {
        self.total_output_power = Some(value);
    }

    /// Gets the value of TotalOutputPower
    pub fn get_total_output_power(&self) -> Option<&u32> {
        self.total_output_power.as_ref()
    }

    /// Sets the value of TypeOfRangeSwitching
    pub fn set_type_of_range_switching(&mut self, value: u16) {
        self.type_of_range_switching = Some(value);
    }

    /// Gets the value of TypeOfRangeSwitching
    pub fn get_type_of_range_switching(&self) -> Option<&u16> {
        self.type_of_range_switching.as_ref()
    }
}

