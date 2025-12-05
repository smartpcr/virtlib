// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_HyperVVirtualStorageDevice struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_HyperVVirtualStorageDevice {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "AdapterOpenChannelCount")]
    pub adapter_open_channel_count: Option<u32>,

/// 
    #[serde(rename = "ByteQuotaReplenishmentRate")]
    pub byte_quota_replenishment_rate: Option<u64>,

/// 
    #[serde(rename = "ErrorCount")]
    pub error_count: Option<u32>,

/// 
    #[serde(rename = "FlushCount")]
    pub flush_count: Option<u32>,

/// 
    #[serde(rename = "IoQuotaReplenishmentRate")]
    pub io_quota_replenishment_rate: Option<u64>,

/// 
    #[serde(rename = "Latency")]
    pub latency: Option<u32>,

/// 
    #[serde(rename = "Latency_Base")]
    pub latency__base: Option<u32>,

/// 
    #[serde(rename = "LowerLatency")]
    pub lower_latency: Option<u32>,

/// 
    #[serde(rename = "LowerLatency_Base")]
    pub lower_latency__base: Option<u32>,

/// 
    #[serde(rename = "LowerQueueLength")]
    pub lower_queue_length: Option<u64>,

/// 
    #[serde(rename = "MaximumAdapterWorkerCount")]
    pub maximum_adapter_worker_count: Option<u32>,

/// 
    #[serde(rename = "MaximumBandwidth")]
    pub maximum_bandwidth: Option<u64>,

/// 
    #[serde(rename = "MaximumIORate")]
    pub maximum_iorate: Option<u64>,

/// 
    #[serde(rename = "MinimumIORate")]
    pub minimum_iorate: Option<u64>,

/// 
    #[serde(rename = "NormalizedThroughput")]
    pub normalized_throughput: Option<u64>,

/// 
    #[serde(rename = "QueueLength")]
    pub queue_length: Option<u64>,

/// 
    #[serde(rename = "ReadBytesPersec")]
    pub read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "ReadCount")]
    pub read_count: Option<u32>,

/// 
    #[serde(rename = "ReadOperationsPerSec")]
    pub read_operations_per_sec: Option<u32>,

/// 
    #[serde(rename = "Throughput")]
    pub throughput: Option<u32>,

/// 
    #[serde(rename = "WriteBytesPersec")]
    pub write_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "WriteCount")]
    pub write_count: Option<u32>,

/// 
    #[serde(rename = "WriteOperationsPerSec")]
    pub write_operations_per_sec: Option<u32>,
}

impl Win32_PerfRawData_Counters_HyperVVirtualStorageDevice {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            adapter_open_channel_count: None,
            byte_quota_replenishment_rate: None,
            error_count: None,
            flush_count: None,
            io_quota_replenishment_rate: None,
            latency: None,
            latency__base: None,
            lower_latency: None,
            lower_latency__base: None,
            lower_queue_length: None,
            maximum_adapter_worker_count: None,
            maximum_bandwidth: None,
            maximum_iorate: None,
            minimum_iorate: None,
            normalized_throughput: None,
            queue_length: None,
            read_bytes_persec: None,
            read_count: None,
            read_operations_per_sec: None,
            throughput: None,
            write_bytes_persec: None,
            write_count: None,
            write_operations_per_sec: None,
        }
    }


    /// Sets the value of AdapterOpenChannelCount
    pub fn set_adapter_open_channel_count(&mut self, value: u32) {
        self.adapter_open_channel_count = Some(value);
    }

    /// Gets the value of AdapterOpenChannelCount
    pub fn get_adapter_open_channel_count(&self) -> Option<&u32> {
        self.adapter_open_channel_count.as_ref()
    }

    /// Sets the value of ByteQuotaReplenishmentRate
    pub fn set_byte_quota_replenishment_rate(&mut self, value: u64) {
        self.byte_quota_replenishment_rate = Some(value);
    }

    /// Gets the value of ByteQuotaReplenishmentRate
    pub fn get_byte_quota_replenishment_rate(&self) -> Option<&u64> {
        self.byte_quota_replenishment_rate.as_ref()
    }

    /// Sets the value of ErrorCount
    pub fn set_error_count(&mut self, value: u32) {
        self.error_count = Some(value);
    }

    /// Gets the value of ErrorCount
    pub fn get_error_count(&self) -> Option<&u32> {
        self.error_count.as_ref()
    }

    /// Sets the value of FlushCount
    pub fn set_flush_count(&mut self, value: u32) {
        self.flush_count = Some(value);
    }

    /// Gets the value of FlushCount
    pub fn get_flush_count(&self) -> Option<&u32> {
        self.flush_count.as_ref()
    }

    /// Sets the value of IoQuotaReplenishmentRate
    pub fn set_io_quota_replenishment_rate(&mut self, value: u64) {
        self.io_quota_replenishment_rate = Some(value);
    }

    /// Gets the value of IoQuotaReplenishmentRate
    pub fn get_io_quota_replenishment_rate(&self) -> Option<&u64> {
        self.io_quota_replenishment_rate.as_ref()
    }

    /// Sets the value of Latency
    pub fn set_latency(&mut self, value: u32) {
        self.latency = Some(value);
    }

    /// Gets the value of Latency
    pub fn get_latency(&self) -> Option<&u32> {
        self.latency.as_ref()
    }

    /// Sets the value of Latency_Base
    pub fn set_latency__base(&mut self, value: u32) {
        self.latency__base = Some(value);
    }

    /// Gets the value of Latency_Base
    pub fn get_latency__base(&self) -> Option<&u32> {
        self.latency__base.as_ref()
    }

    /// Sets the value of LowerLatency
    pub fn set_lower_latency(&mut self, value: u32) {
        self.lower_latency = Some(value);
    }

    /// Gets the value of LowerLatency
    pub fn get_lower_latency(&self) -> Option<&u32> {
        self.lower_latency.as_ref()
    }

    /// Sets the value of LowerLatency_Base
    pub fn set_lower_latency__base(&mut self, value: u32) {
        self.lower_latency__base = Some(value);
    }

    /// Gets the value of LowerLatency_Base
    pub fn get_lower_latency__base(&self) -> Option<&u32> {
        self.lower_latency__base.as_ref()
    }

    /// Sets the value of LowerQueueLength
    pub fn set_lower_queue_length(&mut self, value: u64) {
        self.lower_queue_length = Some(value);
    }

    /// Gets the value of LowerQueueLength
    pub fn get_lower_queue_length(&self) -> Option<&u64> {
        self.lower_queue_length.as_ref()
    }

    /// Sets the value of MaximumAdapterWorkerCount
    pub fn set_maximum_adapter_worker_count(&mut self, value: u32) {
        self.maximum_adapter_worker_count = Some(value);
    }

    /// Gets the value of MaximumAdapterWorkerCount
    pub fn get_maximum_adapter_worker_count(&self) -> Option<&u32> {
        self.maximum_adapter_worker_count.as_ref()
    }

    /// Sets the value of MaximumBandwidth
    pub fn set_maximum_bandwidth(&mut self, value: u64) {
        self.maximum_bandwidth = Some(value);
    }

    /// Gets the value of MaximumBandwidth
    pub fn get_maximum_bandwidth(&self) -> Option<&u64> {
        self.maximum_bandwidth.as_ref()
    }

    /// Sets the value of MaximumIORate
    pub fn set_maximum_iorate(&mut self, value: u64) {
        self.maximum_iorate = Some(value);
    }

    /// Gets the value of MaximumIORate
    pub fn get_maximum_iorate(&self) -> Option<&u64> {
        self.maximum_iorate.as_ref()
    }

    /// Sets the value of MinimumIORate
    pub fn set_minimum_iorate(&mut self, value: u64) {
        self.minimum_iorate = Some(value);
    }

    /// Gets the value of MinimumIORate
    pub fn get_minimum_iorate(&self) -> Option<&u64> {
        self.minimum_iorate.as_ref()
    }

    /// Sets the value of NormalizedThroughput
    pub fn set_normalized_throughput(&mut self, value: u64) {
        self.normalized_throughput = Some(value);
    }

    /// Gets the value of NormalizedThroughput
    pub fn get_normalized_throughput(&self) -> Option<&u64> {
        self.normalized_throughput.as_ref()
    }

    /// Sets the value of QueueLength
    pub fn set_queue_length(&mut self, value: u64) {
        self.queue_length = Some(value);
    }

    /// Gets the value of QueueLength
    pub fn get_queue_length(&self) -> Option<&u64> {
        self.queue_length.as_ref()
    }

    /// Sets the value of ReadBytesPersec
    pub fn set_read_bytes_persec(&mut self, value: u64) {
        self.read_bytes_persec = Some(value);
    }

    /// Gets the value of ReadBytesPersec
    pub fn get_read_bytes_persec(&self) -> Option<&u64> {
        self.read_bytes_persec.as_ref()
    }

    /// Sets the value of ReadCount
    pub fn set_read_count(&mut self, value: u32) {
        self.read_count = Some(value);
    }

    /// Gets the value of ReadCount
    pub fn get_read_count(&self) -> Option<&u32> {
        self.read_count.as_ref()
    }

    /// Sets the value of ReadOperationsPerSec
    pub fn set_read_operations_per_sec(&mut self, value: u32) {
        self.read_operations_per_sec = Some(value);
    }

    /// Gets the value of ReadOperationsPerSec
    pub fn get_read_operations_per_sec(&self) -> Option<&u32> {
        self.read_operations_per_sec.as_ref()
    }

    /// Sets the value of Throughput
    pub fn set_throughput(&mut self, value: u32) {
        self.throughput = Some(value);
    }

    /// Gets the value of Throughput
    pub fn get_throughput(&self) -> Option<&u32> {
        self.throughput.as_ref()
    }

    /// Sets the value of WriteBytesPersec
    pub fn set_write_bytes_persec(&mut self, value: u64) {
        self.write_bytes_persec = Some(value);
    }

    /// Gets the value of WriteBytesPersec
    pub fn get_write_bytes_persec(&self) -> Option<&u64> {
        self.write_bytes_persec.as_ref()
    }

    /// Sets the value of WriteCount
    pub fn set_write_count(&mut self, value: u32) {
        self.write_count = Some(value);
    }

    /// Gets the value of WriteCount
    pub fn get_write_count(&self) -> Option<&u32> {
        self.write_count.as_ref()
    }

    /// Sets the value of WriteOperationsPerSec
    pub fn set_write_operations_per_sec(&mut self, value: u32) {
        self.write_operations_per_sec = Some(value);
    }

    /// Gets the value of WriteOperationsPerSec
    pub fn get_write_operations_per_sec(&self) -> Option<&u32> {
        self.write_operations_per_sec.as_ref()
    }
}

