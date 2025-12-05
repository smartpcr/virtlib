// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_VSmbPerfProvider_HyperVVirtualSMB struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_VSmbPerfProvider_HyperVVirtualSMB {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "AvgsecPerRequest")]
    pub avgsec_per_request: Option<u32>,

/// 
    #[serde(rename = "AvgsecPerRequest_Base")]
    pub avgsec_per_request__base: Option<u32>,

/// 
    #[serde(rename = "CurrentOpenFileCount")]
    pub current_open_file_count: Option<u32>,

/// 
    #[serde(rename = "CurrentPendingRequests")]
    pub current_pending_requests: Option<u32>,

/// 
    #[serde(rename = "DirectMappedPages")]
    pub direct_mapped_pages: Option<u64>,

/// 
    #[serde(rename = "DirectMappedSections")]
    pub direct_mapped_sections: Option<u32>,

/// 
    #[serde(rename = "FlushRequestsPersec")]
    pub flush_requests_persec: Option<u32>,

/// 
    #[serde(rename = "ReadBytesPersec")]
    pub read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "ReadBytesPersecRDMA")]
    pub read_bytes_persec_rdma: Option<u64>,

/// 
    #[serde(rename = "ReadRequestsPersec")]
    pub read_requests_persec: Option<u32>,

/// 
    #[serde(rename = "ReadRequestsPersecRDMA")]
    pub read_requests_persec_rdma: Option<u32>,

/// 
    #[serde(rename = "ReceivedBytesPersec")]
    pub received_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "RequestsPersec")]
    pub requests_persec: Option<u32>,

/// 
    #[serde(rename = "SentBytesPersec")]
    pub sent_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "TreeConnectCount")]
    pub tree_connect_count: Option<u32>,

/// 
    #[serde(rename = "WriteBytesPersec")]
    pub write_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "WriteBytesPersecRDMA")]
    pub write_bytes_persec_rdma: Option<u64>,

/// 
    #[serde(rename = "WriteRequestsPersec")]
    pub write_requests_persec: Option<u32>,

/// 
    #[serde(rename = "WriteRequestsPersecRDMA")]
    pub write_requests_persec_rdma: Option<u32>,
}

impl Win32_PerfRawData_VSmbPerfProvider_HyperVVirtualSMB {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            avgsec_per_request: None,
            avgsec_per_request__base: None,
            current_open_file_count: None,
            current_pending_requests: None,
            direct_mapped_pages: None,
            direct_mapped_sections: None,
            flush_requests_persec: None,
            read_bytes_persec: None,
            read_bytes_persec_rdma: None,
            read_requests_persec: None,
            read_requests_persec_rdma: None,
            received_bytes_persec: None,
            requests_persec: None,
            sent_bytes_persec: None,
            tree_connect_count: None,
            write_bytes_persec: None,
            write_bytes_persec_rdma: None,
            write_requests_persec: None,
            write_requests_persec_rdma: None,
        }
    }


    /// Sets the value of AvgsecPerRequest
    pub fn set_avgsec_per_request(&mut self, value: u32) {
        self.avgsec_per_request = Some(value);
    }

    /// Gets the value of AvgsecPerRequest
    pub fn get_avgsec_per_request(&self) -> Option<&u32> {
        self.avgsec_per_request.as_ref()
    }

    /// Sets the value of AvgsecPerRequest_Base
    pub fn set_avgsec_per_request__base(&mut self, value: u32) {
        self.avgsec_per_request__base = Some(value);
    }

    /// Gets the value of AvgsecPerRequest_Base
    pub fn get_avgsec_per_request__base(&self) -> Option<&u32> {
        self.avgsec_per_request__base.as_ref()
    }

    /// Sets the value of CurrentOpenFileCount
    pub fn set_current_open_file_count(&mut self, value: u32) {
        self.current_open_file_count = Some(value);
    }

    /// Gets the value of CurrentOpenFileCount
    pub fn get_current_open_file_count(&self) -> Option<&u32> {
        self.current_open_file_count.as_ref()
    }

    /// Sets the value of CurrentPendingRequests
    pub fn set_current_pending_requests(&mut self, value: u32) {
        self.current_pending_requests = Some(value);
    }

    /// Gets the value of CurrentPendingRequests
    pub fn get_current_pending_requests(&self) -> Option<&u32> {
        self.current_pending_requests.as_ref()
    }

    /// Sets the value of DirectMappedPages
    pub fn set_direct_mapped_pages(&mut self, value: u64) {
        self.direct_mapped_pages = Some(value);
    }

    /// Gets the value of DirectMappedPages
    pub fn get_direct_mapped_pages(&self) -> Option<&u64> {
        self.direct_mapped_pages.as_ref()
    }

    /// Sets the value of DirectMappedSections
    pub fn set_direct_mapped_sections(&mut self, value: u32) {
        self.direct_mapped_sections = Some(value);
    }

    /// Gets the value of DirectMappedSections
    pub fn get_direct_mapped_sections(&self) -> Option<&u32> {
        self.direct_mapped_sections.as_ref()
    }

    /// Sets the value of FlushRequestsPersec
    pub fn set_flush_requests_persec(&mut self, value: u32) {
        self.flush_requests_persec = Some(value);
    }

    /// Gets the value of FlushRequestsPersec
    pub fn get_flush_requests_persec(&self) -> Option<&u32> {
        self.flush_requests_persec.as_ref()
    }

    /// Sets the value of ReadBytesPersec
    pub fn set_read_bytes_persec(&mut self, value: u64) {
        self.read_bytes_persec = Some(value);
    }

    /// Gets the value of ReadBytesPersec
    pub fn get_read_bytes_persec(&self) -> Option<&u64> {
        self.read_bytes_persec.as_ref()
    }

    /// Sets the value of ReadBytesPersecRDMA
    pub fn set_read_bytes_persec_rdma(&mut self, value: u64) {
        self.read_bytes_persec_rdma = Some(value);
    }

    /// Gets the value of ReadBytesPersecRDMA
    pub fn get_read_bytes_persec_rdma(&self) -> Option<&u64> {
        self.read_bytes_persec_rdma.as_ref()
    }

    /// Sets the value of ReadRequestsPersec
    pub fn set_read_requests_persec(&mut self, value: u32) {
        self.read_requests_persec = Some(value);
    }

    /// Gets the value of ReadRequestsPersec
    pub fn get_read_requests_persec(&self) -> Option<&u32> {
        self.read_requests_persec.as_ref()
    }

    /// Sets the value of ReadRequestsPersecRDMA
    pub fn set_read_requests_persec_rdma(&mut self, value: u32) {
        self.read_requests_persec_rdma = Some(value);
    }

    /// Gets the value of ReadRequestsPersecRDMA
    pub fn get_read_requests_persec_rdma(&self) -> Option<&u32> {
        self.read_requests_persec_rdma.as_ref()
    }

    /// Sets the value of ReceivedBytesPersec
    pub fn set_received_bytes_persec(&mut self, value: u64) {
        self.received_bytes_persec = Some(value);
    }

    /// Gets the value of ReceivedBytesPersec
    pub fn get_received_bytes_persec(&self) -> Option<&u64> {
        self.received_bytes_persec.as_ref()
    }

    /// Sets the value of RequestsPersec
    pub fn set_requests_persec(&mut self, value: u32) {
        self.requests_persec = Some(value);
    }

    /// Gets the value of RequestsPersec
    pub fn get_requests_persec(&self) -> Option<&u32> {
        self.requests_persec.as_ref()
    }

    /// Sets the value of SentBytesPersec
    pub fn set_sent_bytes_persec(&mut self, value: u64) {
        self.sent_bytes_persec = Some(value);
    }

    /// Gets the value of SentBytesPersec
    pub fn get_sent_bytes_persec(&self) -> Option<&u64> {
        self.sent_bytes_persec.as_ref()
    }

    /// Sets the value of TreeConnectCount
    pub fn set_tree_connect_count(&mut self, value: u32) {
        self.tree_connect_count = Some(value);
    }

    /// Gets the value of TreeConnectCount
    pub fn get_tree_connect_count(&self) -> Option<&u32> {
        self.tree_connect_count.as_ref()
    }

    /// Sets the value of WriteBytesPersec
    pub fn set_write_bytes_persec(&mut self, value: u64) {
        self.write_bytes_persec = Some(value);
    }

    /// Gets the value of WriteBytesPersec
    pub fn get_write_bytes_persec(&self) -> Option<&u64> {
        self.write_bytes_persec.as_ref()
    }

    /// Sets the value of WriteBytesPersecRDMA
    pub fn set_write_bytes_persec_rdma(&mut self, value: u64) {
        self.write_bytes_persec_rdma = Some(value);
    }

    /// Gets the value of WriteBytesPersecRDMA
    pub fn get_write_bytes_persec_rdma(&self) -> Option<&u64> {
        self.write_bytes_persec_rdma.as_ref()
    }

    /// Sets the value of WriteRequestsPersec
    pub fn set_write_requests_persec(&mut self, value: u32) {
        self.write_requests_persec = Some(value);
    }

    /// Gets the value of WriteRequestsPersec
    pub fn get_write_requests_persec(&self) -> Option<&u32> {
        self.write_requests_persec.as_ref()
    }

    /// Sets the value of WriteRequestsPersecRDMA
    pub fn set_write_requests_persec_rdma(&mut self, value: u32) {
        self.write_requests_persec_rdma = Some(value);
    }

    /// Gets the value of WriteRequestsPersecRDMA
    pub fn get_write_requests_persec_rdma(&self) -> Option<&u32> {
        self.write_requests_persec_rdma.as_ref()
    }
}

