// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V2_MobilePlatform struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V2_MobilePlatform {
    #[serde(flatten)]
    pub base: SystemConfig_V2,

/// 
    #[serde(rename = "BootLoaderVersion")]
    pub boot_loader_version: Option<String>,

/// 
    #[serde(rename = "FirmwareRevision")]
    pub firmware_revision: Option<String>,

/// 
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// 
    #[serde(rename = "HardwareRevision")]
    pub hardware_revision: Option<String>,

/// 
    #[serde(rename = "HardwareVariant")]
    pub hardware_variant: Option<String>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "ManufacturerDisplayName")]
    pub manufacturer_display_name: Option<String>,

/// 
    #[serde(rename = "ManufacturerModelName")]
    pub manufacturer_model_name: Option<String>,

/// 
    #[serde(rename = "MobileOperatorDisplayName")]
    pub mobile_operator_display_name: Option<String>,

/// 
    #[serde(rename = "MobileOperatorName")]
    pub mobile_operator_name: Option<String>,

/// 
    #[serde(rename = "ModelName")]
    pub model_name: Option<String>,

/// 
    #[serde(rename = "RadioHardwareRevision")]
    pub radio_hardware_revision: Option<String>,

/// 
    #[serde(rename = "RadioSoftwareRevision")]
    pub radio_software_revision: Option<String>,

/// 
    #[serde(rename = "ROMVersion")]
    pub romversion: Option<String>,

/// 
    #[serde(rename = "SOCVersion")]
    pub socversion: Option<String>,
}

impl SystemConfig_V2_MobilePlatform {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V2::new(),
            boot_loader_version: None,
            firmware_revision: None,
            friendly_name: None,
            hardware_revision: None,
            hardware_variant: None,
            manufacturer: None,
            manufacturer_display_name: None,
            manufacturer_model_name: None,
            mobile_operator_display_name: None,
            mobile_operator_name: None,
            model_name: None,
            radio_hardware_revision: None,
            radio_software_revision: None,
            romversion: None,
            socversion: None,
        }
    }


    /// Sets the value of BootLoaderVersion
    pub fn set_boot_loader_version(&mut self, value: String) {
        self.boot_loader_version = Some(value);
    }

    /// Gets the value of BootLoaderVersion
    pub fn get_boot_loader_version(&self) -> Option<&String> {
        self.boot_loader_version.as_ref()
    }

    /// Sets the value of FirmwareRevision
    pub fn set_firmware_revision(&mut self, value: String) {
        self.firmware_revision = Some(value);
    }

    /// Gets the value of FirmwareRevision
    pub fn get_firmware_revision(&self) -> Option<&String> {
        self.firmware_revision.as_ref()
    }

    /// Sets the value of FriendlyName
    pub fn set_friendly_name(&mut self, value: String) {
        self.friendly_name = Some(value);
    }

    /// Gets the value of FriendlyName
    pub fn get_friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    /// Sets the value of HardwareRevision
    pub fn set_hardware_revision(&mut self, value: String) {
        self.hardware_revision = Some(value);
    }

    /// Gets the value of HardwareRevision
    pub fn get_hardware_revision(&self) -> Option<&String> {
        self.hardware_revision.as_ref()
    }

    /// Sets the value of HardwareVariant
    pub fn set_hardware_variant(&mut self, value: String) {
        self.hardware_variant = Some(value);
    }

    /// Gets the value of HardwareVariant
    pub fn get_hardware_variant(&self) -> Option<&String> {
        self.hardware_variant.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of ManufacturerDisplayName
    pub fn set_manufacturer_display_name(&mut self, value: String) {
        self.manufacturer_display_name = Some(value);
    }

    /// Gets the value of ManufacturerDisplayName
    pub fn get_manufacturer_display_name(&self) -> Option<&String> {
        self.manufacturer_display_name.as_ref()
    }

    /// Sets the value of ManufacturerModelName
    pub fn set_manufacturer_model_name(&mut self, value: String) {
        self.manufacturer_model_name = Some(value);
    }

    /// Gets the value of ManufacturerModelName
    pub fn get_manufacturer_model_name(&self) -> Option<&String> {
        self.manufacturer_model_name.as_ref()
    }

    /// Sets the value of MobileOperatorDisplayName
    pub fn set_mobile_operator_display_name(&mut self, value: String) {
        self.mobile_operator_display_name = Some(value);
    }

    /// Gets the value of MobileOperatorDisplayName
    pub fn get_mobile_operator_display_name(&self) -> Option<&String> {
        self.mobile_operator_display_name.as_ref()
    }

    /// Sets the value of MobileOperatorName
    pub fn set_mobile_operator_name(&mut self, value: String) {
        self.mobile_operator_name = Some(value);
    }

    /// Gets the value of MobileOperatorName
    pub fn get_mobile_operator_name(&self) -> Option<&String> {
        self.mobile_operator_name.as_ref()
    }

    /// Sets the value of ModelName
    pub fn set_model_name(&mut self, value: String) {
        self.model_name = Some(value);
    }

    /// Gets the value of ModelName
    pub fn get_model_name(&self) -> Option<&String> {
        self.model_name.as_ref()
    }

    /// Sets the value of RadioHardwareRevision
    pub fn set_radio_hardware_revision(&mut self, value: String) {
        self.radio_hardware_revision = Some(value);
    }

    /// Gets the value of RadioHardwareRevision
    pub fn get_radio_hardware_revision(&self) -> Option<&String> {
        self.radio_hardware_revision.as_ref()
    }

    /// Sets the value of RadioSoftwareRevision
    pub fn set_radio_software_revision(&mut self, value: String) {
        self.radio_software_revision = Some(value);
    }

    /// Gets the value of RadioSoftwareRevision
    pub fn get_radio_software_revision(&self) -> Option<&String> {
        self.radio_software_revision.as_ref()
    }

    /// Sets the value of ROMVersion
    pub fn set_romversion(&mut self, value: String) {
        self.romversion = Some(value);
    }

    /// Gets the value of ROMVersion
    pub fn get_romversion(&self) -> Option<&String> {
        self.romversion.as_ref()
    }

    /// Sets the value of SOCVersion
    pub fn set_socversion(&mut self, value: String) {
        self.socversion = Some(value);
    }

    /// Gets the value of SOCVersion
    pub fn get_socversion(&self) -> Option<&String> {
        self.socversion.as_ref()
    }
}

