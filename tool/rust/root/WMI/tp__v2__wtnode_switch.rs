// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// TP_V2_WTNodeSwitch struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TP_V2_WTNodeSwitch {
    #[serde(flatten)]
    pub base: ThreadPoolTrace_V2,

/// 
    #[serde(rename = "CurrentGroup")]
    pub current_group: Option<u16>,

/// 
    #[serde(rename = "CurrentNode")]
    pub current_node: Option<u32>,

/// 
    #[serde(rename = "CurrentWorkerCount")]
    pub current_worker_count: Option<u32>,

/// 
    #[serde(rename = "NextGroup")]
    pub next_group: Option<u16>,

/// 
    #[serde(rename = "NextNode")]
    pub next_node: Option<u32>,

/// 
    #[serde(rename = "NextWorkerCount")]
    pub next_worker_count: Option<u32>,

/// 
    #[serde(rename = "PoolId")]
    pub pool_id: Option<u32>,
}

impl TP_V2_WTNodeSwitch {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: ThreadPoolTrace_V2::new(),
            current_group: None,
            current_node: None,
            current_worker_count: None,
            next_group: None,
            next_node: None,
            next_worker_count: None,
            pool_id: None,
        }
    }


    /// Sets the value of CurrentGroup
    pub fn set_current_group(&mut self, value: u16) {
        self.current_group = Some(value);
    }

    /// Gets the value of CurrentGroup
    pub fn get_current_group(&self) -> Option<&u16> {
        self.current_group.as_ref()
    }

    /// Sets the value of CurrentNode
    pub fn set_current_node(&mut self, value: u32) {
        self.current_node = Some(value);
    }

    /// Gets the value of CurrentNode
    pub fn get_current_node(&self) -> Option<&u32> {
        self.current_node.as_ref()
    }

    /// Sets the value of CurrentWorkerCount
    pub fn set_current_worker_count(&mut self, value: u32) {
        self.current_worker_count = Some(value);
    }

    /// Gets the value of CurrentWorkerCount
    pub fn get_current_worker_count(&self) -> Option<&u32> {
        self.current_worker_count.as_ref()
    }

    /// Sets the value of NextGroup
    pub fn set_next_group(&mut self, value: u16) {
        self.next_group = Some(value);
    }

    /// Gets the value of NextGroup
    pub fn get_next_group(&self) -> Option<&u16> {
        self.next_group.as_ref()
    }

    /// Sets the value of NextNode
    pub fn set_next_node(&mut self, value: u32) {
        self.next_node = Some(value);
    }

    /// Gets the value of NextNode
    pub fn get_next_node(&self) -> Option<&u32> {
        self.next_node.as_ref()
    }

    /// Sets the value of NextWorkerCount
    pub fn set_next_worker_count(&mut self, value: u32) {
        self.next_worker_count = Some(value);
    }

    /// Gets the value of NextWorkerCount
    pub fn get_next_worker_count(&self) -> Option<&u32> {
        self.next_worker_count.as_ref()
    }

    /// Sets the value of PoolId
    pub fn set_pool_id(&mut self, value: u32) {
        self.pool_id = Some(value);
    }

    /// Gets the value of PoolId
    pub fn get_pool_id(&self) -> Option<&u32> {
        self.pool_id.as_ref()
    }
}

