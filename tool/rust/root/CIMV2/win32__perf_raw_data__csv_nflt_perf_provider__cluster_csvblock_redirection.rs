// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_CsvNfltPerfProvider_ClusterCSVBlockRedirection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_CsvNfltPerfProvider_ClusterCSVBlockRedirection {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "IOReadBytes")]
    pub ioread_bytes: Option<u64>,

/// 
    #[serde(rename = "IOReadBytesPersec")]
    pub ioread_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "IOReads")]
    pub ioreads: Option<u64>,

/// 
    #[serde(rename = "IOReadsPersec")]
    pub ioreads_persec: Option<u64>,

/// 
    #[serde(rename = "IOWriteBytes")]
    pub iowrite_bytes: Option<u64>,

/// 
    #[serde(rename = "IOWriteBytesPersec")]
    pub iowrite_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "IOWrites")]
    pub iowrites: Option<u64>,

/// 
    #[serde(rename = "IOWritesPersec")]
    pub iowrites_persec: Option<u64>,
}

impl Win32_PerfRawData_CsvNfltPerfProvider_ClusterCSVBlockRedirection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            ioread_bytes: None,
            ioread_bytes_persec: None,
            ioreads: None,
            ioreads_persec: None,
            iowrite_bytes: None,
            iowrite_bytes_persec: None,
            iowrites: None,
            iowrites_persec: None,
        }
    }


    /// Sets the value of IOReadBytes
    pub fn set_ioread_bytes(&mut self, value: u64) {
        self.ioread_bytes = Some(value);
    }

    /// Gets the value of IOReadBytes
    pub fn get_ioread_bytes(&self) -> Option<&u64> {
        self.ioread_bytes.as_ref()
    }

    /// Sets the value of IOReadBytesPersec
    pub fn set_ioread_bytes_persec(&mut self, value: u64) {
        self.ioread_bytes_persec = Some(value);
    }

    /// Gets the value of IOReadBytesPersec
    pub fn get_ioread_bytes_persec(&self) -> Option<&u64> {
        self.ioread_bytes_persec.as_ref()
    }

    /// Sets the value of IOReads
    pub fn set_ioreads(&mut self, value: u64) {
        self.ioreads = Some(value);
    }

    /// Gets the value of IOReads
    pub fn get_ioreads(&self) -> Option<&u64> {
        self.ioreads.as_ref()
    }

    /// Sets the value of IOReadsPersec
    pub fn set_ioreads_persec(&mut self, value: u64) {
        self.ioreads_persec = Some(value);
    }

    /// Gets the value of IOReadsPersec
    pub fn get_ioreads_persec(&self) -> Option<&u64> {
        self.ioreads_persec.as_ref()
    }

    /// Sets the value of IOWriteBytes
    pub fn set_iowrite_bytes(&mut self, value: u64) {
        self.iowrite_bytes = Some(value);
    }

    /// Gets the value of IOWriteBytes
    pub fn get_iowrite_bytes(&self) -> Option<&u64> {
        self.iowrite_bytes.as_ref()
    }

    /// Sets the value of IOWriteBytesPersec
    pub fn set_iowrite_bytes_persec(&mut self, value: u64) {
        self.iowrite_bytes_persec = Some(value);
    }

    /// Gets the value of IOWriteBytesPersec
    pub fn get_iowrite_bytes_persec(&self) -> Option<&u64> {
        self.iowrite_bytes_persec.as_ref()
    }

    /// Sets the value of IOWrites
    pub fn set_iowrites(&mut self, value: u64) {
        self.iowrites = Some(value);
    }

    /// Gets the value of IOWrites
    pub fn get_iowrites(&self) -> Option<&u64> {
        self.iowrites.as_ref()
    }

    /// Sets the value of IOWritesPersec
    pub fn set_iowrites_persec(&mut self, value: u64) {
        self.iowrites_persec = Some(value);
    }

    /// Gets the value of IOWritesPersec
    pub fn get_iowrites_persec(&self) -> Option<&u64> {
        self.iowrites_persec.as_ref()
    }
}

