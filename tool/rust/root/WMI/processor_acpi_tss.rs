// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ProcessorAcpiTss struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessorAcpiTss {

/// 
    #[serde(rename = "Count")]
    pub count: Option<u32>,

/// 
    #[serde(rename = "State")]
    pub state: Vec<ProcessorAcpiTssState>,
}

impl ProcessorAcpiTss {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            count: None,
            state: Vec::new(),
        }
    }


    /// Sets the value of Count
    pub fn set_count(&mut self, value: u32) {
        self.count = Some(value);
    }

    /// Gets the value of Count
    pub fn get_count(&self) -> Option<&u32> {
        self.count.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: Vec<ProcessorAcpiTssState>) {
        self.state = value;
    }

    /// Gets the value of State
    pub fn get_state(&self) -> &Vec<ProcessorAcpiTssState> {
        &self.state
    }
}

