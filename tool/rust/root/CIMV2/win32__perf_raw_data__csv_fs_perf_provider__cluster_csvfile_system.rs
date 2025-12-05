// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_CsvFsPerfProvider_ClusterCSVFileSystem struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_CsvFsPerfProvider_ClusterCSVFileSystem {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "CreateFile")]
    pub create_file: Option<u64>,

/// 
    #[serde(rename = "CreateFilePersec")]
    pub create_file_persec: Option<u64>,

/// 
    #[serde(rename = "FilesInvalidatedDuringResume")]
    pub files_invalidated_during_resume: Option<u64>,

/// 
    #[serde(rename = "FilesInvalidatedOther")]
    pub files_invalidated_other: Option<u64>,

/// 
    #[serde(rename = "FilesOpened")]
    pub files_opened: Option<u32>,

/// 
    #[serde(rename = "Flushes")]
    pub flushes: Option<u64>,

/// 
    #[serde(rename = "FlushesPersec")]
    pub flushes_persec: Option<u64>,

/// 
    #[serde(rename = "IOReadAvgQueueLength")]
    pub ioread_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "IOReadBytes")]
    pub ioread_bytes: Option<u64>,

/// 
    #[serde(rename = "IOReadBytesPersec")]
    pub ioread_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "IOReadLatency")]
    pub ioread_latency: Option<u32>,

/// 
    #[serde(rename = "IOReadLatency_Base")]
    pub ioread_latency__base: Option<u32>,

/// 
    #[serde(rename = "IOReadQueueLength")]
    pub ioread_queue_length: Option<u64>,

/// 
    #[serde(rename = "IOReads")]
    pub ioreads: Option<u64>,

/// 
    #[serde(rename = "IOReadsPersec")]
    pub ioreads_persec: Option<u64>,

/// 
    #[serde(rename = "IOSingleReads")]
    pub iosingle_reads: Option<u64>,

/// 
    #[serde(rename = "IOSingleReadsPersec")]
    pub iosingle_reads_persec: Option<u64>,

/// 
    #[serde(rename = "IOSingleWrites")]
    pub iosingle_writes: Option<u64>,

/// 
    #[serde(rename = "IOSingleWritesPersec")]
    pub iosingle_writes_persec: Option<u64>,

/// 
    #[serde(rename = "IOSplitReads")]
    pub iosplit_reads: Option<u64>,

/// 
    #[serde(rename = "IOSplitReadsPersec")]
    pub iosplit_reads_persec: Option<u64>,

/// 
    #[serde(rename = "IOSplitWrites")]
    pub iosplit_writes: Option<u64>,

/// 
    #[serde(rename = "IOSplitWritesPersec")]
    pub iosplit_writes_persec: Option<u64>,

/// 
    #[serde(rename = "IOWriteAvgQueueLength")]
    pub iowrite_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "IOWriteBytes")]
    pub iowrite_bytes: Option<u64>,

/// 
    #[serde(rename = "IOWriteBytesPersec")]
    pub iowrite_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "IOWriteLatency")]
    pub iowrite_latency: Option<u32>,

/// 
    #[serde(rename = "IOWriteLatency_Base")]
    pub iowrite_latency__base: Option<u32>,

/// 
    #[serde(rename = "IOWriteQueueLength")]
    pub iowrite_queue_length: Option<u64>,

/// 
    #[serde(rename = "IOWrites")]
    pub iowrites: Option<u64>,

/// 
    #[serde(rename = "IOWritesPersec")]
    pub iowrites_persec: Option<u64>,

/// 
    #[serde(rename = "MetadataIO")]
    pub metadata_io: Option<u64>,

/// 
    #[serde(rename = "MetadataIOPersec")]
    pub metadata_iopersec: Option<u64>,

/// 
    #[serde(rename = "ReadLatency")]
    pub read_latency: Option<u32>,

/// 
    #[serde(rename = "ReadLatency_Base")]
    pub read_latency__base: Option<u32>,

/// 
    #[serde(rename = "ReadQueueLength")]
    pub read_queue_length: Option<u64>,

/// 
    #[serde(rename = "Reads")]
    pub reads: Option<u64>,

/// 
    #[serde(rename = "ReadsPersec")]
    pub reads_persec: Option<u64>,

/// 
    #[serde(rename = "RedirectedReadBytes")]
    pub redirected_read_bytes: Option<u64>,

/// 
    #[serde(rename = "RedirectedReadBytesPersec")]
    pub redirected_read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "RedirectedReadLatency")]
    pub redirected_read_latency: Option<u32>,

/// 
    #[serde(rename = "RedirectedReadLatency_Base")]
    pub redirected_read_latency__base: Option<u32>,

/// 
    #[serde(rename = "RedirectedReadQueueLength")]
    pub redirected_read_queue_length: Option<u64>,

/// 
    #[serde(rename = "RedirectedReads")]
    pub redirected_reads: Option<u64>,

/// 
    #[serde(rename = "RedirectedReadsAvgQueueLength")]
    pub redirected_reads_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "RedirectedReadsPersec")]
    pub redirected_reads_persec: Option<u64>,

/// 
    #[serde(rename = "RedirectedWriteBytes")]
    pub redirected_write_bytes: Option<u64>,

/// 
    #[serde(rename = "RedirectedWriteBytesPersec")]
    pub redirected_write_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "RedirectedWriteLatency")]
    pub redirected_write_latency: Option<u32>,

/// 
    #[serde(rename = "RedirectedWriteLatency_Base")]
    pub redirected_write_latency__base: Option<u32>,

/// 
    #[serde(rename = "RedirectedWriteQueueLength")]
    pub redirected_write_queue_length: Option<u64>,

/// 
    #[serde(rename = "RedirectedWrites")]
    pub redirected_writes: Option<u64>,

/// 
    #[serde(rename = "RedirectedWritesAvgQueueLength")]
    pub redirected_writes_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "RedirectedWritesPersec")]
    pub redirected_writes_persec: Option<u64>,

/// 
    #[serde(rename = "VolumePauseCountDisk")]
    pub volume_pause_count_disk: Option<u64>,

/// 
    #[serde(rename = "VolumePauseCountNetwork")]
    pub volume_pause_count_network: Option<u64>,

/// 
    #[serde(rename = "VolumePauseCountOther")]
    pub volume_pause_count_other: Option<u64>,

/// 
    #[serde(rename = "VolumePauseCountTotal")]
    pub volume_pause_count_total: Option<u64>,

/// 
    #[serde(rename = "VolumeState")]
    pub volume_state: Option<u32>,

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
    pub writes_persec: Option<u64>,
}

impl Win32_PerfRawData_CsvFsPerfProvider_ClusterCSVFileSystem {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            create_file: None,
            create_file_persec: None,
            files_invalidated_during_resume: None,
            files_invalidated_other: None,
            files_opened: None,
            flushes: None,
            flushes_persec: None,
            ioread_avg_queue_length: None,
            ioread_bytes: None,
            ioread_bytes_persec: None,
            ioread_latency: None,
            ioread_latency__base: None,
            ioread_queue_length: None,
            ioreads: None,
            ioreads_persec: None,
            iosingle_reads: None,
            iosingle_reads_persec: None,
            iosingle_writes: None,
            iosingle_writes_persec: None,
            iosplit_reads: None,
            iosplit_reads_persec: None,
            iosplit_writes: None,
            iosplit_writes_persec: None,
            iowrite_avg_queue_length: None,
            iowrite_bytes: None,
            iowrite_bytes_persec: None,
            iowrite_latency: None,
            iowrite_latency__base: None,
            iowrite_queue_length: None,
            iowrites: None,
            iowrites_persec: None,
            metadata_io: None,
            metadata_iopersec: None,
            read_latency: None,
            read_latency__base: None,
            read_queue_length: None,
            reads: None,
            reads_persec: None,
            redirected_read_bytes: None,
            redirected_read_bytes_persec: None,
            redirected_read_latency: None,
            redirected_read_latency__base: None,
            redirected_read_queue_length: None,
            redirected_reads: None,
            redirected_reads_avg_queue_length: None,
            redirected_reads_persec: None,
            redirected_write_bytes: None,
            redirected_write_bytes_persec: None,
            redirected_write_latency: None,
            redirected_write_latency__base: None,
            redirected_write_queue_length: None,
            redirected_writes: None,
            redirected_writes_avg_queue_length: None,
            redirected_writes_persec: None,
            volume_pause_count_disk: None,
            volume_pause_count_network: None,
            volume_pause_count_other: None,
            volume_pause_count_total: None,
            volume_state: None,
            write_latency: None,
            write_latency__base: None,
            write_queue_length: None,
            writes: None,
            writes_persec: None,
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

    /// Sets the value of FilesInvalidatedDuringResume
    pub fn set_files_invalidated_during_resume(&mut self, value: u64) {
        self.files_invalidated_during_resume = Some(value);
    }

    /// Gets the value of FilesInvalidatedDuringResume
    pub fn get_files_invalidated_during_resume(&self) -> Option<&u64> {
        self.files_invalidated_during_resume.as_ref()
    }

    /// Sets the value of FilesInvalidatedOther
    pub fn set_files_invalidated_other(&mut self, value: u64) {
        self.files_invalidated_other = Some(value);
    }

    /// Gets the value of FilesInvalidatedOther
    pub fn get_files_invalidated_other(&self) -> Option<&u64> {
        self.files_invalidated_other.as_ref()
    }

    /// Sets the value of FilesOpened
    pub fn set_files_opened(&mut self, value: u32) {
        self.files_opened = Some(value);
    }

    /// Gets the value of FilesOpened
    pub fn get_files_opened(&self) -> Option<&u32> {
        self.files_opened.as_ref()
    }

    /// Sets the value of Flushes
    pub fn set_flushes(&mut self, value: u64) {
        self.flushes = Some(value);
    }

    /// Gets the value of Flushes
    pub fn get_flushes(&self) -> Option<&u64> {
        self.flushes.as_ref()
    }

    /// Sets the value of FlushesPersec
    pub fn set_flushes_persec(&mut self, value: u64) {
        self.flushes_persec = Some(value);
    }

    /// Gets the value of FlushesPersec
    pub fn get_flushes_persec(&self) -> Option<&u64> {
        self.flushes_persec.as_ref()
    }

    /// Sets the value of IOReadAvgQueueLength
    pub fn set_ioread_avg_queue_length(&mut self, value: u64) {
        self.ioread_avg_queue_length = Some(value);
    }

    /// Gets the value of IOReadAvgQueueLength
    pub fn get_ioread_avg_queue_length(&self) -> Option<&u64> {
        self.ioread_avg_queue_length.as_ref()
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

    /// Sets the value of IOReadLatency
    pub fn set_ioread_latency(&mut self, value: u32) {
        self.ioread_latency = Some(value);
    }

    /// Gets the value of IOReadLatency
    pub fn get_ioread_latency(&self) -> Option<&u32> {
        self.ioread_latency.as_ref()
    }

    /// Sets the value of IOReadLatency_Base
    pub fn set_ioread_latency__base(&mut self, value: u32) {
        self.ioread_latency__base = Some(value);
    }

    /// Gets the value of IOReadLatency_Base
    pub fn get_ioread_latency__base(&self) -> Option<&u32> {
        self.ioread_latency__base.as_ref()
    }

    /// Sets the value of IOReadQueueLength
    pub fn set_ioread_queue_length(&mut self, value: u64) {
        self.ioread_queue_length = Some(value);
    }

    /// Gets the value of IOReadQueueLength
    pub fn get_ioread_queue_length(&self) -> Option<&u64> {
        self.ioread_queue_length.as_ref()
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

    /// Sets the value of IOSingleReads
    pub fn set_iosingle_reads(&mut self, value: u64) {
        self.iosingle_reads = Some(value);
    }

    /// Gets the value of IOSingleReads
    pub fn get_iosingle_reads(&self) -> Option<&u64> {
        self.iosingle_reads.as_ref()
    }

    /// Sets the value of IOSingleReadsPersec
    pub fn set_iosingle_reads_persec(&mut self, value: u64) {
        self.iosingle_reads_persec = Some(value);
    }

    /// Gets the value of IOSingleReadsPersec
    pub fn get_iosingle_reads_persec(&self) -> Option<&u64> {
        self.iosingle_reads_persec.as_ref()
    }

    /// Sets the value of IOSingleWrites
    pub fn set_iosingle_writes(&mut self, value: u64) {
        self.iosingle_writes = Some(value);
    }

    /// Gets the value of IOSingleWrites
    pub fn get_iosingle_writes(&self) -> Option<&u64> {
        self.iosingle_writes.as_ref()
    }

    /// Sets the value of IOSingleWritesPersec
    pub fn set_iosingle_writes_persec(&mut self, value: u64) {
        self.iosingle_writes_persec = Some(value);
    }

    /// Gets the value of IOSingleWritesPersec
    pub fn get_iosingle_writes_persec(&self) -> Option<&u64> {
        self.iosingle_writes_persec.as_ref()
    }

    /// Sets the value of IOSplitReads
    pub fn set_iosplit_reads(&mut self, value: u64) {
        self.iosplit_reads = Some(value);
    }

    /// Gets the value of IOSplitReads
    pub fn get_iosplit_reads(&self) -> Option<&u64> {
        self.iosplit_reads.as_ref()
    }

    /// Sets the value of IOSplitReadsPersec
    pub fn set_iosplit_reads_persec(&mut self, value: u64) {
        self.iosplit_reads_persec = Some(value);
    }

    /// Gets the value of IOSplitReadsPersec
    pub fn get_iosplit_reads_persec(&self) -> Option<&u64> {
        self.iosplit_reads_persec.as_ref()
    }

    /// Sets the value of IOSplitWrites
    pub fn set_iosplit_writes(&mut self, value: u64) {
        self.iosplit_writes = Some(value);
    }

    /// Gets the value of IOSplitWrites
    pub fn get_iosplit_writes(&self) -> Option<&u64> {
        self.iosplit_writes.as_ref()
    }

    /// Sets the value of IOSplitWritesPersec
    pub fn set_iosplit_writes_persec(&mut self, value: u64) {
        self.iosplit_writes_persec = Some(value);
    }

    /// Gets the value of IOSplitWritesPersec
    pub fn get_iosplit_writes_persec(&self) -> Option<&u64> {
        self.iosplit_writes_persec.as_ref()
    }

    /// Sets the value of IOWriteAvgQueueLength
    pub fn set_iowrite_avg_queue_length(&mut self, value: u64) {
        self.iowrite_avg_queue_length = Some(value);
    }

    /// Gets the value of IOWriteAvgQueueLength
    pub fn get_iowrite_avg_queue_length(&self) -> Option<&u64> {
        self.iowrite_avg_queue_length.as_ref()
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

    /// Sets the value of IOWriteLatency
    pub fn set_iowrite_latency(&mut self, value: u32) {
        self.iowrite_latency = Some(value);
    }

    /// Gets the value of IOWriteLatency
    pub fn get_iowrite_latency(&self) -> Option<&u32> {
        self.iowrite_latency.as_ref()
    }

    /// Sets the value of IOWriteLatency_Base
    pub fn set_iowrite_latency__base(&mut self, value: u32) {
        self.iowrite_latency__base = Some(value);
    }

    /// Gets the value of IOWriteLatency_Base
    pub fn get_iowrite_latency__base(&self) -> Option<&u32> {
        self.iowrite_latency__base.as_ref()
    }

    /// Sets the value of IOWriteQueueLength
    pub fn set_iowrite_queue_length(&mut self, value: u64) {
        self.iowrite_queue_length = Some(value);
    }

    /// Gets the value of IOWriteQueueLength
    pub fn get_iowrite_queue_length(&self) -> Option<&u64> {
        self.iowrite_queue_length.as_ref()
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

    /// Sets the value of ReadsPersec
    pub fn set_reads_persec(&mut self, value: u64) {
        self.reads_persec = Some(value);
    }

    /// Gets the value of ReadsPersec
    pub fn get_reads_persec(&self) -> Option<&u64> {
        self.reads_persec.as_ref()
    }

    /// Sets the value of RedirectedReadBytes
    pub fn set_redirected_read_bytes(&mut self, value: u64) {
        self.redirected_read_bytes = Some(value);
    }

    /// Gets the value of RedirectedReadBytes
    pub fn get_redirected_read_bytes(&self) -> Option<&u64> {
        self.redirected_read_bytes.as_ref()
    }

    /// Sets the value of RedirectedReadBytesPersec
    pub fn set_redirected_read_bytes_persec(&mut self, value: u64) {
        self.redirected_read_bytes_persec = Some(value);
    }

    /// Gets the value of RedirectedReadBytesPersec
    pub fn get_redirected_read_bytes_persec(&self) -> Option<&u64> {
        self.redirected_read_bytes_persec.as_ref()
    }

    /// Sets the value of RedirectedReadLatency
    pub fn set_redirected_read_latency(&mut self, value: u32) {
        self.redirected_read_latency = Some(value);
    }

    /// Gets the value of RedirectedReadLatency
    pub fn get_redirected_read_latency(&self) -> Option<&u32> {
        self.redirected_read_latency.as_ref()
    }

    /// Sets the value of RedirectedReadLatency_Base
    pub fn set_redirected_read_latency__base(&mut self, value: u32) {
        self.redirected_read_latency__base = Some(value);
    }

    /// Gets the value of RedirectedReadLatency_Base
    pub fn get_redirected_read_latency__base(&self) -> Option<&u32> {
        self.redirected_read_latency__base.as_ref()
    }

    /// Sets the value of RedirectedReadQueueLength
    pub fn set_redirected_read_queue_length(&mut self, value: u64) {
        self.redirected_read_queue_length = Some(value);
    }

    /// Gets the value of RedirectedReadQueueLength
    pub fn get_redirected_read_queue_length(&self) -> Option<&u64> {
        self.redirected_read_queue_length.as_ref()
    }

    /// Sets the value of RedirectedReads
    pub fn set_redirected_reads(&mut self, value: u64) {
        self.redirected_reads = Some(value);
    }

    /// Gets the value of RedirectedReads
    pub fn get_redirected_reads(&self) -> Option<&u64> {
        self.redirected_reads.as_ref()
    }

    /// Sets the value of RedirectedReadsAvgQueueLength
    pub fn set_redirected_reads_avg_queue_length(&mut self, value: u64) {
        self.redirected_reads_avg_queue_length = Some(value);
    }

    /// Gets the value of RedirectedReadsAvgQueueLength
    pub fn get_redirected_reads_avg_queue_length(&self) -> Option<&u64> {
        self.redirected_reads_avg_queue_length.as_ref()
    }

    /// Sets the value of RedirectedReadsPersec
    pub fn set_redirected_reads_persec(&mut self, value: u64) {
        self.redirected_reads_persec = Some(value);
    }

    /// Gets the value of RedirectedReadsPersec
    pub fn get_redirected_reads_persec(&self) -> Option<&u64> {
        self.redirected_reads_persec.as_ref()
    }

    /// Sets the value of RedirectedWriteBytes
    pub fn set_redirected_write_bytes(&mut self, value: u64) {
        self.redirected_write_bytes = Some(value);
    }

    /// Gets the value of RedirectedWriteBytes
    pub fn get_redirected_write_bytes(&self) -> Option<&u64> {
        self.redirected_write_bytes.as_ref()
    }

    /// Sets the value of RedirectedWriteBytesPersec
    pub fn set_redirected_write_bytes_persec(&mut self, value: u64) {
        self.redirected_write_bytes_persec = Some(value);
    }

    /// Gets the value of RedirectedWriteBytesPersec
    pub fn get_redirected_write_bytes_persec(&self) -> Option<&u64> {
        self.redirected_write_bytes_persec.as_ref()
    }

    /// Sets the value of RedirectedWriteLatency
    pub fn set_redirected_write_latency(&mut self, value: u32) {
        self.redirected_write_latency = Some(value);
    }

    /// Gets the value of RedirectedWriteLatency
    pub fn get_redirected_write_latency(&self) -> Option<&u32> {
        self.redirected_write_latency.as_ref()
    }

    /// Sets the value of RedirectedWriteLatency_Base
    pub fn set_redirected_write_latency__base(&mut self, value: u32) {
        self.redirected_write_latency__base = Some(value);
    }

    /// Gets the value of RedirectedWriteLatency_Base
    pub fn get_redirected_write_latency__base(&self) -> Option<&u32> {
        self.redirected_write_latency__base.as_ref()
    }

    /// Sets the value of RedirectedWriteQueueLength
    pub fn set_redirected_write_queue_length(&mut self, value: u64) {
        self.redirected_write_queue_length = Some(value);
    }

    /// Gets the value of RedirectedWriteQueueLength
    pub fn get_redirected_write_queue_length(&self) -> Option<&u64> {
        self.redirected_write_queue_length.as_ref()
    }

    /// Sets the value of RedirectedWrites
    pub fn set_redirected_writes(&mut self, value: u64) {
        self.redirected_writes = Some(value);
    }

    /// Gets the value of RedirectedWrites
    pub fn get_redirected_writes(&self) -> Option<&u64> {
        self.redirected_writes.as_ref()
    }

    /// Sets the value of RedirectedWritesAvgQueueLength
    pub fn set_redirected_writes_avg_queue_length(&mut self, value: u64) {
        self.redirected_writes_avg_queue_length = Some(value);
    }

    /// Gets the value of RedirectedWritesAvgQueueLength
    pub fn get_redirected_writes_avg_queue_length(&self) -> Option<&u64> {
        self.redirected_writes_avg_queue_length.as_ref()
    }

    /// Sets the value of RedirectedWritesPersec
    pub fn set_redirected_writes_persec(&mut self, value: u64) {
        self.redirected_writes_persec = Some(value);
    }

    /// Gets the value of RedirectedWritesPersec
    pub fn get_redirected_writes_persec(&self) -> Option<&u64> {
        self.redirected_writes_persec.as_ref()
    }

    /// Sets the value of VolumePauseCountDisk
    pub fn set_volume_pause_count_disk(&mut self, value: u64) {
        self.volume_pause_count_disk = Some(value);
    }

    /// Gets the value of VolumePauseCountDisk
    pub fn get_volume_pause_count_disk(&self) -> Option<&u64> {
        self.volume_pause_count_disk.as_ref()
    }

    /// Sets the value of VolumePauseCountNetwork
    pub fn set_volume_pause_count_network(&mut self, value: u64) {
        self.volume_pause_count_network = Some(value);
    }

    /// Gets the value of VolumePauseCountNetwork
    pub fn get_volume_pause_count_network(&self) -> Option<&u64> {
        self.volume_pause_count_network.as_ref()
    }

    /// Sets the value of VolumePauseCountOther
    pub fn set_volume_pause_count_other(&mut self, value: u64) {
        self.volume_pause_count_other = Some(value);
    }

    /// Gets the value of VolumePauseCountOther
    pub fn get_volume_pause_count_other(&self) -> Option<&u64> {
        self.volume_pause_count_other.as_ref()
    }

    /// Sets the value of VolumePauseCountTotal
    pub fn set_volume_pause_count_total(&mut self, value: u64) {
        self.volume_pause_count_total = Some(value);
    }

    /// Gets the value of VolumePauseCountTotal
    pub fn get_volume_pause_count_total(&self) -> Option<&u64> {
        self.volume_pause_count_total.as_ref()
    }

    /// Sets the value of VolumeState
    pub fn set_volume_state(&mut self, value: u32) {
        self.volume_state = Some(value);
    }

    /// Gets the value of VolumeState
    pub fn get_volume_state(&self) -> Option<&u32> {
        self.volume_state.as_ref()
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
    pub fn set_writes_persec(&mut self, value: u64) {
        self.writes_persec = Some(value);
    }

    /// Gets the value of WritesPersec
    pub fn get_writes_persec(&self) -> Option<&u64> {
        self.writes_persec.as_ref()
    }
}

