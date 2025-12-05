// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_StorageSpacesVirtualDiskIo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_StorageSpacesVirtualDiskIo {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "VirtualDiskFlushLatencyms")]
    pub virtual_disk_flush_latencyms: Option<u32>,

/// 
    #[serde(rename = "VirtualDiskReadBytesPersec")]
    pub virtual_disk_read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskReadLatencyms")]
    pub virtual_disk_read_latencyms: Option<u32>,

/// 
    #[serde(rename = "VirtualDiskWriteBytesPersec")]
    pub virtual_disk_write_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskWriteLatencyms")]
    pub virtual_disk_write_latencyms: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_StorageSpacesVirtualDiskIo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            virtual_disk_flush_latencyms: None,
            virtual_disk_read_bytes_persec: None,
            virtual_disk_read_latencyms: None,
            virtual_disk_write_bytes_persec: None,
            virtual_disk_write_latencyms: None,
        }
    }


    /// Sets the value of VirtualDiskFlushLatencyms
    pub fn set_virtual_disk_flush_latencyms(&mut self, value: u32) {
        self.virtual_disk_flush_latencyms = Some(value);
    }

    /// Gets the value of VirtualDiskFlushLatencyms
    pub fn get_virtual_disk_flush_latencyms(&self) -> Option<&u32> {
        self.virtual_disk_flush_latencyms.as_ref()
    }

    /// Sets the value of VirtualDiskReadBytesPersec
    pub fn set_virtual_disk_read_bytes_persec(&mut self, value: u64) {
        self.virtual_disk_read_bytes_persec = Some(value);
    }

    /// Gets the value of VirtualDiskReadBytesPersec
    pub fn get_virtual_disk_read_bytes_persec(&self) -> Option<&u64> {
        self.virtual_disk_read_bytes_persec.as_ref()
    }

    /// Sets the value of VirtualDiskReadLatencyms
    pub fn set_virtual_disk_read_latencyms(&mut self, value: u32) {
        self.virtual_disk_read_latencyms = Some(value);
    }

    /// Gets the value of VirtualDiskReadLatencyms
    pub fn get_virtual_disk_read_latencyms(&self) -> Option<&u32> {
        self.virtual_disk_read_latencyms.as_ref()
    }

    /// Sets the value of VirtualDiskWriteBytesPersec
    pub fn set_virtual_disk_write_bytes_persec(&mut self, value: u64) {
        self.virtual_disk_write_bytes_persec = Some(value);
    }

    /// Gets the value of VirtualDiskWriteBytesPersec
    pub fn get_virtual_disk_write_bytes_persec(&self) -> Option<&u64> {
        self.virtual_disk_write_bytes_persec.as_ref()
    }

    /// Sets the value of VirtualDiskWriteLatencyms
    pub fn set_virtual_disk_write_latencyms(&mut self, value: u32) {
        self.virtual_disk_write_latencyms = Some(value);
    }

    /// Gets the value of VirtualDiskWriteLatencyms
    pub fn get_virtual_disk_write_latencyms(&self) -> Option<&u32> {
        self.virtual_disk_write_latencyms.as_ref()
    }
}

