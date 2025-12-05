// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_WmiConsumerProviderLoaded struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_WmiConsumerProviderLoaded {
    #[serde(flatten)]
    pub base: MSFT_WmiConsumerProviderEvent,
}

impl MSFT_WmiConsumerProviderLoaded {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_WmiConsumerProviderEvent::new(),
        }
    }

}

