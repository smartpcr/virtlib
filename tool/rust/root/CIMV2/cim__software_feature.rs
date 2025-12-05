// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SoftwareFeature struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SoftwareFeature {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "IdentifyingNumber")]
    pub identifying_number: Option<String>,

/// 
    #[serde(rename = "ProductName")]
    pub product_name: Option<String>,

/// 
    #[serde(rename = "Vendor")]
    pub vendor: Option<String>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

impl CIM_SoftwareFeature {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            identifying_number: None,
            product_name: None,
            vendor: None,
            version: None,
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

    /// Sets the value of ProductName
    pub fn set_product_name(&mut self, value: String) {
        self.product_name = Some(value);
    }

    /// Gets the value of ProductName
    pub fn get_product_name(&self) -> Option<&String> {
        self.product_name.as_ref()
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

