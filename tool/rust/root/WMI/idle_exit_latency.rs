// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// IdleExitLatency struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IdleExitLatency {
    #[serde(flatten)]
    pub base: PowerEvents,

/// 
    #[serde(rename = "PlatformState")]
    pub platform_state: Option<u32>,

/// 
    #[serde(rename = "ProcessorState")]
    pub processor_state: Option<u32>,

/// 
    #[serde(rename = "ReturnLatency")]
    pub return_latency: Option<u32>,

/// 
    #[serde(rename = "TotalLatency")]
    pub total_latency: Option<u32>,
}

impl IdleExitLatency {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PowerEvents::new(),
            platform_state: None,
            processor_state: None,
            return_latency: None,
            total_latency: None,
        }
    }


    /// Sets the value of PlatformState
    pub fn set_platform_state(&mut self, value: u32) {
        self.platform_state = Some(value);
    }

    /// Gets the value of PlatformState
    pub fn get_platform_state(&self) -> Option<&u32> {
        self.platform_state.as_ref()
    }

    /// Sets the value of ProcessorState
    pub fn set_processor_state(&mut self, value: u32) {
        self.processor_state = Some(value);
    }

    /// Gets the value of ProcessorState
    pub fn get_processor_state(&self) -> Option<&u32> {
        self.processor_state.as_ref()
    }

    /// Sets the value of ReturnLatency
    pub fn set_return_latency(&mut self, value: u32) {
        self.return_latency = Some(value);
    }

    /// Gets the value of ReturnLatency
    pub fn get_return_latency(&self) -> Option<&u32> {
        self.return_latency.as_ref()
    }

    /// Sets the value of TotalLatency
    pub fn set_total_latency(&mut self, value: u32) {
        self.total_latency = Some(value);
    }

    /// Gets the value of TotalLatency
    pub fn get_total_latency(&self) -> Option<&u32> {
        self.total_latency.as_ref()
    }
}

