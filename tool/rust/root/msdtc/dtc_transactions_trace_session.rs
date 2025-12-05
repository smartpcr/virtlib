// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.msdtc
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DtcTransactionsTraceSession struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DtcTransactionsTraceSession {

/// 
    #[serde(rename = "BufferCount")]
    pub buffer_count: Option<u32>,

/// 
    #[serde(rename = "SessionStatus")]
    pub session_status: Option<String>,
}

impl DtcTransactionsTraceSession {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            buffer_count: None,
            session_status: None,
        }
    }


    /// Sets the value of BufferCount
    pub fn set_buffer_count(&mut self, value: u32) {
        self.buffer_count = Some(value);
    }

    /// Gets the value of BufferCount
    pub fn get_buffer_count(&self) -> Option<&u32> {
        self.buffer_count.as_ref()
    }

    /// Sets the value of SessionStatus
    pub fn set_session_status(&mut self, value: String) {
        self.session_status = Some(value);
    }

    /// Gets the value of SessionStatus
    pub fn get_session_status(&self) -> Option<&String> {
        self.session_status.as_ref()
    }
}

