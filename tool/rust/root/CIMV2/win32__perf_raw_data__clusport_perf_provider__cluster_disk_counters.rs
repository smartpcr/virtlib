// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_ClusportPerfProvider_ClusterDiskCounters struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_ClusportPerfProvider_ClusterDiskCounters {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "ExceededLatencyLimit")]
    pub exceeded_latency_limit: Option<u64>,

/// 
    #[serde(rename = "ExceededLatencyLimitPersec")]
    pub exceeded_latency_limit_persec: Option<u32>,

/// 
    #[serde(rename = "IO10000msPersec")]
    pub io10000ms_persec: Option<u32>,

/// 
    #[serde(rename = "IO1000msPersec")]
    pub io1000ms_persec: Option<u32>,

/// 
    #[serde(rename = "IO100msPersec")]
    pub io100ms_persec: Option<u32>,

/// 
    #[serde(rename = "IO10msPersec")]
    pub io10ms_persec: Option<u32>,

/// 
    #[serde(rename = "IO1msPersec")]
    pub io1ms_persec: Option<u32>,

/// 
    #[serde(rename = "IO5msPersec")]
    pub io5ms_persec: Option<u32>,

/// 
    #[serde(rename = "LocalReadAvgQueueLength")]
    pub local_read_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "LocalReadBytes")]
    pub local_read_bytes: Option<u64>,

/// 
    #[serde(rename = "LocalReadBytesPersec")]
    pub local_read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "LocalReadLatency")]
    pub local_read_latency: Option<u32>,

/// 
    #[serde(rename = "LocalReadLatency_Base")]
    pub local_read_latency__base: Option<u32>,

/// 
    #[serde(rename = "LocalReadPersec")]
    pub local_read_persec: Option<u32>,

/// 
    #[serde(rename = "LocalReadQueueLength")]
    pub local_read_queue_length: Option<u64>,

/// 
    #[serde(rename = "LocalReads")]
    pub local_reads: Option<u64>,

/// 
    #[serde(rename = "LocalWriteAvgQueueLength")]
    pub local_write_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "LocalWriteBytes")]
    pub local_write_bytes: Option<u64>,

/// 
    #[serde(rename = "LocalWriteBytesPersec")]
    pub local_write_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "LocalWriteLatency")]
    pub local_write_latency: Option<u32>,

/// 
    #[serde(rename = "LocalWriteLatency_Base")]
    pub local_write_latency__base: Option<u32>,

/// 
    #[serde(rename = "LocalWriteQueueLength")]
    pub local_write_queue_length: Option<u64>,

/// 
    #[serde(rename = "LocalWrites")]
    pub local_writes: Option<u64>,

/// 
    #[serde(rename = "LocalWritesPersec")]
    pub local_writes_persec: Option<u32>,

/// 
    #[serde(rename = "ReadAvgQueueLength")]
    pub read_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "ReadBytes")]
    pub read_bytes: Option<u64>,

/// 
    #[serde(rename = "ReadBytesPersec")]
    pub read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "ReadLatency")]
    pub read_latency: Option<u32>,

/// 
    #[serde(rename = "ReadLatency_Base")]
    pub read_latency__base: Option<u32>,

/// 
    #[serde(rename = "ReadPersec")]
    pub read_persec: Option<u32>,

/// 
    #[serde(rename = "ReadQueueLength")]
    pub read_queue_length: Option<u64>,

/// 
    #[serde(rename = "Reads")]
    pub reads: Option<u64>,

/// 
    #[serde(rename = "RemoteReadAvgQueueLength")]
    pub remote_read_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "RemoteReadBytes")]
    pub remote_read_bytes: Option<u64>,

/// 
    #[serde(rename = "RemoteReadBytesPersec")]
    pub remote_read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "RemoteReadLatency")]
    pub remote_read_latency: Option<u32>,

/// 
    #[serde(rename = "RemoteReadLatency_Base")]
    pub remote_read_latency__base: Option<u32>,

/// 
    #[serde(rename = "RemoteReadPersec")]
    pub remote_read_persec: Option<u32>,

/// 
    #[serde(rename = "RemoteReadQueueLength")]
    pub remote_read_queue_length: Option<u64>,

/// 
    #[serde(rename = "RemoteReads")]
    pub remote_reads: Option<u64>,

/// 
    #[serde(rename = "RemoteWriteAvgQueueLength")]
    pub remote_write_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "RemoteWriteBytes")]
    pub remote_write_bytes: Option<u64>,

/// 
    #[serde(rename = "RemoteWriteBytesPersec")]
    pub remote_write_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "RemoteWriteLatency")]
    pub remote_write_latency: Option<u32>,

/// 
    #[serde(rename = "RemoteWriteLatency_Base")]
    pub remote_write_latency__base: Option<u32>,

/// 
    #[serde(rename = "RemoteWriteQueueLength")]
    pub remote_write_queue_length: Option<u64>,

/// 
    #[serde(rename = "RemoteWrites")]
    pub remote_writes: Option<u64>,

/// 
    #[serde(rename = "RemoteWritesPersec")]
    pub remote_writes_persec: Option<u32>,

/// 
    #[serde(rename = "WriteAvgQueueLength")]
    pub write_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "WriteBytes")]
    pub write_bytes: Option<u64>,

/// 
    #[serde(rename = "WriteBytesPersec")]
    pub write_bytes_persec: Option<u32>,

/// 
    #[serde(rename = "WriteLatency")]
    pub write_latency: Option<u32>,

/// 
    #[serde(rename = "WriteLatency_Base")]
    pub write_latency__base: Option<u32>,

/// 
    #[serde(rename = "WriteQueueLength")]
    pub write_queue_length: Option<u64>,

/// 
    #[serde(rename = "Writes")]
    pub writes: Option<u64>,

/// 
    #[serde(rename = "WritesPersec")]
    pub writes_persec: Option<u32>,
}

impl Win32_PerfRawData_ClusportPerfProvider_ClusterDiskCounters {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            exceeded_latency_limit: None,
            exceeded_latency_limit_persec: None,
            io10000ms_persec: None,
            io1000ms_persec: None,
            io100ms_persec: None,
            io10ms_persec: None,
            io1ms_persec: None,
            io5ms_persec: None,
            local_read_avg_queue_length: None,
            local_read_bytes: None,
            local_read_bytes_persec: None,
            local_read_latency: None,
            local_read_latency__base: None,
            local_read_persec: None,
            local_read_queue_length: None,
            local_reads: None,
            local_write_avg_queue_length: None,
            local_write_bytes: None,
            local_write_bytes_persec: None,
            local_write_latency: None,
            local_write_latency__base: None,
            local_write_queue_length: None,
            local_writes: None,
            local_writes_persec: None,
            read_avg_queue_length: None,
            read_bytes: None,
            read_bytes_persec: None,
            read_latency: None,
            read_latency__base: None,
            read_persec: None,
            read_queue_length: None,
            reads: None,
            remote_read_avg_queue_length: None,
            remote_read_bytes: None,
            remote_read_bytes_persec: None,
            remote_read_latency: None,
            remote_read_latency__base: None,
            remote_read_persec: None,
            remote_read_queue_length: None,
            remote_reads: None,
            remote_write_avg_queue_length: None,
            remote_write_bytes: None,
            remote_write_bytes_persec: None,
            remote_write_latency: None,
            remote_write_latency__base: None,
            remote_write_queue_length: None,
            remote_writes: None,
            remote_writes_persec: None,
            write_avg_queue_length: None,
            write_bytes: None,
            write_bytes_persec: None,
            write_latency: None,
            write_latency__base: None,
            write_queue_length: None,
            writes: None,
            writes_persec: None,
        }
    }


    /// Sets the value of ExceededLatencyLimit
    pub fn set_exceeded_latency_limit(&mut self, value: u64) {
        self.exceeded_latency_limit = Some(value);
    }

    /// Gets the value of ExceededLatencyLimit
    pub fn get_exceeded_latency_limit(&self) -> Option<&u64> {
        self.exceeded_latency_limit.as_ref()
    }

    /// Sets the value of ExceededLatencyLimitPersec
    pub fn set_exceeded_latency_limit_persec(&mut self, value: u32) {
        self.exceeded_latency_limit_persec = Some(value);
    }

    /// Gets the value of ExceededLatencyLimitPersec
    pub fn get_exceeded_latency_limit_persec(&self) -> Option<&u32> {
        self.exceeded_latency_limit_persec.as_ref()
    }

    /// Sets the value of IO10000msPersec
    pub fn set_io10000ms_persec(&mut self, value: u32) {
        self.io10000ms_persec = Some(value);
    }

    /// Gets the value of IO10000msPersec
    pub fn get_io10000ms_persec(&self) -> Option<&u32> {
        self.io10000ms_persec.as_ref()
    }

    /// Sets the value of IO1000msPersec
    pub fn set_io1000ms_persec(&mut self, value: u32) {
        self.io1000ms_persec = Some(value);
    }

    /// Gets the value of IO1000msPersec
    pub fn get_io1000ms_persec(&self) -> Option<&u32> {
        self.io1000ms_persec.as_ref()
    }

    /// Sets the value of IO100msPersec
    pub fn set_io100ms_persec(&mut self, value: u32) {
        self.io100ms_persec = Some(value);
    }

    /// Gets the value of IO100msPersec
    pub fn get_io100ms_persec(&self) -> Option<&u32> {
        self.io100ms_persec.as_ref()
    }

    /// Sets the value of IO10msPersec
    pub fn set_io10ms_persec(&mut self, value: u32) {
        self.io10ms_persec = Some(value);
    }

    /// Gets the value of IO10msPersec
    pub fn get_io10ms_persec(&self) -> Option<&u32> {
        self.io10ms_persec.as_ref()
    }

    /// Sets the value of IO1msPersec
    pub fn set_io1ms_persec(&mut self, value: u32) {
        self.io1ms_persec = Some(value);
    }

    /// Gets the value of IO1msPersec
    pub fn get_io1ms_persec(&self) -> Option<&u32> {
        self.io1ms_persec.as_ref()
    }

    /// Sets the value of IO5msPersec
    pub fn set_io5ms_persec(&mut self, value: u32) {
        self.io5ms_persec = Some(value);
    }

    /// Gets the value of IO5msPersec
    pub fn get_io5ms_persec(&self) -> Option<&u32> {
        self.io5ms_persec.as_ref()
    }

    /// Sets the value of LocalReadAvgQueueLength
    pub fn set_local_read_avg_queue_length(&mut self, value: u64) {
        self.local_read_avg_queue_length = Some(value);
    }

    /// Gets the value of LocalReadAvgQueueLength
    pub fn get_local_read_avg_queue_length(&self) -> Option<&u64> {
        self.local_read_avg_queue_length.as_ref()
    }

    /// Sets the value of LocalReadBytes
    pub fn set_local_read_bytes(&mut self, value: u64) {
        self.local_read_bytes = Some(value);
    }

    /// Gets the value of LocalReadBytes
    pub fn get_local_read_bytes(&self) -> Option<&u64> {
        self.local_read_bytes.as_ref()
    }

    /// Sets the value of LocalReadBytesPersec
    pub fn set_local_read_bytes_persec(&mut self, value: u64) {
        self.local_read_bytes_persec = Some(value);
    }

    /// Gets the value of LocalReadBytesPersec
    pub fn get_local_read_bytes_persec(&self) -> Option<&u64> {
        self.local_read_bytes_persec.as_ref()
    }

    /// Sets the value of LocalReadLatency
    pub fn set_local_read_latency(&mut self, value: u32) {
        self.local_read_latency = Some(value);
    }

    /// Gets the value of LocalReadLatency
    pub fn get_local_read_latency(&self) -> Option<&u32> {
        self.local_read_latency.as_ref()
    }

    /// Sets the value of LocalReadLatency_Base
    pub fn set_local_read_latency__base(&mut self, value: u32) {
        self.local_read_latency__base = Some(value);
    }

    /// Gets the value of LocalReadLatency_Base
    pub fn get_local_read_latency__base(&self) -> Option<&u32> {
        self.local_read_latency__base.as_ref()
    }

    /// Sets the value of LocalReadPersec
    pub fn set_local_read_persec(&mut self, value: u32) {
        self.local_read_persec = Some(value);
    }

    /// Gets the value of LocalReadPersec
    pub fn get_local_read_persec(&self) -> Option<&u32> {
        self.local_read_persec.as_ref()
    }

    /// Sets the value of LocalReadQueueLength
    pub fn set_local_read_queue_length(&mut self, value: u64) {
        self.local_read_queue_length = Some(value);
    }

    /// Gets the value of LocalReadQueueLength
    pub fn get_local_read_queue_length(&self) -> Option<&u64> {
        self.local_read_queue_length.as_ref()
    }

    /// Sets the value of LocalReads
    pub fn set_local_reads(&mut self, value: u64) {
        self.local_reads = Some(value);
    }

    /// Gets the value of LocalReads
    pub fn get_local_reads(&self) -> Option<&u64> {
        self.local_reads.as_ref()
    }

    /// Sets the value of LocalWriteAvgQueueLength
    pub fn set_local_write_avg_queue_length(&mut self, value: u64) {
        self.local_write_avg_queue_length = Some(value);
    }

    /// Gets the value of LocalWriteAvgQueueLength
    pub fn get_local_write_avg_queue_length(&self) -> Option<&u64> {
        self.local_write_avg_queue_length.as_ref()
    }

    /// Sets the value of LocalWriteBytes
    pub fn set_local_write_bytes(&mut self, value: u64) {
        self.local_write_bytes = Some(value);
    }

    /// Gets the value of LocalWriteBytes
    pub fn get_local_write_bytes(&self) -> Option<&u64> {
        self.local_write_bytes.as_ref()
    }

    /// Sets the value of LocalWriteBytesPersec
    pub fn set_local_write_bytes_persec(&mut self, value: u64) {
        self.local_write_bytes_persec = Some(value);
    }

    /// Gets the value of LocalWriteBytesPersec
    pub fn get_local_write_bytes_persec(&self) -> Option<&u64> {
        self.local_write_bytes_persec.as_ref()
    }

    /// Sets the value of LocalWriteLatency
    pub fn set_local_write_latency(&mut self, value: u32) {
        self.local_write_latency = Some(value);
    }

    /// Gets the value of LocalWriteLatency
    pub fn get_local_write_latency(&self) -> Option<&u32> {
        self.local_write_latency.as_ref()
    }

    /// Sets the value of LocalWriteLatency_Base
    pub fn set_local_write_latency__base(&mut self, value: u32) {
        self.local_write_latency__base = Some(value);
    }

    /// Gets the value of LocalWriteLatency_Base
    pub fn get_local_write_latency__base(&self) -> Option<&u32> {
        self.local_write_latency__base.as_ref()
    }

    /// Sets the value of LocalWriteQueueLength
    pub fn set_local_write_queue_length(&mut self, value: u64) {
        self.local_write_queue_length = Some(value);
    }

    /// Gets the value of LocalWriteQueueLength
    pub fn get_local_write_queue_length(&self) -> Option<&u64> {
        self.local_write_queue_length.as_ref()
    }

    /// Sets the value of LocalWrites
    pub fn set_local_writes(&mut self, value: u64) {
        self.local_writes = Some(value);
    }

    /// Gets the value of LocalWrites
    pub fn get_local_writes(&self) -> Option<&u64> {
        self.local_writes.as_ref()
    }

    /// Sets the value of LocalWritesPersec
    pub fn set_local_writes_persec(&mut self, value: u32) {
        self.local_writes_persec = Some(value);
    }

    /// Gets the value of LocalWritesPersec
    pub fn get_local_writes_persec(&self) -> Option<&u32> {
        self.local_writes_persec.as_ref()
    }

    /// Sets the value of ReadAvgQueueLength
    pub fn set_read_avg_queue_length(&mut self, value: u64) {
        self.read_avg_queue_length = Some(value);
    }

    /// Gets the value of ReadAvgQueueLength
    pub fn get_read_avg_queue_length(&self) -> Option<&u64> {
        self.read_avg_queue_length.as_ref()
    }

    /// Sets the value of ReadBytes
    pub fn set_read_bytes(&mut self, value: u64) {
        self.read_bytes = Some(value);
    }

    /// Gets the value of ReadBytes
    pub fn get_read_bytes(&self) -> Option<&u64> {
        self.read_bytes.as_ref()
    }

    /// Sets the value of ReadBytesPersec
    pub fn set_read_bytes_persec(&mut self, value: u64) {
        self.read_bytes_persec = Some(value);
    }

    /// Gets the value of ReadBytesPersec
    pub fn get_read_bytes_persec(&self) -> Option<&u64> {
        self.read_bytes_persec.as_ref()
    }

    /// Sets the value of ReadLatency
    pub fn set_read_latency(&mut self, value: u32) {
        self.read_latency = Some(value);
    }

    /// Gets the value of ReadLatency
    pub fn get_read_latency(&self) -> Option<&u32> {
        self.read_latency.as_ref()
    }

    /// Sets the value of ReadLatency_Base
    pub fn set_read_latency__base(&mut self, value: u32) {
        self.read_latency__base = Some(value);
    }

    /// Gets the value of ReadLatency_Base
    pub fn get_read_latency__base(&self) -> Option<&u32> {
        self.read_latency__base.as_ref()
    }

    /// Sets the value of ReadPersec
    pub fn set_read_persec(&mut self, value: u32) {
        self.read_persec = Some(value);
    }

    /// Gets the value of ReadPersec
    pub fn get_read_persec(&self) -> Option<&u32> {
        self.read_persec.as_ref()
    }

    /// Sets the value of ReadQueueLength
    pub fn set_read_queue_length(&mut self, value: u64) {
        self.read_queue_length = Some(value);
    }

    /// Gets the value of ReadQueueLength
    pub fn get_read_queue_length(&self) -> Option<&u64> {
        self.read_queue_length.as_ref()
    }

    /// Sets the value of Reads
    pub fn set_reads(&mut self, value: u64) {
        self.reads = Some(value);
    }

    /// Gets the value of Reads
    pub fn get_reads(&self) -> Option<&u64> {
        self.reads.as_ref()
    }

    /// Sets the value of RemoteReadAvgQueueLength
    pub fn set_remote_read_avg_queue_length(&mut self, value: u64) {
        self.remote_read_avg_queue_length = Some(value);
    }

    /// Gets the value of RemoteReadAvgQueueLength
    pub fn get_remote_read_avg_queue_length(&self) -> Option<&u64> {
        self.remote_read_avg_queue_length.as_ref()
    }

    /// Sets the value of RemoteReadBytes
    pub fn set_remote_read_bytes(&mut self, value: u64) {
        self.remote_read_bytes = Some(value);
    }

    /// Gets the value of RemoteReadBytes
    pub fn get_remote_read_bytes(&self) -> Option<&u64> {
        self.remote_read_bytes.as_ref()
    }

    /// Sets the value of RemoteReadBytesPersec
    pub fn set_remote_read_bytes_persec(&mut self, value: u64) {
        self.remote_read_bytes_persec = Some(value);
    }

    /// Gets the value of RemoteReadBytesPersec
    pub fn get_remote_read_bytes_persec(&self) -> Option<&u64> {
        self.remote_read_bytes_persec.as_ref()
    }

    /// Sets the value of RemoteReadLatency
    pub fn set_remote_read_latency(&mut self, value: u32) {
        self.remote_read_latency = Some(value);
    }

    /// Gets the value of RemoteReadLatency
    pub fn get_remote_read_latency(&self) -> Option<&u32> {
        self.remote_read_latency.as_ref()
    }

    /// Sets the value of RemoteReadLatency_Base
    pub fn set_remote_read_latency__base(&mut self, value: u32) {
        self.remote_read_latency__base = Some(value);
    }

    /// Gets the value of RemoteReadLatency_Base
    pub fn get_remote_read_latency__base(&self) -> Option<&u32> {
        self.remote_read_latency__base.as_ref()
    }

    /// Sets the value of RemoteReadPersec
    pub fn set_remote_read_persec(&mut self, value: u32) {
        self.remote_read_persec = Some(value);
    }

    /// Gets the value of RemoteReadPersec
    pub fn get_remote_read_persec(&self) -> Option<&u32> {
        self.remote_read_persec.as_ref()
    }

    /// Sets the value of RemoteReadQueueLength
    pub fn set_remote_read_queue_length(&mut self, value: u64) {
        self.remote_read_queue_length = Some(value);
    }

    /// Gets the value of RemoteReadQueueLength
    pub fn get_remote_read_queue_length(&self) -> Option<&u64> {
        self.remote_read_queue_length.as_ref()
    }

    /// Sets the value of RemoteReads
    pub fn set_remote_reads(&mut self, value: u64) {
        self.remote_reads = Some(value);
    }

    /// Gets the value of RemoteReads
    pub fn get_remote_reads(&self) -> Option<&u64> {
        self.remote_reads.as_ref()
    }

    /// Sets the value of RemoteWriteAvgQueueLength
    pub fn set_remote_write_avg_queue_length(&mut self, value: u64) {
        self.remote_write_avg_queue_length = Some(value);
    }

    /// Gets the value of RemoteWriteAvgQueueLength
    pub fn get_remote_write_avg_queue_length(&self) -> Option<&u64> {
        self.remote_write_avg_queue_length.as_ref()
    }

    /// Sets the value of RemoteWriteBytes
    pub fn set_remote_write_bytes(&mut self, value: u64) {
        self.remote_write_bytes = Some(value);
    }

    /// Gets the value of RemoteWriteBytes
    pub fn get_remote_write_bytes(&self) -> Option<&u64> {
        self.remote_write_bytes.as_ref()
    }

    /// Sets the value of RemoteWriteBytesPersec
    pub fn set_remote_write_bytes_persec(&mut self, value: u64) {
        self.remote_write_bytes_persec = Some(value);
    }

    /// Gets the value of RemoteWriteBytesPersec
    pub fn get_remote_write_bytes_persec(&self) -> Option<&u64> {
        self.remote_write_bytes_persec.as_ref()
    }

    /// Sets the value of RemoteWriteLatency
    pub fn set_remote_write_latency(&mut self, value: u32) {
        self.remote_write_latency = Some(value);
    }

    /// Gets the value of RemoteWriteLatency
    pub fn get_remote_write_latency(&self) -> Option<&u32> {
        self.remote_write_latency.as_ref()
    }

    /// Sets the value of RemoteWriteLatency_Base
    pub fn set_remote_write_latency__base(&mut self, value: u32) {
        self.remote_write_latency__base = Some(value);
    }

    /// Gets the value of RemoteWriteLatency_Base
    pub fn get_remote_write_latency__base(&self) -> Option<&u32> {
        self.remote_write_latency__base.as_ref()
    }

    /// Sets the value of RemoteWriteQueueLength
    pub fn set_remote_write_queue_length(&mut self, value: u64) {
        self.remote_write_queue_length = Some(value);
    }

    /// Gets the value of RemoteWriteQueueLength
    pub fn get_remote_write_queue_length(&self) -> Option<&u64> {
        self.remote_write_queue_length.as_ref()
    }

    /// Sets the value of RemoteWrites
    pub fn set_remote_writes(&mut self, value: u64) {
        self.remote_writes = Some(value);
    }

    /// Gets the value of RemoteWrites
    pub fn get_remote_writes(&self) -> Option<&u64> {
        self.remote_writes.as_ref()
    }

    /// Sets the value of RemoteWritesPersec
    pub fn set_remote_writes_persec(&mut self, value: u32) {
        self.remote_writes_persec = Some(value);
    }

    /// Gets the value of RemoteWritesPersec
    pub fn get_remote_writes_persec(&self) -> Option<&u32> {
        self.remote_writes_persec.as_ref()
    }

    /// Sets the value of WriteAvgQueueLength
    pub fn set_write_avg_queue_length(&mut self, value: u64) {
        self.write_avg_queue_length = Some(value);
    }

    /// Gets the value of WriteAvgQueueLength
    pub fn get_write_avg_queue_length(&self) -> Option<&u64> {
        self.write_avg_queue_length.as_ref()
    }

    /// Sets the value of WriteBytes
    pub fn set_write_bytes(&mut self, value: u64) {
        self.write_bytes = Some(value);
    }

    /// Gets the value of WriteBytes
    pub fn get_write_bytes(&self) -> Option<&u64> {
        self.write_bytes.as_ref()
    }

    /// Sets the value of WriteBytesPersec
    pub fn set_write_bytes_persec(&mut self, value: u32) {
        self.write_bytes_persec = Some(value);
    }

    /// Gets the value of WriteBytesPersec
    pub fn get_write_bytes_persec(&self) -> Option<&u32> {
        self.write_bytes_persec.as_ref()
    }

    /// Sets the value of WriteLatency
    pub fn set_write_latency(&mut self, value: u32) {
        self.write_latency = Some(value);
    }

    /// Gets the value of WriteLatency
    pub fn get_write_latency(&self) -> Option<&u32> {
        self.write_latency.as_ref()
    }

    /// Sets the value of WriteLatency_Base
    pub fn set_write_latency__base(&mut self, value: u32) {
        self.write_latency__base = Some(value);
    }

    /// Gets the value of WriteLatency_Base
    pub fn get_write_latency__base(&self) -> Option<&u32> {
        self.write_latency__base.as_ref()
    }

    /// Sets the value of WriteQueueLength
    pub fn set_write_queue_length(&mut self, value: u64) {
        self.write_queue_length = Some(value);
    }

    /// Gets the value of WriteQueueLength
    pub fn get_write_queue_length(&self) -> Option<&u64> {
        self.write_queue_length.as_ref()
    }

    /// Sets the value of Writes
    pub fn set_writes(&mut self, value: u64) {
        self.writes = Some(value);
    }

    /// Gets the value of Writes
    pub fn get_writes(&self) -> Option<&u64> {
        self.writes.as_ref()
    }

    /// Sets the value of WritesPersec
    pub fn set_writes_persec(&mut self, value: u32) {
        self.writes_persec = Some(value);
    }

    /// Gets the value of WritesPersec
    pub fn get_writes_persec(&self) -> Option<&u32> {
        self.writes_persec.as_ref()
    }
}

