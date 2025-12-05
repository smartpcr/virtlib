// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_DefragAnalysis struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_DefragAnalysis {

/// 
    #[serde(rename = "AverageFileSize")]
    pub average_file_size: Option<u64>,

/// 
    #[serde(rename = "AverageFragmentsPerFile")]
    pub average_fragments_per_file: Option<f64>,

/// 
    #[serde(rename = "AverageFreeSpacePerExtent")]
    pub average_free_space_per_extent: Option<u64>,

/// 
    #[serde(rename = "ClusterSize")]
    pub cluster_size: Option<u64>,

/// 
    #[serde(rename = "ExcessFolderFragments")]
    pub excess_folder_fragments: Option<u64>,

/// 
    #[serde(rename = "FilePercentFragmentation")]
    pub file_percent_fragmentation: Option<u32>,

/// 
    #[serde(rename = "FragmentedFolders")]
    pub fragmented_folders: Option<u64>,

/// 
    #[serde(rename = "FreeSpace")]
    pub free_space: Option<u64>,

/// 
    #[serde(rename = "FreeSpacePercent")]
    pub free_space_percent: Option<u32>,

/// 
    #[serde(rename = "FreeSpacePercentFragmentation")]
    pub free_space_percent_fragmentation: Option<u32>,

/// 
    #[serde(rename = "LargestFreeSpaceExtent")]
    pub largest_free_space_extent: Option<u64>,

/// 
    #[serde(rename = "MFTPercentInUse")]
    pub mftpercent_in_use: Option<u32>,

/// 
    #[serde(rename = "MFTRecordCount")]
    pub mftrecord_count: Option<u64>,

/// 
    #[serde(rename = "PageFileSize")]
    pub page_file_size: Option<u64>,

/// 
    #[serde(rename = "TotalExcessFragments")]
    pub total_excess_fragments: Option<u64>,

/// 
    #[serde(rename = "TotalFiles")]
    pub total_files: Option<u64>,

/// 
    #[serde(rename = "TotalFolders")]
    pub total_folders: Option<u64>,

/// 
    #[serde(rename = "TotalFragmentedFiles")]
    pub total_fragmented_files: Option<u64>,

/// 
    #[serde(rename = "TotalFreeSpaceExtents")]
    pub total_free_space_extents: Option<u64>,

/// 
    #[serde(rename = "TotalMFTFragments")]
    pub total_mftfragments: Option<u64>,

/// 
    #[serde(rename = "TotalMFTSize")]
    pub total_mftsize: Option<u64>,

/// 
    #[serde(rename = "TotalPageFileFragments")]
    pub total_page_file_fragments: Option<u64>,

/// 
    #[serde(rename = "TotalPercentFragmentation")]
    pub total_percent_fragmentation: Option<u32>,

/// 
    #[serde(rename = "TotalUnmovableFiles")]
    pub total_unmovable_files: Option<u64>,

/// 
    #[serde(rename = "UsedSpace")]
    pub used_space: Option<u64>,

/// 
    #[serde(rename = "VolumeName")]
    pub volume_name: Option<String>,

/// 
    #[serde(rename = "VolumeSize")]
    pub volume_size: Option<u64>,
}

impl Win32_DefragAnalysis {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            average_file_size: None,
            average_fragments_per_file: None,
            average_free_space_per_extent: None,
            cluster_size: None,
            excess_folder_fragments: None,
            file_percent_fragmentation: None,
            fragmented_folders: None,
            free_space: None,
            free_space_percent: None,
            free_space_percent_fragmentation: None,
            largest_free_space_extent: None,
            mftpercent_in_use: None,
            mftrecord_count: None,
            page_file_size: None,
            total_excess_fragments: None,
            total_files: None,
            total_folders: None,
            total_fragmented_files: None,
            total_free_space_extents: None,
            total_mftfragments: None,
            total_mftsize: None,
            total_page_file_fragments: None,
            total_percent_fragmentation: None,
            total_unmovable_files: None,
            used_space: None,
            volume_name: None,
            volume_size: None,
        }
    }


    /// Sets the value of AverageFileSize
    pub fn set_average_file_size(&mut self, value: u64) {
        self.average_file_size = Some(value);
    }

    /// Gets the value of AverageFileSize
    pub fn get_average_file_size(&self) -> Option<&u64> {
        self.average_file_size.as_ref()
    }

    /// Sets the value of AverageFragmentsPerFile
    pub fn set_average_fragments_per_file(&mut self, value: f64) {
        self.average_fragments_per_file = Some(value);
    }

    /// Gets the value of AverageFragmentsPerFile
    pub fn get_average_fragments_per_file(&self) -> Option<&f64> {
        self.average_fragments_per_file.as_ref()
    }

    /// Sets the value of AverageFreeSpacePerExtent
    pub fn set_average_free_space_per_extent(&mut self, value: u64) {
        self.average_free_space_per_extent = Some(value);
    }

    /// Gets the value of AverageFreeSpacePerExtent
    pub fn get_average_free_space_per_extent(&self) -> Option<&u64> {
        self.average_free_space_per_extent.as_ref()
    }

    /// Sets the value of ClusterSize
    pub fn set_cluster_size(&mut self, value: u64) {
        self.cluster_size = Some(value);
    }

    /// Gets the value of ClusterSize
    pub fn get_cluster_size(&self) -> Option<&u64> {
        self.cluster_size.as_ref()
    }

    /// Sets the value of ExcessFolderFragments
    pub fn set_excess_folder_fragments(&mut self, value: u64) {
        self.excess_folder_fragments = Some(value);
    }

    /// Gets the value of ExcessFolderFragments
    pub fn get_excess_folder_fragments(&self) -> Option<&u64> {
        self.excess_folder_fragments.as_ref()
    }

    /// Sets the value of FilePercentFragmentation
    pub fn set_file_percent_fragmentation(&mut self, value: u32) {
        self.file_percent_fragmentation = Some(value);
    }

    /// Gets the value of FilePercentFragmentation
    pub fn get_file_percent_fragmentation(&self) -> Option<&u32> {
        self.file_percent_fragmentation.as_ref()
    }

    /// Sets the value of FragmentedFolders
    pub fn set_fragmented_folders(&mut self, value: u64) {
        self.fragmented_folders = Some(value);
    }

    /// Gets the value of FragmentedFolders
    pub fn get_fragmented_folders(&self) -> Option<&u64> {
        self.fragmented_folders.as_ref()
    }

    /// Sets the value of FreeSpace
    pub fn set_free_space(&mut self, value: u64) {
        self.free_space = Some(value);
    }

    /// Gets the value of FreeSpace
    pub fn get_free_space(&self) -> Option<&u64> {
        self.free_space.as_ref()
    }

    /// Sets the value of FreeSpacePercent
    pub fn set_free_space_percent(&mut self, value: u32) {
        self.free_space_percent = Some(value);
    }

    /// Gets the value of FreeSpacePercent
    pub fn get_free_space_percent(&self) -> Option<&u32> {
        self.free_space_percent.as_ref()
    }

    /// Sets the value of FreeSpacePercentFragmentation
    pub fn set_free_space_percent_fragmentation(&mut self, value: u32) {
        self.free_space_percent_fragmentation = Some(value);
    }

    /// Gets the value of FreeSpacePercentFragmentation
    pub fn get_free_space_percent_fragmentation(&self) -> Option<&u32> {
        self.free_space_percent_fragmentation.as_ref()
    }

    /// Sets the value of LargestFreeSpaceExtent
    pub fn set_largest_free_space_extent(&mut self, value: u64) {
        self.largest_free_space_extent = Some(value);
    }

    /// Gets the value of LargestFreeSpaceExtent
    pub fn get_largest_free_space_extent(&self) -> Option<&u64> {
        self.largest_free_space_extent.as_ref()
    }

    /// Sets the value of MFTPercentInUse
    pub fn set_mftpercent_in_use(&mut self, value: u32) {
        self.mftpercent_in_use = Some(value);
    }

    /// Gets the value of MFTPercentInUse
    pub fn get_mftpercent_in_use(&self) -> Option<&u32> {
        self.mftpercent_in_use.as_ref()
    }

    /// Sets the value of MFTRecordCount
    pub fn set_mftrecord_count(&mut self, value: u64) {
        self.mftrecord_count = Some(value);
    }

    /// Gets the value of MFTRecordCount
    pub fn get_mftrecord_count(&self) -> Option<&u64> {
        self.mftrecord_count.as_ref()
    }

    /// Sets the value of PageFileSize
    pub fn set_page_file_size(&mut self, value: u64) {
        self.page_file_size = Some(value);
    }

    /// Gets the value of PageFileSize
    pub fn get_page_file_size(&self) -> Option<&u64> {
        self.page_file_size.as_ref()
    }

    /// Sets the value of TotalExcessFragments
    pub fn set_total_excess_fragments(&mut self, value: u64) {
        self.total_excess_fragments = Some(value);
    }

    /// Gets the value of TotalExcessFragments
    pub fn get_total_excess_fragments(&self) -> Option<&u64> {
        self.total_excess_fragments.as_ref()
    }

    /// Sets the value of TotalFiles
    pub fn set_total_files(&mut self, value: u64) {
        self.total_files = Some(value);
    }

    /// Gets the value of TotalFiles
    pub fn get_total_files(&self) -> Option<&u64> {
        self.total_files.as_ref()
    }

    /// Sets the value of TotalFolders
    pub fn set_total_folders(&mut self, value: u64) {
        self.total_folders = Some(value);
    }

    /// Gets the value of TotalFolders
    pub fn get_total_folders(&self) -> Option<&u64> {
        self.total_folders.as_ref()
    }

    /// Sets the value of TotalFragmentedFiles
    pub fn set_total_fragmented_files(&mut self, value: u64) {
        self.total_fragmented_files = Some(value);
    }

    /// Gets the value of TotalFragmentedFiles
    pub fn get_total_fragmented_files(&self) -> Option<&u64> {
        self.total_fragmented_files.as_ref()
    }

    /// Sets the value of TotalFreeSpaceExtents
    pub fn set_total_free_space_extents(&mut self, value: u64) {
        self.total_free_space_extents = Some(value);
    }

    /// Gets the value of TotalFreeSpaceExtents
    pub fn get_total_free_space_extents(&self) -> Option<&u64> {
        self.total_free_space_extents.as_ref()
    }

    /// Sets the value of TotalMFTFragments
    pub fn set_total_mftfragments(&mut self, value: u64) {
        self.total_mftfragments = Some(value);
    }

    /// Gets the value of TotalMFTFragments
    pub fn get_total_mftfragments(&self) -> Option<&u64> {
        self.total_mftfragments.as_ref()
    }

    /// Sets the value of TotalMFTSize
    pub fn set_total_mftsize(&mut self, value: u64) {
        self.total_mftsize = Some(value);
    }

    /// Gets the value of TotalMFTSize
    pub fn get_total_mftsize(&self) -> Option<&u64> {
        self.total_mftsize.as_ref()
    }

    /// Sets the value of TotalPageFileFragments
    pub fn set_total_page_file_fragments(&mut self, value: u64) {
        self.total_page_file_fragments = Some(value);
    }

    /// Gets the value of TotalPageFileFragments
    pub fn get_total_page_file_fragments(&self) -> Option<&u64> {
        self.total_page_file_fragments.as_ref()
    }

    /// Sets the value of TotalPercentFragmentation
    pub fn set_total_percent_fragmentation(&mut self, value: u32) {
        self.total_percent_fragmentation = Some(value);
    }

    /// Gets the value of TotalPercentFragmentation
    pub fn get_total_percent_fragmentation(&self) -> Option<&u32> {
        self.total_percent_fragmentation.as_ref()
    }

    /// Sets the value of TotalUnmovableFiles
    pub fn set_total_unmovable_files(&mut self, value: u64) {
        self.total_unmovable_files = Some(value);
    }

    /// Gets the value of TotalUnmovableFiles
    pub fn get_total_unmovable_files(&self) -> Option<&u64> {
        self.total_unmovable_files.as_ref()
    }

    /// Sets the value of UsedSpace
    pub fn set_used_space(&mut self, value: u64) {
        self.used_space = Some(value);
    }

    /// Gets the value of UsedSpace
    pub fn get_used_space(&self) -> Option<&u64> {
        self.used_space.as_ref()
    }

    /// Sets the value of VolumeName
    pub fn set_volume_name(&mut self, value: String) {
        self.volume_name = Some(value);
    }

    /// Gets the value of VolumeName
    pub fn get_volume_name(&self) -> Option<&String> {
        self.volume_name.as_ref()
    }

    /// Sets the value of VolumeSize
    pub fn set_volume_size(&mut self, value: u64) {
        self.volume_size = Some(value);
    }

    /// Gets the value of VolumeSize
    pub fn get_volume_size(&self) -> Option<&u64> {
        self.volume_size.as_ref()
    }
}

