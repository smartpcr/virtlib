// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SDDC.Management
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __ProviderRegistration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __ProviderRegistration {
    #[serde(flatten)]
    pub base: __SystemClass,

/// 
    #[serde(rename = "provider")]
    pub provider: Option<__Provider>,
}

impl __ProviderRegistration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __SystemClass::new(),
            provider: None,
        }
    }


    /// Sets the value of provider
    pub fn set_provider(&mut self, value: __Provider) {
        self.provider = Some(value);
    }

    /// Gets the value of provider
    pub fn get_provider(&self) -> Option<&__Provider> {
        self.provider.as_ref()
    }
}

