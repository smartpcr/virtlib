// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PointingDevice struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PointingDevice {
    #[serde(flatten)]
    pub base: CIM_PointingDevice,

/// 
    #[serde(rename = "DeviceInterface")]
    pub device_interface: Option<u16>,

/// 
    #[serde(rename = "DoubleSpeedThreshold")]
    pub double_speed_threshold: Option<u32>,

/// 
    #[serde(rename = "HardwareType")]
    pub hardware_type: Option<String>,

/// 
    #[serde(rename = "InfFileName")]
    pub inf_file_name: Option<String>,

/// 
    #[serde(rename = "InfSection")]
    pub inf_section: Option<String>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "QuadSpeedThreshold")]
    pub quad_speed_threshold: Option<u32>,

/// 
    #[serde(rename = "SampleRate")]
    pub sample_rate: Option<u32>,

/// 
    #[serde(rename = "Synch")]
    pub synch: Option<u32>,
}

impl Win32_PointingDevice {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PointingDevice::new(),
            device_interface: None,
            double_speed_threshold: None,
            hardware_type: None,
            inf_file_name: None,
            inf_section: None,
            manufacturer: None,
            quad_speed_threshold: None,
            sample_rate: None,
            synch: None,
        }
    }


    /// Sets the value of DeviceInterface
    pub fn set_device_interface(&mut self, value: u16) {
        self.device_interface = Some(value);
    }

    /// Gets the value of DeviceInterface
    pub fn get_device_interface(&self) -> Option<&u16> {
        self.device_interface.as_ref()
    }

    /// Sets the value of DoubleSpeedThreshold
    pub fn set_double_speed_threshold(&mut self, value: u32) {
        self.double_speed_threshold = Some(value);
    }

    /// Gets the value of DoubleSpeedThreshold
    pub fn get_double_speed_threshold(&self) -> Option<&u32> {
        self.double_speed_threshold.as_ref()
    }

    /// Sets the value of HardwareType
    pub fn set_hardware_type(&mut self, value: String) {
        self.hardware_type = Some(value);
    }

    /// Gets the value of HardwareType
    pub fn get_hardware_type(&self) -> Option<&String> {
        self.hardware_type.as_ref()
    }

    /// Sets the value of InfFileName
    pub fn set_inf_file_name(&mut self, value: String) {
        self.inf_file_name = Some(value);
    }

    /// Gets the value of InfFileName
    pub fn get_inf_file_name(&self) -> Option<&String> {
        self.inf_file_name.as_ref()
    }

    /// Sets the value of InfSection
    pub fn set_inf_section(&mut self, value: String) {
        self.inf_section = Some(value);
    }

    /// Gets the value of InfSection
    pub fn get_inf_section(&self) -> Option<&String> {
        self.inf_section.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of QuadSpeedThreshold
    pub fn set_quad_speed_threshold(&mut self, value: u32) {
        self.quad_speed_threshold = Some(value);
    }

    /// Gets the value of QuadSpeedThreshold
    pub fn get_quad_speed_threshold(&self) -> Option<&u32> {
        self.quad_speed_threshold.as_ref()
    }

    /// Sets the value of SampleRate
    pub fn set_sample_rate(&mut self, value: u32) {
        self.sample_rate = Some(value);
    }

    /// Gets the value of SampleRate
    pub fn get_sample_rate(&self) -> Option<&u32> {
        self.sample_rate.as_ref()
    }

    /// Sets the value of Synch
    pub fn set_synch(&mut self, value: u32) {
        self.synch = Some(value);
    }

    /// Gets the value of Synch
    pub fn get_synch(&self) -> Option<&u32> {
        self.synch.as_ref()
    }
}

