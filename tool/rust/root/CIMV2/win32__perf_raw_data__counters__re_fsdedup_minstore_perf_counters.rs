// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_ReFSDedupMinstorePerfCounters struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_ReFSDedupMinstorePerfCounters {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "AllocationofMetadataClustersonFastTierPersec")]
    pub allocationof_metadata_clusterson_fast_tier_persec: Option<u64>,

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
    #[serde(rename = "AllocatorRegionsExaminedAverage_Base")]
    pub allocator_regions_examined_average__base: Option<u32>,

/// 
    #[serde(rename = "AllocatorRegionsExaminedMax")]
    pub allocator_regions_examined_max: Option<u32>,

/// 
    #[serde(rename = "Checkpointastreeupdatelatency100ns")]
    pub checkpointastreeupdatelatency100ns: Option<u64>,

/// 
    #[serde(rename = "Checkpointastreeupdatelatency100ns_Base")]
    pub checkpointastreeupdatelatency100ns__base: Option<u32>,

/// 
    #[serde(rename = "Checkpointexclusivelockslatency100ns")]
    pub checkpointexclusivelockslatency100ns: Option<u64>,

/// 
    #[serde(rename = "Checkpointexclusivelockslatency100ns_Base")]
    pub checkpointexclusivelockslatency100ns__base: Option<u32>,

/// 
    #[serde(rename = "CheckpointsPersec")]
    pub checkpoints_persec: Option<u64>,

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
    #[serde(rename = "TotalAllocationofClustersPersec")]
    pub total_allocationof_clusters_persec: Option<u64>,

/// 
    #[serde(rename = "Totaltimeneededtoprocesscheckpointsin100ns")]
    pub totaltimeneededtoprocesscheckpointsin100ns: Option<u64>,

/// 
    #[serde(rename = "Totaltimeneededtoprocesscheckpointsin100ns_Base")]
    pub totaltimeneededtoprocesscheckpointsin100ns__base: Option<u32>,

/// 
    #[serde(rename = "Treeupdateexclusivelockslatency100ns")]
    pub treeupdateexclusivelockslatency100ns: Option<u64>,

/// 
    #[serde(rename = "Treeupdateexclusivelockslatency100ns_Base")]
    pub treeupdateexclusivelockslatency100ns__base: Option<u32>,

/// 
    #[serde(rename = "Treeupdatelatency100ns")]
    pub treeupdatelatency100ns: Option<u64>,

/// 
    #[serde(rename = "Treeupdatelatency100ns_Base")]
    pub treeupdatelatency100ns__base: Option<u32>,

/// 
    #[serde(rename = "TreeupdatesPersec")]
    pub treeupdates_persec: Option<u64>,
}

impl Win32_PerfRawData_Counters_ReFSDedupMinstorePerfCounters {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            allocationof_metadata_clusterson_fast_tier_persec: None,
            allocator_cache_collisions: None,
            allocator_cache_deletions: None,
            allocator_cache_insertions: None,
            allocator_cache_purges: None,
            allocator_cache_size: None,
            allocator_regions_examined_average: None,
            allocator_regions_examined_average__base: None,
            allocator_regions_examined_max: None,
            checkpointastreeupdatelatency100ns: None,
            checkpointastreeupdatelatency100ns__base: None,
            checkpointexclusivelockslatency100ns: None,
            checkpointexclusivelockslatency100ns__base: None,
            checkpoints_persec: None,
            delete_queueentries: None,
            dirtymetadatapages: None,
            dirtytablelistentries: None,
            total_allocationof_clusters_persec: None,
            totaltimeneededtoprocesscheckpointsin100ns: None,
            totaltimeneededtoprocesscheckpointsin100ns__base: None,
            treeupdateexclusivelockslatency100ns: None,
            treeupdateexclusivelockslatency100ns__base: None,
            treeupdatelatency100ns: None,
            treeupdatelatency100ns__base: None,
            treeupdates_persec: None,
        }
    }


    /// Sets the value of AllocationofMetadataClustersonFastTierPersec
    pub fn set_allocationof_metadata_clusterson_fast_tier_persec(&mut self, value: u64) {
        self.allocationof_metadata_clusterson_fast_tier_persec = Some(value);
    }

    /// Gets the value of AllocationofMetadataClustersonFastTierPersec
    pub fn get_allocationof_metadata_clusterson_fast_tier_persec(&self) -> Option<&u64> {
        self.allocationof_metadata_clusterson_fast_tier_persec.as_ref()
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

    /// Sets the value of AllocatorRegionsExaminedAverage_Base
    pub fn set_allocator_regions_examined_average__base(&mut self, value: u32) {
        self.allocator_regions_examined_average__base = Some(value);
    }

    /// Gets the value of AllocatorRegionsExaminedAverage_Base
    pub fn get_allocator_regions_examined_average__base(&self) -> Option<&u32> {
        self.allocator_regions_examined_average__base.as_ref()
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

    /// Sets the value of Checkpointastreeupdatelatency100ns_Base
    pub fn set_checkpointastreeupdatelatency100ns__base(&mut self, value: u32) {
        self.checkpointastreeupdatelatency100ns__base = Some(value);
    }

    /// Gets the value of Checkpointastreeupdatelatency100ns_Base
    pub fn get_checkpointastreeupdatelatency100ns__base(&self) -> Option<&u32> {
        self.checkpointastreeupdatelatency100ns__base.as_ref()
    }

    /// Sets the value of Checkpointexclusivelockslatency100ns
    pub fn set_checkpointexclusivelockslatency100ns(&mut self, value: u64) {
        self.checkpointexclusivelockslatency100ns = Some(value);
    }

    /// Gets the value of Checkpointexclusivelockslatency100ns
    pub fn get_checkpointexclusivelockslatency100ns(&self) -> Option<&u64> {
        self.checkpointexclusivelockslatency100ns.as_ref()
    }

    /// Sets the value of Checkpointexclusivelockslatency100ns_Base
    pub fn set_checkpointexclusivelockslatency100ns__base(&mut self, value: u32) {
        self.checkpointexclusivelockslatency100ns__base = Some(value);
    }

    /// Gets the value of Checkpointexclusivelockslatency100ns_Base
    pub fn get_checkpointexclusivelockslatency100ns__base(&self) -> Option<&u32> {
        self.checkpointexclusivelockslatency100ns__base.as_ref()
    }

    /// Sets the value of CheckpointsPersec
    pub fn set_checkpoints_persec(&mut self, value: u64) {
        self.checkpoints_persec = Some(value);
    }

    /// Gets the value of CheckpointsPersec
    pub fn get_checkpoints_persec(&self) -> Option<&u64> {
        self.checkpoints_persec.as_ref()
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

    /// Sets the value of TotalAllocationofClustersPersec
    pub fn set_total_allocationof_clusters_persec(&mut self, value: u64) {
        self.total_allocationof_clusters_persec = Some(value);
    }

    /// Gets the value of TotalAllocationofClustersPersec
    pub fn get_total_allocationof_clusters_persec(&self) -> Option<&u64> {
        self.total_allocationof_clusters_persec.as_ref()
    }

    /// Sets the value of Totaltimeneededtoprocesscheckpointsin100ns
    pub fn set_totaltimeneededtoprocesscheckpointsin100ns(&mut self, value: u64) {
        self.totaltimeneededtoprocesscheckpointsin100ns = Some(value);
    }

    /// Gets the value of Totaltimeneededtoprocesscheckpointsin100ns
    pub fn get_totaltimeneededtoprocesscheckpointsin100ns(&self) -> Option<&u64> {
        self.totaltimeneededtoprocesscheckpointsin100ns.as_ref()
    }

    /// Sets the value of Totaltimeneededtoprocesscheckpointsin100ns_Base
    pub fn set_totaltimeneededtoprocesscheckpointsin100ns__base(&mut self, value: u32) {
        self.totaltimeneededtoprocesscheckpointsin100ns__base = Some(value);
    }

    /// Gets the value of Totaltimeneededtoprocesscheckpointsin100ns_Base
    pub fn get_totaltimeneededtoprocesscheckpointsin100ns__base(&self) -> Option<&u32> {
        self.totaltimeneededtoprocesscheckpointsin100ns__base.as_ref()
    }

    /// Sets the value of Treeupdateexclusivelockslatency100ns
    pub fn set_treeupdateexclusivelockslatency100ns(&mut self, value: u64) {
        self.treeupdateexclusivelockslatency100ns = Some(value);
    }

    /// Gets the value of Treeupdateexclusivelockslatency100ns
    pub fn get_treeupdateexclusivelockslatency100ns(&self) -> Option<&u64> {
        self.treeupdateexclusivelockslatency100ns.as_ref()
    }

    /// Sets the value of Treeupdateexclusivelockslatency100ns_Base
    pub fn set_treeupdateexclusivelockslatency100ns__base(&mut self, value: u32) {
        self.treeupdateexclusivelockslatency100ns__base = Some(value);
    }

    /// Gets the value of Treeupdateexclusivelockslatency100ns_Base
    pub fn get_treeupdateexclusivelockslatency100ns__base(&self) -> Option<&u32> {
        self.treeupdateexclusivelockslatency100ns__base.as_ref()
    }

    /// Sets the value of Treeupdatelatency100ns
    pub fn set_treeupdatelatency100ns(&mut self, value: u64) {
        self.treeupdatelatency100ns = Some(value);
    }

    /// Gets the value of Treeupdatelatency100ns
    pub fn get_treeupdatelatency100ns(&self) -> Option<&u64> {
        self.treeupdatelatency100ns.as_ref()
    }

    /// Sets the value of Treeupdatelatency100ns_Base
    pub fn set_treeupdatelatency100ns__base(&mut self, value: u32) {
        self.treeupdatelatency100ns__base = Some(value);
    }

    /// Gets the value of Treeupdatelatency100ns_Base
    pub fn get_treeupdatelatency100ns__base(&self) -> Option<&u32> {
        self.treeupdatelatency100ns__base.as_ref()
    }

    /// Sets the value of TreeupdatesPersec
    pub fn set_treeupdates_persec(&mut self, value: u64) {
        self.treeupdates_persec = Some(value);
    }

    /// Gets the value of TreeupdatesPersec
    pub fn get_treeupdates_persec(&self) -> Option<&u64> {
        self.treeupdates_persec.as_ref()
    }
}

