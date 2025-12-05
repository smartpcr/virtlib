// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Hardware
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_RegisteredProfile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_RegisteredProfile {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "AdvertiseTypeDescriptions")]
    pub advertise_type_descriptions: Vec<String>,

/// 
    #[serde(rename = "AdvertiseTypes")]
    pub advertise_types: Vec<u16>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "OtherRegisteredOrganization")]
    pub other_registered_organization: Option<String>,

/// 
    #[serde(rename = "RegisteredName")]
    pub registered_name: Option<String>,

/// 
    #[serde(rename = "RegisteredOrganization")]
    pub registered_organization: Option<u16>,

/// 
    #[serde(rename = "RegisteredVersion")]
    pub registered_version: Option<String>,
}

impl CIM_RegisteredProfile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            advertise_type_descriptions: Vec::new(),
            advertise_types: Vec::new(),
            instance_id: None,
            other_registered_organization: None,
            registered_name: None,
            registered_organization: None,
            registered_version: None,
        }
    }


    /// Sets the value of AdvertiseTypeDescriptions
    pub fn set_advertise_type_descriptions(&mut self, value: Vec<String>) {
        self.advertise_type_descriptions = value;
    }

    /// Gets the value of AdvertiseTypeDescriptions
    pub fn get_advertise_type_descriptions(&self) -> &Vec<String> {
        &self.advertise_type_descriptions
    }

    /// Sets the value of AdvertiseTypes
    pub fn set_advertise_types(&mut self, value: Vec<u16>) {
        self.advertise_types = value;
    }

    /// Gets the value of AdvertiseTypes
    pub fn get_advertise_types(&self) -> &Vec<u16> {
        &self.advertise_types
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of OtherRegisteredOrganization
    pub fn set_other_registered_organization(&mut self, value: String) {
        self.other_registered_organization = Some(value);
    }

    /// Gets the value of OtherRegisteredOrganization
    pub fn get_other_registered_organization(&self) -> Option<&String> {
        self.other_registered_organization.as_ref()
    }

    /// Sets the value of RegisteredName
    pub fn set_registered_name(&mut self, value: String) {
        self.registered_name = Some(value);
    }

    /// Gets the value of RegisteredName
    pub fn get_registered_name(&self) -> Option<&String> {
        self.registered_name.as_ref()
    }

    /// Sets the value of RegisteredOrganization
    pub fn set_registered_organization(&mut self, value: u16) {
        self.registered_organization = Some(value);
    }

    /// Gets the value of RegisteredOrganization
    pub fn get_registered_organization(&self) -> Option<&u16> {
        self.registered_organization.as_ref()
    }

    /// Sets the value of RegisteredVersion
    pub fn set_registered_version(&mut self, value: String) {
        self.registered_version = Some(value);
    }

    /// Gets the value of RegisteredVersion
    pub fn get_registered_version(&self) -> Option<&String> {
        self.registered_version.as_ref()
    }
}

