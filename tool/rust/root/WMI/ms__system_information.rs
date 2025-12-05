// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MS_SystemInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MS_SystemInformation {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "BaseBoardManufacturer")]
    pub base_board_manufacturer: Option<String>,

/// 
    #[serde(rename = "BaseBoardProduct")]
    pub base_board_product: Option<String>,

/// 
    #[serde(rename = "BaseBoardVersion")]
    pub base_board_version: Option<String>,

/// 
    #[serde(rename = "BiosMajorRelease")]
    pub bios_major_release: Option<u8>,

/// 
    #[serde(rename = "BiosMinorRelease")]
    pub bios_minor_release: Option<u8>,

/// 
    #[serde(rename = "BIOSReleaseDate")]
    pub biosrelease_date: Option<String>,

/// 
    #[serde(rename = "BIOSVendor")]
    pub biosvendor: Option<String>,

/// 
    #[serde(rename = "BIOSVersion")]
    pub biosversion: Option<String>,

/// 
    #[serde(rename = "ECFirmwareMajorRelease")]
    pub ecfirmware_major_release: Option<u8>,

/// 
    #[serde(rename = "ECFirmwareMinorRelease")]
    pub ecfirmware_minor_release: Option<u8>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "SystemFamily")]
    pub system_family: Option<String>,

/// 
    #[serde(rename = "SystemManufacturer")]
    pub system_manufacturer: Option<String>,

/// 
    #[serde(rename = "SystemProductName")]
    pub system_product_name: Option<String>,

/// 
    #[serde(rename = "SystemSKU")]
    pub system_sku: Option<String>,

/// 
    #[serde(rename = "SystemVersion")]
    pub system_version: Option<String>,
}

impl MS_SystemInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            base_board_manufacturer: None,
            base_board_product: None,
            base_board_version: None,
            bios_major_release: None,
            bios_minor_release: None,
            biosrelease_date: None,
            biosvendor: None,
            biosversion: None,
            ecfirmware_major_release: None,
            ecfirmware_minor_release: None,
            instance_name: None,
            system_family: None,
            system_manufacturer: None,
            system_product_name: None,
            system_sku: None,
            system_version: None,
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

    /// Sets the value of BaseBoardManufacturer
    pub fn set_base_board_manufacturer(&mut self, value: String) {
        self.base_board_manufacturer = Some(value);
    }

    /// Gets the value of BaseBoardManufacturer
    pub fn get_base_board_manufacturer(&self) -> Option<&String> {
        self.base_board_manufacturer.as_ref()
    }

    /// Sets the value of BaseBoardProduct
    pub fn set_base_board_product(&mut self, value: String) {
        self.base_board_product = Some(value);
    }

    /// Gets the value of BaseBoardProduct
    pub fn get_base_board_product(&self) -> Option<&String> {
        self.base_board_product.as_ref()
    }

    /// Sets the value of BaseBoardVersion
    pub fn set_base_board_version(&mut self, value: String) {
        self.base_board_version = Some(value);
    }

    /// Gets the value of BaseBoardVersion
    pub fn get_base_board_version(&self) -> Option<&String> {
        self.base_board_version.as_ref()
    }

    /// Sets the value of BiosMajorRelease
    pub fn set_bios_major_release(&mut self, value: u8) {
        self.bios_major_release = Some(value);
    }

    /// Gets the value of BiosMajorRelease
    pub fn get_bios_major_release(&self) -> Option<&u8> {
        self.bios_major_release.as_ref()
    }

    /// Sets the value of BiosMinorRelease
    pub fn set_bios_minor_release(&mut self, value: u8) {
        self.bios_minor_release = Some(value);
    }

    /// Gets the value of BiosMinorRelease
    pub fn get_bios_minor_release(&self) -> Option<&u8> {
        self.bios_minor_release.as_ref()
    }

    /// Sets the value of BIOSReleaseDate
    pub fn set_biosrelease_date(&mut self, value: String) {
        self.biosrelease_date = Some(value);
    }

    /// Gets the value of BIOSReleaseDate
    pub fn get_biosrelease_date(&self) -> Option<&String> {
        self.biosrelease_date.as_ref()
    }

    /// Sets the value of BIOSVendor
    pub fn set_biosvendor(&mut self, value: String) {
        self.biosvendor = Some(value);
    }

    /// Gets the value of BIOSVendor
    pub fn get_biosvendor(&self) -> Option<&String> {
        self.biosvendor.as_ref()
    }

    /// Sets the value of BIOSVersion
    pub fn set_biosversion(&mut self, value: String) {
        self.biosversion = Some(value);
    }

    /// Gets the value of BIOSVersion
    pub fn get_biosversion(&self) -> Option<&String> {
        self.biosversion.as_ref()
    }

    /// Sets the value of ECFirmwareMajorRelease
    pub fn set_ecfirmware_major_release(&mut self, value: u8) {
        self.ecfirmware_major_release = Some(value);
    }

    /// Gets the value of ECFirmwareMajorRelease
    pub fn get_ecfirmware_major_release(&self) -> Option<&u8> {
        self.ecfirmware_major_release.as_ref()
    }

    /// Sets the value of ECFirmwareMinorRelease
    pub fn set_ecfirmware_minor_release(&mut self, value: u8) {
        self.ecfirmware_minor_release = Some(value);
    }

    /// Gets the value of ECFirmwareMinorRelease
    pub fn get_ecfirmware_minor_release(&self) -> Option<&u8> {
        self.ecfirmware_minor_release.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of SystemFamily
    pub fn set_system_family(&mut self, value: String) {
        self.system_family = Some(value);
    }

    /// Gets the value of SystemFamily
    pub fn get_system_family(&self) -> Option<&String> {
        self.system_family.as_ref()
    }

    /// Sets the value of SystemManufacturer
    pub fn set_system_manufacturer(&mut self, value: String) {
        self.system_manufacturer = Some(value);
    }

    /// Gets the value of SystemManufacturer
    pub fn get_system_manufacturer(&self) -> Option<&String> {
        self.system_manufacturer.as_ref()
    }

    /// Sets the value of SystemProductName
    pub fn set_system_product_name(&mut self, value: String) {
        self.system_product_name = Some(value);
    }

    /// Gets the value of SystemProductName
    pub fn get_system_product_name(&self) -> Option<&String> {
        self.system_product_name.as_ref()
    }

    /// Sets the value of SystemSKU
    pub fn set_system_sku(&mut self, value: String) {
        self.system_sku = Some(value);
    }

    /// Gets the value of SystemSKU
    pub fn get_system_sku(&self) -> Option<&String> {
        self.system_sku.as_ref()
    }

    /// Sets the value of SystemVersion
    pub fn set_system_version(&mut self, value: String) {
        self.system_version = Some(value);
    }

    /// Gets the value of SystemVersion
    pub fn get_system_version(&self) -> Option<&String> {
        self.system_version.as_ref()
    }
}

