// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_DMAChannel struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_DMAChannel {
    #[serde(flatten)]
    pub base: CIM_DMA,

/// 
    #[serde(rename = "Port")]
    pub port: Option<u32>,
}

impl Win32_DMAChannel {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_DMA::new(),
            port: None,
        }
    }


    /// Sets the value of Port
    pub fn set_port(&mut self, value: u32) {
        self.port = Some(value);
    }

    /// Gets the value of Port
    pub fn get_port(&self) -> Option<&u32> {
        self.port.as_ref()
    }
}

