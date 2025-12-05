// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_IdePerfProvider_HyperVVirtualIDEControllerEmulated struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_IdePerfProvider_HyperVVirtualIDEControllerEmulated {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "ReadBytesPersec")]
    pub read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "ReadSectorsPersec")]
    pub read_sectors_persec: Option<u64>,

/// 
    #[serde(rename = "WriteBytesPersec")]
    pub write_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "WrittenSectorsPersec")]
    pub written_sectors_persec: Option<u64>,
}

impl Win32_PerfRawData_IdePerfProvider_HyperVVirtualIDEControllerEmulated {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            read_bytes_persec: None,
            read_sectors_persec: None,
            write_bytes_persec: None,
            written_sectors_persec: None,
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

    /// Sets the value of ReadSectorsPersec
    pub fn set_read_sectors_persec(&mut self, value: u64) {
        self.read_sectors_persec = Some(value);
    }

    /// Gets the value of ReadSectorsPersec
    pub fn get_read_sectors_persec(&self) -> Option<&u64> {
        self.read_sectors_persec.as_ref()
    }

    /// Sets the value of WriteBytesPersec
    pub fn set_write_bytes_persec(&mut self, value: u64) {
        self.write_bytes_persec = Some(value);
    }

    /// Gets the value of WriteBytesPersec
    pub fn get_write_bytes_persec(&self) -> Option<&u64> {
        self.write_bytes_persec.as_ref()
    }

    /// Sets the value of WrittenSectorsPersec
    pub fn set_written_sectors_persec(&mut self, value: u64) {
        self.written_sectors_persec = Some(value);
    }

    /// Gets the value of WrittenSectorsPersec
    pub fn get_written_sectors_persec(&self) -> Option<&u64> {
        self.written_sectors_persec.as_ref()
    }
}

