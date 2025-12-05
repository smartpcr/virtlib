// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_SMBServer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_SMBServer {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "ReadBytesPersec")]
    pub read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "ReadRequestsPersec")]
    pub read_requests_persec: Option<u64>,

/// 
    #[serde(rename = "ReceiveBytesPersec")]
    pub receive_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "SendBytesPersec")]
    pub send_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "WriteBytesPersec")]
    pub write_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "WriteRequestsPersec")]
    pub write_requests_persec: Option<u64>,
}

impl Win32_PerfRawData_Counters_SMBServer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            read_bytes_persec: None,
            read_requests_persec: None,
            receive_bytes_persec: None,
            send_bytes_persec: None,
            write_bytes_persec: None,
            write_requests_persec: None,
        }
    }


    /// Sets the value of ReadBytesPersec
    pub fn set_read_bytes_persec(&mut self, value: u64) {
        self.read_bytes_persec = Some(value);
    }

    /// Gets the value of ReadBytesPersec
    pub fn get_read_bytes_persec(&self) -> Option<&u64> {
        self.read_bytes_persec.as_ref()
    }

    /// Sets the value of ReadRequestsPersec
    pub fn set_read_requests_persec(&mut self, value: u64) {
        self.read_requests_persec = Some(value);
    }

    /// Gets the value of ReadRequestsPersec
    pub fn get_read_requests_persec(&self) -> Option<&u64> {
        self.read_requests_persec.as_ref()
    }

    /// Sets the value of ReceiveBytesPersec
    pub fn set_receive_bytes_persec(&mut self, value: u64) {
        self.receive_bytes_persec = Some(value);
    }

    /// Gets the value of ReceiveBytesPersec
    pub fn get_receive_bytes_persec(&self) -> Option<&u64> {
        self.receive_bytes_persec.as_ref()
    }

    /// Sets the value of SendBytesPersec
    pub fn set_send_bytes_persec(&mut self, value: u64) {
        self.send_bytes_persec = Some(value);
    }

    /// Gets the value of SendBytesPersec
    pub fn get_send_bytes_persec(&self) -> Option<&u64> {
        self.send_bytes_persec.as_ref()
    }

    /// Sets the value of WriteBytesPersec
    pub fn set_write_bytes_persec(&mut self, value: u64) {
        self.write_bytes_persec = Some(value);
    }

    /// Gets the value of WriteBytesPersec
    pub fn get_write_bytes_persec(&self) -> Option<&u64> {
        self.write_bytes_persec.as_ref()
    }

    /// Sets the value of WriteRequestsPersec
    pub fn set_write_requests_persec(&mut self, value: u64) {
        self.write_requests_persec = Some(value);
    }

    /// Gets the value of WriteRequestsPersec
    pub fn get_write_requests_persec(&self) -> Option<&u64> {
        self.write_requests_persec.as_ref()
    }
}

