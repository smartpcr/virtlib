// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V2_Platform struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V2_Platform {
    #[serde(flatten)]
    pub base: SystemConfig_V2,

/// 
    #[serde(rename = "BiosDate")]
    pub bios_date: Option<String>,

/// 
    #[serde(rename = "BiosVersion")]
    pub bios_version: Option<String>,

/// 
    #[serde(rename = "SystemManufacturer")]
    pub system_manufacturer: Option<String>,

/// 
    #[serde(rename = "SystemProductName")]
    pub system_product_name: Option<String>,
}

impl SystemConfig_V2_Platform {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V2::new(),
            bios_date: None,
            bios_version: None,
            system_manufacturer: None,
            system_product_name: None,
        }
    }


    /// Sets the value of BiosDate
    pub fn set_bios_date(&mut self, value: String) {
        self.bios_date = Some(value);
    }

    /// Gets the value of BiosDate
    pub fn get_bios_date(&self) -> Option<&String> {
        self.bios_date.as_ref()
    }

    /// Sets the value of BiosVersion
    pub fn set_bios_version(&mut self, value: String) {
        self.bios_version = Some(value);
    }

    /// Gets the value of BiosVersion
    pub fn get_bios_version(&self) -> Option<&String> {
        self.bios_version.as_ref()
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
}

