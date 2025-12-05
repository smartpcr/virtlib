// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.vs
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Product struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Product {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// Product identification such as a serial number on software, a die number on a hardware chip, or (for non-commercial Products) a project number.
    #[serde(rename = "IdentifyingNumber")]
    pub identifying_number: Option<String>,

/// Commonly used Product name.
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// Product SKU (stock keeping unit) information.
    #[serde(rename = "SKUNumber")]
    pub skunumber: Option<String>,

/// The name of the Product's supplier, or entity selling the Product (the manufacturer, reseller, OEM, etc.). Corresponds to the Vendor property in the Product object in the DMTF Solution Exchange Standard.
    #[serde(rename = "Vendor")]
    pub vendor: Option<String>,

/// Product version information.
    #[serde(rename = "Version")]
    pub version: Option<String>,

/// If this Product is under warranty, the duration of the warranty in days.
    #[serde(rename = "WarrantyDuration")]
    pub warranty_duration: Option<u32>,

/// If this Product is under warranty, the start date of the warranty.
    #[serde(rename = "WarrantyStartDate")]
    pub warranty_start_date: Option<String>,
}

impl CIM_Product {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            identifying_number: None,
            name: None,
            skunumber: None,
            vendor: None,
            version: None,
            warranty_duration: None,
            warranty_start_date: None,
        }
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

    /// Sets the value of WarrantyDuration
    pub fn set_warranty_duration(&mut self, value: u32) {
        self.warranty_duration = Some(value);
    }

    /// Gets the value of WarrantyDuration
    pub fn get_warranty_duration(&self) -> Option<&u32> {
        self.warranty_duration.as_ref()
    }

    /// Sets the value of WarrantyStartDate
    pub fn set_warranty_start_date(&mut self, value: String) {
        self.warranty_start_date = Some(value);
    }

    /// Gets the value of WarrantyStartDate
    pub fn get_warranty_start_date(&self) -> Option<&String> {
        self.warranty_start_date.as_ref()
    }
}

