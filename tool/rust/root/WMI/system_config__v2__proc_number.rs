// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V2_ProcNumber struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V2_ProcNumber {
    #[serde(flatten)]
    pub base: SystemConfig_V2,

/// 
    #[serde(rename = "ProcessorCount")]
    pub processor_count: Option<u32>,

/// 
    #[serde(rename = "ProcessorNumber")]
    pub processor_number: Vec<u32>,
}

impl SystemConfig_V2_ProcNumber {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V2::new(),
            processor_count: None,
            processor_number: Vec::new(),
        }
    }


    /// Sets the value of ProcessorCount
    pub fn set_processor_count(&mut self, value: u32) {
        self.processor_count = Some(value);
    }

    /// Gets the value of ProcessorCount
    pub fn get_processor_count(&self) -> Option<&u32> {
        self.processor_count.as_ref()
    }

    /// Sets the value of ProcessorNumber
    pub fn set_processor_number(&mut self, value: Vec<u32>) {
        self.processor_number = value;
    }

    /// Gets the value of ProcessorNumber
    pub fn get_processor_number(&self) -> &Vec<u32> {
        &self.processor_number
    }
}

