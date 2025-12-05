// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Csv20FilterPerfProvider_ClusterCSVCoordinator struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Csv20FilterPerfProvider_ClusterCSVCoordinator {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "CreateFile")]
    pub create_file: Option<u64>,

/// 
    #[serde(rename = "CreateFilePersec")]
    pub create_file_persec: Option<u64>,

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
    #[serde(rename = "IOWriteBytesPersec")]
    pub iowrite_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "IOWrites")]
    pub iowrites: Option<u64>,

/// 
    #[serde(rename = "IOWritesBytes")]
    pub iowrites_bytes: Option<u64>,

/// 
    #[serde(rename = "IOWritesPersec")]
    pub iowrites_persec: Option<u64>,

/// 
    #[serde(rename = "MetadataIO")]
    pub metadata_io: Option<u64>,

/// 
    #[serde(rename = "MetadataIOPersec")]
    pub metadata_iopersec: Option<u64>,
}

impl Win32_PerfRawData_Csv20FilterPerfProvider_ClusterCSVCoordinator {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            create_file: None,
            create_file_persec: None,
            ioread_bytes: None,
            ioread_bytes_persec: None,
            ioreads: None,
            ioreads_persec: None,
            iowrite_bytes_persec: None,
            iowrites: None,
            iowrites_bytes: None,
            iowrites_persec: None,
            metadata_io: None,
            metadata_iopersec: None,
        }
    }


    /// Sets the value of CreateFile
    pub fn set_create_file(&mut self, value: u64) {
        self.create_file = Some(value);
    }

    /// Gets the value of CreateFile
    pub fn get_create_file(&self) -> Option<&u64> {
        self.create_file.as_ref()
    }

    /// Sets the value of CreateFilePersec
    pub fn set_create_file_persec(&mut self, value: u64) {
        self.create_file_persec = Some(value);
    }

    /// Gets the value of CreateFilePersec
    pub fn get_create_file_persec(&self) -> Option<&u64> {
        self.create_file_persec.as_ref()
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

    /// Sets the value of IOWritesBytes
    pub fn set_iowrites_bytes(&mut self, value: u64) {
        self.iowrites_bytes = Some(value);
    }

    /// Gets the value of IOWritesBytes
    pub fn get_iowrites_bytes(&self) -> Option<&u64> {
        self.iowrites_bytes.as_ref()
    }

    /// Sets the value of IOWritesPersec
    pub fn set_iowrites_persec(&mut self, value: u64) {
        self.iowrites_persec = Some(value);
    }

    /// Gets the value of IOWritesPersec
    pub fn get_iowrites_persec(&self) -> Option<&u64> {
        self.iowrites_persec.as_ref()
    }

    /// Sets the value of MetadataIO
    pub fn set_metadata_io(&mut self, value: u64) {
        self.metadata_io = Some(value);
    }

    /// Gets the value of MetadataIO
    pub fn get_metadata_io(&self) -> Option<&u64> {
        self.metadata_io.as_ref()
    }

    /// Sets the value of MetadataIOPersec
    pub fn set_metadata_iopersec(&mut self, value: u64) {
        self.metadata_iopersec = Some(value);
    }

    /// Gets the value of MetadataIOPersec
    pub fn get_metadata_iopersec(&self) -> Option<&u64> {
        self.metadata_iopersec.as_ref()
    }
}

