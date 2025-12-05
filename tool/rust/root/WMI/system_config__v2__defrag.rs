// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V2_Defrag struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V2_Defrag {
    #[serde(flatten)]
    pub base: SystemConfig_V2,

/// 
    #[serde(rename = "AlignmentClusters")]
    pub alignment_clusters: Option<u64>,

/// 
    #[serde(rename = "AvgFragmentsPerFile")]
    pub avg_fragments_per_file: Option<u32>,

/// 
    #[serde(rename = "AvgFreeSpaceSize")]
    pub avg_free_space_size: Option<u64>,

/// 
    #[serde(rename = "BytesPerCluster")]
    pub bytes_per_cluster: Option<u32>,

/// 
    #[serde(rename = "ClustersPerSlab")]
    pub clusters_per_slab: Option<u64>,

/// 
    #[serde(rename = "DirectoryCount")]
    pub directory_count: Option<u32>,

/// 
    #[serde(rename = "FragmentedDirectories")]
    pub fragmented_directories: Option<u32>,

/// 
    #[serde(rename = "FragmentedDirectoryExtents")]
    pub fragmented_directory_extents: Option<u64>,

/// 
    #[serde(rename = "FragmentedExtents")]
    pub fragmented_extents: Option<u64>,

/// 
    #[serde(rename = "FragmentedFiles")]
    pub fragmented_files: Option<u32>,

/// 
    #[serde(rename = "FragmentedSpace")]
    pub fragmented_space: Option<u32>,

/// 
    #[serde(rename = "FreeSpaceCount")]
    pub free_space_count: Option<u64>,

/// 
    #[serde(rename = "HardwareIssue")]
    pub hardware_issue: Option<u32>,

/// 
    #[serde(rename = "InUseMFTRecords")]
    pub in_use_mftrecords: Option<u32>,

/// 
    #[serde(rename = "InUseSlabs")]
    pub in_use_slabs: Option<u32>,

/// 
    #[serde(rename = "LargestFreeSpaceSize")]
    pub largest_free_space_size: Option<u64>,

/// 
    #[serde(rename = "LastRunActualPurgeClusters")]
    pub last_run_actual_purge_clusters: Option<u64>,

/// 
    #[serde(rename = "LastRunActualPurgeSlabs")]
    pub last_run_actual_purge_slabs: Option<u32>,

/// 
    #[serde(rename = "LastRunClustersTrimmed")]
    pub last_run_clusters_trimmed: Option<u64>,

/// 
    #[serde(rename = "LastRunFullDefragTime")]
    pub last_run_full_defrag_time: Option<u64>,

/// 
    #[serde(rename = "LastRunInitialBackedSlabs")]
    pub last_run_initial_backed_slabs: Option<u32>,

/// 
    #[serde(rename = "LastRunPercentFragmentation")]
    pub last_run_percent_fragmentation: Option<u32>,

/// 
    #[serde(rename = "LastRunPinnedSlabs")]
    pub last_run_pinned_slabs: Option<u32>,

/// 
    #[serde(rename = "LastRunPotentialPurgeSlabs")]
    pub last_run_potential_purge_slabs: Option<u32>,

/// 
    #[serde(rename = "LastRunSpaceInefficientSlabs")]
    pub last_run_space_inefficient_slabs: Option<u32>,

/// 
    #[serde(rename = "LastRunTime")]
    pub last_run_time: Option<u64>,

/// 
    #[serde(rename = "LastRunTrimmedSlabs")]
    pub last_run_trimmed_slabs: Option<u32>,

/// 
    #[serde(rename = "LastRunUnknownEvictFailSlabs")]
    pub last_run_unknown_evict_fail_slabs: Option<u32>,

/// 
    #[serde(rename = "LastRunVolsnapPinnedSlabs")]
    pub last_run_volsnap_pinned_slabs: Option<u32>,

/// 
    #[serde(rename = "MFTFragmentCount")]
    pub mftfragment_count: Option<u32>,

/// 
    #[serde(rename = "MFTSize")]
    pub mftsize: Option<u64>,

/// 
    #[serde(rename = "MovableFiles")]
    pub movable_files: Option<u32>,

/// 
    #[serde(rename = "TotalClusters")]
    pub total_clusters: Option<u64>,

/// 
    #[serde(rename = "TotalMFTRecords")]
    pub total_mftrecords: Option<u32>,

/// 
    #[serde(rename = "TotalSlabs")]
    pub total_slabs: Option<u32>,

/// 
    #[serde(rename = "TotalUsedClusters")]
    pub total_used_clusters: Option<u64>,

/// 
    #[serde(rename = "UnmovableFiles")]
    pub unmovable_files: Option<u32>,

/// 
    #[serde(rename = "VolumeId")]
    pub volume_id: Option<serde_json::Value>,

/// 
    #[serde(rename = "VolumePathNames")]
    pub volume_path_names: Option<String>,
}

impl SystemConfig_V2_Defrag {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V2::new(),
            alignment_clusters: None,
            avg_fragments_per_file: None,
            avg_free_space_size: None,
            bytes_per_cluster: None,
            clusters_per_slab: None,
            directory_count: None,
            fragmented_directories: None,
            fragmented_directory_extents: None,
            fragmented_extents: None,
            fragmented_files: None,
            fragmented_space: None,
            free_space_count: None,
            hardware_issue: None,
            in_use_mftrecords: None,
            in_use_slabs: None,
            largest_free_space_size: None,
            last_run_actual_purge_clusters: None,
            last_run_actual_purge_slabs: None,
            last_run_clusters_trimmed: None,
            last_run_full_defrag_time: None,
            last_run_initial_backed_slabs: None,
            last_run_percent_fragmentation: None,
            last_run_pinned_slabs: None,
            last_run_potential_purge_slabs: None,
            last_run_space_inefficient_slabs: None,
            last_run_time: None,
            last_run_trimmed_slabs: None,
            last_run_unknown_evict_fail_slabs: None,
            last_run_volsnap_pinned_slabs: None,
            mftfragment_count: None,
            mftsize: None,
            movable_files: None,
            total_clusters: None,
            total_mftrecords: None,
            total_slabs: None,
            total_used_clusters: None,
            unmovable_files: None,
            volume_id: None,
            volume_path_names: None,
        }
    }


    /// Sets the value of AlignmentClusters
    pub fn set_alignment_clusters(&mut self, value: u64) {
        self.alignment_clusters = Some(value);
    }

    /// Gets the value of AlignmentClusters
    pub fn get_alignment_clusters(&self) -> Option<&u64> {
        self.alignment_clusters.as_ref()
    }

    /// Sets the value of AvgFragmentsPerFile
    pub fn set_avg_fragments_per_file(&mut self, value: u32) {
        self.avg_fragments_per_file = Some(value);
    }

    /// Gets the value of AvgFragmentsPerFile
    pub fn get_avg_fragments_per_file(&self) -> Option<&u32> {
        self.avg_fragments_per_file.as_ref()
    }

    /// Sets the value of AvgFreeSpaceSize
    pub fn set_avg_free_space_size(&mut self, value: u64) {
        self.avg_free_space_size = Some(value);
    }

    /// Gets the value of AvgFreeSpaceSize
    pub fn get_avg_free_space_size(&self) -> Option<&u64> {
        self.avg_free_space_size.as_ref()
    }

    /// Sets the value of BytesPerCluster
    pub fn set_bytes_per_cluster(&mut self, value: u32) {
        self.bytes_per_cluster = Some(value);
    }

    /// Gets the value of BytesPerCluster
    pub fn get_bytes_per_cluster(&self) -> Option<&u32> {
        self.bytes_per_cluster.as_ref()
    }

    /// Sets the value of ClustersPerSlab
    pub fn set_clusters_per_slab(&mut self, value: u64) {
        self.clusters_per_slab = Some(value);
    }

    /// Gets the value of ClustersPerSlab
    pub fn get_clusters_per_slab(&self) -> Option<&u64> {
        self.clusters_per_slab.as_ref()
    }

    /// Sets the value of DirectoryCount
    pub fn set_directory_count(&mut self, value: u32) {
        self.directory_count = Some(value);
    }

    /// Gets the value of DirectoryCount
    pub fn get_directory_count(&self) -> Option<&u32> {
        self.directory_count.as_ref()
    }

    /// Sets the value of FragmentedDirectories
    pub fn set_fragmented_directories(&mut self, value: u32) {
        self.fragmented_directories = Some(value);
    }

    /// Gets the value of FragmentedDirectories
    pub fn get_fragmented_directories(&self) -> Option<&u32> {
        self.fragmented_directories.as_ref()
    }

    /// Sets the value of FragmentedDirectoryExtents
    pub fn set_fragmented_directory_extents(&mut self, value: u64) {
        self.fragmented_directory_extents = Some(value);
    }

    /// Gets the value of FragmentedDirectoryExtents
    pub fn get_fragmented_directory_extents(&self) -> Option<&u64> {
        self.fragmented_directory_extents.as_ref()
    }

    /// Sets the value of FragmentedExtents
    pub fn set_fragmented_extents(&mut self, value: u64) {
        self.fragmented_extents = Some(value);
    }

    /// Gets the value of FragmentedExtents
    pub fn get_fragmented_extents(&self) -> Option<&u64> {
        self.fragmented_extents.as_ref()
    }

    /// Sets the value of FragmentedFiles
    pub fn set_fragmented_files(&mut self, value: u32) {
        self.fragmented_files = Some(value);
    }

    /// Gets the value of FragmentedFiles
    pub fn get_fragmented_files(&self) -> Option<&u32> {
        self.fragmented_files.as_ref()
    }

    /// Sets the value of FragmentedSpace
    pub fn set_fragmented_space(&mut self, value: u32) {
        self.fragmented_space = Some(value);
    }

    /// Gets the value of FragmentedSpace
    pub fn get_fragmented_space(&self) -> Option<&u32> {
        self.fragmented_space.as_ref()
    }

    /// Sets the value of FreeSpaceCount
    pub fn set_free_space_count(&mut self, value: u64) {
        self.free_space_count = Some(value);
    }

    /// Gets the value of FreeSpaceCount
    pub fn get_free_space_count(&self) -> Option<&u64> {
        self.free_space_count.as_ref()
    }

    /// Sets the value of HardwareIssue
    pub fn set_hardware_issue(&mut self, value: u32) {
        self.hardware_issue = Some(value);
    }

    /// Gets the value of HardwareIssue
    pub fn get_hardware_issue(&self) -> Option<&u32> {
        self.hardware_issue.as_ref()
    }

    /// Sets the value of InUseMFTRecords
    pub fn set_in_use_mftrecords(&mut self, value: u32) {
        self.in_use_mftrecords = Some(value);
    }

    /// Gets the value of InUseMFTRecords
    pub fn get_in_use_mftrecords(&self) -> Option<&u32> {
        self.in_use_mftrecords.as_ref()
    }

    /// Sets the value of InUseSlabs
    pub fn set_in_use_slabs(&mut self, value: u32) {
        self.in_use_slabs = Some(value);
    }

    /// Gets the value of InUseSlabs
    pub fn get_in_use_slabs(&self) -> Option<&u32> {
        self.in_use_slabs.as_ref()
    }

    /// Sets the value of LargestFreeSpaceSize
    pub fn set_largest_free_space_size(&mut self, value: u64) {
        self.largest_free_space_size = Some(value);
    }

    /// Gets the value of LargestFreeSpaceSize
    pub fn get_largest_free_space_size(&self) -> Option<&u64> {
        self.largest_free_space_size.as_ref()
    }

    /// Sets the value of LastRunActualPurgeClusters
    pub fn set_last_run_actual_purge_clusters(&mut self, value: u64) {
        self.last_run_actual_purge_clusters = Some(value);
    }

    /// Gets the value of LastRunActualPurgeClusters
    pub fn get_last_run_actual_purge_clusters(&self) -> Option<&u64> {
        self.last_run_actual_purge_clusters.as_ref()
    }

    /// Sets the value of LastRunActualPurgeSlabs
    pub fn set_last_run_actual_purge_slabs(&mut self, value: u32) {
        self.last_run_actual_purge_slabs = Some(value);
    }

    /// Gets the value of LastRunActualPurgeSlabs
    pub fn get_last_run_actual_purge_slabs(&self) -> Option<&u32> {
        self.last_run_actual_purge_slabs.as_ref()
    }

    /// Sets the value of LastRunClustersTrimmed
    pub fn set_last_run_clusters_trimmed(&mut self, value: u64) {
        self.last_run_clusters_trimmed = Some(value);
    }

    /// Gets the value of LastRunClustersTrimmed
    pub fn get_last_run_clusters_trimmed(&self) -> Option<&u64> {
        self.last_run_clusters_trimmed.as_ref()
    }

    /// Sets the value of LastRunFullDefragTime
    pub fn set_last_run_full_defrag_time(&mut self, value: u64) {
        self.last_run_full_defrag_time = Some(value);
    }

    /// Gets the value of LastRunFullDefragTime
    pub fn get_last_run_full_defrag_time(&self) -> Option<&u64> {
        self.last_run_full_defrag_time.as_ref()
    }

    /// Sets the value of LastRunInitialBackedSlabs
    pub fn set_last_run_initial_backed_slabs(&mut self, value: u32) {
        self.last_run_initial_backed_slabs = Some(value);
    }

    /// Gets the value of LastRunInitialBackedSlabs
    pub fn get_last_run_initial_backed_slabs(&self) -> Option<&u32> {
        self.last_run_initial_backed_slabs.as_ref()
    }

    /// Sets the value of LastRunPercentFragmentation
    pub fn set_last_run_percent_fragmentation(&mut self, value: u32) {
        self.last_run_percent_fragmentation = Some(value);
    }

    /// Gets the value of LastRunPercentFragmentation
    pub fn get_last_run_percent_fragmentation(&self) -> Option<&u32> {
        self.last_run_percent_fragmentation.as_ref()
    }

    /// Sets the value of LastRunPinnedSlabs
    pub fn set_last_run_pinned_slabs(&mut self, value: u32) {
        self.last_run_pinned_slabs = Some(value);
    }

    /// Gets the value of LastRunPinnedSlabs
    pub fn get_last_run_pinned_slabs(&self) -> Option<&u32> {
        self.last_run_pinned_slabs.as_ref()
    }

    /// Sets the value of LastRunPotentialPurgeSlabs
    pub fn set_last_run_potential_purge_slabs(&mut self, value: u32) {
        self.last_run_potential_purge_slabs = Some(value);
    }

    /// Gets the value of LastRunPotentialPurgeSlabs
    pub fn get_last_run_potential_purge_slabs(&self) -> Option<&u32> {
        self.last_run_potential_purge_slabs.as_ref()
    }

    /// Sets the value of LastRunSpaceInefficientSlabs
    pub fn set_last_run_space_inefficient_slabs(&mut self, value: u32) {
        self.last_run_space_inefficient_slabs = Some(value);
    }

    /// Gets the value of LastRunSpaceInefficientSlabs
    pub fn get_last_run_space_inefficient_slabs(&self) -> Option<&u32> {
        self.last_run_space_inefficient_slabs.as_ref()
    }

    /// Sets the value of LastRunTime
    pub fn set_last_run_time(&mut self, value: u64) {
        self.last_run_time = Some(value);
    }

    /// Gets the value of LastRunTime
    pub fn get_last_run_time(&self) -> Option<&u64> {
        self.last_run_time.as_ref()
    }

    /// Sets the value of LastRunTrimmedSlabs
    pub fn set_last_run_trimmed_slabs(&mut self, value: u32) {
        self.last_run_trimmed_slabs = Some(value);
    }

    /// Gets the value of LastRunTrimmedSlabs
    pub fn get_last_run_trimmed_slabs(&self) -> Option<&u32> {
        self.last_run_trimmed_slabs.as_ref()
    }

    /// Sets the value of LastRunUnknownEvictFailSlabs
    pub fn set_last_run_unknown_evict_fail_slabs(&mut self, value: u32) {
        self.last_run_unknown_evict_fail_slabs = Some(value);
    }

    /// Gets the value of LastRunUnknownEvictFailSlabs
    pub fn get_last_run_unknown_evict_fail_slabs(&self) -> Option<&u32> {
        self.last_run_unknown_evict_fail_slabs.as_ref()
    }

    /// Sets the value of LastRunVolsnapPinnedSlabs
    pub fn set_last_run_volsnap_pinned_slabs(&mut self, value: u32) {
        self.last_run_volsnap_pinned_slabs = Some(value);
    }

    /// Gets the value of LastRunVolsnapPinnedSlabs
    pub fn get_last_run_volsnap_pinned_slabs(&self) -> Option<&u32> {
        self.last_run_volsnap_pinned_slabs.as_ref()
    }

    /// Sets the value of MFTFragmentCount
    pub fn set_mftfragment_count(&mut self, value: u32) {
        self.mftfragment_count = Some(value);
    }

    /// Gets the value of MFTFragmentCount
    pub fn get_mftfragment_count(&self) -> Option<&u32> {
        self.mftfragment_count.as_ref()
    }

    /// Sets the value of MFTSize
    pub fn set_mftsize(&mut self, value: u64) {
        self.mftsize = Some(value);
    }

    /// Gets the value of MFTSize
    pub fn get_mftsize(&self) -> Option<&u64> {
        self.mftsize.as_ref()
    }

    /// Sets the value of MovableFiles
    pub fn set_movable_files(&mut self, value: u32) {
        self.movable_files = Some(value);
    }

    /// Gets the value of MovableFiles
    pub fn get_movable_files(&self) -> Option<&u32> {
        self.movable_files.as_ref()
    }

    /// Sets the value of TotalClusters
    pub fn set_total_clusters(&mut self, value: u64) {
        self.total_clusters = Some(value);
    }

    /// Gets the value of TotalClusters
    pub fn get_total_clusters(&self) -> Option<&u64> {
        self.total_clusters.as_ref()
    }

    /// Sets the value of TotalMFTRecords
    pub fn set_total_mftrecords(&mut self, value: u32) {
        self.total_mftrecords = Some(value);
    }

    /// Gets the value of TotalMFTRecords
    pub fn get_total_mftrecords(&self) -> Option<&u32> {
        self.total_mftrecords.as_ref()
    }

    /// Sets the value of TotalSlabs
    pub fn set_total_slabs(&mut self, value: u32) {
        self.total_slabs = Some(value);
    }

    /// Gets the value of TotalSlabs
    pub fn get_total_slabs(&self) -> Option<&u32> {
        self.total_slabs.as_ref()
    }

    /// Sets the value of TotalUsedClusters
    pub fn set_total_used_clusters(&mut self, value: u64) {
        self.total_used_clusters = Some(value);
    }

    /// Gets the value of TotalUsedClusters
    pub fn get_total_used_clusters(&self) -> Option<&u64> {
        self.total_used_clusters.as_ref()
    }

    /// Sets the value of UnmovableFiles
    pub fn set_unmovable_files(&mut self, value: u32) {
        self.unmovable_files = Some(value);
    }

    /// Gets the value of UnmovableFiles
    pub fn get_unmovable_files(&self) -> Option<&u32> {
        self.unmovable_files.as_ref()
    }

    /// Sets the value of VolumeId
    pub fn set_volume_id(&mut self, value: serde_json::Value) {
        self.volume_id = Some(value);
    }

    /// Gets the value of VolumeId
    pub fn get_volume_id(&self) -> Option<&serde_json::Value> {
        self.volume_id.as_ref()
    }

    /// Sets the value of VolumePathNames
    pub fn set_volume_path_names(&mut self, value: String) {
        self.volume_path_names = Some(value);
    }

    /// Gets the value of VolumePathNames
    pub fn get_volume_path_names(&self) -> Option<&String> {
        self.volume_path_names.as_ref()
    }
}

