// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PmcCounterConfig_V2 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PmcCounterConfig_V2 {
    #[serde(flatten)]
    pub base: PerfInfo_V2,

/// 
    #[serde(rename = "CounterCount")]
    pub counter_count: Option<u32>,

/// 
    #[serde(rename = "CounterName")]
    pub counter_name: Vec<String>,
}

impl PmcCounterConfig_V2 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PerfInfo_V2::new(),
            counter_count: None,
            counter_name: Vec::new(),
        }
    }


    /// Sets the value of CounterCount
    pub fn set_counter_count(&mut self, value: u32) {
        self.counter_count = Some(value);
    }

    /// Gets the value of CounterCount
    pub fn get_counter_count(&self) -> Option<&u32> {
        self.counter_count.as_ref()
    }

    /// Sets the value of CounterName
    pub fn set_counter_name(&mut self, value: Vec<String>) {
        self.counter_name = value;
    }

    /// Gets the value of CounterName
    pub fn get_counter_name(&self) -> &Vec<String> {
        &self.counter_name
    }
}

