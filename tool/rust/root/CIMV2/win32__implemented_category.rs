// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ImplementedCategory struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ImplementedCategory {

/// 
    #[serde(rename = "Category")]
    pub category: Option<Win32_ComponentCategory>,

/// 
    #[serde(rename = "Component")]
    pub component: Option<Win32_ClassicCOMClass>,
}

impl Win32_ImplementedCategory {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            category: None,
            component: None,
        }
    }


    /// Sets the value of Category
    pub fn set_category(&mut self, value: Win32_ComponentCategory) {
        self.category = Some(value);
    }

    /// Gets the value of Category
    pub fn get_category(&self) -> Option<&Win32_ComponentCategory> {
        self.category.as_ref()
    }

    /// Sets the value of Component
    pub fn set_component(&mut self, value: Win32_ClassicCOMClass) {
        self.component = Some(value);
    }

    /// Gets the value of Component
    pub fn get_component(&self) -> Option<&Win32_ClassicCOMClass> {
        self.component.as_ref()
    }
}

