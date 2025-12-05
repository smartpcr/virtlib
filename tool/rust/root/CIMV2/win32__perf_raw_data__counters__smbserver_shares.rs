// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_SMBServerShares struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_SMBServerShares {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "AttemptedCompressedResponsesPersec")]
    pub attempted_compressed_responses_persec: Option<u64>,

/// 
    #[serde(rename = "AvgBytesPerRead")]
    pub avg_bytes_per_read: Option<u64>,

/// 
    #[serde(rename = "AvgBytesPerRead_Base")]
    pub avg_bytes_per_read__base: Option<u32>,

/// 
    #[serde(rename = "AvgBytesPerWrite")]
    pub avg_bytes_per_write: Option<u64>,

/// 
    #[serde(rename = "AvgBytesPerWrite_Base")]
    pub avg_bytes_per_write__base: Option<u32>,

/// 
    #[serde(rename = "AvgDataBytesPerRequest")]
    pub avg_data_bytes_per_request: Option<u64>,

/// 
    #[serde(rename = "AvgDataBytesPerRequest_Base")]
    pub avg_data_bytes_per_request__base: Option<u32>,

/// 
    #[serde(rename = "AvgDataQueueLength")]
    pub avg_data_queue_length: Option<u64>,

/// 
    #[serde(rename = "AvgReadQueueLength")]
    pub avg_read_queue_length: Option<u64>,

/// 
    #[serde(rename = "AvgsecPerDataRequest")]
    pub avgsec_per_data_request: Option<u32>,

/// 
    #[serde(rename = "AvgsecPerDataRequest_Base")]
    pub avgsec_per_data_request__base: Option<u32>,

/// 
    #[serde(rename = "AvgsecPerRead")]
    pub avgsec_per_read: Option<u32>,

/// 
    #[serde(rename = "AvgsecPerRead_Base")]
    pub avgsec_per_read__base: Option<u32>,

/// 
    #[serde(rename = "AvgsecPerRequest")]
    pub avgsec_per_request: Option<u32>,

/// 
    #[serde(rename = "AvgsecPerRequest_Base")]
    pub avgsec_per_request__base: Option<u32>,

/// 
    #[serde(rename = "AvgsecPerWrite")]
    pub avgsec_per_write: Option<u32>,

/// 
    #[serde(rename = "AvgsecPerWrite_Base")]
    pub avgsec_per_write__base: Option<u32>,

/// 
    #[serde(rename = "AvgWriteQueueLength")]
    pub avg_write_queue_length: Option<u64>,

/// 
    #[serde(rename = "BytesCompressedPersec")]
    pub bytes_compressed_persec: Option<u64>,

/// 
    #[serde(rename = "CompressedRequestsPersec")]
    pub compressed_requests_persec: Option<u64>,

/// 
    #[serde(rename = "CurrentBypassOpenFileCount")]
    pub current_bypass_open_file_count: Option<u64>,

/// 
    #[serde(rename = "CurrentDataQueueLength")]
    pub current_data_queue_length: Option<u64>,

/// 
    #[serde(rename = "CurrentDurableOpenFileCount")]
    pub current_durable_open_file_count: Option<u64>,

/// 
    #[serde(rename = "CurrentOpenFileCount")]
    pub current_open_file_count: Option<u64>,

/// 
    #[serde(rename = "CurrentPendingRequests")]
    pub current_pending_requests: Option<u64>,

/// 
    #[serde(rename = "DataBytesPersec")]
    pub data_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "DataRequestsPersec")]
    pub data_requests_persec: Option<u32>,

/// 
    #[serde(rename = "FilesOpenedPersec")]
    pub files_opened_persec: Option<u64>,

/// 
    #[serde(rename = "MetadataRequestsPersec")]
    pub metadata_requests_persec: Option<u64>,

/// 
    #[serde(rename = "PercentPersistentHandles")]
    pub percent_persistent_handles: Option<u64>,

/// 
    #[serde(rename = "PercentPersistentHandles_Base")]
    pub percent_persistent_handles__base: Option<u64>,

/// 
    #[serde(rename = "PercentResilientHandles")]
    pub percent_resilient_handles: Option<u64>,

/// 
    #[serde(rename = "PercentResilientHandles_Base")]
    pub percent_resilient_handles__base: Option<u64>,

/// 
    #[serde(rename = "ReadBytesPersec")]
    pub read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "ReadBytestransmittedByPassCSVPersec")]
    pub read_bytestransmitted_by_pass_csvpersec: Option<u64>,

/// 
    #[serde(rename = "ReadBytestransmittedviaSMBDirectPersec")]
    pub read_bytestransmittedvia_smbdirect_persec: Option<u64>,

/// 
    #[serde(rename = "ReadRequestsPersec")]
    pub read_requests_persec: Option<u32>,

/// 
    #[serde(rename = "ReadRequeststransmittedviaBypassCSVPersec")]
    pub read_requeststransmittedvia_bypass_csvpersec: Option<u32>,

/// 
    #[serde(rename = "ReadRequeststransmittedviaSMBDirectPersec")]
    pub read_requeststransmittedvia_smbdirect_persec: Option<u32>,

/// 
    #[serde(rename = "ReceivedBytesPersec")]
    pub received_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "RequestsPersec")]
    pub requests_persec: Option<u64>,

/// 
    #[serde(rename = "SentBytesPersec")]
    pub sent_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "SuccessfulCompressedResponsesPersec")]
    pub successful_compressed_responses_persec: Option<u64>,

/// 
    #[serde(rename = "TotalDurableHandleReopenCount")]
    pub total_durable_handle_reopen_count: Option<u64>,

/// 
    #[serde(rename = "TotalFailedDurableHandleReopenCount")]
    pub total_failed_durable_handle_reopen_count: Option<u64>,

/// 
    #[serde(rename = "TotalFailedPersistentHandleReopenCount")]
    pub total_failed_persistent_handle_reopen_count: Option<u64>,

/// 
    #[serde(rename = "TotalFailedResilientHandleReopenCount")]
    pub total_failed_resilient_handle_reopen_count: Option<u64>,

/// 
    #[serde(rename = "TotalFileOpenCount")]
    pub total_file_open_count: Option<u64>,

/// 
    #[serde(rename = "TotalPersistentHandleReopenCount")]
    pub total_persistent_handle_reopen_count: Option<u64>,

/// 
    #[serde(rename = "TotalResilientHandleReopenCount")]
    pub total_resilient_handle_reopen_count: Option<u64>,

/// 
    #[serde(rename = "TransferredBytesPersec")]
    pub transferred_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "TreeConnectCount")]
    pub tree_connect_count: Option<u64>,

/// 
    #[serde(rename = "WriteBytesPersec")]
    pub write_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "WriteBytestransmittedByPassCSVPersec")]
    pub write_bytestransmitted_by_pass_csvpersec: Option<u64>,

/// 
    #[serde(rename = "WriteBytestransmittedviaSMBDirectPersec")]
    pub write_bytestransmittedvia_smbdirect_persec: Option<u64>,

/// 
    #[serde(rename = "WriteRequestsPersec")]
    pub write_requests_persec: Option<u32>,

/// 
    #[serde(rename = "WriteRequeststransmittedviaBypassCSVPersec")]
    pub write_requeststransmittedvia_bypass_csvpersec: Option<u32>,

/// 
    #[serde(rename = "WriteRequeststransmittedviaSMBDirectPersec")]
    pub write_requeststransmittedvia_smbdirect_persec: Option<u32>,
}

impl Win32_PerfRawData_Counters_SMBServerShares {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            attempted_compressed_responses_persec: None,
            avg_bytes_per_read: None,
            avg_bytes_per_read__base: None,
            avg_bytes_per_write: None,
            avg_bytes_per_write__base: None,
            avg_data_bytes_per_request: None,
            avg_data_bytes_per_request__base: None,
            avg_data_queue_length: None,
            avg_read_queue_length: None,
            avgsec_per_data_request: None,
            avgsec_per_data_request__base: None,
            avgsec_per_read: None,
            avgsec_per_read__base: None,
            avgsec_per_request: None,
            avgsec_per_request__base: None,
            avgsec_per_write: None,
            avgsec_per_write__base: None,
            avg_write_queue_length: None,
            bytes_compressed_persec: None,
            compressed_requests_persec: None,
            current_bypass_open_file_count: None,
            current_data_queue_length: None,
            current_durable_open_file_count: None,
            current_open_file_count: None,
            current_pending_requests: None,
            data_bytes_persec: None,
            data_requests_persec: None,
            files_opened_persec: None,
            metadata_requests_persec: None,
            percent_persistent_handles: None,
            percent_persistent_handles__base: None,
            percent_resilient_handles: None,
            percent_resilient_handles__base: None,
            read_bytes_persec: None,
            read_bytestransmitted_by_pass_csvpersec: None,
            read_bytestransmittedvia_smbdirect_persec: None,
            read_requests_persec: None,
            read_requeststransmittedvia_bypass_csvpersec: None,
            read_requeststransmittedvia_smbdirect_persec: None,
            received_bytes_persec: None,
            requests_persec: None,
            sent_bytes_persec: None,
            successful_compressed_responses_persec: None,
            total_durable_handle_reopen_count: None,
            total_failed_durable_handle_reopen_count: None,
            total_failed_persistent_handle_reopen_count: None,
            total_failed_resilient_handle_reopen_count: None,
            total_file_open_count: None,
            total_persistent_handle_reopen_count: None,
            total_resilient_handle_reopen_count: None,
            transferred_bytes_persec: None,
            tree_connect_count: None,
            write_bytes_persec: None,
            write_bytestransmitted_by_pass_csvpersec: None,
            write_bytestransmittedvia_smbdirect_persec: None,
            write_requests_persec: None,
            write_requeststransmittedvia_bypass_csvpersec: None,
            write_requeststransmittedvia_smbdirect_persec: None,
        }
    }


    /// Sets the value of AttemptedCompressedResponsesPersec
    pub fn set_attempted_compressed_responses_persec(&mut self, value: u64) {
        self.attempted_compressed_responses_persec = Some(value);
    }

    /// Gets the value of AttemptedCompressedResponsesPersec
    pub fn get_attempted_compressed_responses_persec(&self) -> Option<&u64> {
        self.attempted_compressed_responses_persec.as_ref()
    }

    /// Sets the value of AvgBytesPerRead
    pub fn set_avg_bytes_per_read(&mut self, value: u64) {
        self.avg_bytes_per_read = Some(value);
    }

    /// Gets the value of AvgBytesPerRead
    pub fn get_avg_bytes_per_read(&self) -> Option<&u64> {
        self.avg_bytes_per_read.as_ref()
    }

    /// Sets the value of AvgBytesPerRead_Base
    pub fn set_avg_bytes_per_read__base(&mut self, value: u32) {
        self.avg_bytes_per_read__base = Some(value);
    }

    /// Gets the value of AvgBytesPerRead_Base
    pub fn get_avg_bytes_per_read__base(&self) -> Option<&u32> {
        self.avg_bytes_per_read__base.as_ref()
    }

    /// Sets the value of AvgBytesPerWrite
    pub fn set_avg_bytes_per_write(&mut self, value: u64) {
        self.avg_bytes_per_write = Some(value);
    }

    /// Gets the value of AvgBytesPerWrite
    pub fn get_avg_bytes_per_write(&self) -> Option<&u64> {
        self.avg_bytes_per_write.as_ref()
    }

    /// Sets the value of AvgBytesPerWrite_Base
    pub fn set_avg_bytes_per_write__base(&mut self, value: u32) {
        self.avg_bytes_per_write__base = Some(value);
    }

    /// Gets the value of AvgBytesPerWrite_Base
    pub fn get_avg_bytes_per_write__base(&self) -> Option<&u32> {
        self.avg_bytes_per_write__base.as_ref()
    }

    /// Sets the value of AvgDataBytesPerRequest
    pub fn set_avg_data_bytes_per_request(&mut self, value: u64) {
        self.avg_data_bytes_per_request = Some(value);
    }

    /// Gets the value of AvgDataBytesPerRequest
    pub fn get_avg_data_bytes_per_request(&self) -> Option<&u64> {
        self.avg_data_bytes_per_request.as_ref()
    }

    /// Sets the value of AvgDataBytesPerRequest_Base
    pub fn set_avg_data_bytes_per_request__base(&mut self, value: u32) {
        self.avg_data_bytes_per_request__base = Some(value);
    }

    /// Gets the value of AvgDataBytesPerRequest_Base
    pub fn get_avg_data_bytes_per_request__base(&self) -> Option<&u32> {
        self.avg_data_bytes_per_request__base.as_ref()
    }

    /// Sets the value of AvgDataQueueLength
    pub fn set_avg_data_queue_length(&mut self, value: u64) {
        self.avg_data_queue_length = Some(value);
    }

    /// Gets the value of AvgDataQueueLength
    pub fn get_avg_data_queue_length(&self) -> Option<&u64> {
        self.avg_data_queue_length.as_ref()
    }

    /// Sets the value of AvgReadQueueLength
    pub fn set_avg_read_queue_length(&mut self, value: u64) {
        self.avg_read_queue_length = Some(value);
    }

    /// Gets the value of AvgReadQueueLength
    pub fn get_avg_read_queue_length(&self) -> Option<&u64> {
        self.avg_read_queue_length.as_ref()
    }

    /// Sets the value of AvgsecPerDataRequest
    pub fn set_avgsec_per_data_request(&mut self, value: u32) {
        self.avgsec_per_data_request = Some(value);
    }

    /// Gets the value of AvgsecPerDataRequest
    pub fn get_avgsec_per_data_request(&self) -> Option<&u32> {
        self.avgsec_per_data_request.as_ref()
    }

    /// Sets the value of AvgsecPerDataRequest_Base
    pub fn set_avgsec_per_data_request__base(&mut self, value: u32) {
        self.avgsec_per_data_request__base = Some(value);
    }

    /// Gets the value of AvgsecPerDataRequest_Base
    pub fn get_avgsec_per_data_request__base(&self) -> Option<&u32> {
        self.avgsec_per_data_request__base.as_ref()
    }

    /// Sets the value of AvgsecPerRead
    pub fn set_avgsec_per_read(&mut self, value: u32) {
        self.avgsec_per_read = Some(value);
    }

    /// Gets the value of AvgsecPerRead
    pub fn get_avgsec_per_read(&self) -> Option<&u32> {
        self.avgsec_per_read.as_ref()
    }

    /// Sets the value of AvgsecPerRead_Base
    pub fn set_avgsec_per_read__base(&mut self, value: u32) {
        self.avgsec_per_read__base = Some(value);
    }

    /// Gets the value of AvgsecPerRead_Base
    pub fn get_avgsec_per_read__base(&self) -> Option<&u32> {
        self.avgsec_per_read__base.as_ref()
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

    /// Sets the value of AvgsecPerWrite
    pub fn set_avgsec_per_write(&mut self, value: u32) {
        self.avgsec_per_write = Some(value);
    }

    /// Gets the value of AvgsecPerWrite
    pub fn get_avgsec_per_write(&self) -> Option<&u32> {
        self.avgsec_per_write.as_ref()
    }

    /// Sets the value of AvgsecPerWrite_Base
    pub fn set_avgsec_per_write__base(&mut self, value: u32) {
        self.avgsec_per_write__base = Some(value);
    }

    /// Gets the value of AvgsecPerWrite_Base
    pub fn get_avgsec_per_write__base(&self) -> Option<&u32> {
        self.avgsec_per_write__base.as_ref()
    }

    /// Sets the value of AvgWriteQueueLength
    pub fn set_avg_write_queue_length(&mut self, value: u64) {
        self.avg_write_queue_length = Some(value);
    }

    /// Gets the value of AvgWriteQueueLength
    pub fn get_avg_write_queue_length(&self) -> Option<&u64> {
        self.avg_write_queue_length.as_ref()
    }

    /// Sets the value of BytesCompressedPersec
    pub fn set_bytes_compressed_persec(&mut self, value: u64) {
        self.bytes_compressed_persec = Some(value);
    }

    /// Gets the value of BytesCompressedPersec
    pub fn get_bytes_compressed_persec(&self) -> Option<&u64> {
        self.bytes_compressed_persec.as_ref()
    }

    /// Sets the value of CompressedRequestsPersec
    pub fn set_compressed_requests_persec(&mut self, value: u64) {
        self.compressed_requests_persec = Some(value);
    }

    /// Gets the value of CompressedRequestsPersec
    pub fn get_compressed_requests_persec(&self) -> Option<&u64> {
        self.compressed_requests_persec.as_ref()
    }

    /// Sets the value of CurrentBypassOpenFileCount
    pub fn set_current_bypass_open_file_count(&mut self, value: u64) {
        self.current_bypass_open_file_count = Some(value);
    }

    /// Gets the value of CurrentBypassOpenFileCount
    pub fn get_current_bypass_open_file_count(&self) -> Option<&u64> {
        self.current_bypass_open_file_count.as_ref()
    }

    /// Sets the value of CurrentDataQueueLength
    pub fn set_current_data_queue_length(&mut self, value: u64) {
        self.current_data_queue_length = Some(value);
    }

    /// Gets the value of CurrentDataQueueLength
    pub fn get_current_data_queue_length(&self) -> Option<&u64> {
        self.current_data_queue_length.as_ref()
    }

    /// Sets the value of CurrentDurableOpenFileCount
    pub fn set_current_durable_open_file_count(&mut self, value: u64) {
        self.current_durable_open_file_count = Some(value);
    }

    /// Gets the value of CurrentDurableOpenFileCount
    pub fn get_current_durable_open_file_count(&self) -> Option<&u64> {
        self.current_durable_open_file_count.as_ref()
    }

    /// Sets the value of CurrentOpenFileCount
    pub fn set_current_open_file_count(&mut self, value: u64) {
        self.current_open_file_count = Some(value);
    }

    /// Gets the value of CurrentOpenFileCount
    pub fn get_current_open_file_count(&self) -> Option<&u64> {
        self.current_open_file_count.as_ref()
    }

    /// Sets the value of CurrentPendingRequests
    pub fn set_current_pending_requests(&mut self, value: u64) {
        self.current_pending_requests = Some(value);
    }

    /// Gets the value of CurrentPendingRequests
    pub fn get_current_pending_requests(&self) -> Option<&u64> {
        self.current_pending_requests.as_ref()
    }

    /// Sets the value of DataBytesPersec
    pub fn set_data_bytes_persec(&mut self, value: u64) {
        self.data_bytes_persec = Some(value);
    }

    /// Gets the value of DataBytesPersec
    pub fn get_data_bytes_persec(&self) -> Option<&u64> {
        self.data_bytes_persec.as_ref()
    }

    /// Sets the value of DataRequestsPersec
    pub fn set_data_requests_persec(&mut self, value: u32) {
        self.data_requests_persec = Some(value);
    }

    /// Gets the value of DataRequestsPersec
    pub fn get_data_requests_persec(&self) -> Option<&u32> {
        self.data_requests_persec.as_ref()
    }

    /// Sets the value of FilesOpenedPersec
    pub fn set_files_opened_persec(&mut self, value: u64) {
        self.files_opened_persec = Some(value);
    }

    /// Gets the value of FilesOpenedPersec
    pub fn get_files_opened_persec(&self) -> Option<&u64> {
        self.files_opened_persec.as_ref()
    }

    /// Sets the value of MetadataRequestsPersec
    pub fn set_metadata_requests_persec(&mut self, value: u64) {
        self.metadata_requests_persec = Some(value);
    }

    /// Gets the value of MetadataRequestsPersec
    pub fn get_metadata_requests_persec(&self) -> Option<&u64> {
        self.metadata_requests_persec.as_ref()
    }

    /// Sets the value of PercentPersistentHandles
    pub fn set_percent_persistent_handles(&mut self, value: u64) {
        self.percent_persistent_handles = Some(value);
    }

    /// Gets the value of PercentPersistentHandles
    pub fn get_percent_persistent_handles(&self) -> Option<&u64> {
        self.percent_persistent_handles.as_ref()
    }

    /// Sets the value of PercentPersistentHandles_Base
    pub fn set_percent_persistent_handles__base(&mut self, value: u64) {
        self.percent_persistent_handles__base = Some(value);
    }

    /// Gets the value of PercentPersistentHandles_Base
    pub fn get_percent_persistent_handles__base(&self) -> Option<&u64> {
        self.percent_persistent_handles__base.as_ref()
    }

    /// Sets the value of PercentResilientHandles
    pub fn set_percent_resilient_handles(&mut self, value: u64) {
        self.percent_resilient_handles = Some(value);
    }

    /// Gets the value of PercentResilientHandles
    pub fn get_percent_resilient_handles(&self) -> Option<&u64> {
        self.percent_resilient_handles.as_ref()
    }

    /// Sets the value of PercentResilientHandles_Base
    pub fn set_percent_resilient_handles__base(&mut self, value: u64) {
        self.percent_resilient_handles__base = Some(value);
    }

    /// Gets the value of PercentResilientHandles_Base
    pub fn get_percent_resilient_handles__base(&self) -> Option<&u64> {
        self.percent_resilient_handles__base.as_ref()
    }

    /// Sets the value of ReadBytesPersec
    pub fn set_read_bytes_persec(&mut self, value: u64) {
        self.read_bytes_persec = Some(value);
    }

    /// Gets the value of ReadBytesPersec
    pub fn get_read_bytes_persec(&self) -> Option<&u64> {
        self.read_bytes_persec.as_ref()
    }

    /// Sets the value of ReadBytestransmittedByPassCSVPersec
    pub fn set_read_bytestransmitted_by_pass_csvpersec(&mut self, value: u64) {
        self.read_bytestransmitted_by_pass_csvpersec = Some(value);
    }

    /// Gets the value of ReadBytestransmittedByPassCSVPersec
    pub fn get_read_bytestransmitted_by_pass_csvpersec(&self) -> Option<&u64> {
        self.read_bytestransmitted_by_pass_csvpersec.as_ref()
    }

    /// Sets the value of ReadBytestransmittedviaSMBDirectPersec
    pub fn set_read_bytestransmittedvia_smbdirect_persec(&mut self, value: u64) {
        self.read_bytestransmittedvia_smbdirect_persec = Some(value);
    }

    /// Gets the value of ReadBytestransmittedviaSMBDirectPersec
    pub fn get_read_bytestransmittedvia_smbdirect_persec(&self) -> Option<&u64> {
        self.read_bytestransmittedvia_smbdirect_persec.as_ref()
    }

    /// Sets the value of ReadRequestsPersec
    pub fn set_read_requests_persec(&mut self, value: u32) {
        self.read_requests_persec = Some(value);
    }

    /// Gets the value of ReadRequestsPersec
    pub fn get_read_requests_persec(&self) -> Option<&u32> {
        self.read_requests_persec.as_ref()
    }

    /// Sets the value of ReadRequeststransmittedviaBypassCSVPersec
    pub fn set_read_requeststransmittedvia_bypass_csvpersec(&mut self, value: u32) {
        self.read_requeststransmittedvia_bypass_csvpersec = Some(value);
    }

    /// Gets the value of ReadRequeststransmittedviaBypassCSVPersec
    pub fn get_read_requeststransmittedvia_bypass_csvpersec(&self) -> Option<&u32> {
        self.read_requeststransmittedvia_bypass_csvpersec.as_ref()
    }

    /// Sets the value of ReadRequeststransmittedviaSMBDirectPersec
    pub fn set_read_requeststransmittedvia_smbdirect_persec(&mut self, value: u32) {
        self.read_requeststransmittedvia_smbdirect_persec = Some(value);
    }

    /// Gets the value of ReadRequeststransmittedviaSMBDirectPersec
    pub fn get_read_requeststransmittedvia_smbdirect_persec(&self) -> Option<&u32> {
        self.read_requeststransmittedvia_smbdirect_persec.as_ref()
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
    pub fn set_requests_persec(&mut self, value: u64) {
        self.requests_persec = Some(value);
    }

    /// Gets the value of RequestsPersec
    pub fn get_requests_persec(&self) -> Option<&u64> {
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

    /// Sets the value of SuccessfulCompressedResponsesPersec
    pub fn set_successful_compressed_responses_persec(&mut self, value: u64) {
        self.successful_compressed_responses_persec = Some(value);
    }

    /// Gets the value of SuccessfulCompressedResponsesPersec
    pub fn get_successful_compressed_responses_persec(&self) -> Option<&u64> {
        self.successful_compressed_responses_persec.as_ref()
    }

    /// Sets the value of TotalDurableHandleReopenCount
    pub fn set_total_durable_handle_reopen_count(&mut self, value: u64) {
        self.total_durable_handle_reopen_count = Some(value);
    }

    /// Gets the value of TotalDurableHandleReopenCount
    pub fn get_total_durable_handle_reopen_count(&self) -> Option<&u64> {
        self.total_durable_handle_reopen_count.as_ref()
    }

    /// Sets the value of TotalFailedDurableHandleReopenCount
    pub fn set_total_failed_durable_handle_reopen_count(&mut self, value: u64) {
        self.total_failed_durable_handle_reopen_count = Some(value);
    }

    /// Gets the value of TotalFailedDurableHandleReopenCount
    pub fn get_total_failed_durable_handle_reopen_count(&self) -> Option<&u64> {
        self.total_failed_durable_handle_reopen_count.as_ref()
    }

    /// Sets the value of TotalFailedPersistentHandleReopenCount
    pub fn set_total_failed_persistent_handle_reopen_count(&mut self, value: u64) {
        self.total_failed_persistent_handle_reopen_count = Some(value);
    }

    /// Gets the value of TotalFailedPersistentHandleReopenCount
    pub fn get_total_failed_persistent_handle_reopen_count(&self) -> Option<&u64> {
        self.total_failed_persistent_handle_reopen_count.as_ref()
    }

    /// Sets the value of TotalFailedResilientHandleReopenCount
    pub fn set_total_failed_resilient_handle_reopen_count(&mut self, value: u64) {
        self.total_failed_resilient_handle_reopen_count = Some(value);
    }

    /// Gets the value of TotalFailedResilientHandleReopenCount
    pub fn get_total_failed_resilient_handle_reopen_count(&self) -> Option<&u64> {
        self.total_failed_resilient_handle_reopen_count.as_ref()
    }

    /// Sets the value of TotalFileOpenCount
    pub fn set_total_file_open_count(&mut self, value: u64) {
        self.total_file_open_count = Some(value);
    }

    /// Gets the value of TotalFileOpenCount
    pub fn get_total_file_open_count(&self) -> Option<&u64> {
        self.total_file_open_count.as_ref()
    }

    /// Sets the value of TotalPersistentHandleReopenCount
    pub fn set_total_persistent_handle_reopen_count(&mut self, value: u64) {
        self.total_persistent_handle_reopen_count = Some(value);
    }

    /// Gets the value of TotalPersistentHandleReopenCount
    pub fn get_total_persistent_handle_reopen_count(&self) -> Option<&u64> {
        self.total_persistent_handle_reopen_count.as_ref()
    }

    /// Sets the value of TotalResilientHandleReopenCount
    pub fn set_total_resilient_handle_reopen_count(&mut self, value: u64) {
        self.total_resilient_handle_reopen_count = Some(value);
    }

    /// Gets the value of TotalResilientHandleReopenCount
    pub fn get_total_resilient_handle_reopen_count(&self) -> Option<&u64> {
        self.total_resilient_handle_reopen_count.as_ref()
    }

    /// Sets the value of TransferredBytesPersec
    pub fn set_transferred_bytes_persec(&mut self, value: u64) {
        self.transferred_bytes_persec = Some(value);
    }

    /// Gets the value of TransferredBytesPersec
    pub fn get_transferred_bytes_persec(&self) -> Option<&u64> {
        self.transferred_bytes_persec.as_ref()
    }

    /// Sets the value of TreeConnectCount
    pub fn set_tree_connect_count(&mut self, value: u64) {
        self.tree_connect_count = Some(value);
    }

    /// Gets the value of TreeConnectCount
    pub fn get_tree_connect_count(&self) -> Option<&u64> {
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

    /// Sets the value of WriteBytestransmittedByPassCSVPersec
    pub fn set_write_bytestransmitted_by_pass_csvpersec(&mut self, value: u64) {
        self.write_bytestransmitted_by_pass_csvpersec = Some(value);
    }

    /// Gets the value of WriteBytestransmittedByPassCSVPersec
    pub fn get_write_bytestransmitted_by_pass_csvpersec(&self) -> Option<&u64> {
        self.write_bytestransmitted_by_pass_csvpersec.as_ref()
    }

    /// Sets the value of WriteBytestransmittedviaSMBDirectPersec
    pub fn set_write_bytestransmittedvia_smbdirect_persec(&mut self, value: u64) {
        self.write_bytestransmittedvia_smbdirect_persec = Some(value);
    }

    /// Gets the value of WriteBytestransmittedviaSMBDirectPersec
    pub fn get_write_bytestransmittedvia_smbdirect_persec(&self) -> Option<&u64> {
        self.write_bytestransmittedvia_smbdirect_persec.as_ref()
    }

    /// Sets the value of WriteRequestsPersec
    pub fn set_write_requests_persec(&mut self, value: u32) {
        self.write_requests_persec = Some(value);
    }

    /// Gets the value of WriteRequestsPersec
    pub fn get_write_requests_persec(&self) -> Option<&u32> {
        self.write_requests_persec.as_ref()
    }

    /// Sets the value of WriteRequeststransmittedviaBypassCSVPersec
    pub fn set_write_requeststransmittedvia_bypass_csvpersec(&mut self, value: u32) {
        self.write_requeststransmittedvia_bypass_csvpersec = Some(value);
    }

    /// Gets the value of WriteRequeststransmittedviaBypassCSVPersec
    pub fn get_write_requeststransmittedvia_bypass_csvpersec(&self) -> Option<&u32> {
        self.write_requeststransmittedvia_bypass_csvpersec.as_ref()
    }

    /// Sets the value of WriteRequeststransmittedviaSMBDirectPersec
    pub fn set_write_requeststransmittedvia_smbdirect_persec(&mut self, value: u32) {
        self.write_requeststransmittedvia_smbdirect_persec = Some(value);
    }

    /// Gets the value of WriteRequeststransmittedviaSMBDirectPersec
    pub fn get_write_requeststransmittedvia_smbdirect_persec(&self) -> Option<&u32> {
        self.write_requeststransmittedvia_smbdirect_persec.as_ref()
    }
}

