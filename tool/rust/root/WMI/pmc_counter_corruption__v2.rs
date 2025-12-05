// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PmcCounterCorruption_V2 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PmcCounterCorruption_V2 {
    #[serde(flatten)]
    pub base: PerfInfo_V2,

/// 
    #[serde(rename = "CounterCount")]
    pub counter_count: Option<u32>,

/// 
    #[serde(rename = "CounterStatus")]
    pub counter_status: Vec<CounterCorruptionStatus>,

/// 
    #[serde(rename = "ProcessorNumber")]
    pub processor_number: Option<u32>,
}

impl PmcCounterCorruption_V2 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PerfInfo_V2::new(),
            counter_count: None,
            counter_status: Vec::new(),
            processor_number: None,
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

    /// Sets the value of CounterStatus
    pub fn set_counter_status(&mut self, value: Vec<CounterCorruptionStatus>) {
        self.counter_status = value;
    }

    /// Gets the value of CounterStatus
    pub fn get_counter_status(&self) -> &Vec<CounterCorruptionStatus> {
        &self.counter_status
    }

    /// Sets the value of ProcessorNumber
    pub fn set_processor_number(&mut self, value: u32) {
        self.processor_number = Some(value);
    }

    /// Gets the value of ProcessorNumber
    pub fn get_processor_number(&self) -> Option<&u32> {
        self.processor_number.as_ref()
    }
}

