// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_SvhdxFltPerfProvider_HyperVSharedVHDX struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_SvhdxFltPerfProvider_HyperVSharedVHDX {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "Averagebytesperread")]
    pub averagebytesperread: Option<u64>,

/// 
    #[serde(rename = "Averagebytesperrequest")]
    pub averagebytesperrequest: Option<u64>,

/// 
    #[serde(rename = "Averagebytesperwrite")]
    pub averagebytesperwrite: Option<u64>,

/// 
    #[serde(rename = "Averagequeuelength")]
    pub averagequeuelength: Option<u64>,

/// 
    #[serde(rename = "Averagereadqueuelength")]
    pub averagereadqueuelength: Option<u64>,

/// 
    #[serde(rename = "AverageSharedVHDXdisklogsize")]
    pub average_shared_vhdxdisklogsize: Option<u64>,

/// 
    #[serde(rename = "AverageSharedVHDXdisktotalsize")]
    pub average_shared_vhdxdisktotalsize: Option<u64>,

/// 
    #[serde(rename = "AverageSharedVHDXmounttime")]
    pub average_shared_vhdxmounttime: Option<u32>,

/// 
    #[serde(rename = "Averagetimeperread")]
    pub averagetimeperread: Option<u32>,

/// 
    #[serde(rename = "Averagetimeperrequest")]
    pub averagetimeperrequest: Option<u32>,

/// 
    #[serde(rename = "Averagetimeperwrite")]
    pub averagetimeperwrite: Option<u32>,

/// 
    #[serde(rename = "Averagewritequeuelength")]
    pub averagewritequeuelength: Option<u64>,

/// 
    #[serde(rename = "Currentqueuelength")]
    pub currentqueuelength: Option<u32>,

/// 
    #[serde(rename = "Currentreadqueuelength")]
    pub currentreadqueuelength: Option<u32>,

/// 
    #[serde(rename = "Currentwritequeuelength")]
    pub currentwritequeuelength: Option<u32>,

/// 
    #[serde(rename = "Errorspersecond")]
    pub errorspersecond: Option<u64>,

/// 
    #[serde(rename = "InitiatorHandleOpenspersecond")]
    pub initiator_handle_openspersecond: Option<u32>,

/// 
    #[serde(rename = "ReadBytesPersec")]
    pub read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "ReadRequestsPersec")]
    pub read_requests_persec: Option<u32>,

/// 
    #[serde(rename = "SharedVHDXMountspersecond")]
    pub shared_vhdxmountspersecond: Option<u32>,

/// 
    #[serde(rename = "TotalBytesPersec")]
    pub total_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "TotalRequestsPersec")]
    pub total_requests_persec: Option<u32>,

/// 
    #[serde(rename = "WriteRequestsPersec")]
    pub write_requests_persec: Option<u32>,

/// 
    #[serde(rename = "WrittenBytesPersec")]
    pub written_bytes_persec: Option<u64>,
}

impl Win32_PerfFormattedData_SvhdxFltPerfProvider_HyperVSharedVHDX {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            averagebytesperread: None,
            averagebytesperrequest: None,
            averagebytesperwrite: None,
            averagequeuelength: None,
            averagereadqueuelength: None,
            average_shared_vhdxdisklogsize: None,
            average_shared_vhdxdisktotalsize: None,
            average_shared_vhdxmounttime: None,
            averagetimeperread: None,
            averagetimeperrequest: None,
            averagetimeperwrite: None,
            averagewritequeuelength: None,
            currentqueuelength: None,
            currentreadqueuelength: None,
            currentwritequeuelength: None,
            errorspersecond: None,
            initiator_handle_openspersecond: None,
            read_bytes_persec: None,
            read_requests_persec: None,
            shared_vhdxmountspersecond: None,
            total_bytes_persec: None,
            total_requests_persec: None,
            write_requests_persec: None,
            written_bytes_persec: None,
        }
    }


    /// Sets the value of Averagebytesperread
    pub fn set_averagebytesperread(&mut self, value: u64) {
        self.averagebytesperread = Some(value);
    }

    /// Gets the value of Averagebytesperread
    pub fn get_averagebytesperread(&self) -> Option<&u64> {
        self.averagebytesperread.as_ref()
    }

    /// Sets the value of Averagebytesperrequest
    pub fn set_averagebytesperrequest(&mut self, value: u64) {
        self.averagebytesperrequest = Some(value);
    }

    /// Gets the value of Averagebytesperrequest
    pub fn get_averagebytesperrequest(&self) -> Option<&u64> {
        self.averagebytesperrequest.as_ref()
    }

    /// Sets the value of Averagebytesperwrite
    pub fn set_averagebytesperwrite(&mut self, value: u64) {
        self.averagebytesperwrite = Some(value);
    }

    /// Gets the value of Averagebytesperwrite
    pub fn get_averagebytesperwrite(&self) -> Option<&u64> {
        self.averagebytesperwrite.as_ref()
    }

    /// Sets the value of Averagequeuelength
    pub fn set_averagequeuelength(&mut self, value: u64) {
        self.averagequeuelength = Some(value);
    }

    /// Gets the value of Averagequeuelength
    pub fn get_averagequeuelength(&self) -> Option<&u64> {
        self.averagequeuelength.as_ref()
    }

    /// Sets the value of Averagereadqueuelength
    pub fn set_averagereadqueuelength(&mut self, value: u64) {
        self.averagereadqueuelength = Some(value);
    }

    /// Gets the value of Averagereadqueuelength
    pub fn get_averagereadqueuelength(&self) -> Option<&u64> {
        self.averagereadqueuelength.as_ref()
    }

    /// Sets the value of AverageSharedVHDXdisklogsize
    pub fn set_average_shared_vhdxdisklogsize(&mut self, value: u64) {
        self.average_shared_vhdxdisklogsize = Some(value);
    }

    /// Gets the value of AverageSharedVHDXdisklogsize
    pub fn get_average_shared_vhdxdisklogsize(&self) -> Option<&u64> {
        self.average_shared_vhdxdisklogsize.as_ref()
    }

    /// Sets the value of AverageSharedVHDXdisktotalsize
    pub fn set_average_shared_vhdxdisktotalsize(&mut self, value: u64) {
        self.average_shared_vhdxdisktotalsize = Some(value);
    }

    /// Gets the value of AverageSharedVHDXdisktotalsize
    pub fn get_average_shared_vhdxdisktotalsize(&self) -> Option<&u64> {
        self.average_shared_vhdxdisktotalsize.as_ref()
    }

    /// Sets the value of AverageSharedVHDXmounttime
    pub fn set_average_shared_vhdxmounttime(&mut self, value: u32) {
        self.average_shared_vhdxmounttime = Some(value);
    }

    /// Gets the value of AverageSharedVHDXmounttime
    pub fn get_average_shared_vhdxmounttime(&self) -> Option<&u32> {
        self.average_shared_vhdxmounttime.as_ref()
    }

    /// Sets the value of Averagetimeperread
    pub fn set_averagetimeperread(&mut self, value: u32) {
        self.averagetimeperread = Some(value);
    }

    /// Gets the value of Averagetimeperread
    pub fn get_averagetimeperread(&self) -> Option<&u32> {
        self.averagetimeperread.as_ref()
    }

    /// Sets the value of Averagetimeperrequest
    pub fn set_averagetimeperrequest(&mut self, value: u32) {
        self.averagetimeperrequest = Some(value);
    }

    /// Gets the value of Averagetimeperrequest
    pub fn get_averagetimeperrequest(&self) -> Option<&u32> {
        self.averagetimeperrequest.as_ref()
    }

    /// Sets the value of Averagetimeperwrite
    pub fn set_averagetimeperwrite(&mut self, value: u32) {
        self.averagetimeperwrite = Some(value);
    }

    /// Gets the value of Averagetimeperwrite
    pub fn get_averagetimeperwrite(&self) -> Option<&u32> {
        self.averagetimeperwrite.as_ref()
    }

    /// Sets the value of Averagewritequeuelength
    pub fn set_averagewritequeuelength(&mut self, value: u64) {
        self.averagewritequeuelength = Some(value);
    }

    /// Gets the value of Averagewritequeuelength
    pub fn get_averagewritequeuelength(&self) -> Option<&u64> {
        self.averagewritequeuelength.as_ref()
    }

    /// Sets the value of Currentqueuelength
    pub fn set_currentqueuelength(&mut self, value: u32) {
        self.currentqueuelength = Some(value);
    }

    /// Gets the value of Currentqueuelength
    pub fn get_currentqueuelength(&self) -> Option<&u32> {
        self.currentqueuelength.as_ref()
    }

    /// Sets the value of Currentreadqueuelength
    pub fn set_currentreadqueuelength(&mut self, value: u32) {
        self.currentreadqueuelength = Some(value);
    }

    /// Gets the value of Currentreadqueuelength
    pub fn get_currentreadqueuelength(&self) -> Option<&u32> {
        self.currentreadqueuelength.as_ref()
    }

    /// Sets the value of Currentwritequeuelength
    pub fn set_currentwritequeuelength(&mut self, value: u32) {
        self.currentwritequeuelength = Some(value);
    }

    /// Gets the value of Currentwritequeuelength
    pub fn get_currentwritequeuelength(&self) -> Option<&u32> {
        self.currentwritequeuelength.as_ref()
    }

    /// Sets the value of Errorspersecond
    pub fn set_errorspersecond(&mut self, value: u64) {
        self.errorspersecond = Some(value);
    }

    /// Gets the value of Errorspersecond
    pub fn get_errorspersecond(&self) -> Option<&u64> {
        self.errorspersecond.as_ref()
    }

    /// Sets the value of InitiatorHandleOpenspersecond
    pub fn set_initiator_handle_openspersecond(&mut self, value: u32) {
        self.initiator_handle_openspersecond = Some(value);
    }

    /// Gets the value of InitiatorHandleOpenspersecond
    pub fn get_initiator_handle_openspersecond(&self) -> Option<&u32> {
        self.initiator_handle_openspersecond.as_ref()
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
    pub fn set_read_requests_persec(&mut self, value: u32) {
        self.read_requests_persec = Some(value);
    }

    /// Gets the value of ReadRequestsPersec
    pub fn get_read_requests_persec(&self) -> Option<&u32> {
        self.read_requests_persec.as_ref()
    }

    /// Sets the value of SharedVHDXMountspersecond
    pub fn set_shared_vhdxmountspersecond(&mut self, value: u32) {
        self.shared_vhdxmountspersecond = Some(value);
    }

    /// Gets the value of SharedVHDXMountspersecond
    pub fn get_shared_vhdxmountspersecond(&self) -> Option<&u32> {
        self.shared_vhdxmountspersecond.as_ref()
    }

    /// Sets the value of TotalBytesPersec
    pub fn set_total_bytes_persec(&mut self, value: u64) {
        self.total_bytes_persec = Some(value);
    }

    /// Gets the value of TotalBytesPersec
    pub fn get_total_bytes_persec(&self) -> Option<&u64> {
        self.total_bytes_persec.as_ref()
    }

    /// Sets the value of TotalRequestsPersec
    pub fn set_total_requests_persec(&mut self, value: u32) {
        self.total_requests_persec = Some(value);
    }

    /// Gets the value of TotalRequestsPersec
    pub fn get_total_requests_persec(&self) -> Option<&u32> {
        self.total_requests_persec.as_ref()
    }

    /// Sets the value of WriteRequestsPersec
    pub fn set_write_requests_persec(&mut self, value: u32) {
        self.write_requests_persec = Some(value);
    }

    /// Gets the value of WriteRequestsPersec
    pub fn get_write_requests_persec(&self) -> Option<&u32> {
        self.write_requests_persec.as_ref()
    }

    /// Sets the value of WrittenBytesPersec
    pub fn set_written_bytes_persec(&mut self, value: u64) {
        self.written_bytes_persec = Some(value);
    }

    /// Gets the value of WrittenBytesPersec
    pub fn get_written_bytes_persec(&self) -> Option<&u64> {
        self.written_bytes_persec.as_ref()
    }
}

