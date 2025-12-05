// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_BIOSElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_BIOSElement {
    #[serde(flatten)]
    pub base: CIM_BIOSElement,

/// 
    #[serde(rename = "BaseBoardSerialNumber")]
    pub base_board_serial_number: Option<String>,

/// 
    #[serde(rename = "BIOSGUID")]
    pub biosguid: Option<String>,

/// 
    #[serde(rename = "BIOSNumLock")]
    pub biosnum_lock: Option<bool>,

/// 
    #[serde(rename = "BIOSSerialNumber")]
    pub biosserial_number: Option<String>,

/// 
    #[serde(rename = "BootOrder")]
    pub boot_order: Vec<u16>,

/// 
    #[serde(rename = "BootPciExpress")]
    pub boot_pci_express: Option<bool>,

/// 
    #[serde(rename = "BootPciExpressInstanceFilter")]
    pub boot_pci_express_instance_filter: Option<String>,

/// 
    #[serde(rename = "ChassisAssetTag")]
    pub chassis_asset_tag: Option<String>,

/// 
    #[serde(rename = "ChassisSerialNumber")]
    pub chassis_serial_number: Option<String>,

/// 
    #[serde(rename = "EnableHibernation")]
    pub enable_hibernation: Option<bool>,

/// 
    #[serde(rename = "WatchdogEnabled")]
    pub watchdog_enabled: Option<bool>,
}

impl Msvm_BIOSElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_BIOSElement::new(),
            base_board_serial_number: None,
            biosguid: None,
            biosnum_lock: None,
            biosserial_number: None,
            boot_order: Vec::new(),
            boot_pci_express: None,
            boot_pci_express_instance_filter: None,
            chassis_asset_tag: None,
            chassis_serial_number: None,
            enable_hibernation: None,
            watchdog_enabled: None,
        }
    }


    /// Sets the value of BaseBoardSerialNumber
    pub fn set_base_board_serial_number(&mut self, value: String) {
        self.base_board_serial_number = Some(value);
    }

    /// Gets the value of BaseBoardSerialNumber
    pub fn get_base_board_serial_number(&self) -> Option<&String> {
        self.base_board_serial_number.as_ref()
    }

    /// Sets the value of BIOSGUID
    pub fn set_biosguid(&mut self, value: String) {
        self.biosguid = Some(value);
    }

    /// Gets the value of BIOSGUID
    pub fn get_biosguid(&self) -> Option<&String> {
        self.biosguid.as_ref()
    }

    /// Sets the value of BIOSNumLock
    pub fn set_biosnum_lock(&mut self, value: bool) {
        self.biosnum_lock = Some(value);
    }

    /// Gets the value of BIOSNumLock
    pub fn get_biosnum_lock(&self) -> Option<&bool> {
        self.biosnum_lock.as_ref()
    }

    /// Sets the value of BIOSSerialNumber
    pub fn set_biosserial_number(&mut self, value: String) {
        self.biosserial_number = Some(value);
    }

    /// Gets the value of BIOSSerialNumber
    pub fn get_biosserial_number(&self) -> Option<&String> {
        self.biosserial_number.as_ref()
    }

    /// Sets the value of BootOrder
    pub fn set_boot_order(&mut self, value: Vec<u16>) {
        self.boot_order = value;
    }

    /// Gets the value of BootOrder
    pub fn get_boot_order(&self) -> &Vec<u16> {
        &self.boot_order
    }

    /// Sets the value of BootPciExpress
    pub fn set_boot_pci_express(&mut self, value: bool) {
        self.boot_pci_express = Some(value);
    }

    /// Gets the value of BootPciExpress
    pub fn get_boot_pci_express(&self) -> Option<&bool> {
        self.boot_pci_express.as_ref()
    }

    /// Sets the value of BootPciExpressInstanceFilter
    pub fn set_boot_pci_express_instance_filter(&mut self, value: String) {
        self.boot_pci_express_instance_filter = Some(value);
    }

    /// Gets the value of BootPciExpressInstanceFilter
    pub fn get_boot_pci_express_instance_filter(&self) -> Option<&String> {
        self.boot_pci_express_instance_filter.as_ref()
    }

    /// Sets the value of ChassisAssetTag
    pub fn set_chassis_asset_tag(&mut self, value: String) {
        self.chassis_asset_tag = Some(value);
    }

    /// Gets the value of ChassisAssetTag
    pub fn get_chassis_asset_tag(&self) -> Option<&String> {
        self.chassis_asset_tag.as_ref()
    }

    /// Sets the value of ChassisSerialNumber
    pub fn set_chassis_serial_number(&mut self, value: String) {
        self.chassis_serial_number = Some(value);
    }

    /// Gets the value of ChassisSerialNumber
    pub fn get_chassis_serial_number(&self) -> Option<&String> {
        self.chassis_serial_number.as_ref()
    }

    /// Sets the value of EnableHibernation
    pub fn set_enable_hibernation(&mut self, value: bool) {
        self.enable_hibernation = Some(value);
    }

    /// Gets the value of EnableHibernation
    pub fn get_enable_hibernation(&self) -> Option<&bool> {
        self.enable_hibernation.as_ref()
    }

    /// Sets the value of WatchdogEnabled
    pub fn set_watchdog_enabled(&mut self, value: bool) {
        self.watchdog_enabled = Some(value);
    }

    /// Gets the value of WatchdogEnabled
    pub fn get_watchdog_enabled(&self) -> Option<&bool> {
        self.watchdog_enabled.as_ref()
    }
}

