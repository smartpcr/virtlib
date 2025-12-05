// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSWmi_ProviderInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSWmi_ProviderInfo {
    #[serde(flatten)]
    pub base: MS_WmiInternal,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "Location")]
    pub location: Option<String>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "Service")]
    pub service: Option<String>,
}

impl MSWmi_ProviderInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MS_WmiInternal::new(),
            active: None,
            description: None,
            friendly_name: None,
            instance_name: None,
            location: None,
            manufacturer: None,
            service: None,
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

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
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

    /// Sets the value of Service
    pub fn set_service(&mut self, value: String) {
        self.service = Some(value);
    }

    /// Gets the value of Service
    pub fn get_service(&self) -> Option<&String> {
        self.service.as_ref()
    }
}

