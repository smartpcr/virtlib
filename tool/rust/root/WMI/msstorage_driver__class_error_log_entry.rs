// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSStorageDriver_ClassErrorLogEntry struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSStorageDriver_ClassErrorLogEntry {

/// Error Paging
    #[serde(rename = "errorPaging")]
    pub error_paging: Option<bool>,

/// Error Reserved
    #[serde(rename = "errorReserved")]
    pub error_reserved: Option<u8>,

/// Error Retried
    #[serde(rename = "errorRetried")]
    pub error_retried: Option<bool>,

/// Error Unhandled
    #[serde(rename = "errorUnhandled")]
    pub error_unhandled: Option<bool>,

/// Event Time
    #[serde(rename = "eventTime")]
    pub event_time: Option<String>,

/// Port Number
    #[serde(rename = "portNumber")]
    pub port_number: Option<u32>,

/// Reserved
    #[serde(rename = "reserved")]
    pub reserved: Vec<u8>,

/// Sense Data
    #[serde(rename = "senseData")]
    pub sense_data: Option<MSStorageDriver_SenseData>,

/// SCSI Request Block
    #[serde(rename = "srb")]
    pub srb: Option<MSStorageDriver_ScsiRequestBlock>,

/// Tick Count
    #[serde(rename = "tickCount")]
    pub tick_count: Option<u64>,
}

impl MSStorageDriver_ClassErrorLogEntry {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            error_paging: None,
            error_reserved: None,
            error_retried: None,
            error_unhandled: None,
            event_time: None,
            port_number: None,
            reserved: Vec::new(),
            sense_data: None,
            srb: None,
            tick_count: None,
        }
    }


    /// Sets the value of errorPaging
    pub fn set_error_paging(&mut self, value: bool) {
        self.error_paging = Some(value);
    }

    /// Gets the value of errorPaging
    pub fn get_error_paging(&self) -> Option<&bool> {
        self.error_paging.as_ref()
    }

    /// Sets the value of errorReserved
    pub fn set_error_reserved(&mut self, value: u8) {
        self.error_reserved = Some(value);
    }

    /// Gets the value of errorReserved
    pub fn get_error_reserved(&self) -> Option<&u8> {
        self.error_reserved.as_ref()
    }

    /// Sets the value of errorRetried
    pub fn set_error_retried(&mut self, value: bool) {
        self.error_retried = Some(value);
    }

    /// Gets the value of errorRetried
    pub fn get_error_retried(&self) -> Option<&bool> {
        self.error_retried.as_ref()
    }

    /// Sets the value of errorUnhandled
    pub fn set_error_unhandled(&mut self, value: bool) {
        self.error_unhandled = Some(value);
    }

    /// Gets the value of errorUnhandled
    pub fn get_error_unhandled(&self) -> Option<&bool> {
        self.error_unhandled.as_ref()
    }

    /// Sets the value of eventTime
    pub fn set_event_time(&mut self, value: String) {
        self.event_time = Some(value);
    }

    /// Gets the value of eventTime
    pub fn get_event_time(&self) -> Option<&String> {
        self.event_time.as_ref()
    }

    /// Sets the value of portNumber
    pub fn set_port_number(&mut self, value: u32) {
        self.port_number = Some(value);
    }

    /// Gets the value of portNumber
    pub fn get_port_number(&self) -> Option<&u32> {
        self.port_number.as_ref()
    }

    /// Sets the value of reserved
    pub fn set_reserved(&mut self, value: Vec<u8>) {
        self.reserved = value;
    }

    /// Gets the value of reserved
    pub fn get_reserved(&self) -> &Vec<u8> {
        &self.reserved
    }

    /// Sets the value of senseData
    pub fn set_sense_data(&mut self, value: MSStorageDriver_SenseData) {
        self.sense_data = Some(value);
    }

    /// Gets the value of senseData
    pub fn get_sense_data(&self) -> Option<&MSStorageDriver_SenseData> {
        self.sense_data.as_ref()
    }

    /// Sets the value of srb
    pub fn set_srb(&mut self, value: MSStorageDriver_ScsiRequestBlock) {
        self.srb = Some(value);
    }

    /// Gets the value of srb
    pub fn get_srb(&self) -> Option<&MSStorageDriver_ScsiRequestBlock> {
        self.srb.as_ref()
    }

    /// Sets the value of tickCount
    pub fn set_tick_count(&mut self, value: u64) {
        self.tick_count = Some(value);
    }

    /// Gets the value of tickCount
    pub fn get_tick_count(&self) -> Option<&u64> {
        self.tick_count.as_ref()
    }
}

