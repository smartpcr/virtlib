// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_SMBClientShares struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_SMBClientShares {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "AttemptedCompressedRequestsPersec")]
    pub attempted_compressed_requests_persec: Option<u32>,

/// 
    #[serde(rename = "AvgBytesPerRead")]
    pub avg_bytes_per_read: Option<u64>,

/// 
    #[serde(rename = "AvgBytesPerWrite")]
    pub avg_bytes_per_write: Option<u64>,

/// 
    #[serde(rename = "AvgDataBytesPerRequest")]
    pub avg_data_bytes_per_request: Option<u64>,

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
    #[serde(rename = "AvgsecPerRead")]
    pub avgsec_per_read: Option<u32>,

/// 
    #[serde(rename = "AvgsecPerWrite")]
    pub avgsec_per_write: Option<u32>,

/// 
    #[serde(rename = "AvgWriteQueueLength")]
    pub avg_write_queue_length: Option<u64>,

/// 
    #[serde(rename = "CompressedBytesSentPersec")]
    pub compressed_bytes_sent_persec: Option<u32>,

/// 
    #[serde(rename = "CompressedResponsesPersec")]
    pub compressed_responses_persec: Option<u32>,

/// 
    #[serde(rename = "CreditStallsPersec")]
    pub credit_stalls_persec: Option<u32>,

/// 
    #[serde(rename = "CurrentDataQueueLength")]
    pub current_data_queue_length: Option<u32>,

/// 
    #[serde(rename = "DataBytesPersec")]
    pub data_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "DataRequestsPersec")]
    pub data_requests_persec: Option<u32>,

/// 
    #[serde(rename = "MetadataRequestsPersec")]
    pub metadata_requests_persec: Option<u32>,

/// 
    #[serde(rename = "ReadBytesPersec")]
    pub read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "ReadBytestransmittedviaSMBDirectPersec")]
    pub read_bytestransmittedvia_smbdirect_persec: Option<u64>,

/// 
    #[serde(rename = "ReadRequestsPersec")]
    pub read_requests_persec: Option<u32>,

/// 
    #[serde(rename = "ReadRequeststransmittedviaSMBDirectPersec")]
    pub read_requeststransmittedvia_smbdirect_persec: Option<u32>,

/// 
    #[serde(rename = "SuccessfulCompressedRequestsPersec")]
    pub successful_compressed_requests_persec: Option<u32>,

/// 
    #[serde(rename = "TurboIOReadsPersec")]
    pub turbo_ioreads_persec: Option<u32>,

/// 
    #[serde(rename = "TurboIOWritesPersec")]
    pub turbo_iowrites_persec: Option<u32>,

/// 
    #[serde(rename = "WriteBytesPersec")]
    pub write_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "WriteBytestransmittedviaSMBDirectPersec")]
    pub write_bytestransmittedvia_smbdirect_persec: Option<u64>,

/// 
    #[serde(rename = "WriteRequestsPersec")]
    pub write_requests_persec: Option<u32>,

/// 
    #[serde(rename = "WriteRequeststransmittedviaSMBDirectPersec")]
    pub write_requeststransmittedvia_smbdirect_persec: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_SMBClientShares {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            attempted_compressed_requests_persec: None,
            avg_bytes_per_read: None,
            avg_bytes_per_write: None,
            avg_data_bytes_per_request: None,
            avg_data_queue_length: None,
            avg_read_queue_length: None,
            avgsec_per_data_request: None,
            avgsec_per_read: None,
            avgsec_per_write: None,
            avg_write_queue_length: None,
            compressed_bytes_sent_persec: None,
            compressed_responses_persec: None,
            credit_stalls_persec: None,
            current_data_queue_length: None,
            data_bytes_persec: None,
            data_requests_persec: None,
            metadata_requests_persec: None,
            read_bytes_persec: None,
            read_bytestransmittedvia_smbdirect_persec: None,
            read_requests_persec: None,
            read_requeststransmittedvia_smbdirect_persec: None,
            successful_compressed_requests_persec: None,
            turbo_ioreads_persec: None,
            turbo_iowrites_persec: None,
            write_bytes_persec: None,
            write_bytestransmittedvia_smbdirect_persec: None,
            write_requests_persec: None,
            write_requeststransmittedvia_smbdirect_persec: None,
        }
    }


    /// Sets the value of AttemptedCompressedRequestsPersec
    pub fn set_attempted_compressed_requests_persec(&mut self, value: u32) {
        self.attempted_compressed_requests_persec = Some(value);
    }

    /// Gets the value of AttemptedCompressedRequestsPersec
    pub fn get_attempted_compressed_requests_persec(&self) -> Option<&u32> {
        self.attempted_compressed_requests_persec.as_ref()
    }

    /// Sets the value of AvgBytesPerRead
    pub fn set_avg_bytes_per_read(&mut self, value: u64) {
        self.avg_bytes_per_read = Some(value);
    }

    /// Gets the value of AvgBytesPerRead
    pub fn get_avg_bytes_per_read(&self) -> Option<&u64> {
        self.avg_bytes_per_read.as_ref()
    }

    /// Sets the value of AvgBytesPerWrite
    pub fn set_avg_bytes_per_write(&mut self, value: u64) {
        self.avg_bytes_per_write = Some(value);
    }

    /// Gets the value of AvgBytesPerWrite
    pub fn get_avg_bytes_per_write(&self) -> Option<&u64> {
        self.avg_bytes_per_write.as_ref()
    }

    /// Sets the value of AvgDataBytesPerRequest
    pub fn set_avg_data_bytes_per_request(&mut self, value: u64) {
        self.avg_data_bytes_per_request = Some(value);
    }

    /// Gets the value of AvgDataBytesPerRequest
    pub fn get_avg_data_bytes_per_request(&self) -> Option<&u64> {
        self.avg_data_bytes_per_request.as_ref()
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

    /// Sets the value of AvgsecPerRead
    pub fn set_avgsec_per_read(&mut self, value: u32) {
        self.avgsec_per_read = Some(value);
    }

    /// Gets the value of AvgsecPerRead
    pub fn get_avgsec_per_read(&self) -> Option<&u32> {
        self.avgsec_per_read.as_ref()
    }

    /// Sets the value of AvgsecPerWrite
    pub fn set_avgsec_per_write(&mut self, value: u32) {
        self.avgsec_per_write = Some(value);
    }

    /// Gets the value of AvgsecPerWrite
    pub fn get_avgsec_per_write(&self) -> Option<&u32> {
        self.avgsec_per_write.as_ref()
    }

    /// Sets the value of AvgWriteQueueLength
    pub fn set_avg_write_queue_length(&mut self, value: u64) {
        self.avg_write_queue_length = Some(value);
    }

    /// Gets the value of AvgWriteQueueLength
    pub fn get_avg_write_queue_length(&self) -> Option<&u64> {
        self.avg_write_queue_length.as_ref()
    }

    /// Sets the value of CompressedBytesSentPersec
    pub fn set_compressed_bytes_sent_persec(&mut self, value: u32) {
        self.compressed_bytes_sent_persec = Some(value);
    }

    /// Gets the value of CompressedBytesSentPersec
    pub fn get_compressed_bytes_sent_persec(&self) -> Option<&u32> {
        self.compressed_bytes_sent_persec.as_ref()
    }

    /// Sets the value of CompressedResponsesPersec
    pub fn set_compressed_responses_persec(&mut self, value: u32) {
        self.compressed_responses_persec = Some(value);
    }

    /// Gets the value of CompressedResponsesPersec
    pub fn get_compressed_responses_persec(&self) -> Option<&u32> {
        self.compressed_responses_persec.as_ref()
    }

    /// Sets the value of CreditStallsPersec
    pub fn set_credit_stalls_persec(&mut self, value: u32) {
        self.credit_stalls_persec = Some(value);
    }

    /// Gets the value of CreditStallsPersec
    pub fn get_credit_stalls_persec(&self) -> Option<&u32> {
        self.credit_stalls_persec.as_ref()
    }

    /// Sets the value of CurrentDataQueueLength
    pub fn set_current_data_queue_length(&mut self, value: u32) {
        self.current_data_queue_length = Some(value);
    }

    /// Gets the value of CurrentDataQueueLength
    pub fn get_current_data_queue_length(&self) -> Option<&u32> {
        self.current_data_queue_length.as_ref()
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

    /// Sets the value of MetadataRequestsPersec
    pub fn set_metadata_requests_persec(&mut self, value: u32) {
        self.metadata_requests_persec = Some(value);
    }

    /// Gets the value of MetadataRequestsPersec
    pub fn get_metadata_requests_persec(&self) -> Option<&u32> {
        self.metadata_requests_persec.as_ref()
    }

    /// Sets the value of ReadBytesPersec
    pub fn set_read_bytes_persec(&mut self, value: u64) {
        self.read_bytes_persec = Some(value);
    }

    /// Gets the value of ReadBytesPersec
    pub fn get_read_bytes_persec(&self) -> Option<&u64> {
        self.read_bytes_persec.as_ref()
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

    /// Sets the value of ReadRequeststransmittedviaSMBDirectPersec
    pub fn set_read_requeststransmittedvia_smbdirect_persec(&mut self, value: u32) {
        self.read_requeststransmittedvia_smbdirect_persec = Some(value);
    }

    /// Gets the value of ReadRequeststransmittedviaSMBDirectPersec
    pub fn get_read_requeststransmittedvia_smbdirect_persec(&self) -> Option<&u32> {
        self.read_requeststransmittedvia_smbdirect_persec.as_ref()
    }

    /// Sets the value of SuccessfulCompressedRequestsPersec
    pub fn set_successful_compressed_requests_persec(&mut self, value: u32) {
        self.successful_compressed_requests_persec = Some(value);
    }

    /// Gets the value of SuccessfulCompressedRequestsPersec
    pub fn get_successful_compressed_requests_persec(&self) -> Option<&u32> {
        self.successful_compressed_requests_persec.as_ref()
    }

    /// Sets the value of TurboIOReadsPersec
    pub fn set_turbo_ioreads_persec(&mut self, value: u32) {
        self.turbo_ioreads_persec = Some(value);
    }

    /// Gets the value of TurboIOReadsPersec
    pub fn get_turbo_ioreads_persec(&self) -> Option<&u32> {
        self.turbo_ioreads_persec.as_ref()
    }

    /// Sets the value of TurboIOWritesPersec
    pub fn set_turbo_iowrites_persec(&mut self, value: u32) {
        self.turbo_iowrites_persec = Some(value);
    }

    /// Gets the value of TurboIOWritesPersec
    pub fn get_turbo_iowrites_persec(&self) -> Option<&u32> {
        self.turbo_iowrites_persec.as_ref()
    }

    /// Sets the value of WriteBytesPersec
    pub fn set_write_bytes_persec(&mut self, value: u64) {
        self.write_bytes_persec = Some(value);
    }

    /// Gets the value of WriteBytesPersec
    pub fn get_write_bytes_persec(&self) -> Option<&u64> {
        self.write_bytes_persec.as_ref()
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

    /// Sets the value of WriteRequeststransmittedviaSMBDirectPersec
    pub fn set_write_requeststransmittedvia_smbdirect_persec(&mut self, value: u32) {
        self.write_requeststransmittedvia_smbdirect_persec = Some(value);
    }

    /// Gets the value of WriteRequeststransmittedviaSMBDirectPersec
    pub fn get_write_requeststransmittedvia_smbdirect_persec(&self) -> Option<&u32> {
        self.write_requeststransmittedvia_smbdirect_persec.as_ref()
    }
}

