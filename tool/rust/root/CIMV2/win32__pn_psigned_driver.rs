// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PnPSignedDriver struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PnPSignedDriver {
    #[serde(flatten)]
    pub base: CIM_Service,

/// 
    #[serde(rename = "ClassGuid")]
    pub class_guid: Option<String>,

/// 
    #[serde(rename = "CompatID")]
    pub compat_id: Option<String>,

/// 
    #[serde(rename = "DeviceClass")]
    pub device_class: Option<String>,

/// 
    #[serde(rename = "DeviceID")]
    pub device_id: Option<String>,

/// 
    #[serde(rename = "DeviceName")]
    pub device_name: Option<String>,

/// 
    #[serde(rename = "DevLoader")]
    pub dev_loader: Option<String>,

/// 
    #[serde(rename = "DriverDate")]
    pub driver_date: Option<String>,

/// 
    #[serde(rename = "DriverName")]
    pub driver_name: Option<String>,

/// 
    #[serde(rename = "DriverProviderName")]
    pub driver_provider_name: Option<String>,

/// 
    #[serde(rename = "DriverVersion")]
    pub driver_version: Option<String>,

/// 
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// 
    #[serde(rename = "HardWareID")]
    pub hard_ware_id: Option<String>,

/// 
    #[serde(rename = "InfName")]
    pub inf_name: Option<String>,

/// 
    #[serde(rename = "IsSigned")]
    pub is_signed: Option<bool>,

/// 
    #[serde(rename = "Location")]
    pub location: Option<String>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "PDO")]
    pub pdo: Option<String>,

/// 
    #[serde(rename = "Signer")]
    pub signer: Option<String>,
}

impl Win32_PnPSignedDriver {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Service::new(),
            class_guid: None,
            compat_id: None,
            device_class: None,
            device_id: None,
            device_name: None,
            dev_loader: None,
            driver_date: None,
            driver_name: None,
            driver_provider_name: None,
            driver_version: None,
            friendly_name: None,
            hard_ware_id: None,
            inf_name: None,
            is_signed: None,
            location: None,
            manufacturer: None,
            pdo: None,
            signer: None,
        }
    }


    /// Sets the value of ClassGuid
    pub fn set_class_guid(&mut self, value: String) {
        self.class_guid = Some(value);
    }

    /// Gets the value of ClassGuid
    pub fn get_class_guid(&self) -> Option<&String> {
        self.class_guid.as_ref()
    }

    /// Sets the value of CompatID
    pub fn set_compat_id(&mut self, value: String) {
        self.compat_id = Some(value);
    }

    /// Gets the value of CompatID
    pub fn get_compat_id(&self) -> Option<&String> {
        self.compat_id.as_ref()
    }

    /// Sets the value of DeviceClass
    pub fn set_device_class(&mut self, value: String) {
        self.device_class = Some(value);
    }

    /// Gets the value of DeviceClass
    pub fn get_device_class(&self) -> Option<&String> {
        self.device_class.as_ref()
    }

    /// Sets the value of DeviceID
    pub fn set_device_id(&mut self, value: String) {
        self.device_id = Some(value);
    }

    /// Gets the value of DeviceID
    pub fn get_device_id(&self) -> Option<&String> {
        self.device_id.as_ref()
    }

    /// Sets the value of DeviceName
    pub fn set_device_name(&mut self, value: String) {
        self.device_name = Some(value);
    }

    /// Gets the value of DeviceName
    pub fn get_device_name(&self) -> Option<&String> {
        self.device_name.as_ref()
    }

    /// Sets the value of DevLoader
    pub fn set_dev_loader(&mut self, value: String) {
        self.dev_loader = Some(value);
    }

    /// Gets the value of DevLoader
    pub fn get_dev_loader(&self) -> Option<&String> {
        self.dev_loader.as_ref()
    }

    /// Sets the value of DriverDate
    pub fn set_driver_date(&mut self, value: String) {
        self.driver_date = Some(value);
    }

    /// Gets the value of DriverDate
    pub fn get_driver_date(&self) -> Option<&String> {
        self.driver_date.as_ref()
    }

    /// Sets the value of DriverName
    pub fn set_driver_name(&mut self, value: String) {
        self.driver_name = Some(value);
    }

    /// Gets the value of DriverName
    pub fn get_driver_name(&self) -> Option<&String> {
        self.driver_name.as_ref()
    }

    /// Sets the value of DriverProviderName
    pub fn set_driver_provider_name(&mut self, value: String) {
        self.driver_provider_name = Some(value);
    }

    /// Gets the value of DriverProviderName
    pub fn get_driver_provider_name(&self) -> Option<&String> {
        self.driver_provider_name.as_ref()
    }

    /// Sets the value of DriverVersion
    pub fn set_driver_version(&mut self, value: String) {
        self.driver_version = Some(value);
    }

    /// Gets the value of DriverVersion
    pub fn get_driver_version(&self) -> Option<&String> {
        self.driver_version.as_ref()
    }

    /// Sets the value of FriendlyName
    pub fn set_friendly_name(&mut self, value: String) {
        self.friendly_name = Some(value);
    }

    /// Gets the value of FriendlyName
    pub fn get_friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    /// Sets the value of HardWareID
    pub fn set_hard_ware_id(&mut self, value: String) {
        self.hard_ware_id = Some(value);
    }

    /// Gets the value of HardWareID
    pub fn get_hard_ware_id(&self) -> Option<&String> {
        self.hard_ware_id.as_ref()
    }

    /// Sets the value of InfName
    pub fn set_inf_name(&mut self, value: String) {
        self.inf_name = Some(value);
    }

    /// Gets the value of InfName
    pub fn get_inf_name(&self) -> Option<&String> {
        self.inf_name.as_ref()
    }

    /// Sets the value of IsSigned
    pub fn set_is_signed(&mut self, value: bool) {
        self.is_signed = Some(value);
    }

    /// Gets the value of IsSigned
    pub fn get_is_signed(&self) -> Option<&bool> {
        self.is_signed.as_ref()
    }

    /// Sets the value of Location
    pub fn set_location(&mut self, value: String) {
        self.location = Some(value);
    }

    /// Gets the value of Location
    pub fn get_location(&self) -> Option<&String> {
        self.location.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of PDO
    pub fn set_pdo(&mut self, value: String) {
        self.pdo = Some(value);
    }

    /// Gets the value of PDO
    pub fn get_pdo(&self) -> Option<&String> {
        self.pdo.as_ref()
    }

    /// Sets the value of Signer
    pub fn set_signer(&mut self, value: String) {
        self.signer = Some(value);
    }

    /// Gets the value of Signer
    pub fn get_signer(&self) -> Option<&String> {
        self.signer.as_ref()
    }
}

