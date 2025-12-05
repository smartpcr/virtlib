// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Product struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Product {

/// 
    #[serde(rename = "Caption")]
    pub caption: Option<String>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "IdentifyingNumber")]
    pub identifying_number: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "SKUNumber")]
    pub skunumber: Option<String>,

/// 
    #[serde(rename = "Vendor")]
    pub vendor: Option<String>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

impl CIM_Product {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            caption: None,
            description: None,
            identifying_number: None,
            name: None,
            skunumber: None,
            vendor: None,
            version: None,
        }
    }


    /// Sets the value of Caption
    pub fn set_caption(&mut self, value: String) {
        self.caption = Some(value);
    }

    /// Gets the value of Caption
    pub fn get_caption(&self) -> Option<&String> {
        self.caption.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of IdentifyingNumber
    pub fn set_identifying_number(&mut self, value: String) {
        self.identifying_number = Some(value);
    }

    /// Gets the value of IdentifyingNumber
    pub fn get_identifying_number(&self) -> Option<&String> {
        self.identifying_number.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of SKUNumber
    pub fn set_skunumber(&mut self, value: String) {
        self.skunumber = Some(value);
    }

    /// Gets the value of SKUNumber
    pub fn get_skunumber(&self) -> Option<&String> {
        self.skunumber.as_ref()
    }

    /// Sets the value of Vendor
    pub fn set_vendor(&mut self, value: String) {
        self.vendor = Some(value);
    }

    /// Gets the value of Vendor
    pub fn get_vendor(&self) -> Option<&String> {
        self.vendor.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: String) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&String> {
        self.version.as_ref()
    }
}

