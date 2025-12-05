// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_CsvFsPerfProvider_ClusterCSVFS struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_CsvFsPerfProvider_ClusterCSVFS {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "AvgBytesPerRead")]
    pub avg_bytes_per_read: Option<u64>,

/// 
    #[serde(rename = "AvgBytesPerWrite")]
    pub avg_bytes_per_write: Option<u64>,

/// 
    #[serde(rename = "AvgReadQueueLength")]
    pub avg_read_queue_length: Option<u64>,

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
    #[serde(rename = "CreateFile")]
    pub create_file: Option<u64>,

/// 
    #[serde(rename = "CreateFilePersec")]
    pub create_file_persec: Option<u64>,

/// 
    #[serde(rename = "CurrentReadQueueLength")]
    pub current_read_queue_length: Option<u64>,

/// 
    #[serde(rename = "CurrentWriteQueueLength")]
    pub current_write_queue_length: Option<u64>,

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
    #[serde(rename = "MetadataIO")]
    pub metadata_io: Option<u64>,

/// 
    #[serde(rename = "MetadataIOPersec")]
    pub metadata_iopersec: Option<u64>,

/// 
    #[serde(rename = "ReadBytesPersec")]
    pub read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "Reads")]
    pub reads: Option<u64>,

/// 
    #[serde(rename = "ReadsPersec")]
    pub reads_persec: Option<u64>,

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
    #[serde(rename = "WriteBytesPersec")]
    pub write_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "Writes")]
    pub writes: Option<u64>,

/// 
    #[serde(rename = "WritesPersec")]
    pub writes_persec: Option<u64>,
}

impl Win32_PerfFormattedData_CsvFsPerfProvider_ClusterCSVFS {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            avg_bytes_per_read: None,
            avg_bytes_per_write: None,
            avg_read_queue_length: None,
            avgsec_per_read: None,
            avgsec_per_write: None,
            avg_write_queue_length: None,
            create_file: None,
            create_file_persec: None,
            current_read_queue_length: None,
            current_write_queue_length: None,
            files_invalidated_during_resume: None,
            files_invalidated_other: None,
            files_opened: None,
            flushes: None,
            flushes_persec: None,
            metadata_io: None,
            metadata_iopersec: None,
            read_bytes_persec: None,
            reads: None,
            reads_persec: None,
            volume_pause_count_disk: None,
            volume_pause_count_network: None,
            volume_pause_count_other: None,
            volume_pause_count_total: None,
            volume_state: None,
            write_bytes_persec: None,
            writes: None,
            writes_persec: None,
        }
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

    /// Sets the value of AvgReadQueueLength
    pub fn set_avg_read_queue_length(&mut self, value: u64) {
        self.avg_read_queue_length = Some(value);
    }

    /// Gets the value of AvgReadQueueLength
    pub fn get_avg_read_queue_length(&self) -> Option<&u64> {
        self.avg_read_queue_length.as_ref()
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

    /// Sets the value of CurrentReadQueueLength
    pub fn set_current_read_queue_length(&mut self, value: u64) {
        self.current_read_queue_length = Some(value);
    }

    /// Gets the value of CurrentReadQueueLength
    pub fn get_current_read_queue_length(&self) -> Option<&u64> {
        self.current_read_queue_length.as_ref()
    }

    /// Sets the value of CurrentWriteQueueLength
    pub fn set_current_write_queue_length(&mut self, value: u64) {
        self.current_write_queue_length = Some(value);
    }

    /// Gets the value of CurrentWriteQueueLength
    pub fn get_current_write_queue_length(&self) -> Option<&u64> {
        self.current_write_queue_length.as_ref()
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

    /// Sets the value of ReadBytesPersec
    pub fn set_read_bytes_persec(&mut self, value: u64) {
        self.read_bytes_persec = Some(value);
    }

    /// Gets the value of ReadBytesPersec
    pub fn get_read_bytes_persec(&self) -> Option<&u64> {
        self.read_bytes_persec.as_ref()
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

    /// Sets the value of WriteBytesPersec
    pub fn set_write_bytes_persec(&mut self, value: u64) {
        self.write_bytes_persec = Some(value);
    }

    /// Gets the value of WriteBytesPersec
    pub fn get_write_bytes_persec(&self) -> Option<&u64> {
        self.write_bytes_persec.as_ref()
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

