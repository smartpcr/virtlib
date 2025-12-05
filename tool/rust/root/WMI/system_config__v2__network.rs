// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V2_Network struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V2_Network {
    #[serde(flatten)]
    pub base: SystemConfig_V2,

/// 
    #[serde(rename = "MaxHashTableSize")]
    pub max_hash_table_size: Option<u32>,

/// 
    #[serde(rename = "MaxUserPort")]
    pub max_user_port: Option<u32>,

/// 
    #[serde(rename = "TcbTablePartitions")]
    pub tcb_table_partitions: Option<u32>,

/// 
    #[serde(rename = "TcpTimedWaitDelay")]
    pub tcp_timed_wait_delay: Option<u32>,
}

impl SystemConfig_V2_Network {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V2::new(),
            max_hash_table_size: None,
            max_user_port: None,
            tcb_table_partitions: None,
            tcp_timed_wait_delay: None,
        }
    }


    /// Sets the value of MaxHashTableSize
    pub fn set_max_hash_table_size(&mut self, value: u32) {
        self.max_hash_table_size = Some(value);
    }

    /// Gets the value of MaxHashTableSize
    pub fn get_max_hash_table_size(&self) -> Option<&u32> {
        self.max_hash_table_size.as_ref()
    }

    /// Sets the value of MaxUserPort
    pub fn set_max_user_port(&mut self, value: u32) {
        self.max_user_port = Some(value);
    }

    /// Gets the value of MaxUserPort
    pub fn get_max_user_port(&self) -> Option<&u32> {
        self.max_user_port.as_ref()
    }

    /// Sets the value of TcbTablePartitions
    pub fn set_tcb_table_partitions(&mut self, value: u32) {
        self.tcb_table_partitions = Some(value);
    }

    /// Gets the value of TcbTablePartitions
    pub fn get_tcb_table_partitions(&self) -> Option<&u32> {
        self.tcb_table_partitions.as_ref()
    }

    /// Sets the value of TcpTimedWaitDelay
    pub fn set_tcp_timed_wait_delay(&mut self, value: u32) {
        self.tcp_timed_wait_delay = Some(value);
    }

    /// Gets the value of TcpTimedWaitDelay
    pub fn get_tcp_timed_wait_delay(&self) -> Option<&u32> {
        self.tcp_timed_wait_delay.as_ref()
    }
}

