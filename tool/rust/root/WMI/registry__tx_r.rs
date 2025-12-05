// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Registry_TxR struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Registry_TxR {
    #[serde(flatten)]
    pub base: Registry,

/// 
    #[serde(rename = "Hive")]
    pub hive: Option<String>,

/// 
    #[serde(rename = "OperationTime")]
    pub operation_time: Option<u64>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<u32>,

/// 
    #[serde(rename = "TxrGUID")]
    pub txr_guid: Option<serde_json::Value>,

/// 
    #[serde(rename = "UowCount")]
    pub uow_count: Option<u32>,
}

impl Registry_TxR {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Registry::new(),
            hive: None,
            operation_time: None,
            status: None,
            txr_guid: None,
            uow_count: None,
        }
    }


    /// Sets the value of Hive
    pub fn set_hive(&mut self, value: String) {
        self.hive = Some(value);
    }

    /// Gets the value of Hive
    pub fn get_hive(&self) -> Option<&String> {
        self.hive.as_ref()
    }

    /// Sets the value of OperationTime
    pub fn set_operation_time(&mut self, value: u64) {
        self.operation_time = Some(value);
    }

    /// Gets the value of OperationTime
    pub fn get_operation_time(&self) -> Option<&u64> {
        self.operation_time.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u32> {
        self.status.as_ref()
    }

    /// Sets the value of TxrGUID
    pub fn set_txr_guid(&mut self, value: serde_json::Value) {
        self.txr_guid = Some(value);
    }

    /// Gets the value of TxrGUID
    pub fn get_txr_guid(&self) -> Option<&serde_json::Value> {
        self.txr_guid.as_ref()
    }

    /// Sets the value of UowCount
    pub fn set_uow_count(&mut self, value: u32) {
        self.uow_count = Some(value);
    }

    /// Gets the value of UowCount
    pub fn get_uow_count(&self) -> Option<&u32> {
        self.uow_count.as_ref()
    }
}

