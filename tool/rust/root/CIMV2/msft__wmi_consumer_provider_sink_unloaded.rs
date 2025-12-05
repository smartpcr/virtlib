// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_WmiConsumerProviderSinkUnloaded struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_WmiConsumerProviderSinkUnloaded {
    #[serde(flatten)]
    pub base: MSFT_WmiConsumerProviderEvent,

/// 
    #[serde(rename = "Consumer")]
    pub consumer: Option<__EventConsumer>,
}

impl MSFT_WmiConsumerProviderSinkUnloaded {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_WmiConsumerProviderEvent::new(),
            consumer: None,
        }
    }


    /// Sets the value of Consumer
    pub fn set_consumer(&mut self, value: __EventConsumer) {
        self.consumer = Some(value);
    }

    /// Gets the value of Consumer
    pub fn get_consumer(&self) -> Option<&__EventConsumer> {
        self.consumer.as_ref()
    }
}

