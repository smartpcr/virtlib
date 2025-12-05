// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DPC_V1 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DPC_V1 {
    #[serde(flatten)]
    pub base: PerfInfo_V1,

/// 
    #[serde(rename = "InitialTime")]
    pub initial_time: Option<serde_json::Value>,

/// 
    #[serde(rename = "Routine")]
    pub routine: Option<u32>,
}

impl DPC_V1 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PerfInfo_V1::new(),
            initial_time: None,
            routine: None,
        }
    }


    /// Sets the value of InitialTime
    pub fn set_initial_time(&mut self, value: serde_json::Value) {
        self.initial_time = Some(value);
    }

    /// Gets the value of InitialTime
    pub fn get_initial_time(&self) -> Option<&serde_json::Value> {
        self.initial_time.as_ref()
    }

    /// Sets the value of Routine
    pub fn set_routine(&mut self, value: u32) {
        self.routine = Some(value);
    }

    /// Gets the value of Routine
    pub fn get_routine(&self) -> Option<&u32> {
        self.routine.as_ref()
    }
}

