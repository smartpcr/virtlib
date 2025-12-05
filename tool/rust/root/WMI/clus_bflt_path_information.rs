// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ClusBfltPathInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusBfltPathInformation {

/// Attributes.
    #[serde(rename = "Attributes")]
    pub attributes: Option<u32>,

/// Bus Type.
    #[serde(rename = "BusType")]
    pub bus_type: Option<u32>,

/// Device Guid.
    #[serde(rename = "DeviceGuid")]
    pub device_guid: Option<String>,

/// Device Number.
    #[serde(rename = "DeviceNumber")]
    pub device_number: Option<u32>,

/// Type.
    #[serde(rename = "DeviceType")]
    pub device_type: Option<u32>,

/// Id.
    #[serde(rename = "Id")]
    pub id: Option<u32>,

/// Type.
    #[serde(rename = "PathType")]
    pub path_type: Option<u32>,

/// Registration Key.
    #[serde(rename = "RegistrationKey")]
    pub registration_key: Option<u64>,

/// Status.
    #[serde(rename = "Status")]
    pub status: Option<u32>,
}

impl ClusBfltPathInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            attributes: None,
            bus_type: None,
            device_guid: None,
            device_number: None,
            device_type: None,
            id: None,
            path_type: None,
            registration_key: None,
            status: None,
        }
    }


    /// Sets the value of Attributes
    pub fn set_attributes(&mut self, value: u32) {
        self.attributes = Some(value);
    }

    /// Gets the value of Attributes
    pub fn get_attributes(&self) -> Option<&u32> {
        self.attributes.as_ref()
    }

    /// Sets the value of BusType
    pub fn set_bus_type(&mut self, value: u32) {
        self.bus_type = Some(value);
    }

    /// Gets the value of BusType
    pub fn get_bus_type(&self) -> Option<&u32> {
        self.bus_type.as_ref()
    }

    /// Sets the value of DeviceGuid
    pub fn set_device_guid(&mut self, value: String) {
        self.device_guid = Some(value);
    }

    /// Gets the value of DeviceGuid
    pub fn get_device_guid(&self) -> Option<&String> {
        self.device_guid.as_ref()
    }

    /// Sets the value of DeviceNumber
    pub fn set_device_number(&mut self, value: u32) {
        self.device_number = Some(value);
    }

    /// Gets the value of DeviceNumber
    pub fn get_device_number(&self) -> Option<&u32> {
        self.device_number.as_ref()
    }

    /// Sets the value of DeviceType
    pub fn set_device_type(&mut self, value: u32) {
        self.device_type = Some(value);
    }

    /// Gets the value of DeviceType
    pub fn get_device_type(&self) -> Option<&u32> {
        self.device_type.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: u32) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&u32> {
        self.id.as_ref()
    }

    /// Sets the value of PathType
    pub fn set_path_type(&mut self, value: u32) {
        self.path_type = Some(value);
    }

    /// Gets the value of PathType
    pub fn get_path_type(&self) -> Option<&u32> {
        self.path_type.as_ref()
    }

    /// Sets the value of RegistrationKey
    pub fn set_registration_key(&mut self, value: u64) {
        self.registration_key = Some(value);
    }

    /// Gets the value of RegistrationKey
    pub fn get_registration_key(&self) -> Option<&u64> {
        self.registration_key.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u32> {
        self.status.as_ref()
    }
}

