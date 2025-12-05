// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_FRU struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_FRU {

/// 
    #[serde(rename = "Caption")]
    pub caption: Option<String>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "FRUNumber")]
    pub frunumber: Option<String>,

/// 
    #[serde(rename = "IdentifyingNumber")]
    pub identifying_number: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "RevisionLevel")]
    pub revision_level: Option<String>,

/// 
    #[serde(rename = "Vendor")]
    pub vendor: Option<String>,
}

impl CIM_FRU {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            caption: None,
            description: None,
            frunumber: None,
            identifying_number: None,
            name: None,
            revision_level: None,
            vendor: None,
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

    /// Sets the value of FRUNumber
    pub fn set_frunumber(&mut self, value: String) {
        self.frunumber = Some(value);
    }

    /// Gets the value of FRUNumber
    pub fn get_frunumber(&self) -> Option<&String> {
        self.frunumber.as_ref()
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

    /// Sets the value of RevisionLevel
    pub fn set_revision_level(&mut self, value: String) {
        self.revision_level = Some(value);
    }

    /// Gets the value of RevisionLevel
    pub fn get_revision_level(&self) -> Option<&String> {
        self.revision_level.as_ref()
    }

    /// Sets the value of Vendor
    pub fn set_vendor(&mut self, value: String) {
        self.vendor = Some(value);
    }

    /// Gets the value of Vendor
    pub fn get_vendor(&self) -> Option<&String> {
        self.vendor.as_ref()
    }
}

