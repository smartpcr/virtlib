// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageFaultDomain struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageFaultDomain {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// 
    #[serde(rename = "HealthStatus")]
    pub health_status: Option<u16>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "Model")]
    pub model: Option<String>,

/// 
    #[serde(rename = "OperationalDetails")]
    pub operational_details: Vec<String>,

/// 
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<u16>,

/// 
    #[serde(rename = "PhysicalLocation")]
    pub physical_location: Option<String>,

/// 
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,
}

impl MSFT_StorageFaultDomain {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            description: None,
            friendly_name: None,
            health_status: None,
            manufacturer: None,
            model: None,
            operational_details: Vec::new(),
            operational_status: Vec::new(),
            physical_location: None,
            serial_number: None,
        }
    }


    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of FriendlyName
    pub fn set_friendly_name(&mut self, value: String) {
        self.friendly_name = Some(value);
    }

    /// Gets the value of FriendlyName
    pub fn get_friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    /// Sets the value of HealthStatus
    pub fn set_health_status(&mut self, value: u16) {
        self.health_status = Some(value);
    }

    /// Gets the value of HealthStatus
    pub fn get_health_status(&self) -> Option<&u16> {
        self.health_status.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of Model
    pub fn set_model(&mut self, value: String) {
        self.model = Some(value);
    }

    /// Gets the value of Model
    pub fn get_model(&self) -> Option<&String> {
        self.model.as_ref()
    }

    /// Sets the value of OperationalDetails
    pub fn set_operational_details(&mut self, value: Vec<String>) {
        self.operational_details = value;
    }

    /// Gets the value of OperationalDetails
    pub fn get_operational_details(&self) -> &Vec<String> {
        &self.operational_details
    }

    /// Sets the value of OperationalStatus
    pub fn set_operational_status(&mut self, value: Vec<u16>) {
        self.operational_status = value;
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> &Vec<u16> {
        &self.operational_status
    }

    /// Sets the value of PhysicalLocation
    pub fn set_physical_location(&mut self, value: String) {
        self.physical_location = Some(value);
    }

    /// Gets the value of PhysicalLocation
    pub fn get_physical_location(&self) -> Option<&String> {
        self.physical_location.as_ref()
    }

    /// Sets the value of SerialNumber
    pub fn set_serial_number(&mut self, value: String) {
        self.serial_number = Some(value);
    }

    /// Gets the value of SerialNumber
    pub fn get_serial_number(&self) -> Option<&String> {
        self.serial_number.as_ref()
    }
}

