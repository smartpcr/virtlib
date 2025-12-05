// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NCProvClientConnected struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NCProvClientConnected {
    #[serde(flatten)]
    pub base: MSFT_NCProvEvent,

/// 
    #[serde(rename = "Inproc")]
    pub inproc: Option<bool>,
}

impl MSFT_NCProvClientConnected {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NCProvEvent::new(),
            inproc: None,
        }
    }


    /// Sets the value of Inproc
    pub fn set_inproc(&mut self, value: bool) {
        self.inproc = Some(value);
    }

    /// Gets the value of Inproc
    pub fn get_inproc(&self) -> Option<&bool> {
        self.inproc.as_ref()
    }
}

