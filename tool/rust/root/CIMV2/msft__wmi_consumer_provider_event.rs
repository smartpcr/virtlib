// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_WmiConsumerProviderEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_WmiConsumerProviderEvent {
    #[serde(flatten)]
    pub base: MSFT_WmiProviderEvent,

/// 
    #[serde(rename = "Machine")]
    pub machine: Option<String>,
}

impl MSFT_WmiConsumerProviderEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_WmiProviderEvent::new(),
            machine: None,
        }
    }


    /// Sets the value of Machine
    pub fn set_machine(&mut self, value: String) {
        self.machine = Some(value);
    }

    /// Gets the value of Machine
    pub fn get_machine(&self) -> Option<&String> {
        self.machine.as_ref()
    }
}

