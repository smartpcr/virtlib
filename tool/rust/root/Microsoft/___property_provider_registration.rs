// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __PropertyProviderRegistration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __PropertyProviderRegistration {
    #[serde(flatten)]
    pub base: __ProviderRegistration,

/// 
    #[serde(rename = "SupportsGet")]
    pub supports_get: Option<bool>,

/// 
    #[serde(rename = "SupportsPut")]
    pub supports_put: Option<bool>,
}

impl __PropertyProviderRegistration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __ProviderRegistration::new(),
            supports_get: None,
            supports_put: None,
        }
    }


    /// Sets the value of SupportsGet
    pub fn set_supports_get(&mut self, value: bool) {
        self.supports_get = Some(value);
    }

    /// Gets the value of SupportsGet
    pub fn get_supports_get(&self) -> Option<&bool> {
        self.supports_get.as_ref()
    }

    /// Sets the value of SupportsPut
    pub fn set_supports_put(&mut self, value: bool) {
        self.supports_put = Some(value);
    }

    /// Gets the value of SupportsPut
    pub fn get_supports_put(&self) -> Option<&bool> {
        self.supports_put.as_ref()
    }
}

