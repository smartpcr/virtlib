// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// TP_V2_CBCancel struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TP_V2_CBCancel {
    #[serde(flatten)]
    pub base: ThreadPoolTrace_V2,

/// 
    #[serde(rename = "CancelCount")]
    pub cancel_count: Option<u32>,

/// 
    #[serde(rename = "TaskId")]
    pub task_id: Option<u32>,
}

impl TP_V2_CBCancel {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: ThreadPoolTrace_V2::new(),
            cancel_count: None,
            task_id: None,
        }
    }


    /// Sets the value of CancelCount
    pub fn set_cancel_count(&mut self, value: u32) {
        self.cancel_count = Some(value);
    }

    /// Gets the value of CancelCount
    pub fn get_cancel_count(&self) -> Option<&u32> {
        self.cancel_count.as_ref()
    }

    /// Sets the value of TaskId
    pub fn set_task_id(&mut self, value: u32) {
        self.task_id = Some(value);
    }

    /// Gets the value of TaskId
    pub fn get_task_id(&self) -> Option<&u32> {
        self.task_id.as_ref()
    }
}

