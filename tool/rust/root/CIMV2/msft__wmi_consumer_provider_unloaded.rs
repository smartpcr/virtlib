// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_WmiConsumerProviderUnloaded struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_WmiConsumerProviderUnloaded {
    #[serde(flatten)]
    pub base: MSFT_WmiConsumerProviderEvent,
}

impl MSFT_WmiConsumerProviderUnloaded {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_WmiConsumerProviderEvent::new(),
        }
    }

}

