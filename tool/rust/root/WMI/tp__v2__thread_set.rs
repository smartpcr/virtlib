// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// TP_V2_ThreadSet struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TP_V2_ThreadSet {
    #[serde(flatten)]
    pub base: ThreadPoolTrace_V2,

/// 
    #[serde(rename = "PoolId")]
    pub pool_id: Option<u32>,

/// 
    #[serde(rename = "ThreadNum")]
    pub thread_num: Option<u32>,
}

impl TP_V2_ThreadSet {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: ThreadPoolTrace_V2::new(),
            pool_id: None,
            thread_num: None,
        }
    }


    /// Sets the value of PoolId
    pub fn set_pool_id(&mut self, value: u32) {
        self.pool_id = Some(value);
    }

    /// Gets the value of PoolId
    pub fn get_pool_id(&self) -> Option<&u32> {
        self.pool_id.as_ref()
    }

    /// Sets the value of ThreadNum
    pub fn set_thread_num(&mut self, value: u32) {
        self.thread_num = Some(value);
    }

    /// Gets the value of ThreadNum
    pub fn get_thread_num(&self) -> Option<&u32> {
        self.thread_num.as_ref()
    }
}

