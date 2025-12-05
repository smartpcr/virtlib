// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SDDC
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_WmiError struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_WmiError {
    #[serde(flatten)]
    pub base: CIM_Error,

/// Error Category.
    #[serde(rename = "error_Category")]
    pub error__category: Option<u16>,

/// Error code.
    #[serde(rename = "error_Code")]
    pub error__code: Option<u32>,

/// Error Type.
    #[serde(rename = "error_Type")]
    pub error__type: Option<String>,

/// Windows error message.
    #[serde(rename = "error_WindowsErrorMessage")]
    pub error__windows_error_message: Option<String>,
}

impl MSFT_WmiError {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Error::new(),
            error__category: None,
            error__code: None,
            error__type: None,
            error__windows_error_message: None,
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

    /// Sets the value of error_WindowsErrorMessage
    pub fn set_error__windows_error_message(&mut self, value: String) {
        self.error__windows_error_message = Some(value);
    }

    /// Gets the value of error_WindowsErrorMessage
    pub fn get_error__windows_error_message(&self) -> Option<&String> {
        self.error__windows_error_message.as_ref()
    }
}

