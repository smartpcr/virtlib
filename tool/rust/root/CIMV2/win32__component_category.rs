// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ComponentCategory struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ComponentCategory {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "CategoryId")]
    pub category_id: Option<String>,
}

impl Win32_ComponentCategory {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            category_id: None,
        }
    }


    /// Sets the value of CategoryId
    pub fn set_category_id(&mut self, value: String) {
        self.category_id = Some(value);
    }

    /// Gets the value of CategoryId
    pub fn get_category_id(&self) -> Option<&String> {
        self.category_id.as_ref()
    }
}

