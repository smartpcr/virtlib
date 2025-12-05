// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.PEH
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __QOSFailureEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __QOSFailureEvent {
    #[serde(flatten)]
    pub base: __EventDroppedEvent,

/// 
    #[serde(rename = "ErrorCode")]
    pub error_code: Option<u32>,

/// 
    #[serde(rename = "ErrorDescription")]
    pub error_description: Option<String>,
}

impl __QOSFailureEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __EventDroppedEvent::new(),
            error_code: None,
            error_description: None,
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

    /// Sets the value of ErrorDescription
    pub fn set_error_description(&mut self, value: String) {
        self.error_description = Some(value);
    }

    /// Gets the value of ErrorDescription
    pub fn get_error_description(&self) -> Option<&String> {
        self.error_description.as_ref()
    }
}

