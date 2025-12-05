// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_HyperVVirtualMachineBusPipes struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_HyperVVirtualMachineBusPipes {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "BytesReadPersec")]
    pub bytes_read_persec: Option<u64>,

/// 
    #[serde(rename = "BytesWrittenPersec")]
    pub bytes_written_persec: Option<u64>,

/// 
    #[serde(rename = "ReadsPersec")]
    pub reads_persec: Option<u64>,

/// 
    #[serde(rename = "WritesPersec")]
    pub writes_persec: Option<u64>,
}

impl Win32_PerfFormattedData_Counters_HyperVVirtualMachineBusPipes {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            bytes_read_persec: None,
            bytes_written_persec: None,
            reads_persec: None,
            writes_persec: None,
        }
    }


    /// Sets the value of BytesReadPersec
    pub fn set_bytes_read_persec(&mut self, value: u64) {
        self.bytes_read_persec = Some(value);
    }

    /// Gets the value of BytesReadPersec
    pub fn get_bytes_read_persec(&self) -> Option<&u64> {
        self.bytes_read_persec.as_ref()
    }

    /// Sets the value of BytesWrittenPersec
    pub fn set_bytes_written_persec(&mut self, value: u64) {
        self.bytes_written_persec = Some(value);
    }

    /// Gets the value of BytesWrittenPersec
    pub fn get_bytes_written_persec(&self) -> Option<&u64> {
        self.bytes_written_persec.as_ref()
    }

    /// Sets the value of ReadsPersec
    pub fn set_reads_persec(&mut self, value: u64) {
        self.reads_persec = Some(value);
    }

    /// Gets the value of ReadsPersec
    pub fn get_reads_persec(&self) -> Option<&u64> {
        self.reads_persec.as_ref()
    }

    /// Sets the value of WritesPersec
    pub fn set_writes_persec(&mut self, value: u64) {
        self.writes_persec = Some(value);
    }

    /// Gets the value of WritesPersec
    pub fn get_writes_persec(&self) -> Option<&u64> {
        self.writes_persec.as_ref()
    }
}

