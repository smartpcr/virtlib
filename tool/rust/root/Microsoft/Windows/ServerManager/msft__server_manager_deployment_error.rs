// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ServerManager
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ServerManagerDeploymentError struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ServerManagerDeploymentError {

/// 
    #[serde(rename = "ErrorCategory")]
    pub error_category: Option<u8>,

/// 
    #[serde(rename = "ErrorId")]
    pub error_id: Option<String>,

/// 
    #[serde(rename = "ErrorMessage")]
    pub error_message: Option<String>,
}

impl MSFT_ServerManagerDeploymentError {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            error_category: None,
            error_id: None,
            error_message: None,
        }
    }


    /// Sets the value of ErrorCategory
    pub fn set_error_category(&mut self, value: u8) {
        self.error_category = Some(value);
    }

    /// Gets the value of ErrorCategory
    pub fn get_error_category(&self) -> Option<&u8> {
        self.error_category.as_ref()
    }

    /// Sets the value of ErrorId
    pub fn set_error_id(&mut self, value: String) {
        self.error_id = Some(value);
    }

    /// Gets the value of ErrorId
    pub fn get_error_id(&self) -> Option<&String> {
        self.error_id.as_ref()
    }

    /// Sets the value of ErrorMessage
    pub fn set_error_message(&mut self, value: String) {
        self.error_message = Some(value);
    }

    /// Gets the value of ErrorMessage
    pub fn get_error_message(&self) -> Option<&String> {
        self.error_message.as_ref()
    }
}

