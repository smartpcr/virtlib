// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetCompartment struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetCompartment {
    #[serde(flatten)]
    pub base: MSFT_NetSettingData,

/// 
    #[serde(rename = "CompartmentDescription")]
    pub compartment_description: Option<String>,

/// 
    #[serde(rename = "CompartmentGuid")]
    pub compartment_guid: Option<String>,

/// 
    #[serde(rename = "CompartmentId")]
    pub compartment_id: Option<u32>,

/// 
    #[serde(rename = "CompartmentType")]
    pub compartment_type: Option<u32>,
}

impl MSFT_NetCompartment {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSettingData::new(),
            compartment_description: None,
            compartment_guid: None,
            compartment_id: None,
            compartment_type: None,
        }
    }


    /// Sets the value of CompartmentDescription
    pub fn set_compartment_description(&mut self, value: String) {
        self.compartment_description = Some(value);
    }

    /// Gets the value of CompartmentDescription
    pub fn get_compartment_description(&self) -> Option<&String> {
        self.compartment_description.as_ref()
    }

    /// Sets the value of CompartmentGuid
    pub fn set_compartment_guid(&mut self, value: String) {
        self.compartment_guid = Some(value);
    }

    /// Gets the value of CompartmentGuid
    pub fn get_compartment_guid(&self) -> Option<&String> {
        self.compartment_guid.as_ref()
    }

    /// Sets the value of CompartmentId
    pub fn set_compartment_id(&mut self, value: u32) {
        self.compartment_id = Some(value);
    }

    /// Gets the value of CompartmentId
    pub fn get_compartment_id(&self) -> Option<&u32> {
        self.compartment_id.as_ref()
    }

    /// Sets the value of CompartmentType
    pub fn set_compartment_type(&mut self, value: u32) {
        self.compartment_type = Some(value);
    }

    /// Gets the value of CompartmentType
    pub fn get_compartment_type(&self) -> Option<&u32> {
        self.compartment_type.as_ref()
    }
}

