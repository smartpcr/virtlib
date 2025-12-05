// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NCProvEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NCProvEvent {
    #[serde(flatten)]
    pub base: __ExtrinsicEvent,

/// 
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,

/// 
    #[serde(rename = "ProviderName")]
    pub provider_name: Option<String>,

/// 
    #[serde(rename = "Result")]
    pub result: Option<u32>,
}

impl MSFT_NCProvEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __ExtrinsicEvent::new(),
            namespace: None,
            provider_name: None,
            result: None,
        }
    }


    /// Sets the value of Namespace
    pub fn set_namespace(&mut self, value: String) {
        self.namespace = Some(value);
    }

    /// Gets the value of Namespace
    pub fn get_namespace(&self) -> Option<&String> {
        self.namespace.as_ref()
    }

    /// Sets the value of ProviderName
    pub fn set_provider_name(&mut self, value: String) {
        self.provider_name = Some(value);
    }

    /// Gets the value of ProviderName
    pub fn get_provider_name(&self) -> Option<&String> {
        self.provider_name.as_ref()
    }

    /// Sets the value of Result
    pub fn set_result(&mut self, value: u32) {
        self.result = Some(value);
    }

    /// Gets the value of Result
    pub fn get_result(&self) -> Option<&u32> {
        self.result.as_ref()
    }
}

