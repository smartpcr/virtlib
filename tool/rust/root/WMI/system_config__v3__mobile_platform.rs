// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V3_MobilePlatform struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V3_MobilePlatform {
    #[serde(flatten)]
    pub base: SystemConfig_V3,

/// 
    #[serde(rename = "BspVersion")]
    pub bsp_version: Option<String>,

/// 
    #[serde(rename = "DeviceManufacturer")]
    pub device_manufacturer: Option<String>,

/// 
    #[serde(rename = "DeviceManufacturerDisplayName")]
    pub device_manufacturer_display_name: Option<String>,

/// 
    #[serde(rename = "DeviceModel")]
    pub device_model: Option<String>,

/// 
    #[serde(rename = "DeviceModelDisplayName")]
    pub device_model_display_name: Option<String>,

/// 
    #[serde(rename = "HardwareVersion")]
    pub hardware_version: Option<String>,

/// 
    #[serde(rename = "MobileOperator")]
    pub mobile_operator: Option<String>,

/// 
    #[serde(rename = "MobileOperatorDisplayName")]
    pub mobile_operator_display_name: Option<String>,

/// 
    #[serde(rename = "OemSoftwareVersion")]
    pub oem_software_version: Option<String>,

/// 
    #[serde(rename = "RadioHardwareVersion")]
    pub radio_hardware_version: Option<String>,

/// 
    #[serde(rename = "RadioSoftwareVersion")]
    pub radio_software_version: Option<String>,

/// 
    #[serde(rename = "SocVersion")]
    pub soc_version: Option<String>,
}

impl SystemConfig_V3_MobilePlatform {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V3::new(),
            bsp_version: None,
            device_manufacturer: None,
            device_manufacturer_display_name: None,
            device_model: None,
            device_model_display_name: None,
            hardware_version: None,
            mobile_operator: None,
            mobile_operator_display_name: None,
            oem_software_version: None,
            radio_hardware_version: None,
            radio_software_version: None,
            soc_version: None,
        }
    }


    /// Sets the value of BspVersion
    pub fn set_bsp_version(&mut self, value: String) {
        self.bsp_version = Some(value);
    }

    /// Gets the value of BspVersion
    pub fn get_bsp_version(&self) -> Option<&String> {
        self.bsp_version.as_ref()
    }

    /// Sets the value of DeviceManufacturer
    pub fn set_device_manufacturer(&mut self, value: String) {
        self.device_manufacturer = Some(value);
    }

    /// Gets the value of DeviceManufacturer
    pub fn get_device_manufacturer(&self) -> Option<&String> {
        self.device_manufacturer.as_ref()
    }

    /// Sets the value of DeviceManufacturerDisplayName
    pub fn set_device_manufacturer_display_name(&mut self, value: String) {
        self.device_manufacturer_display_name = Some(value);
    }

    /// Gets the value of DeviceManufacturerDisplayName
    pub fn get_device_manufacturer_display_name(&self) -> Option<&String> {
        self.device_manufacturer_display_name.as_ref()
    }

    /// Sets the value of DeviceModel
    pub fn set_device_model(&mut self, value: String) {
        self.device_model = Some(value);
    }

    /// Gets the value of DeviceModel
    pub fn get_device_model(&self) -> Option<&String> {
        self.device_model.as_ref()
    }

    /// Sets the value of DeviceModelDisplayName
    pub fn set_device_model_display_name(&mut self, value: String) {
        self.device_model_display_name = Some(value);
    }

    /// Gets the value of DeviceModelDisplayName
    pub fn get_device_model_display_name(&self) -> Option<&String> {
        self.device_model_display_name.as_ref()
    }

    /// Sets the value of HardwareVersion
    pub fn set_hardware_version(&mut self, value: String) {
        self.hardware_version = Some(value);
    }

    /// Gets the value of HardwareVersion
    pub fn get_hardware_version(&self) -> Option<&String> {
        self.hardware_version.as_ref()
    }

    /// Sets the value of MobileOperator
    pub fn set_mobile_operator(&mut self, value: String) {
        self.mobile_operator = Some(value);
    }

    /// Gets the value of MobileOperator
    pub fn get_mobile_operator(&self) -> Option<&String> {
        self.mobile_operator.as_ref()
    }

    /// Sets the value of MobileOperatorDisplayName
    pub fn set_mobile_operator_display_name(&mut self, value: String) {
        self.mobile_operator_display_name = Some(value);
    }

    /// Gets the value of MobileOperatorDisplayName
    pub fn get_mobile_operator_display_name(&self) -> Option<&String> {
        self.mobile_operator_display_name.as_ref()
    }

    /// Sets the value of OemSoftwareVersion
    pub fn set_oem_software_version(&mut self, value: String) {
        self.oem_software_version = Some(value);
    }

    /// Gets the value of OemSoftwareVersion
    pub fn get_oem_software_version(&self) -> Option<&String> {
        self.oem_software_version.as_ref()
    }

    /// Sets the value of RadioHardwareVersion
    pub fn set_radio_hardware_version(&mut self, value: String) {
        self.radio_hardware_version = Some(value);
    }

    /// Gets the value of RadioHardwareVersion
    pub fn get_radio_hardware_version(&self) -> Option<&String> {
        self.radio_hardware_version.as_ref()
    }

    /// Sets the value of RadioSoftwareVersion
    pub fn set_radio_software_version(&mut self, value: String) {
        self.radio_software_version = Some(value);
    }

    /// Gets the value of RadioSoftwareVersion
    pub fn get_radio_software_version(&self) -> Option<&String> {
        self.radio_software_version.as_ref()
    }

    /// Sets the value of SocVersion
    pub fn set_soc_version(&mut self, value: String) {
        self.soc_version = Some(value);
    }

    /// Gets the value of SocVersion
    pub fn get_soc_version(&self) -> Option<&String> {
        self.soc_version.as_ref()
    }
}

