// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Thread_V1_TypeGroup2 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Thread_V1_TypeGroup2 {
    #[serde(flatten)]
    pub base: Thread_V1,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "TThreadId")]
    pub tthread_id: Option<u32>,
}

impl Thread_V1_TypeGroup2 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Thread_V1::new(),
            process_id: None,
            tthread_id: None,
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

    /// Sets the value of TThreadId
    pub fn set_tthread_id(&mut self, value: u32) {
        self.tthread_id = Some(value);
    }

    /// Gets the value of TThreadId
    pub fn get_tthread_id(&self) -> Option<&u32> {
        self.tthread_id.as_ref()
    }
}

