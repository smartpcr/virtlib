// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WorkerThread_StartStop_V2 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkerThread_StartStop_V2 {
    #[serde(flatten)]
    pub base: Thread_V2,

/// 
    #[serde(rename = "CallbackRoutine")]
    pub callback_routine: Option<u32>,
}

impl WorkerThread_StartStop_V2 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Thread_V2::new(),
            callback_routine: None,
        }
    }


    /// Sets the value of CallbackRoutine
    pub fn set_callback_routine(&mut self, value: u32) {
        self.callback_routine = Some(value);
    }

    /// Gets the value of CallbackRoutine
    pub fn get_callback_routine(&self) -> Option<&u32> {
        self.callback_routine.as_ref()
    }
}

