// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_WmiProviderEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_WmiProviderEvent {
    #[serde(flatten)]
    pub base: MSFT_WmiEssEvent,

/// 
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,

/// 
    #[serde(rename = "ProviderName")]
    pub provider_name: Option<String>,
}

impl MSFT_WmiProviderEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_WmiEssEvent::new(),
            namespace: None,
            provider_name: None,
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
}

