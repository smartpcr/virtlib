// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_WindowsMediaPlayer_WindowsMediaPlayerMetadata struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_WindowsMediaPlayer_WindowsMediaPlayerMetadata {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "AFTSExecutionTimems")]
    pub aftsexecution_timems: Option<u32>,

/// 
    #[serde(rename = "ArtExtractionTimems")]
    pub art_extraction_timems: Option<u32>,

/// 
    #[serde(rename = "CommitTimems")]
    pub commit_timems: Option<u32>,

/// 
    #[serde(rename = "DirectoryChangeQueueLength")]
    pub directory_change_queue_length: Option<u32>,

/// 
    #[serde(rename = "DirtyDirectoryHitCount")]
    pub dirty_directory_hit_count: Option<u32>,

/// 
    #[serde(rename = "FileScanningThreadPrioirty")]
    pub file_scanning_thread_prioirty: Option<u32>,

/// 
    #[serde(rename = "FilesScannedPerMinute")]
    pub files_scanned_per_minute: Option<u64>,

/// 
    #[serde(rename = "GrovelerServiceRoutineExecutionsPerSecond")]
    pub groveler_service_routine_executions_per_second: Option<u64>,

/// 
    #[serde(rename = "LibraryDescriptionChangeNotificationsPerSecond")]
    pub library_description_change_notifications_per_second: Option<u64>,

/// 
    #[serde(rename = "LibraryDescriptionUpdatesPerSecond")]
    pub library_description_updates_per_second: Option<u64>,

/// 
    #[serde(rename = "MonitoredFolderUpdatesPerSecond")]
    pub monitored_folder_updates_per_second: Option<u64>,

/// 
    #[serde(rename = "NormalizationTimems")]
    pub normalization_timems: Option<u32>,

/// 
    #[serde(rename = "PropertyExtractionTimems")]
    pub property_extraction_timems: Option<u32>,

/// 
    #[serde(rename = "ReorganizeTimems")]
    pub reorganize_timems: Option<u32>,

/// 
    #[serde(rename = "ScanningState")]
    pub scanning_state: Option<u32>,

/// 
    #[serde(rename = "TimestampDirectoryHitCount")]
    pub timestamp_directory_hit_count: Option<u32>,

/// 
    #[serde(rename = "URLClassificationTimems")]
    pub urlclassification_timems: Option<u32>,
}

impl Win32_PerfFormattedData_WindowsMediaPlayer_WindowsMediaPlayerMetadata {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            aftsexecution_timems: None,
            art_extraction_timems: None,
            commit_timems: None,
            directory_change_queue_length: None,
            dirty_directory_hit_count: None,
            file_scanning_thread_prioirty: None,
            files_scanned_per_minute: None,
            groveler_service_routine_executions_per_second: None,
            library_description_change_notifications_per_second: None,
            library_description_updates_per_second: None,
            monitored_folder_updates_per_second: None,
            normalization_timems: None,
            property_extraction_timems: None,
            reorganize_timems: None,
            scanning_state: None,
            timestamp_directory_hit_count: None,
            urlclassification_timems: None,
        }
    }


    /// Sets the value of AFTSExecutionTimems
    pub fn set_aftsexecution_timems(&mut self, value: u32) {
        self.aftsexecution_timems = Some(value);
    }

    /// Gets the value of AFTSExecutionTimems
    pub fn get_aftsexecution_timems(&self) -> Option<&u32> {
        self.aftsexecution_timems.as_ref()
    }

    /// Sets the value of ArtExtractionTimems
    pub fn set_art_extraction_timems(&mut self, value: u32) {
        self.art_extraction_timems = Some(value);
    }

    /// Gets the value of ArtExtractionTimems
    pub fn get_art_extraction_timems(&self) -> Option<&u32> {
        self.art_extraction_timems.as_ref()
    }

    /// Sets the value of CommitTimems
    pub fn set_commit_timems(&mut self, value: u32) {
        self.commit_timems = Some(value);
    }

    /// Gets the value of CommitTimems
    pub fn get_commit_timems(&self) -> Option<&u32> {
        self.commit_timems.as_ref()
    }

    /// Sets the value of DirectoryChangeQueueLength
    pub fn set_directory_change_queue_length(&mut self, value: u32) {
        self.directory_change_queue_length = Some(value);
    }

    /// Gets the value of DirectoryChangeQueueLength
    pub fn get_directory_change_queue_length(&self) -> Option<&u32> {
        self.directory_change_queue_length.as_ref()
    }

    /// Sets the value of DirtyDirectoryHitCount
    pub fn set_dirty_directory_hit_count(&mut self, value: u32) {
        self.dirty_directory_hit_count = Some(value);
    }

    /// Gets the value of DirtyDirectoryHitCount
    pub fn get_dirty_directory_hit_count(&self) -> Option<&u32> {
        self.dirty_directory_hit_count.as_ref()
    }

    /// Sets the value of FileScanningThreadPrioirty
    pub fn set_file_scanning_thread_prioirty(&mut self, value: u32) {
        self.file_scanning_thread_prioirty = Some(value);
    }

    /// Gets the value of FileScanningThreadPrioirty
    pub fn get_file_scanning_thread_prioirty(&self) -> Option<&u32> {
        self.file_scanning_thread_prioirty.as_ref()
    }

    /// Sets the value of FilesScannedPerMinute
    pub fn set_files_scanned_per_minute(&mut self, value: u64) {
        self.files_scanned_per_minute = Some(value);
    }

    /// Gets the value of FilesScannedPerMinute
    pub fn get_files_scanned_per_minute(&self) -> Option<&u64> {
        self.files_scanned_per_minute.as_ref()
    }

    /// Sets the value of GrovelerServiceRoutineExecutionsPerSecond
    pub fn set_groveler_service_routine_executions_per_second(&mut self, value: u64) {
        self.groveler_service_routine_executions_per_second = Some(value);
    }

    /// Gets the value of GrovelerServiceRoutineExecutionsPerSecond
    pub fn get_groveler_service_routine_executions_per_second(&self) -> Option<&u64> {
        self.groveler_service_routine_executions_per_second.as_ref()
    }

    /// Sets the value of LibraryDescriptionChangeNotificationsPerSecond
    pub fn set_library_description_change_notifications_per_second(&mut self, value: u64) {
        self.library_description_change_notifications_per_second = Some(value);
    }

    /// Gets the value of LibraryDescriptionChangeNotificationsPerSecond
    pub fn get_library_description_change_notifications_per_second(&self) -> Option<&u64> {
        self.library_description_change_notifications_per_second.as_ref()
    }

    /// Sets the value of LibraryDescriptionUpdatesPerSecond
    pub fn set_library_description_updates_per_second(&mut self, value: u64) {
        self.library_description_updates_per_second = Some(value);
    }

    /// Gets the value of LibraryDescriptionUpdatesPerSecond
    pub fn get_library_description_updates_per_second(&self) -> Option<&u64> {
        self.library_description_updates_per_second.as_ref()
    }

    /// Sets the value of MonitoredFolderUpdatesPerSecond
    pub fn set_monitored_folder_updates_per_second(&mut self, value: u64) {
        self.monitored_folder_updates_per_second = Some(value);
    }

    /// Gets the value of MonitoredFolderUpdatesPerSecond
    pub fn get_monitored_folder_updates_per_second(&self) -> Option<&u64> {
        self.monitored_folder_updates_per_second.as_ref()
    }

    /// Sets the value of NormalizationTimems
    pub fn set_normalization_timems(&mut self, value: u32) {
        self.normalization_timems = Some(value);
    }

    /// Gets the value of NormalizationTimems
    pub fn get_normalization_timems(&self) -> Option<&u32> {
        self.normalization_timems.as_ref()
    }

    /// Sets the value of PropertyExtractionTimems
    pub fn set_property_extraction_timems(&mut self, value: u32) {
        self.property_extraction_timems = Some(value);
    }

    /// Gets the value of PropertyExtractionTimems
    pub fn get_property_extraction_timems(&self) -> Option<&u32> {
        self.property_extraction_timems.as_ref()
    }

    /// Sets the value of ReorganizeTimems
    pub fn set_reorganize_timems(&mut self, value: u32) {
        self.reorganize_timems = Some(value);
    }

    /// Gets the value of ReorganizeTimems
    pub fn get_reorganize_timems(&self) -> Option<&u32> {
        self.reorganize_timems.as_ref()
    }

    /// Sets the value of ScanningState
    pub fn set_scanning_state(&mut self, value: u32) {
        self.scanning_state = Some(value);
    }

    /// Gets the value of ScanningState
    pub fn get_scanning_state(&self) -> Option<&u32> {
        self.scanning_state.as_ref()
    }

    /// Sets the value of TimestampDirectoryHitCount
    pub fn set_timestamp_directory_hit_count(&mut self, value: u32) {
        self.timestamp_directory_hit_count = Some(value);
    }

    /// Gets the value of TimestampDirectoryHitCount
    pub fn get_timestamp_directory_hit_count(&self) -> Option<&u32> {
        self.timestamp_directory_hit_count.as_ref()
    }

    /// Sets the value of URLClassificationTimems
    pub fn set_urlclassification_timems(&mut self, value: u32) {
        self.urlclassification_timems = Some(value);
    }

    /// Gets the value of URLClassificationTimems
    pub fn get_urlclassification_timems(&self) -> Option<&u32> {
        self.urlclassification_timems.as_ref()
    }
}

