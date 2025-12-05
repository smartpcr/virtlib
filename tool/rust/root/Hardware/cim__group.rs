// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Hardware
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Group struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Group {
    #[serde(flatten)]
    pub base: CIM_Collection,

/// 
    #[serde(rename = "BusinessCategory")]
    pub business_category: Option<String>,

/// 
    #[serde(rename = "CommonName")]
    pub common_name: Option<String>,

/// 
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl CIM_Group {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Collection::new(),
            business_category: None,
            common_name: None,
            creation_class_name: None,
            name: None,
        }
    }


    /// Sets the value of BusinessCategory
    pub fn set_business_category(&mut self, value: String) {
        self.business_category = Some(value);
    }

    /// Gets the value of BusinessCategory
    pub fn get_business_category(&self) -> Option<&String> {
        self.business_category.as_ref()
    }

    /// Sets the value of CommonName
    pub fn set_common_name(&mut self, value: String) {
        self.common_name = Some(value);
    }

    /// Gets the value of CommonName
    pub fn get_common_name(&self) -> Option<&String> {
        self.common_name.as_ref()
    }

    /// Sets the value of CreationClassName
    pub fn set_creation_class_name(&mut self, value: String) {
        self.creation_class_name = Some(value);
    }

    /// Gets the value of CreationClassName
    pub fn get_creation_class_name(&self) -> Option<&String> {
        self.creation_class_name.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

