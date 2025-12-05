// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msft_WmiProvider_InitializationOperationFailureEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msft_WmiProvider_InitializationOperationFailureEvent {
    #[serde(flatten)]
    pub base: Msft_WmiProvider_OperationEvent,

/// 
    #[serde(rename = "ResultCode")]
    pub result_code: Option<u32>,
}

impl Msft_WmiProvider_InitializationOperationFailureEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msft_WmiProvider_OperationEvent::new(),
            result_code: None,
        }
    }


    /// Sets the value of ResultCode
    pub fn set_result_code(&mut self, value: u32) {
        self.result_code = Some(value);
    }

    /// Gets the value of ResultCode
    pub fn get_result_code(&self) -> Option<&u32> {
        self.result_code.as_ref()
    }
}

