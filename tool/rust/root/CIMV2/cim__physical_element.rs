// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_PhysicalElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_PhysicalElement {
    #[serde(flatten)]
    pub base: CIM_ManagedSystemElement,

/// 
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "Model")]
    pub model: Option<String>,

/// 
    #[serde(rename = "OtherIdentifyingInfo")]
    pub other_identifying_info: Option<String>,

/// 
    #[serde(rename = "PartNumber")]
    pub part_number: Option<String>,

/// 
    #[serde(rename = "PoweredOn")]
    pub powered_on: Option<bool>,

/// 
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,

/// 
    #[serde(rename = "SKU")]
    pub sku: Option<String>,

/// 
    #[serde(rename = "Tag")]
    pub tag: Option<String>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

impl CIM_PhysicalElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedSystemElement::new(),
            creation_class_name: None,
            manufacturer: None,
            model: None,
            other_identifying_info: None,
            part_number: None,
            powered_on: None,
            serial_number: None,
            sku: None,
            tag: None,
            version: None,
        }
    }


    /// Sets the value of CreationClassName
    pub fn set_creation_class_name(&mut self, value: String) {
        self.creation_class_name = Some(value);
    }

    /// Gets the value of CreationClassName
    pub fn get_creation_class_name(&self) -> Option<&String> {
        self.creation_class_name.as_ref()
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

    /// Sets the value of OtherIdentifyingInfo
    pub fn set_other_identifying_info(&mut self, value: String) {
        self.other_identifying_info = Some(value);
    }

    /// Gets the value of OtherIdentifyingInfo
    pub fn get_other_identifying_info(&self) -> Option<&String> {
        self.other_identifying_info.as_ref()
    }

    /// Sets the value of PartNumber
    pub fn set_part_number(&mut self, value: String) {
        self.part_number = Some(value);
    }

    /// Gets the value of PartNumber
    pub fn get_part_number(&self) -> Option<&String> {
        self.part_number.as_ref()
    }

    /// Sets the value of PoweredOn
    pub fn set_powered_on(&mut self, value: bool) {
        self.powered_on = Some(value);
    }

    /// Gets the value of PoweredOn
    pub fn get_powered_on(&self) -> Option<&bool> {
        self.powered_on.as_ref()
    }

    /// Sets the value of SerialNumber
    pub fn set_serial_number(&mut self, value: String) {
        self.serial_number = Some(value);
    }

    /// Gets the value of SerialNumber
    pub fn get_serial_number(&self) -> Option<&String> {
        self.serial_number.as_ref()
    }

    /// Sets the value of SKU
    pub fn set_sku(&mut self, value: String) {
        self.sku = Some(value);
    }

    /// Gets the value of SKU
    pub fn get_sku(&self) -> Option<&String> {
        self.sku.as_ref()
    }

    /// Sets the value of Tag
    pub fn set_tag(&mut self, value: String) {
        self.tag = Some(value);
    }

    /// Gets the value of Tag
    pub fn get_tag(&self) -> Option<&String> {
        self.tag.as_ref()
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

