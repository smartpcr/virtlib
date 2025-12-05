// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Slot struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Slot {
    #[serde(flatten)]
    pub base: CIM_PhysicalConnector,

/// 
    #[serde(rename = "HeightAllowed")]
    pub height_allowed: Option<f32>,

/// 
    #[serde(rename = "LengthAllowed")]
    pub length_allowed: Option<f32>,

/// 
    #[serde(rename = "MaxDataWidth")]
    pub max_data_width: Option<u16>,

/// 
    #[serde(rename = "Number")]
    pub number: Option<u16>,

/// 
    #[serde(rename = "PurposeDescription")]
    pub purpose_description: Option<String>,

/// 
    #[serde(rename = "SpecialPurpose")]
    pub special_purpose: Option<bool>,

/// 
    #[serde(rename = "SupportsHotPlug")]
    pub supports_hot_plug: Option<bool>,

/// 
    #[serde(rename = "ThermalRating")]
    pub thermal_rating: Option<u32>,

/// 
    #[serde(rename = "VccMixedVoltageSupport")]
    pub vcc_mixed_voltage_support: Vec<u16>,

/// 
    #[serde(rename = "VppMixedVoltageSupport")]
    pub vpp_mixed_voltage_support: Vec<u16>,
}

impl CIM_Slot {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PhysicalConnector::new(),
            height_allowed: None,
            length_allowed: None,
            max_data_width: None,
            number: None,
            purpose_description: None,
            special_purpose: None,
            supports_hot_plug: None,
            thermal_rating: None,
            vcc_mixed_voltage_support: Vec::new(),
            vpp_mixed_voltage_support: Vec::new(),
        }
    }


    /// Sets the value of HeightAllowed
    pub fn set_height_allowed(&mut self, value: f32) {
        self.height_allowed = Some(value);
    }

    /// Gets the value of HeightAllowed
    pub fn get_height_allowed(&self) -> Option<&f32> {
        self.height_allowed.as_ref()
    }

    /// Sets the value of LengthAllowed
    pub fn set_length_allowed(&mut self, value: f32) {
        self.length_allowed = Some(value);
    }

    /// Gets the value of LengthAllowed
    pub fn get_length_allowed(&self) -> Option<&f32> {
        self.length_allowed.as_ref()
    }

    /// Sets the value of MaxDataWidth
    pub fn set_max_data_width(&mut self, value: u16) {
        self.max_data_width = Some(value);
    }

    /// Gets the value of MaxDataWidth
    pub fn get_max_data_width(&self) -> Option<&u16> {
        self.max_data_width.as_ref()
    }

    /// Sets the value of Number
    pub fn set_number(&mut self, value: u16) {
        self.number = Some(value);
    }

    /// Gets the value of Number
    pub fn get_number(&self) -> Option<&u16> {
        self.number.as_ref()
    }

    /// Sets the value of PurposeDescription
    pub fn set_purpose_description(&mut self, value: String) {
        self.purpose_description = Some(value);
    }

    /// Gets the value of PurposeDescription
    pub fn get_purpose_description(&self) -> Option<&String> {
        self.purpose_description.as_ref()
    }

    /// Sets the value of SpecialPurpose
    pub fn set_special_purpose(&mut self, value: bool) {
        self.special_purpose = Some(value);
    }

    /// Gets the value of SpecialPurpose
    pub fn get_special_purpose(&self) -> Option<&bool> {
        self.special_purpose.as_ref()
    }

    /// Sets the value of SupportsHotPlug
    pub fn set_supports_hot_plug(&mut self, value: bool) {
        self.supports_hot_plug = Some(value);
    }

    /// Gets the value of SupportsHotPlug
    pub fn get_supports_hot_plug(&self) -> Option<&bool> {
        self.supports_hot_plug.as_ref()
    }

    /// Sets the value of ThermalRating
    pub fn set_thermal_rating(&mut self, value: u32) {
        self.thermal_rating = Some(value);
    }

    /// Gets the value of ThermalRating
    pub fn get_thermal_rating(&self) -> Option<&u32> {
        self.thermal_rating.as_ref()
    }

    /// Sets the value of VccMixedVoltageSupport
    pub fn set_vcc_mixed_voltage_support(&mut self, value: Vec<u16>) {
        self.vcc_mixed_voltage_support = value;
    }

    /// Gets the value of VccMixedVoltageSupport
    pub fn get_vcc_mixed_voltage_support(&self) -> &Vec<u16> {
        &self.vcc_mixed_voltage_support
    }

    /// Sets the value of VppMixedVoltageSupport
    pub fn set_vpp_mixed_voltage_support(&mut self, value: Vec<u16>) {
        self.vpp_mixed_voltage_support = value;
    }

    /// Gets the value of VppMixedVoltageSupport
    pub fn get_vpp_mixed_voltage_support(&self) -> &Vec<u16> {
        &self.vpp_mixed_voltage_support
    }
}

