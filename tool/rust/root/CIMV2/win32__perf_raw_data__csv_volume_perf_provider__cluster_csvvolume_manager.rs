// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_CsvVolumePerfProvider_ClusterCSVVolumeManager struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_CsvVolumePerfProvider_ClusterCSVVolumeManager {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "DirectIOFailureRedirection")]
    pub direct_iofailure_redirection: Option<u64>,

/// 
    #[serde(rename = "DirectIOFailureRedirectionPersec")]
    pub direct_iofailure_redirection_persec: Option<u64>,

/// 
    #[serde(rename = "IOReadBytes")]
    pub ioread_bytes: Option<u64>,

/// 
    #[serde(rename = "IOReadBytesPersec")]
    pub ioread_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "IOReadBytesPersecRedirected")]
    pub ioread_bytes_persec_redirected: Option<u64>,

/// 
    #[serde(rename = "IOReadBytesRedirected")]
    pub ioread_bytes_redirected: Option<u64>,

/// 
    #[serde(rename = "IOReadPersecRedirected")]
    pub ioread_persec_redirected: Option<u64>,

/// 
    #[serde(rename = "IOReads")]
    pub ioreads: Option<u64>,

/// 
    #[serde(rename = "IOReadsPersec")]
    pub ioreads_persec: Option<u64>,

/// 
    #[serde(rename = "IOReadsRedirected")]
    pub ioreads_redirected: Option<u64>,

/// 
    #[serde(rename = "IOWriteBytes")]
    pub iowrite_bytes: Option<u64>,

/// 
    #[serde(rename = "IOWriteBytesPersec")]
    pub iowrite_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "IOWriteBytesPersecRedirected")]
    pub iowrite_bytes_persec_redirected: Option<u64>,

/// 
    #[serde(rename = "IOWriteBytesRedirected")]
    pub iowrite_bytes_redirected: Option<u64>,

/// 
    #[serde(rename = "IOWrites")]
    pub iowrites: Option<u64>,

/// 
    #[serde(rename = "IOWritesPersec")]
    pub iowrites_persec: Option<u64>,

/// 
    #[serde(rename = "IOWritesPersecRedirected")]
    pub iowrites_persec_redirected: Option<u64>,

/// 
    #[serde(rename = "IOWritesRedirected")]
    pub iowrites_redirected: Option<u64>,
}

impl Win32_PerfRawData_CsvVolumePerfProvider_ClusterCSVVolumeManager {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            direct_iofailure_redirection: None,
            direct_iofailure_redirection_persec: None,
            ioread_bytes: None,
            ioread_bytes_persec: None,
            ioread_bytes_persec_redirected: None,
            ioread_bytes_redirected: None,
            ioread_persec_redirected: None,
            ioreads: None,
            ioreads_persec: None,
            ioreads_redirected: None,
            iowrite_bytes: None,
            iowrite_bytes_persec: None,
            iowrite_bytes_persec_redirected: None,
            iowrite_bytes_redirected: None,
            iowrites: None,
            iowrites_persec: None,
            iowrites_persec_redirected: None,
            iowrites_redirected: None,
        }
    }


    /// Sets the value of DirectIOFailureRedirection
    pub fn set_direct_iofailure_redirection(&mut self, value: u64) {
        self.direct_iofailure_redirection = Some(value);
    }

    /// Gets the value of DirectIOFailureRedirection
    pub fn get_direct_iofailure_redirection(&self) -> Option<&u64> {
        self.direct_iofailure_redirection.as_ref()
    }

    /// Sets the value of DirectIOFailureRedirectionPersec
    pub fn set_direct_iofailure_redirection_persec(&mut self, value: u64) {
        self.direct_iofailure_redirection_persec = Some(value);
    }

    /// Gets the value of DirectIOFailureRedirectionPersec
    pub fn get_direct_iofailure_redirection_persec(&self) -> Option<&u64> {
        self.direct_iofailure_redirection_persec.as_ref()
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

    /// Sets the value of IOReadBytesPersecRedirected
    pub fn set_ioread_bytes_persec_redirected(&mut self, value: u64) {
        self.ioread_bytes_persec_redirected = Some(value);
    }

    /// Gets the value of IOReadBytesPersecRedirected
    pub fn get_ioread_bytes_persec_redirected(&self) -> Option<&u64> {
        self.ioread_bytes_persec_redirected.as_ref()
    }

    /// Sets the value of IOReadBytesRedirected
    pub fn set_ioread_bytes_redirected(&mut self, value: u64) {
        self.ioread_bytes_redirected = Some(value);
    }

    /// Gets the value of IOReadBytesRedirected
    pub fn get_ioread_bytes_redirected(&self) -> Option<&u64> {
        self.ioread_bytes_redirected.as_ref()
    }

    /// Sets the value of IOReadPersecRedirected
    pub fn set_ioread_persec_redirected(&mut self, value: u64) {
        self.ioread_persec_redirected = Some(value);
    }

    /// Gets the value of IOReadPersecRedirected
    pub fn get_ioread_persec_redirected(&self) -> Option<&u64> {
        self.ioread_persec_redirected.as_ref()
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

    /// Sets the value of IOReadsRedirected
    pub fn set_ioreads_redirected(&mut self, value: u64) {
        self.ioreads_redirected = Some(value);
    }

    /// Gets the value of IOReadsRedirected
    pub fn get_ioreads_redirected(&self) -> Option<&u64> {
        self.ioreads_redirected.as_ref()
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

    /// Sets the value of IOWriteBytesPersecRedirected
    pub fn set_iowrite_bytes_persec_redirected(&mut self, value: u64) {
        self.iowrite_bytes_persec_redirected = Some(value);
    }

    /// Gets the value of IOWriteBytesPersecRedirected
    pub fn get_iowrite_bytes_persec_redirected(&self) -> Option<&u64> {
        self.iowrite_bytes_persec_redirected.as_ref()
    }

    /// Sets the value of IOWriteBytesRedirected
    pub fn set_iowrite_bytes_redirected(&mut self, value: u64) {
        self.iowrite_bytes_redirected = Some(value);
    }

    /// Gets the value of IOWriteBytesRedirected
    pub fn get_iowrite_bytes_redirected(&self) -> Option<&u64> {
        self.iowrite_bytes_redirected.as_ref()
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

    /// Sets the value of IOWritesPersecRedirected
    pub fn set_iowrites_persec_redirected(&mut self, value: u64) {
        self.iowrites_persec_redirected = Some(value);
    }

    /// Gets the value of IOWritesPersecRedirected
    pub fn get_iowrites_persec_redirected(&self) -> Option<&u64> {
        self.iowrites_persec_redirected.as_ref()
    }

    /// Sets the value of IOWritesRedirected
    pub fn set_iowrites_redirected(&mut self, value: u64) {
        self.iowrites_redirected = Some(value);
    }

    /// Gets the value of IOWritesRedirected
    pub fn get_iowrites_redirected(&self) -> Option<&u64> {
        self.iowrites_redirected.as_ref()
    }
}

