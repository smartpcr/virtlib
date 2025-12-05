// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetCallToFunctionFailedII struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetCallToFunctionFailedII {
    #[serde(flatten)]
    pub base: MSFT_SCMEventLogEvent,

/// 
    #[serde(rename = "Argument")]
    pub argument: Option<String>,

/// 
    #[serde(rename = "Error")]
    pub error: Option<u32>,

/// 
    #[serde(rename = "FunctionName")]
    pub function_name: Option<String>,
}

impl MSFT_NetCallToFunctionFailedII {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_SCMEventLogEvent::new(),
            argument: None,
            error: None,
            function_name: None,
        }
    }


    /// Sets the value of Argument
    pub fn set_argument(&mut self, value: String) {
        self.argument = Some(value);
    }

    /// Gets the value of Argument
    pub fn get_argument(&self) -> Option<&String> {
        self.argument.as_ref()
    }

    /// Sets the value of Error
    pub fn set_error(&mut self, value: u32) {
        self.error = Some(value);
    }

    /// Gets the value of Error
    pub fn get_error(&self) -> Option<&u32> {
        self.error.as_ref()
    }

    /// Sets the value of FunctionName
    pub fn set_function_name(&mut self, value: String) {
        self.function_name = Some(value);
    }

    /// Gets the value of FunctionName
    pub fn get_function_name(&self) -> Option<&String> {
        self.function_name.as_ref()
    }
}

