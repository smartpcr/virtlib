// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_StorageSpacesWriteCache struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_StorageSpacesWriteCache {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "CacheAdvances")]
    pub cache_advances: Option<u32>,

/// 
    #[serde(rename = "CacheCheckpointLatencyms")]
    pub cache_checkpoint_latencyms: Option<u32>,

/// 
    #[serde(rename = "CacheCheckpointLatencyms_Base")]
    pub cache_checkpoint_latencyms__base: Option<u32>,

/// 
    #[serde(rename = "CacheCheckpoints")]
    pub cache_checkpoints: Option<u32>,

/// 
    #[serde(rename = "CacheDataBytes")]
    pub cache_data_bytes: Option<u64>,

/// 
    #[serde(rename = "CacheDataPercent")]
    pub cache_data_percent: Option<u64>,

/// 
    #[serde(rename = "CacheDataPercent_Base")]
    pub cache_data_percent__base: Option<u64>,

/// 
    #[serde(rename = "CacheDestagesCurrent")]
    pub cache_destages_current: Option<u32>,

/// 
    #[serde(rename = "CacheReclaimableBytes")]
    pub cache_reclaimable_bytes: Option<u64>,

/// 
    #[serde(rename = "CacheReclaimablePercent")]
    pub cache_reclaimable_percent: Option<u64>,

/// 
    #[serde(rename = "CacheReclaimablePercent_Base")]
    pub cache_reclaimable_percent__base: Option<u64>,

/// 
    #[serde(rename = "CacheSize")]
    pub cache_size: Option<u64>,

/// 
    #[serde(rename = "CacheUsedBytes")]
    pub cache_used_bytes: Option<u64>,

/// 
    #[serde(rename = "CacheUsedPercent")]
    pub cache_used_percent: Option<u64>,

/// 
    #[serde(rename = "CacheUsedPercent_Base")]
    pub cache_used_percent__base: Option<u64>,

/// 
    #[serde(rename = "DestageReadFailureCount")]
    pub destage_read_failure_count: Option<u32>,

/// 
    #[serde(rename = "DestageWriteFailureCount")]
    pub destage_write_failure_count: Option<u32>,

/// 
    #[serde(rename = "EvictCacheBytesPersec")]
    pub evict_cache_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "EvictCacheDestagedBytesPersec")]
    pub evict_cache_destaged_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "EvictCacheDestagedPercent")]
    pub evict_cache_destaged_percent: Option<u64>,

/// 
    #[serde(rename = "EvictCacheDestagedPercent_Base")]
    pub evict_cache_destaged_percent__base: Option<u64>,

/// 
    #[serde(rename = "EvictCacheOverwriteBytesPersec")]
    pub evict_cache_overwrite_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "EvictCacheOverwritePercent")]
    pub evict_cache_overwrite_percent: Option<u64>,

/// 
    #[serde(rename = "EvictCacheOverwritePercent_Base")]
    pub evict_cache_overwrite_percent__base: Option<u64>,

/// 
    #[serde(rename = "ReadBypassBytesPersec")]
    pub read_bypass_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "ReadBypassPercent")]
    pub read_bypass_percent: Option<u64>,

/// 
    #[serde(rename = "ReadBypassPercent_Base")]
    pub read_bypass_percent__base: Option<u64>,

/// 
    #[serde(rename = "ReadCacheBytesPersec")]
    pub read_cache_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "ReadCachePercent")]
    pub read_cache_percent: Option<u64>,

/// 
    #[serde(rename = "ReadCachePercent_Base")]
    pub read_cache_percent__base: Option<u64>,

/// 
    #[serde(rename = "VdtCheckpointLatencyms")]
    pub vdt_checkpoint_latencyms: Option<u32>,

/// 
    #[serde(rename = "VdtCheckpointLatencyms_Base")]
    pub vdt_checkpoint_latencyms__base: Option<u32>,

/// 
    #[serde(rename = "WriteBypassBytesPersec")]
    pub write_bypass_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "WriteBypassPercent")]
    pub write_bypass_percent: Option<u64>,

/// 
    #[serde(rename = "WriteBypassPercent_Base")]
    pub write_bypass_percent__base: Option<u64>,

/// 
    #[serde(rename = "WriteCacheBytesPersec")]
    pub write_cache_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "WriteCacheOverlapBytesPersec")]
    pub write_cache_overlap_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "WriteCacheOverlapPercent")]
    pub write_cache_overlap_percent: Option<u64>,

/// 
    #[serde(rename = "WriteCacheOverlapPercent_Base")]
    pub write_cache_overlap_percent__base: Option<u64>,

/// 
    #[serde(rename = "WriteCachePercent")]
    pub write_cache_percent: Option<u64>,

/// 
    #[serde(rename = "WriteCachePercent_Base")]
    pub write_cache_percent__base: Option<u64>,

/// 
    #[serde(rename = "WriteCacheUnalignedBytesPersec")]
    pub write_cache_unaligned_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "WriteCacheUnalignedPercent")]
    pub write_cache_unaligned_percent: Option<u64>,

/// 
    #[serde(rename = "WriteCacheUnalignedPercent_Base")]
    pub write_cache_unaligned_percent__base: Option<u64>,

/// 
    #[serde(rename = "WriteCacheUntrimmedBytesPersec")]
    pub write_cache_untrimmed_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "WriteCacheUntrimmedPercent")]
    pub write_cache_untrimmed_percent: Option<u64>,

/// 
    #[serde(rename = "WriteCacheUntrimmedPercent_Base")]
    pub write_cache_untrimmed_percent__base: Option<u64>,
}

impl Win32_PerfRawData_Counters_StorageSpacesWriteCache {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            cache_advances: None,
            cache_checkpoint_latencyms: None,
            cache_checkpoint_latencyms__base: None,
            cache_checkpoints: None,
            cache_data_bytes: None,
            cache_data_percent: None,
            cache_data_percent__base: None,
            cache_destages_current: None,
            cache_reclaimable_bytes: None,
            cache_reclaimable_percent: None,
            cache_reclaimable_percent__base: None,
            cache_size: None,
            cache_used_bytes: None,
            cache_used_percent: None,
            cache_used_percent__base: None,
            destage_read_failure_count: None,
            destage_write_failure_count: None,
            evict_cache_bytes_persec: None,
            evict_cache_destaged_bytes_persec: None,
            evict_cache_destaged_percent: None,
            evict_cache_destaged_percent__base: None,
            evict_cache_overwrite_bytes_persec: None,
            evict_cache_overwrite_percent: None,
            evict_cache_overwrite_percent__base: None,
            read_bypass_bytes_persec: None,
            read_bypass_percent: None,
            read_bypass_percent__base: None,
            read_cache_bytes_persec: None,
            read_cache_percent: None,
            read_cache_percent__base: None,
            vdt_checkpoint_latencyms: None,
            vdt_checkpoint_latencyms__base: None,
            write_bypass_bytes_persec: None,
            write_bypass_percent: None,
            write_bypass_percent__base: None,
            write_cache_bytes_persec: None,
            write_cache_overlap_bytes_persec: None,
            write_cache_overlap_percent: None,
            write_cache_overlap_percent__base: None,
            write_cache_percent: None,
            write_cache_percent__base: None,
            write_cache_unaligned_bytes_persec: None,
            write_cache_unaligned_percent: None,
            write_cache_unaligned_percent__base: None,
            write_cache_untrimmed_bytes_persec: None,
            write_cache_untrimmed_percent: None,
            write_cache_untrimmed_percent__base: None,
        }
    }


    /// Sets the value of CacheAdvances
    pub fn set_cache_advances(&mut self, value: u32) {
        self.cache_advances = Some(value);
    }

    /// Gets the value of CacheAdvances
    pub fn get_cache_advances(&self) -> Option<&u32> {
        self.cache_advances.as_ref()
    }

    /// Sets the value of CacheCheckpointLatencyms
    pub fn set_cache_checkpoint_latencyms(&mut self, value: u32) {
        self.cache_checkpoint_latencyms = Some(value);
    }

    /// Gets the value of CacheCheckpointLatencyms
    pub fn get_cache_checkpoint_latencyms(&self) -> Option<&u32> {
        self.cache_checkpoint_latencyms.as_ref()
    }

    /// Sets the value of CacheCheckpointLatencyms_Base
    pub fn set_cache_checkpoint_latencyms__base(&mut self, value: u32) {
        self.cache_checkpoint_latencyms__base = Some(value);
    }

    /// Gets the value of CacheCheckpointLatencyms_Base
    pub fn get_cache_checkpoint_latencyms__base(&self) -> Option<&u32> {
        self.cache_checkpoint_latencyms__base.as_ref()
    }

    /// Sets the value of CacheCheckpoints
    pub fn set_cache_checkpoints(&mut self, value: u32) {
        self.cache_checkpoints = Some(value);
    }

    /// Gets the value of CacheCheckpoints
    pub fn get_cache_checkpoints(&self) -> Option<&u32> {
        self.cache_checkpoints.as_ref()
    }

    /// Sets the value of CacheDataBytes
    pub fn set_cache_data_bytes(&mut self, value: u64) {
        self.cache_data_bytes = Some(value);
    }

    /// Gets the value of CacheDataBytes
    pub fn get_cache_data_bytes(&self) -> Option<&u64> {
        self.cache_data_bytes.as_ref()
    }

    /// Sets the value of CacheDataPercent
    pub fn set_cache_data_percent(&mut self, value: u64) {
        self.cache_data_percent = Some(value);
    }

    /// Gets the value of CacheDataPercent
    pub fn get_cache_data_percent(&self) -> Option<&u64> {
        self.cache_data_percent.as_ref()
    }

    /// Sets the value of CacheDataPercent_Base
    pub fn set_cache_data_percent__base(&mut self, value: u64) {
        self.cache_data_percent__base = Some(value);
    }

    /// Gets the value of CacheDataPercent_Base
    pub fn get_cache_data_percent__base(&self) -> Option<&u64> {
        self.cache_data_percent__base.as_ref()
    }

    /// Sets the value of CacheDestagesCurrent
    pub fn set_cache_destages_current(&mut self, value: u32) {
        self.cache_destages_current = Some(value);
    }

    /// Gets the value of CacheDestagesCurrent
    pub fn get_cache_destages_current(&self) -> Option<&u32> {
        self.cache_destages_current.as_ref()
    }

    /// Sets the value of CacheReclaimableBytes
    pub fn set_cache_reclaimable_bytes(&mut self, value: u64) {
        self.cache_reclaimable_bytes = Some(value);
    }

    /// Gets the value of CacheReclaimableBytes
    pub fn get_cache_reclaimable_bytes(&self) -> Option<&u64> {
        self.cache_reclaimable_bytes.as_ref()
    }

    /// Sets the value of CacheReclaimablePercent
    pub fn set_cache_reclaimable_percent(&mut self, value: u64) {
        self.cache_reclaimable_percent = Some(value);
    }

    /// Gets the value of CacheReclaimablePercent
    pub fn get_cache_reclaimable_percent(&self) -> Option<&u64> {
        self.cache_reclaimable_percent.as_ref()
    }

    /// Sets the value of CacheReclaimablePercent_Base
    pub fn set_cache_reclaimable_percent__base(&mut self, value: u64) {
        self.cache_reclaimable_percent__base = Some(value);
    }

    /// Gets the value of CacheReclaimablePercent_Base
    pub fn get_cache_reclaimable_percent__base(&self) -> Option<&u64> {
        self.cache_reclaimable_percent__base.as_ref()
    }

    /// Sets the value of CacheSize
    pub fn set_cache_size(&mut self, value: u64) {
        self.cache_size = Some(value);
    }

    /// Gets the value of CacheSize
    pub fn get_cache_size(&self) -> Option<&u64> {
        self.cache_size.as_ref()
    }

    /// Sets the value of CacheUsedBytes
    pub fn set_cache_used_bytes(&mut self, value: u64) {
        self.cache_used_bytes = Some(value);
    }

    /// Gets the value of CacheUsedBytes
    pub fn get_cache_used_bytes(&self) -> Option<&u64> {
        self.cache_used_bytes.as_ref()
    }

    /// Sets the value of CacheUsedPercent
    pub fn set_cache_used_percent(&mut self, value: u64) {
        self.cache_used_percent = Some(value);
    }

    /// Gets the value of CacheUsedPercent
    pub fn get_cache_used_percent(&self) -> Option<&u64> {
        self.cache_used_percent.as_ref()
    }

    /// Sets the value of CacheUsedPercent_Base
    pub fn set_cache_used_percent__base(&mut self, value: u64) {
        self.cache_used_percent__base = Some(value);
    }

    /// Gets the value of CacheUsedPercent_Base
    pub fn get_cache_used_percent__base(&self) -> Option<&u64> {
        self.cache_used_percent__base.as_ref()
    }

    /// Sets the value of DestageReadFailureCount
    pub fn set_destage_read_failure_count(&mut self, value: u32) {
        self.destage_read_failure_count = Some(value);
    }

    /// Gets the value of DestageReadFailureCount
    pub fn get_destage_read_failure_count(&self) -> Option<&u32> {
        self.destage_read_failure_count.as_ref()
    }

    /// Sets the value of DestageWriteFailureCount
    pub fn set_destage_write_failure_count(&mut self, value: u32) {
        self.destage_write_failure_count = Some(value);
    }

    /// Gets the value of DestageWriteFailureCount
    pub fn get_destage_write_failure_count(&self) -> Option<&u32> {
        self.destage_write_failure_count.as_ref()
    }

    /// Sets the value of EvictCacheBytesPersec
    pub fn set_evict_cache_bytes_persec(&mut self, value: u64) {
        self.evict_cache_bytes_persec = Some(value);
    }

    /// Gets the value of EvictCacheBytesPersec
    pub fn get_evict_cache_bytes_persec(&self) -> Option<&u64> {
        self.evict_cache_bytes_persec.as_ref()
    }

    /// Sets the value of EvictCacheDestagedBytesPersec
    pub fn set_evict_cache_destaged_bytes_persec(&mut self, value: u64) {
        self.evict_cache_destaged_bytes_persec = Some(value);
    }

    /// Gets the value of EvictCacheDestagedBytesPersec
    pub fn get_evict_cache_destaged_bytes_persec(&self) -> Option<&u64> {
        self.evict_cache_destaged_bytes_persec.as_ref()
    }

    /// Sets the value of EvictCacheDestagedPercent
    pub fn set_evict_cache_destaged_percent(&mut self, value: u64) {
        self.evict_cache_destaged_percent = Some(value);
    }

    /// Gets the value of EvictCacheDestagedPercent
    pub fn get_evict_cache_destaged_percent(&self) -> Option<&u64> {
        self.evict_cache_destaged_percent.as_ref()
    }

    /// Sets the value of EvictCacheDestagedPercent_Base
    pub fn set_evict_cache_destaged_percent__base(&mut self, value: u64) {
        self.evict_cache_destaged_percent__base = Some(value);
    }

    /// Gets the value of EvictCacheDestagedPercent_Base
    pub fn get_evict_cache_destaged_percent__base(&self) -> Option<&u64> {
        self.evict_cache_destaged_percent__base.as_ref()
    }

    /// Sets the value of EvictCacheOverwriteBytesPersec
    pub fn set_evict_cache_overwrite_bytes_persec(&mut self, value: u64) {
        self.evict_cache_overwrite_bytes_persec = Some(value);
    }

    /// Gets the value of EvictCacheOverwriteBytesPersec
    pub fn get_evict_cache_overwrite_bytes_persec(&self) -> Option<&u64> {
        self.evict_cache_overwrite_bytes_persec.as_ref()
    }

    /// Sets the value of EvictCacheOverwritePercent
    pub fn set_evict_cache_overwrite_percent(&mut self, value: u64) {
        self.evict_cache_overwrite_percent = Some(value);
    }

    /// Gets the value of EvictCacheOverwritePercent
    pub fn get_evict_cache_overwrite_percent(&self) -> Option<&u64> {
        self.evict_cache_overwrite_percent.as_ref()
    }

    /// Sets the value of EvictCacheOverwritePercent_Base
    pub fn set_evict_cache_overwrite_percent__base(&mut self, value: u64) {
        self.evict_cache_overwrite_percent__base = Some(value);
    }

    /// Gets the value of EvictCacheOverwritePercent_Base
    pub fn get_evict_cache_overwrite_percent__base(&self) -> Option<&u64> {
        self.evict_cache_overwrite_percent__base.as_ref()
    }

    /// Sets the value of ReadBypassBytesPersec
    pub fn set_read_bypass_bytes_persec(&mut self, value: u64) {
        self.read_bypass_bytes_persec = Some(value);
    }

    /// Gets the value of ReadBypassBytesPersec
    pub fn get_read_bypass_bytes_persec(&self) -> Option<&u64> {
        self.read_bypass_bytes_persec.as_ref()
    }

    /// Sets the value of ReadBypassPercent
    pub fn set_read_bypass_percent(&mut self, value: u64) {
        self.read_bypass_percent = Some(value);
    }

    /// Gets the value of ReadBypassPercent
    pub fn get_read_bypass_percent(&self) -> Option<&u64> {
        self.read_bypass_percent.as_ref()
    }

    /// Sets the value of ReadBypassPercent_Base
    pub fn set_read_bypass_percent__base(&mut self, value: u64) {
        self.read_bypass_percent__base = Some(value);
    }

    /// Gets the value of ReadBypassPercent_Base
    pub fn get_read_bypass_percent__base(&self) -> Option<&u64> {
        self.read_bypass_percent__base.as_ref()
    }

    /// Sets the value of ReadCacheBytesPersec
    pub fn set_read_cache_bytes_persec(&mut self, value: u64) {
        self.read_cache_bytes_persec = Some(value);
    }

    /// Gets the value of ReadCacheBytesPersec
    pub fn get_read_cache_bytes_persec(&self) -> Option<&u64> {
        self.read_cache_bytes_persec.as_ref()
    }

    /// Sets the value of ReadCachePercent
    pub fn set_read_cache_percent(&mut self, value: u64) {
        self.read_cache_percent = Some(value);
    }

    /// Gets the value of ReadCachePercent
    pub fn get_read_cache_percent(&self) -> Option<&u64> {
        self.read_cache_percent.as_ref()
    }

    /// Sets the value of ReadCachePercent_Base
    pub fn set_read_cache_percent__base(&mut self, value: u64) {
        self.read_cache_percent__base = Some(value);
    }

    /// Gets the value of ReadCachePercent_Base
    pub fn get_read_cache_percent__base(&self) -> Option<&u64> {
        self.read_cache_percent__base.as_ref()
    }

    /// Sets the value of VdtCheckpointLatencyms
    pub fn set_vdt_checkpoint_latencyms(&mut self, value: u32) {
        self.vdt_checkpoint_latencyms = Some(value);
    }

    /// Gets the value of VdtCheckpointLatencyms
    pub fn get_vdt_checkpoint_latencyms(&self) -> Option<&u32> {
        self.vdt_checkpoint_latencyms.as_ref()
    }

    /// Sets the value of VdtCheckpointLatencyms_Base
    pub fn set_vdt_checkpoint_latencyms__base(&mut self, value: u32) {
        self.vdt_checkpoint_latencyms__base = Some(value);
    }

    /// Gets the value of VdtCheckpointLatencyms_Base
    pub fn get_vdt_checkpoint_latencyms__base(&self) -> Option<&u32> {
        self.vdt_checkpoint_latencyms__base.as_ref()
    }

    /// Sets the value of WriteBypassBytesPersec
    pub fn set_write_bypass_bytes_persec(&mut self, value: u64) {
        self.write_bypass_bytes_persec = Some(value);
    }

    /// Gets the value of WriteBypassBytesPersec
    pub fn get_write_bypass_bytes_persec(&self) -> Option<&u64> {
        self.write_bypass_bytes_persec.as_ref()
    }

    /// Sets the value of WriteBypassPercent
    pub fn set_write_bypass_percent(&mut self, value: u64) {
        self.write_bypass_percent = Some(value);
    }

    /// Gets the value of WriteBypassPercent
    pub fn get_write_bypass_percent(&self) -> Option<&u64> {
        self.write_bypass_percent.as_ref()
    }

    /// Sets the value of WriteBypassPercent_Base
    pub fn set_write_bypass_percent__base(&mut self, value: u64) {
        self.write_bypass_percent__base = Some(value);
    }

    /// Gets the value of WriteBypassPercent_Base
    pub fn get_write_bypass_percent__base(&self) -> Option<&u64> {
        self.write_bypass_percent__base.as_ref()
    }

    /// Sets the value of WriteCacheBytesPersec
    pub fn set_write_cache_bytes_persec(&mut self, value: u64) {
        self.write_cache_bytes_persec = Some(value);
    }

    /// Gets the value of WriteCacheBytesPersec
    pub fn get_write_cache_bytes_persec(&self) -> Option<&u64> {
        self.write_cache_bytes_persec.as_ref()
    }

    /// Sets the value of WriteCacheOverlapBytesPersec
    pub fn set_write_cache_overlap_bytes_persec(&mut self, value: u64) {
        self.write_cache_overlap_bytes_persec = Some(value);
    }

    /// Gets the value of WriteCacheOverlapBytesPersec
    pub fn get_write_cache_overlap_bytes_persec(&self) -> Option<&u64> {
        self.write_cache_overlap_bytes_persec.as_ref()
    }

    /// Sets the value of WriteCacheOverlapPercent
    pub fn set_write_cache_overlap_percent(&mut self, value: u64) {
        self.write_cache_overlap_percent = Some(value);
    }

    /// Gets the value of WriteCacheOverlapPercent
    pub fn get_write_cache_overlap_percent(&self) -> Option<&u64> {
        self.write_cache_overlap_percent.as_ref()
    }

    /// Sets the value of WriteCacheOverlapPercent_Base
    pub fn set_write_cache_overlap_percent__base(&mut self, value: u64) {
        self.write_cache_overlap_percent__base = Some(value);
    }

    /// Gets the value of WriteCacheOverlapPercent_Base
    pub fn get_write_cache_overlap_percent__base(&self) -> Option<&u64> {
        self.write_cache_overlap_percent__base.as_ref()
    }

    /// Sets the value of WriteCachePercent
    pub fn set_write_cache_percent(&mut self, value: u64) {
        self.write_cache_percent = Some(value);
    }

    /// Gets the value of WriteCachePercent
    pub fn get_write_cache_percent(&self) -> Option<&u64> {
        self.write_cache_percent.as_ref()
    }

    /// Sets the value of WriteCachePercent_Base
    pub fn set_write_cache_percent__base(&mut self, value: u64) {
        self.write_cache_percent__base = Some(value);
    }

    /// Gets the value of WriteCachePercent_Base
    pub fn get_write_cache_percent__base(&self) -> Option<&u64> {
        self.write_cache_percent__base.as_ref()
    }

    /// Sets the value of WriteCacheUnalignedBytesPersec
    pub fn set_write_cache_unaligned_bytes_persec(&mut self, value: u64) {
        self.write_cache_unaligned_bytes_persec = Some(value);
    }

    /// Gets the value of WriteCacheUnalignedBytesPersec
    pub fn get_write_cache_unaligned_bytes_persec(&self) -> Option<&u64> {
        self.write_cache_unaligned_bytes_persec.as_ref()
    }

    /// Sets the value of WriteCacheUnalignedPercent
    pub fn set_write_cache_unaligned_percent(&mut self, value: u64) {
        self.write_cache_unaligned_percent = Some(value);
    }

    /// Gets the value of WriteCacheUnalignedPercent
    pub fn get_write_cache_unaligned_percent(&self) -> Option<&u64> {
        self.write_cache_unaligned_percent.as_ref()
    }

    /// Sets the value of WriteCacheUnalignedPercent_Base
    pub fn set_write_cache_unaligned_percent__base(&mut self, value: u64) {
        self.write_cache_unaligned_percent__base = Some(value);
    }

    /// Gets the value of WriteCacheUnalignedPercent_Base
    pub fn get_write_cache_unaligned_percent__base(&self) -> Option<&u64> {
        self.write_cache_unaligned_percent__base.as_ref()
    }

    /// Sets the value of WriteCacheUntrimmedBytesPersec
    pub fn set_write_cache_untrimmed_bytes_persec(&mut self, value: u64) {
        self.write_cache_untrimmed_bytes_persec = Some(value);
    }

    /// Gets the value of WriteCacheUntrimmedBytesPersec
    pub fn get_write_cache_untrimmed_bytes_persec(&self) -> Option<&u64> {
        self.write_cache_untrimmed_bytes_persec.as_ref()
    }

    /// Sets the value of WriteCacheUntrimmedPercent
    pub fn set_write_cache_untrimmed_percent(&mut self, value: u64) {
        self.write_cache_untrimmed_percent = Some(value);
    }

    /// Gets the value of WriteCacheUntrimmedPercent
    pub fn get_write_cache_untrimmed_percent(&self) -> Option<&u64> {
        self.write_cache_untrimmed_percent.as_ref()
    }

    /// Sets the value of WriteCacheUntrimmedPercent_Base
    pub fn set_write_cache_untrimmed_percent__base(&mut self, value: u64) {
        self.write_cache_untrimmed_percent__base = Some(value);
    }

    /// Gets the value of WriteCacheUntrimmedPercent_Base
    pub fn get_write_cache_untrimmed_percent__base(&self) -> Option<&u64> {
        self.write_cache_untrimmed_percent__base.as_ref()
    }
}

