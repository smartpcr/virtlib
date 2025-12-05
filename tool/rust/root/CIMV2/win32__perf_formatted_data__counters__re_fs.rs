// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_ReFS struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_ReFS {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "AllocationofDataClustersonFastTierPersec")]
    pub allocationof_data_clusterson_fast_tier_persec: Option<u64>,

/// 
    #[serde(rename = "AllocationofDataClustersonSlowTierPersec")]
    pub allocationof_data_clusterson_slow_tier_persec: Option<u64>,

/// 
    #[serde(rename = "AllocationofMetadataClustersonFastTierPersec")]
    pub allocationof_metadata_clusterson_fast_tier_persec: Option<u64>,

/// 
    #[serde(rename = "AllocationofMetadataClustersonSlowTierPersec")]
    pub allocationof_metadata_clusterson_slow_tier_persec: Option<u64>,

/// 
    #[serde(rename = "AllocatorCacheCollisions")]
    pub allocator_cache_collisions: Option<u32>,

/// 
    #[serde(rename = "AllocatorCacheDeletions")]
    pub allocator_cache_deletions: Option<u32>,

/// 
    #[serde(rename = "AllocatorCacheInsertions")]
    pub allocator_cache_insertions: Option<u32>,

/// 
    #[serde(rename = "AllocatorCachePurges")]
    pub allocator_cache_purges: Option<u32>,

/// 
    #[serde(rename = "AllocatorCacheSize")]
    pub allocator_cache_size: Option<u32>,

/// 
    #[serde(rename = "AllocatorRegionsExaminedAverage")]
    pub allocator_regions_examined_average: Option<u64>,

/// 
    #[serde(rename = "AllocatorRegionsExaminedMax")]
    pub allocator_regions_examined_max: Option<u32>,

/// 
    #[serde(rename = "Checkpointastreeupdatelatency100ns")]
    pub checkpointastreeupdatelatency100ns: Option<u64>,

/// 
    #[serde(rename = "Checkpointexclusivelockslatency100ns")]
    pub checkpointexclusivelockslatency100ns: Option<u64>,

/// 
    #[serde(rename = "Checkpointlatency100ns")]
    pub checkpointlatency100ns: Option<u64>,

/// 
    #[serde(rename = "CheckpointsPersec")]
    pub checkpoints_persec: Option<u64>,

/// 
    #[serde(rename = "CompactedContainerFillRatioPercent")]
    pub compacted_container_fill_ratio_percent: Option<u64>,

/// 
    #[serde(rename = "CompactionFailureCount")]
    pub compaction_failure_count: Option<u32>,

/// 
    #[serde(rename = "Compactionreadlatency100ns")]
    pub compactionreadlatency100ns: Option<u64>,

/// 
    #[serde(rename = "Compactionsfailedduetoineligiblecontainer")]
    pub compactionsfailedduetoineligiblecontainer: Option<u32>,

/// 
    #[serde(rename = "Compactionsfailedduetomaxfragmentation")]
    pub compactionsfailedduetomaxfragmentation: Option<u32>,

/// 
    #[serde(rename = "Compactionwritelatency100ns")]
    pub compactionwritelatency100ns: Option<u64>,

/// 
    #[serde(rename = "ContainerDestagesFromFastTierPersec")]
    pub container_destages_from_fast_tier_persec: Option<u64>,

/// 
    #[serde(rename = "ContainerDestagesFromSlowTierPersec")]
    pub container_destages_from_slow_tier_persec: Option<u64>,

/// 
    #[serde(rename = "ContainerMoveFailureCount")]
    pub container_move_failure_count: Option<u32>,

/// 
    #[serde(rename = "ContainerMoveRetryCount")]
    pub container_move_retry_count: Option<u32>,

/// 
    #[serde(rename = "Containermovesfailedduetoineligiblecontainer")]
    pub containermovesfailedduetoineligiblecontainer: Option<u32>,

/// 
    #[serde(rename = "CurrentFastTierDataFillPercentage")]
    pub current_fast_tier_data_fill_percentage: Option<u32>,

/// 
    #[serde(rename = "CurrentFastTierMetadataFillPercentage")]
    pub current_fast_tier_metadata_fill_percentage: Option<u32>,

/// 
    #[serde(rename = "CurrentSlowTierDataFillPercentage")]
    pub current_slow_tier_data_fill_percentage: Option<u32>,

/// 
    #[serde(rename = "CurrentSlowTierMetadataFillPercentage")]
    pub current_slow_tier_metadata_fill_percentage: Option<u32>,

/// 
    #[serde(rename = "DataCompactionsPersec")]
    pub data_compactions_persec: Option<u64>,

/// 
    #[serde(rename = "DataInPlaceWriteClustersPersec")]
    pub data_in_place_write_clusters_persec: Option<u64>,

/// 
    #[serde(rename = "DeleteQueueentries")]
    pub delete_queueentries: Option<u32>,

/// 
    #[serde(rename = "Dirtymetadatapages")]
    pub dirtymetadatapages: Option<u64>,

/// 
    #[serde(rename = "Dirtytablelistentries")]
    pub dirtytablelistentries: Option<u32>,

/// 
    #[serde(rename = "FastTierDestagedContainerFillRatioPercent")]
    pub fast_tier_destaged_container_fill_ratio_percent: Option<u64>,

/// 
    #[serde(rename = "Fasttierdestagereadlatency100ns")]
    pub fasttierdestagereadlatency100ns: Option<u64>,

/// 
    #[serde(rename = "Fasttierdestagewritelatency100ns")]
    pub fasttierdestagewritelatency100ns: Option<u64>,

/// 
    #[serde(rename = "Logfillpercentage")]
    pub logfillpercentage: Option<u32>,

/// 
    #[serde(rename = "LogwritesPersec")]
    pub logwrites_persec: Option<u64>,

/// 
    #[serde(rename = "SlowTierDestagedContainerFillRatioPercent")]
    pub slow_tier_destaged_container_fill_ratio_percent: Option<u64>,

/// 
    #[serde(rename = "Slowtierdestagereadlatency100ns")]
    pub slowtierdestagereadlatency100ns: Option<u64>,

/// 
    #[serde(rename = "Slowtierdestagewritelatency100ns")]
    pub slowtierdestagewritelatency100ns: Option<u64>,

/// 
    #[serde(rename = "TotalAllocationofClustersPersec")]
    pub total_allocationof_clusters_persec: Option<u64>,

/// 
    #[serde(rename = "Treeupdateexclusivelockslatency100ns")]
    pub treeupdateexclusivelockslatency100ns: Option<u64>,

/// 
    #[serde(rename = "Treeupdatelatency100ns")]
    pub treeupdatelatency100ns: Option<u64>,

/// 
    #[serde(rename = "TreeupdatesPersec")]
    pub treeupdates_persec: Option<u64>,

/// 
    #[serde(rename = "Trimlatency100ns")]
    pub trimlatency100ns: Option<u64>,
}

impl Win32_PerfFormattedData_Counters_ReFS {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            allocationof_data_clusterson_fast_tier_persec: None,
            allocationof_data_clusterson_slow_tier_persec: None,
            allocationof_metadata_clusterson_fast_tier_persec: None,
            allocationof_metadata_clusterson_slow_tier_persec: None,
            allocator_cache_collisions: None,
            allocator_cache_deletions: None,
            allocator_cache_insertions: None,
            allocator_cache_purges: None,
            allocator_cache_size: None,
            allocator_regions_examined_average: None,
            allocator_regions_examined_max: None,
            checkpointastreeupdatelatency100ns: None,
            checkpointexclusivelockslatency100ns: None,
            checkpointlatency100ns: None,
            checkpoints_persec: None,
            compacted_container_fill_ratio_percent: None,
            compaction_failure_count: None,
            compactionreadlatency100ns: None,
            compactionsfailedduetoineligiblecontainer: None,
            compactionsfailedduetomaxfragmentation: None,
            compactionwritelatency100ns: None,
            container_destages_from_fast_tier_persec: None,
            container_destages_from_slow_tier_persec: None,
            container_move_failure_count: None,
            container_move_retry_count: None,
            containermovesfailedduetoineligiblecontainer: None,
            current_fast_tier_data_fill_percentage: None,
            current_fast_tier_metadata_fill_percentage: None,
            current_slow_tier_data_fill_percentage: None,
            current_slow_tier_metadata_fill_percentage: None,
            data_compactions_persec: None,
            data_in_place_write_clusters_persec: None,
            delete_queueentries: None,
            dirtymetadatapages: None,
            dirtytablelistentries: None,
            fast_tier_destaged_container_fill_ratio_percent: None,
            fasttierdestagereadlatency100ns: None,
            fasttierdestagewritelatency100ns: None,
            logfillpercentage: None,
            logwrites_persec: None,
            slow_tier_destaged_container_fill_ratio_percent: None,
            slowtierdestagereadlatency100ns: None,
            slowtierdestagewritelatency100ns: None,
            total_allocationof_clusters_persec: None,
            treeupdateexclusivelockslatency100ns: None,
            treeupdatelatency100ns: None,
            treeupdates_persec: None,
            trimlatency100ns: None,
        }
    }


    /// Sets the value of AllocationofDataClustersonFastTierPersec
    pub fn set_allocationof_data_clusterson_fast_tier_persec(&mut self, value: u64) {
        self.allocationof_data_clusterson_fast_tier_persec = Some(value);
    }

    /// Gets the value of AllocationofDataClustersonFastTierPersec
    pub fn get_allocationof_data_clusterson_fast_tier_persec(&self) -> Option<&u64> {
        self.allocationof_data_clusterson_fast_tier_persec.as_ref()
    }

    /// Sets the value of AllocationofDataClustersonSlowTierPersec
    pub fn set_allocationof_data_clusterson_slow_tier_persec(&mut self, value: u64) {
        self.allocationof_data_clusterson_slow_tier_persec = Some(value);
    }

    /// Gets the value of AllocationofDataClustersonSlowTierPersec
    pub fn get_allocationof_data_clusterson_slow_tier_persec(&self) -> Option<&u64> {
        self.allocationof_data_clusterson_slow_tier_persec.as_ref()
    }

    /// Sets the value of AllocationofMetadataClustersonFastTierPersec
    pub fn set_allocationof_metadata_clusterson_fast_tier_persec(&mut self, value: u64) {
        self.allocationof_metadata_clusterson_fast_tier_persec = Some(value);
    }

    /// Gets the value of AllocationofMetadataClustersonFastTierPersec
    pub fn get_allocationof_metadata_clusterson_fast_tier_persec(&self) -> Option<&u64> {
        self.allocationof_metadata_clusterson_fast_tier_persec.as_ref()
    }

    /// Sets the value of AllocationofMetadataClustersonSlowTierPersec
    pub fn set_allocationof_metadata_clusterson_slow_tier_persec(&mut self, value: u64) {
        self.allocationof_metadata_clusterson_slow_tier_persec = Some(value);
    }

    /// Gets the value of AllocationofMetadataClustersonSlowTierPersec
    pub fn get_allocationof_metadata_clusterson_slow_tier_persec(&self) -> Option<&u64> {
        self.allocationof_metadata_clusterson_slow_tier_persec.as_ref()
    }

    /// Sets the value of AllocatorCacheCollisions
    pub fn set_allocator_cache_collisions(&mut self, value: u32) {
        self.allocator_cache_collisions = Some(value);
    }

    /// Gets the value of AllocatorCacheCollisions
    pub fn get_allocator_cache_collisions(&self) -> Option<&u32> {
        self.allocator_cache_collisions.as_ref()
    }

    /// Sets the value of AllocatorCacheDeletions
    pub fn set_allocator_cache_deletions(&mut self, value: u32) {
        self.allocator_cache_deletions = Some(value);
    }

    /// Gets the value of AllocatorCacheDeletions
    pub fn get_allocator_cache_deletions(&self) -> Option<&u32> {
        self.allocator_cache_deletions.as_ref()
    }

    /// Sets the value of AllocatorCacheInsertions
    pub fn set_allocator_cache_insertions(&mut self, value: u32) {
        self.allocator_cache_insertions = Some(value);
    }

    /// Gets the value of AllocatorCacheInsertions
    pub fn get_allocator_cache_insertions(&self) -> Option<&u32> {
        self.allocator_cache_insertions.as_ref()
    }

    /// Sets the value of AllocatorCachePurges
    pub fn set_allocator_cache_purges(&mut self, value: u32) {
        self.allocator_cache_purges = Some(value);
    }

    /// Gets the value of AllocatorCachePurges
    pub fn get_allocator_cache_purges(&self) -> Option<&u32> {
        self.allocator_cache_purges.as_ref()
    }

    /// Sets the value of AllocatorCacheSize
    pub fn set_allocator_cache_size(&mut self, value: u32) {
        self.allocator_cache_size = Some(value);
    }

    /// Gets the value of AllocatorCacheSize
    pub fn get_allocator_cache_size(&self) -> Option<&u32> {
        self.allocator_cache_size.as_ref()
    }

    /// Sets the value of AllocatorRegionsExaminedAverage
    pub fn set_allocator_regions_examined_average(&mut self, value: u64) {
        self.allocator_regions_examined_average = Some(value);
    }

    /// Gets the value of AllocatorRegionsExaminedAverage
    pub fn get_allocator_regions_examined_average(&self) -> Option<&u64> {
        self.allocator_regions_examined_average.as_ref()
    }

    /// Sets the value of AllocatorRegionsExaminedMax
    pub fn set_allocator_regions_examined_max(&mut self, value: u32) {
        self.allocator_regions_examined_max = Some(value);
    }

    /// Gets the value of AllocatorRegionsExaminedMax
    pub fn get_allocator_regions_examined_max(&self) -> Option<&u32> {
        self.allocator_regions_examined_max.as_ref()
    }

    /// Sets the value of Checkpointastreeupdatelatency100ns
    pub fn set_checkpointastreeupdatelatency100ns(&mut self, value: u64) {
        self.checkpointastreeupdatelatency100ns = Some(value);
    }

    /// Gets the value of Checkpointastreeupdatelatency100ns
    pub fn get_checkpointastreeupdatelatency100ns(&self) -> Option<&u64> {
        self.checkpointastreeupdatelatency100ns.as_ref()
    }

    /// Sets the value of Checkpointexclusivelockslatency100ns
    pub fn set_checkpointexclusivelockslatency100ns(&mut self, value: u64) {
        self.checkpointexclusivelockslatency100ns = Some(value);
    }

    /// Gets the value of Checkpointexclusivelockslatency100ns
    pub fn get_checkpointexclusivelockslatency100ns(&self) -> Option<&u64> {
        self.checkpointexclusivelockslatency100ns.as_ref()
    }

    /// Sets the value of Checkpointlatency100ns
    pub fn set_checkpointlatency100ns(&mut self, value: u64) {
        self.checkpointlatency100ns = Some(value);
    }

    /// Gets the value of Checkpointlatency100ns
    pub fn get_checkpointlatency100ns(&self) -> Option<&u64> {
        self.checkpointlatency100ns.as_ref()
    }

    /// Sets the value of CheckpointsPersec
    pub fn set_checkpoints_persec(&mut self, value: u64) {
        self.checkpoints_persec = Some(value);
    }

    /// Gets the value of CheckpointsPersec
    pub fn get_checkpoints_persec(&self) -> Option<&u64> {
        self.checkpoints_persec.as_ref()
    }

    /// Sets the value of CompactedContainerFillRatioPercent
    pub fn set_compacted_container_fill_ratio_percent(&mut self, value: u64) {
        self.compacted_container_fill_ratio_percent = Some(value);
    }

    /// Gets the value of CompactedContainerFillRatioPercent
    pub fn get_compacted_container_fill_ratio_percent(&self) -> Option<&u64> {
        self.compacted_container_fill_ratio_percent.as_ref()
    }

    /// Sets the value of CompactionFailureCount
    pub fn set_compaction_failure_count(&mut self, value: u32) {
        self.compaction_failure_count = Some(value);
    }

    /// Gets the value of CompactionFailureCount
    pub fn get_compaction_failure_count(&self) -> Option<&u32> {
        self.compaction_failure_count.as_ref()
    }

    /// Sets the value of Compactionreadlatency100ns
    pub fn set_compactionreadlatency100ns(&mut self, value: u64) {
        self.compactionreadlatency100ns = Some(value);
    }

    /// Gets the value of Compactionreadlatency100ns
    pub fn get_compactionreadlatency100ns(&self) -> Option<&u64> {
        self.compactionreadlatency100ns.as_ref()
    }

    /// Sets the value of Compactionsfailedduetoineligiblecontainer
    pub fn set_compactionsfailedduetoineligiblecontainer(&mut self, value: u32) {
        self.compactionsfailedduetoineligiblecontainer = Some(value);
    }

    /// Gets the value of Compactionsfailedduetoineligiblecontainer
    pub fn get_compactionsfailedduetoineligiblecontainer(&self) -> Option<&u32> {
        self.compactionsfailedduetoineligiblecontainer.as_ref()
    }

    /// Sets the value of Compactionsfailedduetomaxfragmentation
    pub fn set_compactionsfailedduetomaxfragmentation(&mut self, value: u32) {
        self.compactionsfailedduetomaxfragmentation = Some(value);
    }

    /// Gets the value of Compactionsfailedduetomaxfragmentation
    pub fn get_compactionsfailedduetomaxfragmentation(&self) -> Option<&u32> {
        self.compactionsfailedduetomaxfragmentation.as_ref()
    }

    /// Sets the value of Compactionwritelatency100ns
    pub fn set_compactionwritelatency100ns(&mut self, value: u64) {
        self.compactionwritelatency100ns = Some(value);
    }

    /// Gets the value of Compactionwritelatency100ns
    pub fn get_compactionwritelatency100ns(&self) -> Option<&u64> {
        self.compactionwritelatency100ns.as_ref()
    }

    /// Sets the value of ContainerDestagesFromFastTierPersec
    pub fn set_container_destages_from_fast_tier_persec(&mut self, value: u64) {
        self.container_destages_from_fast_tier_persec = Some(value);
    }

    /// Gets the value of ContainerDestagesFromFastTierPersec
    pub fn get_container_destages_from_fast_tier_persec(&self) -> Option<&u64> {
        self.container_destages_from_fast_tier_persec.as_ref()
    }

    /// Sets the value of ContainerDestagesFromSlowTierPersec
    pub fn set_container_destages_from_slow_tier_persec(&mut self, value: u64) {
        self.container_destages_from_slow_tier_persec = Some(value);
    }

    /// Gets the value of ContainerDestagesFromSlowTierPersec
    pub fn get_container_destages_from_slow_tier_persec(&self) -> Option<&u64> {
        self.container_destages_from_slow_tier_persec.as_ref()
    }

    /// Sets the value of ContainerMoveFailureCount
    pub fn set_container_move_failure_count(&mut self, value: u32) {
        self.container_move_failure_count = Some(value);
    }

    /// Gets the value of ContainerMoveFailureCount
    pub fn get_container_move_failure_count(&self) -> Option<&u32> {
        self.container_move_failure_count.as_ref()
    }

    /// Sets the value of ContainerMoveRetryCount
    pub fn set_container_move_retry_count(&mut self, value: u32) {
        self.container_move_retry_count = Some(value);
    }

    /// Gets the value of ContainerMoveRetryCount
    pub fn get_container_move_retry_count(&self) -> Option<&u32> {
        self.container_move_retry_count.as_ref()
    }

    /// Sets the value of Containermovesfailedduetoineligiblecontainer
    pub fn set_containermovesfailedduetoineligiblecontainer(&mut self, value: u32) {
        self.containermovesfailedduetoineligiblecontainer = Some(value);
    }

    /// Gets the value of Containermovesfailedduetoineligiblecontainer
    pub fn get_containermovesfailedduetoineligiblecontainer(&self) -> Option<&u32> {
        self.containermovesfailedduetoineligiblecontainer.as_ref()
    }

    /// Sets the value of CurrentFastTierDataFillPercentage
    pub fn set_current_fast_tier_data_fill_percentage(&mut self, value: u32) {
        self.current_fast_tier_data_fill_percentage = Some(value);
    }

    /// Gets the value of CurrentFastTierDataFillPercentage
    pub fn get_current_fast_tier_data_fill_percentage(&self) -> Option<&u32> {
        self.current_fast_tier_data_fill_percentage.as_ref()
    }

    /// Sets the value of CurrentFastTierMetadataFillPercentage
    pub fn set_current_fast_tier_metadata_fill_percentage(&mut self, value: u32) {
        self.current_fast_tier_metadata_fill_percentage = Some(value);
    }

    /// Gets the value of CurrentFastTierMetadataFillPercentage
    pub fn get_current_fast_tier_metadata_fill_percentage(&self) -> Option<&u32> {
        self.current_fast_tier_metadata_fill_percentage.as_ref()
    }

    /// Sets the value of CurrentSlowTierDataFillPercentage
    pub fn set_current_slow_tier_data_fill_percentage(&mut self, value: u32) {
        self.current_slow_tier_data_fill_percentage = Some(value);
    }

    /// Gets the value of CurrentSlowTierDataFillPercentage
    pub fn get_current_slow_tier_data_fill_percentage(&self) -> Option<&u32> {
        self.current_slow_tier_data_fill_percentage.as_ref()
    }

    /// Sets the value of CurrentSlowTierMetadataFillPercentage
    pub fn set_current_slow_tier_metadata_fill_percentage(&mut self, value: u32) {
        self.current_slow_tier_metadata_fill_percentage = Some(value);
    }

    /// Gets the value of CurrentSlowTierMetadataFillPercentage
    pub fn get_current_slow_tier_metadata_fill_percentage(&self) -> Option<&u32> {
        self.current_slow_tier_metadata_fill_percentage.as_ref()
    }

    /// Sets the value of DataCompactionsPersec
    pub fn set_data_compactions_persec(&mut self, value: u64) {
        self.data_compactions_persec = Some(value);
    }

    /// Gets the value of DataCompactionsPersec
    pub fn get_data_compactions_persec(&self) -> Option<&u64> {
        self.data_compactions_persec.as_ref()
    }

    /// Sets the value of DataInPlaceWriteClustersPersec
    pub fn set_data_in_place_write_clusters_persec(&mut self, value: u64) {
        self.data_in_place_write_clusters_persec = Some(value);
    }

    /// Gets the value of DataInPlaceWriteClustersPersec
    pub fn get_data_in_place_write_clusters_persec(&self) -> Option<&u64> {
        self.data_in_place_write_clusters_persec.as_ref()
    }

    /// Sets the value of DeleteQueueentries
    pub fn set_delete_queueentries(&mut self, value: u32) {
        self.delete_queueentries = Some(value);
    }

    /// Gets the value of DeleteQueueentries
    pub fn get_delete_queueentries(&self) -> Option<&u32> {
        self.delete_queueentries.as_ref()
    }

    /// Sets the value of Dirtymetadatapages
    pub fn set_dirtymetadatapages(&mut self, value: u64) {
        self.dirtymetadatapages = Some(value);
    }

    /// Gets the value of Dirtymetadatapages
    pub fn get_dirtymetadatapages(&self) -> Option<&u64> {
        self.dirtymetadatapages.as_ref()
    }

    /// Sets the value of Dirtytablelistentries
    pub fn set_dirtytablelistentries(&mut self, value: u32) {
        self.dirtytablelistentries = Some(value);
    }

    /// Gets the value of Dirtytablelistentries
    pub fn get_dirtytablelistentries(&self) -> Option<&u32> {
        self.dirtytablelistentries.as_ref()
    }

    /// Sets the value of FastTierDestagedContainerFillRatioPercent
    pub fn set_fast_tier_destaged_container_fill_ratio_percent(&mut self, value: u64) {
        self.fast_tier_destaged_container_fill_ratio_percent = Some(value);
    }

    /// Gets the value of FastTierDestagedContainerFillRatioPercent
    pub fn get_fast_tier_destaged_container_fill_ratio_percent(&self) -> Option<&u64> {
        self.fast_tier_destaged_container_fill_ratio_percent.as_ref()
    }

    /// Sets the value of Fasttierdestagereadlatency100ns
    pub fn set_fasttierdestagereadlatency100ns(&mut self, value: u64) {
        self.fasttierdestagereadlatency100ns = Some(value);
    }

    /// Gets the value of Fasttierdestagereadlatency100ns
    pub fn get_fasttierdestagereadlatency100ns(&self) -> Option<&u64> {
        self.fasttierdestagereadlatency100ns.as_ref()
    }

    /// Sets the value of Fasttierdestagewritelatency100ns
    pub fn set_fasttierdestagewritelatency100ns(&mut self, value: u64) {
        self.fasttierdestagewritelatency100ns = Some(value);
    }

    /// Gets the value of Fasttierdestagewritelatency100ns
    pub fn get_fasttierdestagewritelatency100ns(&self) -> Option<&u64> {
        self.fasttierdestagewritelatency100ns.as_ref()
    }

    /// Sets the value of Logfillpercentage
    pub fn set_logfillpercentage(&mut self, value: u32) {
        self.logfillpercentage = Some(value);
    }

    /// Gets the value of Logfillpercentage
    pub fn get_logfillpercentage(&self) -> Option<&u32> {
        self.logfillpercentage.as_ref()
    }

    /// Sets the value of LogwritesPersec
    pub fn set_logwrites_persec(&mut self, value: u64) {
        self.logwrites_persec = Some(value);
    }

    /// Gets the value of LogwritesPersec
    pub fn get_logwrites_persec(&self) -> Option<&u64> {
        self.logwrites_persec.as_ref()
    }

    /// Sets the value of SlowTierDestagedContainerFillRatioPercent
    pub fn set_slow_tier_destaged_container_fill_ratio_percent(&mut self, value: u64) {
        self.slow_tier_destaged_container_fill_ratio_percent = Some(value);
    }

    /// Gets the value of SlowTierDestagedContainerFillRatioPercent
    pub fn get_slow_tier_destaged_container_fill_ratio_percent(&self) -> Option<&u64> {
        self.slow_tier_destaged_container_fill_ratio_percent.as_ref()
    }

    /// Sets the value of Slowtierdestagereadlatency100ns
    pub fn set_slowtierdestagereadlatency100ns(&mut self, value: u64) {
        self.slowtierdestagereadlatency100ns = Some(value);
    }

    /// Gets the value of Slowtierdestagereadlatency100ns
    pub fn get_slowtierdestagereadlatency100ns(&self) -> Option<&u64> {
        self.slowtierdestagereadlatency100ns.as_ref()
    }

    /// Sets the value of Slowtierdestagewritelatency100ns
    pub fn set_slowtierdestagewritelatency100ns(&mut self, value: u64) {
        self.slowtierdestagewritelatency100ns = Some(value);
    }

    /// Gets the value of Slowtierdestagewritelatency100ns
    pub fn get_slowtierdestagewritelatency100ns(&self) -> Option<&u64> {
        self.slowtierdestagewritelatency100ns.as_ref()
    }

    /// Sets the value of TotalAllocationofClustersPersec
    pub fn set_total_allocationof_clusters_persec(&mut self, value: u64) {
        self.total_allocationof_clusters_persec = Some(value);
    }

    /// Gets the value of TotalAllocationofClustersPersec
    pub fn get_total_allocationof_clusters_persec(&self) -> Option<&u64> {
        self.total_allocationof_clusters_persec.as_ref()
    }

    /// Sets the value of Treeupdateexclusivelockslatency100ns
    pub fn set_treeupdateexclusivelockslatency100ns(&mut self, value: u64) {
        self.treeupdateexclusivelockslatency100ns = Some(value);
    }

    /// Gets the value of Treeupdateexclusivelockslatency100ns
    pub fn get_treeupdateexclusivelockslatency100ns(&self) -> Option<&u64> {
        self.treeupdateexclusivelockslatency100ns.as_ref()
    }

    /// Sets the value of Treeupdatelatency100ns
    pub fn set_treeupdatelatency100ns(&mut self, value: u64) {
        self.treeupdatelatency100ns = Some(value);
    }

    /// Gets the value of Treeupdatelatency100ns
    pub fn get_treeupdatelatency100ns(&self) -> Option<&u64> {
        self.treeupdatelatency100ns.as_ref()
    }

    /// Sets the value of TreeupdatesPersec
    pub fn set_treeupdates_persec(&mut self, value: u64) {
        self.treeupdates_persec = Some(value);
    }

    /// Gets the value of TreeupdatesPersec
    pub fn get_treeupdates_persec(&self) -> Option<&u64> {
        self.treeupdates_persec.as_ref()
    }

    /// Sets the value of Trimlatency100ns
    pub fn set_trimlatency100ns(&mut self, value: u64) {
        self.trimlatency100ns = Some(value);
    }

    /// Gets the value of Trimlatency100ns
    pub fn get_trimlatency100ns(&self) -> Option<&u64> {
        self.trimlatency100ns.as_ref()
    }
}

