// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// TP_V2_CBDequeue struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TP_V2_CBDequeue {
    #[serde(flatten)]
    pub base: ThreadPoolTrace_V2,

/// 
    #[serde(rename = "TaskId")]
    pub task_id: Option<u32>,
}

impl TP_V2_CBDequeue {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: ThreadPoolTrace_V2::new(),
            task_id: None,
        }
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

