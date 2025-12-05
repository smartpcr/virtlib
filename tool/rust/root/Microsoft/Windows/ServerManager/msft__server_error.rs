// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ServerManager
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ServerError struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ServerError {
    #[serde(flatten)]
    pub base: CIM_Error,

/// 
    #[serde(rename = "ErrorCode")]
    pub error_code: Option<u32>,

/// 
    #[serde(rename = "ExtendedErrorCode")]
    pub extended_error_code: Option<u32>,
}

impl MSFT_ServerError {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Error::new(),
            error_code: None,
            extended_error_code: None,
        }
    }


    /// Sets the value of ErrorCode
    pub fn set_error_code(&mut self, value: u32) {
        self.error_code = Some(value);
    }

    /// Gets the value of ErrorCode
    pub fn get_error_code(&self) -> Option<&u32> {
        self.error_code.as_ref()
    }

    /// Sets the value of ExtendedErrorCode
    pub fn set_extended_error_code(&mut self, value: u32) {
        self.extended_error_code = Some(value);
    }

    /// Gets the value of ExtendedErrorCode
    pub fn get_extended_error_code(&self) -> Option<&u32> {
        self.extended_error_code.as_ref()
    }
}

