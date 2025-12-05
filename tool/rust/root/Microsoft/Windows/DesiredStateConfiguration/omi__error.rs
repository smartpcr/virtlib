// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfiguration
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// OMI_Error struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OMI_Error {
    #[serde(flatten)]
    pub base: CIM_Error,

/// 
    #[serde(rename = "error_Category")]
    pub error__category: Option<u16>,

/// 
    #[serde(rename = "error_Code")]
    pub error__code: Option<u32>,

/// 
    #[serde(rename = "error_Type")]
    pub error__type: Option<String>,
}

impl OMI_Error {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Error::new(),
            error__category: None,
            error__code: None,
            error__type: None,
        }
    }


    /// Sets the value of error_Category
    pub fn set_error__category(&mut self, value: u16) {
        self.error__category = Some(value);
    }

    /// Gets the value of error_Category
    pub fn get_error__category(&self) -> Option<&u16> {
        self.error__category.as_ref()
    }

    /// Sets the value of error_Code
    pub fn set_error__code(&mut self, value: u32) {
        self.error__code = Some(value);
    }

    /// Gets the value of error_Code
    pub fn get_error__code(&self) -> Option<&u32> {
        self.error__code.as_ref()
    }

    /// Sets the value of error_Type
    pub fn set_error__type(&mut self, value: String) {
        self.error__type = Some(value);
    }

    /// Gets the value of error_Type
    pub fn get_error__type(&self) -> Option<&String> {
        self.error__type.as_ref()
    }
}

