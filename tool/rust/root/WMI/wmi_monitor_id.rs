// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WmiMonitorID struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WmiMonitorID {
    #[serde(flatten)]
    pub base: MSMonitorClass,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "ManufacturerName")]
    pub manufacturer_name: Vec<u16>,

/// 
    #[serde(rename = "ProductCodeID")]
    pub product_code_id: Vec<u16>,

/// 
    #[serde(rename = "SerialNumberID")]
    pub serial_number_id: Vec<u16>,

/// 
    #[serde(rename = "UserFriendlyName")]
    pub user_friendly_name: Vec<u16>,

/// 
    #[serde(rename = "UserFriendlyNameLength")]
    pub user_friendly_name_length: Option<u16>,

/// 
    #[serde(rename = "WeekOfManufacture")]
    pub week_of_manufacture: Option<u8>,

/// 
    #[serde(rename = "YearOfManufacture")]
    pub year_of_manufacture: Option<u16>,
}

impl WmiMonitorID {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSMonitorClass::new(),
            active: None,
            instance_name: None,
            manufacturer_name: Vec::new(),
            product_code_id: Vec::new(),
            serial_number_id: Vec::new(),
            user_friendly_name: Vec::new(),
            user_friendly_name_length: None,
            week_of_manufacture: None,
            year_of_manufacture: None,
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

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of ManufacturerName
    pub fn set_manufacturer_name(&mut self, value: Vec<u16>) {
        self.manufacturer_name = value;
    }

    /// Gets the value of ManufacturerName
    pub fn get_manufacturer_name(&self) -> &Vec<u16> {
        &self.manufacturer_name
    }

    /// Sets the value of ProductCodeID
    pub fn set_product_code_id(&mut self, value: Vec<u16>) {
        self.product_code_id = value;
    }

    /// Gets the value of ProductCodeID
    pub fn get_product_code_id(&self) -> &Vec<u16> {
        &self.product_code_id
    }

    /// Sets the value of SerialNumberID
    pub fn set_serial_number_id(&mut self, value: Vec<u16>) {
        self.serial_number_id = value;
    }

    /// Gets the value of SerialNumberID
    pub fn get_serial_number_id(&self) -> &Vec<u16> {
        &self.serial_number_id
    }

    /// Sets the value of UserFriendlyName
    pub fn set_user_friendly_name(&mut self, value: Vec<u16>) {
        self.user_friendly_name = value;
    }

    /// Gets the value of UserFriendlyName
    pub fn get_user_friendly_name(&self) -> &Vec<u16> {
        &self.user_friendly_name
    }

    /// Sets the value of UserFriendlyNameLength
    pub fn set_user_friendly_name_length(&mut self, value: u16) {
        self.user_friendly_name_length = Some(value);
    }

    /// Gets the value of UserFriendlyNameLength
    pub fn get_user_friendly_name_length(&self) -> Option<&u16> {
        self.user_friendly_name_length.as_ref()
    }

    /// Sets the value of WeekOfManufacture
    pub fn set_week_of_manufacture(&mut self, value: u8) {
        self.week_of_manufacture = Some(value);
    }

    /// Gets the value of WeekOfManufacture
    pub fn get_week_of_manufacture(&self) -> Option<&u8> {
        self.week_of_manufacture.as_ref()
    }

    /// Sets the value of YearOfManufacture
    pub fn set_year_of_manufacture(&mut self, value: u16) {
        self.year_of_manufacture = Some(value);
    }

    /// Gets the value of YearOfManufacture
    pub fn get_year_of_manufacture(&self) -> Option<&u16> {
        self.year_of_manufacture.as_ref()
    }
}

