// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WDF_DPC struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WDF_DPC {
    #[serde(flatten)]
    pub base: PerfInfo_V2,

/// 
    #[serde(rename = "Routine")]
    pub routine: Option<u32>,
}

impl WDF_DPC {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PerfInfo_V2::new(),
            routine: None,
        }
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

