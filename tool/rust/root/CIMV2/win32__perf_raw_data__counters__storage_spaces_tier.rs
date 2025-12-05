// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_StorageSpacesTier struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_StorageSpacesTier {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "TierReadBytesAverage")]
    pub tier_read_bytes_average: Option<u64>,

/// 
    #[serde(rename = "TierReadBytesAverage_Base")]
    pub tier_read_bytes_average__base: Option<u32>,

/// 
    #[serde(rename = "TierReadBytesPersec")]
    pub tier_read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "TierReadLatency")]
    pub tier_read_latency: Option<u32>,

/// 
    #[serde(rename = "TierReadLatency_Base")]
    pub tier_read_latency__base: Option<u32>,

/// 
    #[serde(rename = "TierReadsAverage")]
    pub tier_reads_average: Option<u64>,

/// 
    #[serde(rename = "TierReadsPersec")]
    pub tier_reads_persec: Option<u64>,

/// 
    #[serde(rename = "TierTransferBytesAverage")]
    pub tier_transfer_bytes_average: Option<u64>,

/// 
    #[serde(rename = "TierTransferBytesAverage_Base")]
    pub tier_transfer_bytes_average__base: Option<u32>,

/// 
    #[serde(rename = "TierTransferBytesPersec")]
    pub tier_transfer_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "TierTransferLatency")]
    pub tier_transfer_latency: Option<u32>,

/// 
    #[serde(rename = "TierTransferLatency_Base")]
    pub tier_transfer_latency__base: Option<u32>,

/// 
    #[serde(rename = "TierTransfersAverage")]
    pub tier_transfers_average: Option<u64>,

/// 
    #[serde(rename = "TierTransfersCurrent")]
    pub tier_transfers_current: Option<u32>,

/// 
    #[serde(rename = "TierTransfersPersec")]
    pub tier_transfers_persec: Option<u64>,

/// 
    #[serde(rename = "TierWriteBytesAverage")]
    pub tier_write_bytes_average: Option<u64>,

/// 
    #[serde(rename = "TierWriteBytesAverage_Base")]
    pub tier_write_bytes_average__base: Option<u32>,

/// 
    #[serde(rename = "TierWriteBytesPersec")]
    pub tier_write_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "TierWriteLatency")]
    pub tier_write_latency: Option<u32>,

/// 
    #[serde(rename = "TierWriteLatency_Base")]
    pub tier_write_latency__base: Option<u32>,

/// 
    #[serde(rename = "TierWritesAverage")]
    pub tier_writes_average: Option<u64>,

/// 
    #[serde(rename = "TierWritesPersec")]
    pub tier_writes_persec: Option<u64>,
}

impl Win32_PerfRawData_Counters_StorageSpacesTier {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            tier_read_bytes_average: None,
            tier_read_bytes_average__base: None,
            tier_read_bytes_persec: None,
            tier_read_latency: None,
            tier_read_latency__base: None,
            tier_reads_average: None,
            tier_reads_persec: None,
            tier_transfer_bytes_average: None,
            tier_transfer_bytes_average__base: None,
            tier_transfer_bytes_persec: None,
            tier_transfer_latency: None,
            tier_transfer_latency__base: None,
            tier_transfers_average: None,
            tier_transfers_current: None,
            tier_transfers_persec: None,
            tier_write_bytes_average: None,
            tier_write_bytes_average__base: None,
            tier_write_bytes_persec: None,
            tier_write_latency: None,
            tier_write_latency__base: None,
            tier_writes_average: None,
            tier_writes_persec: None,
        }
    }


    /// Sets the value of TierReadBytesAverage
    pub fn set_tier_read_bytes_average(&mut self, value: u64) {
        self.tier_read_bytes_average = Some(value);
    }

    /// Gets the value of TierReadBytesAverage
    pub fn get_tier_read_bytes_average(&self) -> Option<&u64> {
        self.tier_read_bytes_average.as_ref()
    }

    /// Sets the value of TierReadBytesAverage_Base
    pub fn set_tier_read_bytes_average__base(&mut self, value: u32) {
        self.tier_read_bytes_average__base = Some(value);
    }

    /// Gets the value of TierReadBytesAverage_Base
    pub fn get_tier_read_bytes_average__base(&self) -> Option<&u32> {
        self.tier_read_bytes_average__base.as_ref()
    }

    /// Sets the value of TierReadBytesPersec
    pub fn set_tier_read_bytes_persec(&mut self, value: u64) {
        self.tier_read_bytes_persec = Some(value);
    }

    /// Gets the value of TierReadBytesPersec
    pub fn get_tier_read_bytes_persec(&self) -> Option<&u64> {
        self.tier_read_bytes_persec.as_ref()
    }

    /// Sets the value of TierReadLatency
    pub fn set_tier_read_latency(&mut self, value: u32) {
        self.tier_read_latency = Some(value);
    }

    /// Gets the value of TierReadLatency
    pub fn get_tier_read_latency(&self) -> Option<&u32> {
        self.tier_read_latency.as_ref()
    }

    /// Sets the value of TierReadLatency_Base
    pub fn set_tier_read_latency__base(&mut self, value: u32) {
        self.tier_read_latency__base = Some(value);
    }

    /// Gets the value of TierReadLatency_Base
    pub fn get_tier_read_latency__base(&self) -> Option<&u32> {
        self.tier_read_latency__base.as_ref()
    }

    /// Sets the value of TierReadsAverage
    pub fn set_tier_reads_average(&mut self, value: u64) {
        self.tier_reads_average = Some(value);
    }

    /// Gets the value of TierReadsAverage
    pub fn get_tier_reads_average(&self) -> Option<&u64> {
        self.tier_reads_average.as_ref()
    }

    /// Sets the value of TierReadsPersec
    pub fn set_tier_reads_persec(&mut self, value: u64) {
        self.tier_reads_persec = Some(value);
    }

    /// Gets the value of TierReadsPersec
    pub fn get_tier_reads_persec(&self) -> Option<&u64> {
        self.tier_reads_persec.as_ref()
    }

    /// Sets the value of TierTransferBytesAverage
    pub fn set_tier_transfer_bytes_average(&mut self, value: u64) {
        self.tier_transfer_bytes_average = Some(value);
    }

    /// Gets the value of TierTransferBytesAverage
    pub fn get_tier_transfer_bytes_average(&self) -> Option<&u64> {
        self.tier_transfer_bytes_average.as_ref()
    }

    /// Sets the value of TierTransferBytesAverage_Base
    pub fn set_tier_transfer_bytes_average__base(&mut self, value: u32) {
        self.tier_transfer_bytes_average__base = Some(value);
    }

    /// Gets the value of TierTransferBytesAverage_Base
    pub fn get_tier_transfer_bytes_average__base(&self) -> Option<&u32> {
        self.tier_transfer_bytes_average__base.as_ref()
    }

    /// Sets the value of TierTransferBytesPersec
    pub fn set_tier_transfer_bytes_persec(&mut self, value: u64) {
        self.tier_transfer_bytes_persec = Some(value);
    }

    /// Gets the value of TierTransferBytesPersec
    pub fn get_tier_transfer_bytes_persec(&self) -> Option<&u64> {
        self.tier_transfer_bytes_persec.as_ref()
    }

    /// Sets the value of TierTransferLatency
    pub fn set_tier_transfer_latency(&mut self, value: u32) {
        self.tier_transfer_latency = Some(value);
    }

    /// Gets the value of TierTransferLatency
    pub fn get_tier_transfer_latency(&self) -> Option<&u32> {
        self.tier_transfer_latency.as_ref()
    }

    /// Sets the value of TierTransferLatency_Base
    pub fn set_tier_transfer_latency__base(&mut self, value: u32) {
        self.tier_transfer_latency__base = Some(value);
    }

    /// Gets the value of TierTransferLatency_Base
    pub fn get_tier_transfer_latency__base(&self) -> Option<&u32> {
        self.tier_transfer_latency__base.as_ref()
    }

    /// Sets the value of TierTransfersAverage
    pub fn set_tier_transfers_average(&mut self, value: u64) {
        self.tier_transfers_average = Some(value);
    }

    /// Gets the value of TierTransfersAverage
    pub fn get_tier_transfers_average(&self) -> Option<&u64> {
        self.tier_transfers_average.as_ref()
    }

    /// Sets the value of TierTransfersCurrent
    pub fn set_tier_transfers_current(&mut self, value: u32) {
        self.tier_transfers_current = Some(value);
    }

    /// Gets the value of TierTransfersCurrent
    pub fn get_tier_transfers_current(&self) -> Option<&u32> {
        self.tier_transfers_current.as_ref()
    }

    /// Sets the value of TierTransfersPersec
    pub fn set_tier_transfers_persec(&mut self, value: u64) {
        self.tier_transfers_persec = Some(value);
    }

    /// Gets the value of TierTransfersPersec
    pub fn get_tier_transfers_persec(&self) -> Option<&u64> {
        self.tier_transfers_persec.as_ref()
    }

    /// Sets the value of TierWriteBytesAverage
    pub fn set_tier_write_bytes_average(&mut self, value: u64) {
        self.tier_write_bytes_average = Some(value);
    }

    /// Gets the value of TierWriteBytesAverage
    pub fn get_tier_write_bytes_average(&self) -> Option<&u64> {
        self.tier_write_bytes_average.as_ref()
    }

    /// Sets the value of TierWriteBytesAverage_Base
    pub fn set_tier_write_bytes_average__base(&mut self, value: u32) {
        self.tier_write_bytes_average__base = Some(value);
    }

    /// Gets the value of TierWriteBytesAverage_Base
    pub fn get_tier_write_bytes_average__base(&self) -> Option<&u32> {
        self.tier_write_bytes_average__base.as_ref()
    }

    /// Sets the value of TierWriteBytesPersec
    pub fn set_tier_write_bytes_persec(&mut self, value: u64) {
        self.tier_write_bytes_persec = Some(value);
    }

    /// Gets the value of TierWriteBytesPersec
    pub fn get_tier_write_bytes_persec(&self) -> Option<&u64> {
        self.tier_write_bytes_persec.as_ref()
    }

    /// Sets the value of TierWriteLatency
    pub fn set_tier_write_latency(&mut self, value: u32) {
        self.tier_write_latency = Some(value);
    }

    /// Gets the value of TierWriteLatency
    pub fn get_tier_write_latency(&self) -> Option<&u32> {
        self.tier_write_latency.as_ref()
    }

    /// Sets the value of TierWriteLatency_Base
    pub fn set_tier_write_latency__base(&mut self, value: u32) {
        self.tier_write_latency__base = Some(value);
    }

    /// Gets the value of TierWriteLatency_Base
    pub fn get_tier_write_latency__base(&self) -> Option<&u32> {
        self.tier_write_latency__base.as_ref()
    }

    /// Sets the value of TierWritesAverage
    pub fn set_tier_writes_average(&mut self, value: u64) {
        self.tier_writes_average = Some(value);
    }

    /// Gets the value of TierWritesAverage
    pub fn get_tier_writes_average(&self) -> Option<&u64> {
        self.tier_writes_average.as_ref()
    }

    /// Sets the value of TierWritesPersec
    pub fn set_tier_writes_persec(&mut self, value: u64) {
        self.tier_writes_persec = Some(value);
    }

    /// Gets the value of TierWritesPersec
    pub fn get_tier_writes_persec(&self) -> Option<&u64> {
        self.tier_writes_persec.as_ref()
    }
}

