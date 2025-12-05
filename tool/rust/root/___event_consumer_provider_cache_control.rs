// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __EventConsumerProviderCacheControl struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __EventConsumerProviderCacheControl {
    #[serde(flatten)]
    pub base: __CacheControl,

/// 
    #[serde(rename = "ClearAfter")]
    pub clear_after: Option<String>,
}

impl __EventConsumerProviderCacheControl {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __CacheControl::new(),
            clear_after: None,
        }
    }


    /// Sets the value of ClearAfter
    pub fn set_clear_after(&mut self, value: String) {
        self.clear_after = Some(value);
    }

    /// Gets the value of ClearAfter
    pub fn get_clear_after(&self) -> Option<&String> {
        self.clear_after.as_ref()
    }
}

