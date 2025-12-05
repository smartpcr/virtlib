// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_BootSourceSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_BootSourceSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "BootSourceDescription")]
    pub boot_source_description: Option<String>,

/// 
    #[serde(rename = "BootSourceType")]
    pub boot_source_type: Option<u32>,

/// 
    #[serde(rename = "FirmwareDevicePath")]
    pub firmware_device_path: Option<String>,

/// 
    #[serde(rename = "OptionalData")]
    pub optional_data: Vec<u8>,

/// 
    #[serde(rename = "OtherLocation")]
    pub other_location: Option<String>,
}

impl Msvm_BootSourceSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            boot_source_description: None,
            boot_source_type: None,
            firmware_device_path: None,
            optional_data: Vec::new(),
            other_location: None,
        }
    }


    /// Sets the value of BootSourceDescription
    pub fn set_boot_source_description(&mut self, value: String) {
        self.boot_source_description = Some(value);
    }

    /// Gets the value of BootSourceDescription
    pub fn get_boot_source_description(&self) -> Option<&String> {
        self.boot_source_description.as_ref()
    }

    /// Sets the value of BootSourceType
    pub fn set_boot_source_type(&mut self, value: u32) {
        self.boot_source_type = Some(value);
    }

    /// Gets the value of BootSourceType
    pub fn get_boot_source_type(&self) -> Option<&u32> {
        self.boot_source_type.as_ref()
    }

    /// Sets the value of FirmwareDevicePath
    pub fn set_firmware_device_path(&mut self, value: String) {
        self.firmware_device_path = Some(value);
    }

    /// Gets the value of FirmwareDevicePath
    pub fn get_firmware_device_path(&self) -> Option<&String> {
        self.firmware_device_path.as_ref()
    }

    /// Sets the value of OptionalData
    pub fn set_optional_data(&mut self, value: Vec<u8>) {
        self.optional_data = value;
    }

    /// Gets the value of OptionalData
    pub fn get_optional_data(&self) -> &Vec<u8> {
        &self.optional_data
    }

    /// Sets the value of OtherLocation
    pub fn set_other_location(&mut self, value: String) {
        self.other_location = Some(value);
    }

    /// Gets the value of OtherLocation
    pub fn get_other_location(&self) -> Option<&String> {
        self.other_location.as_ref()
    }
}

impl Msvm_BootSourceSettingData {
    /// Gets the related Msvm_SyntheticEthernetPortSettingData object(s)
    pub fn get_related__synthetic_ethernet_port_setting_data(&self) -> Result<Msvm_SyntheticEthernetPortSettingData, WmiError> {
        self.get_related("Msvm_SyntheticEthernetPortSettingData")
    }

    /// Gets the related Msvm_VirtualSystemSettingData object(s)
    pub fn get_related__virtual_system_setting_data(&self) -> Result<Msvm_VirtualSystemSettingData, WmiError> {
        self.get_related("Msvm_VirtualSystemSettingData")
    }

}

