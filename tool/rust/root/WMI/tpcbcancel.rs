// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// TPCBCancel struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TPCBCancel {
    #[serde(flatten)]
    pub base: ThreadPoolTrace,

/// 
    #[serde(rename = "CallbackContext")]
    pub callback_context: Option<u32>,

/// 
    #[serde(rename = "CallbackFunction")]
    pub callback_function: Option<u32>,

/// 
    #[serde(rename = "CancelCount")]
    pub cancel_count: Option<u32>,

/// 
    #[serde(rename = "PoolId")]
    pub pool_id: Option<u32>,

/// 
    #[serde(rename = "SubProcessTag")]
    pub sub_process_tag: Option<u32>,

/// 
    #[serde(rename = "TaskId")]
    pub task_id: Option<u32>,
}

impl TPCBCancel {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: ThreadPoolTrace::new(),
            callback_context: None,
            callback_function: None,
            cancel_count: None,
            pool_id: None,
            sub_process_tag: None,
            task_id: None,
        }
    }


    /// Sets the value of CallbackContext
    pub fn set_callback_context(&mut self, value: u32) {
        self.callback_context = Some(value);
    }

    /// Gets the value of CallbackContext
    pub fn get_callback_context(&self) -> Option<&u32> {
        self.callback_context.as_ref()
    }

    /// Sets the value of CallbackFunction
    pub fn set_callback_function(&mut self, value: u32) {
        self.callback_function = Some(value);
    }

    /// Gets the value of CallbackFunction
    pub fn get_callback_function(&self) -> Option<&u32> {
        self.callback_function.as_ref()
    }

    /// Sets the value of CancelCount
    pub fn set_cancel_count(&mut self, value: u32) {
        self.cancel_count = Some(value);
    }

    /// Gets the value of CancelCount
    pub fn get_cancel_count(&self) -> Option<&u32> {
        self.cancel_count.as_ref()
    }

    /// Sets the value of PoolId
    pub fn set_pool_id(&mut self, value: u32) {
        self.pool_id = Some(value);
    }

    /// Gets the value of PoolId
    pub fn get_pool_id(&self) -> Option<&u32> {
        self.pool_id.as_ref()
    }

    /// Sets the value of SubProcessTag
    pub fn set_sub_process_tag(&mut self, value: u32) {
        self.sub_process_tag = Some(value);
    }

    /// Gets the value of SubProcessTag
    pub fn get_sub_process_tag(&self) -> Option<&u32> {
        self.sub_process_tag.as_ref()
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

