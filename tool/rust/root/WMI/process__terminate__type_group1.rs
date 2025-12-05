// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Process_Terminate_TypeGroup1 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Process_Terminate_TypeGroup1 {
    #[serde(flatten)]
    pub base: Process_V2,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,
}

impl Process_Terminate_TypeGroup1 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Process_V2::new(),
            process_id: None,
        }
    }


    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }
}

