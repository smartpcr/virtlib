// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_FileSystemDiskActivity struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_FileSystemDiskActivity {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "FileSystemBytesRead")]
    pub file_system_bytes_read: Option<u64>,

/// 
    #[serde(rename = "FileSystemBytesWritten")]
    pub file_system_bytes_written: Option<u64>,
}

impl Win32_PerfFormattedData_Counters_FileSystemDiskActivity {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            file_system_bytes_read: None,
            file_system_bytes_written: None,
        }
    }


    /// Sets the value of FileSystemBytesRead
    pub fn set_file_system_bytes_read(&mut self, value: u64) {
        self.file_system_bytes_read = Some(value);
    }

    /// Gets the value of FileSystemBytesRead
    pub fn get_file_system_bytes_read(&self) -> Option<&u64> {
        self.file_system_bytes_read.as_ref()
    }

    /// Sets the value of FileSystemBytesWritten
    pub fn set_file_system_bytes_written(&mut self, value: u64) {
        self.file_system_bytes_written = Some(value);
    }

    /// Gets the value of FileSystemBytesWritten
    pub fn get_file_system_bytes_written(&self) -> Option<&u64> {
        self.file_system_bytes_written.as_ref()
    }
}

