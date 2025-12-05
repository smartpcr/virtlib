// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_ClusBfltPerfProvider_ClusterStorageHybridDisks struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_ClusBfltPerfProvider_ClusterStorageHybridDisks {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "BindingAttributes")]
    pub binding_attributes: Option<u64>,

/// 
    #[serde(rename = "CacheFirstHitPopulatedBytes")]
    pub cache_first_hit_populated_bytes: Option<u64>,

/// 
    #[serde(rename = "CacheFirstHitPopulatedBytesPersec")]
    pub cache_first_hit_populated_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "CacheFirstHitWrittenBytes")]
    pub cache_first_hit_written_bytes: Option<u64>,

/// 
    #[serde(rename = "CacheFirstHitWrittenBytesPersec")]
    pub cache_first_hit_written_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "CacheHitReadBytes")]
    pub cache_hit_read_bytes: Option<u64>,

/// 
    #[serde(rename = "CacheHitReadBytesPersec")]
    pub cache_hit_read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "CacheHitReads")]
    pub cache_hit_reads: Option<u64>,

/// 
    #[serde(rename = "CacheHitReadsPersec")]
    pub cache_hit_reads_persec: Option<u64>,

/// 
    #[serde(rename = "CacheMissReadBytes")]
    pub cache_miss_read_bytes: Option<u64>,

/// 
    #[serde(rename = "CacheMissReadBytesPersec")]
    pub cache_miss_read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "CacheMissReads")]
    pub cache_miss_reads: Option<u64>,

/// 
    #[serde(rename = "CacheMissReadsPersec")]
    pub cache_miss_reads_persec: Option<u64>,

/// 
    #[serde(rename = "CachePages")]
    pub cache_pages: Option<u64>,

/// 
    #[serde(rename = "CachePagesDirty")]
    pub cache_pages_dirty: Option<u64>,

/// 
    #[serde(rename = "CachePagesDirtyHot")]
    pub cache_pages_dirty_hot: Option<u64>,

/// 
    #[serde(rename = "CachePagesDiscardIgnored")]
    pub cache_pages_discard_ignored: Option<u64>,

/// 
    #[serde(rename = "CachePagesL2")]
    pub cache_pages_l2: Option<u64>,

/// 
    #[serde(rename = "CachePopulateBytes")]
    pub cache_populate_bytes: Option<u64>,

/// 
    #[serde(rename = "CachePopulateBytesPersec")]
    pub cache_populate_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "CacheWriteBytes")]
    pub cache_write_bytes: Option<u64>,

/// 
    #[serde(rename = "CacheWriteBytesPersec")]
    pub cache_write_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "CacheWrites")]
    pub cache_writes: Option<u64>,

/// 
    #[serde(rename = "CacheWritesPersec")]
    pub cache_writes_persec: Option<u64>,

/// 
    #[serde(rename = "DestageBytes")]
    pub destage_bytes: Option<u64>,

/// 
    #[serde(rename = "DestageBytesPersec")]
    pub destage_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "DestageTransfers")]
    pub destage_transfers: Option<u64>,

/// 
    #[serde(rename = "DestageTransfersPersec")]
    pub destage_transfers_persec: Option<u64>,

/// 
    #[serde(rename = "DirectReadBytes")]
    pub direct_read_bytes: Option<u64>,

/// 
    #[serde(rename = "DirectReadBytesPersec")]
    pub direct_read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "DirectReads")]
    pub direct_reads: Option<u64>,

/// 
    #[serde(rename = "DirectReadsPersec")]
    pub direct_reads_persec: Option<u64>,

/// 
    #[serde(rename = "DirectWriteBytes")]
    pub direct_write_bytes: Option<u64>,

/// 
    #[serde(rename = "DirectWriteBytesPersec")]
    pub direct_write_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "DirectWrites")]
    pub direct_writes: Option<u64>,

/// 
    #[serde(rename = "DirectWritesPersec")]
    pub direct_writes_persec: Option<u64>,

/// 
    #[serde(rename = "DirtyReadBytes")]
    pub dirty_read_bytes: Option<u64>,

/// 
    #[serde(rename = "DirtyReadBytesPersec")]
    pub dirty_read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "DirtySlots")]
    pub dirty_slots: Option<u64>,

/// 
    #[serde(rename = "DirtySlotsExpands")]
    pub dirty_slots_expands: Option<u64>,

/// 
    #[serde(rename = "DirtySlotsExpandsPersec")]
    pub dirty_slots_expands_persec: Option<u64>,

/// 
    #[serde(rename = "DiskBytes")]
    pub disk_bytes: Option<u64>,

/// 
    #[serde(rename = "DiskBytesPersec")]
    pub disk_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "DiskReadBytes")]
    pub disk_read_bytes: Option<u64>,

/// 
    #[serde(rename = "DiskReadBytesPersec")]
    pub disk_read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "DiskReads")]
    pub disk_reads: Option<u64>,

/// 
    #[serde(rename = "DiskReadsPersec")]
    pub disk_reads_persec: Option<u64>,

/// 
    #[serde(rename = "DiskTransfers")]
    pub disk_transfers: Option<u64>,

/// 
    #[serde(rename = "DiskTransfersPersec")]
    pub disk_transfers_persec: Option<u64>,

/// 
    #[serde(rename = "DiskWriteBytes")]
    pub disk_write_bytes: Option<u64>,

/// 
    #[serde(rename = "DiskWriteBytesPersec")]
    pub disk_write_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "DiskWrites")]
    pub disk_writes: Option<u64>,

/// 
    #[serde(rename = "DiskWritesPersec")]
    pub disk_writes_persec: Option<u64>,

/// 
    #[serde(rename = "MissingSlots")]
    pub missing_slots: Option<u64>,

/// 
    #[serde(rename = "RateDiskCacheReads")]
    pub rate_disk_cache_reads: Option<u64>,

/// 
    #[serde(rename = "RateDiskCacheWrites")]
    pub rate_disk_cache_writes: Option<u64>,

/// 
    #[serde(rename = "ReadErrorsMedia")]
    pub read_errors_media: Option<u64>,

/// 
    #[serde(rename = "ReadErrorsTimeout")]
    pub read_errors_timeout: Option<u64>,

/// 
    #[serde(rename = "ReadErrorsTotal")]
    pub read_errors_total: Option<u64>,

/// 
    #[serde(rename = "WriteErrorsMedia")]
    pub write_errors_media: Option<u64>,

/// 
    #[serde(rename = "WriteErrorsTimeout")]
    pub write_errors_timeout: Option<u64>,

/// 
    #[serde(rename = "WriteErrorsTotal")]
    pub write_errors_total: Option<u64>,
}

impl Win32_PerfFormattedData_ClusBfltPerfProvider_ClusterStorageHybridDisks {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            binding_attributes: None,
            cache_first_hit_populated_bytes: None,
            cache_first_hit_populated_bytes_persec: None,
            cache_first_hit_written_bytes: None,
            cache_first_hit_written_bytes_persec: None,
            cache_hit_read_bytes: None,
            cache_hit_read_bytes_persec: None,
            cache_hit_reads: None,
            cache_hit_reads_persec: None,
            cache_miss_read_bytes: None,
            cache_miss_read_bytes_persec: None,
            cache_miss_reads: None,
            cache_miss_reads_persec: None,
            cache_pages: None,
            cache_pages_dirty: None,
            cache_pages_dirty_hot: None,
            cache_pages_discard_ignored: None,
            cache_pages_l2: None,
            cache_populate_bytes: None,
            cache_populate_bytes_persec: None,
            cache_write_bytes: None,
            cache_write_bytes_persec: None,
            cache_writes: None,
            cache_writes_persec: None,
            destage_bytes: None,
            destage_bytes_persec: None,
            destage_transfers: None,
            destage_transfers_persec: None,
            direct_read_bytes: None,
            direct_read_bytes_persec: None,
            direct_reads: None,
            direct_reads_persec: None,
            direct_write_bytes: None,
            direct_write_bytes_persec: None,
            direct_writes: None,
            direct_writes_persec: None,
            dirty_read_bytes: None,
            dirty_read_bytes_persec: None,
            dirty_slots: None,
            dirty_slots_expands: None,
            dirty_slots_expands_persec: None,
            disk_bytes: None,
            disk_bytes_persec: None,
            disk_read_bytes: None,
            disk_read_bytes_persec: None,
            disk_reads: None,
            disk_reads_persec: None,
            disk_transfers: None,
            disk_transfers_persec: None,
            disk_write_bytes: None,
            disk_write_bytes_persec: None,
            disk_writes: None,
            disk_writes_persec: None,
            missing_slots: None,
            rate_disk_cache_reads: None,
            rate_disk_cache_writes: None,
            read_errors_media: None,
            read_errors_timeout: None,
            read_errors_total: None,
            write_errors_media: None,
            write_errors_timeout: None,
            write_errors_total: None,
        }
    }


    /// Sets the value of BindingAttributes
    pub fn set_binding_attributes(&mut self, value: u64) {
        self.binding_attributes = Some(value);
    }

    /// Gets the value of BindingAttributes
    pub fn get_binding_attributes(&self) -> Option<&u64> {
        self.binding_attributes.as_ref()
    }

    /// Sets the value of CacheFirstHitPopulatedBytes
    pub fn set_cache_first_hit_populated_bytes(&mut self, value: u64) {
        self.cache_first_hit_populated_bytes = Some(value);
    }

    /// Gets the value of CacheFirstHitPopulatedBytes
    pub fn get_cache_first_hit_populated_bytes(&self) -> Option<&u64> {
        self.cache_first_hit_populated_bytes.as_ref()
    }

    /// Sets the value of CacheFirstHitPopulatedBytesPersec
    pub fn set_cache_first_hit_populated_bytes_persec(&mut self, value: u64) {
        self.cache_first_hit_populated_bytes_persec = Some(value);
    }

    /// Gets the value of CacheFirstHitPopulatedBytesPersec
    pub fn get_cache_first_hit_populated_bytes_persec(&self) -> Option<&u64> {
        self.cache_first_hit_populated_bytes_persec.as_ref()
    }

    /// Sets the value of CacheFirstHitWrittenBytes
    pub fn set_cache_first_hit_written_bytes(&mut self, value: u64) {
        self.cache_first_hit_written_bytes = Some(value);
    }

    /// Gets the value of CacheFirstHitWrittenBytes
    pub fn get_cache_first_hit_written_bytes(&self) -> Option<&u64> {
        self.cache_first_hit_written_bytes.as_ref()
    }

    /// Sets the value of CacheFirstHitWrittenBytesPersec
    pub fn set_cache_first_hit_written_bytes_persec(&mut self, value: u64) {
        self.cache_first_hit_written_bytes_persec = Some(value);
    }

    /// Gets the value of CacheFirstHitWrittenBytesPersec
    pub fn get_cache_first_hit_written_bytes_persec(&self) -> Option<&u64> {
        self.cache_first_hit_written_bytes_persec.as_ref()
    }

    /// Sets the value of CacheHitReadBytes
    pub fn set_cache_hit_read_bytes(&mut self, value: u64) {
        self.cache_hit_read_bytes = Some(value);
    }

    /// Gets the value of CacheHitReadBytes
    pub fn get_cache_hit_read_bytes(&self) -> Option<&u64> {
        self.cache_hit_read_bytes.as_ref()
    }

    /// Sets the value of CacheHitReadBytesPersec
    pub fn set_cache_hit_read_bytes_persec(&mut self, value: u64) {
        self.cache_hit_read_bytes_persec = Some(value);
    }

    /// Gets the value of CacheHitReadBytesPersec
    pub fn get_cache_hit_read_bytes_persec(&self) -> Option<&u64> {
        self.cache_hit_read_bytes_persec.as_ref()
    }

    /// Sets the value of CacheHitReads
    pub fn set_cache_hit_reads(&mut self, value: u64) {
        self.cache_hit_reads = Some(value);
    }

    /// Gets the value of CacheHitReads
    pub fn get_cache_hit_reads(&self) -> Option<&u64> {
        self.cache_hit_reads.as_ref()
    }

    /// Sets the value of CacheHitReadsPersec
    pub fn set_cache_hit_reads_persec(&mut self, value: u64) {
        self.cache_hit_reads_persec = Some(value);
    }

    /// Gets the value of CacheHitReadsPersec
    pub fn get_cache_hit_reads_persec(&self) -> Option<&u64> {
        self.cache_hit_reads_persec.as_ref()
    }

    /// Sets the value of CacheMissReadBytes
    pub fn set_cache_miss_read_bytes(&mut self, value: u64) {
        self.cache_miss_read_bytes = Some(value);
    }

    /// Gets the value of CacheMissReadBytes
    pub fn get_cache_miss_read_bytes(&self) -> Option<&u64> {
        self.cache_miss_read_bytes.as_ref()
    }

    /// Sets the value of CacheMissReadBytesPersec
    pub fn set_cache_miss_read_bytes_persec(&mut self, value: u64) {
        self.cache_miss_read_bytes_persec = Some(value);
    }

    /// Gets the value of CacheMissReadBytesPersec
    pub fn get_cache_miss_read_bytes_persec(&self) -> Option<&u64> {
        self.cache_miss_read_bytes_persec.as_ref()
    }

    /// Sets the value of CacheMissReads
    pub fn set_cache_miss_reads(&mut self, value: u64) {
        self.cache_miss_reads = Some(value);
    }

    /// Gets the value of CacheMissReads
    pub fn get_cache_miss_reads(&self) -> Option<&u64> {
        self.cache_miss_reads.as_ref()
    }

    /// Sets the value of CacheMissReadsPersec
    pub fn set_cache_miss_reads_persec(&mut self, value: u64) {
        self.cache_miss_reads_persec = Some(value);
    }

    /// Gets the value of CacheMissReadsPersec
    pub fn get_cache_miss_reads_persec(&self) -> Option<&u64> {
        self.cache_miss_reads_persec.as_ref()
    }

    /// Sets the value of CachePages
    pub fn set_cache_pages(&mut self, value: u64) {
        self.cache_pages = Some(value);
    }

    /// Gets the value of CachePages
    pub fn get_cache_pages(&self) -> Option<&u64> {
        self.cache_pages.as_ref()
    }

    /// Sets the value of CachePagesDirty
    pub fn set_cache_pages_dirty(&mut self, value: u64) {
        self.cache_pages_dirty = Some(value);
    }

    /// Gets the value of CachePagesDirty
    pub fn get_cache_pages_dirty(&self) -> Option<&u64> {
        self.cache_pages_dirty.as_ref()
    }

    /// Sets the value of CachePagesDirtyHot
    pub fn set_cache_pages_dirty_hot(&mut self, value: u64) {
        self.cache_pages_dirty_hot = Some(value);
    }

    /// Gets the value of CachePagesDirtyHot
    pub fn get_cache_pages_dirty_hot(&self) -> Option<&u64> {
        self.cache_pages_dirty_hot.as_ref()
    }

    /// Sets the value of CachePagesDiscardIgnored
    pub fn set_cache_pages_discard_ignored(&mut self, value: u64) {
        self.cache_pages_discard_ignored = Some(value);
    }

    /// Gets the value of CachePagesDiscardIgnored
    pub fn get_cache_pages_discard_ignored(&self) -> Option<&u64> {
        self.cache_pages_discard_ignored.as_ref()
    }

    /// Sets the value of CachePagesL2
    pub fn set_cache_pages_l2(&mut self, value: u64) {
        self.cache_pages_l2 = Some(value);
    }

    /// Gets the value of CachePagesL2
    pub fn get_cache_pages_l2(&self) -> Option<&u64> {
        self.cache_pages_l2.as_ref()
    }

    /// Sets the value of CachePopulateBytes
    pub fn set_cache_populate_bytes(&mut self, value: u64) {
        self.cache_populate_bytes = Some(value);
    }

    /// Gets the value of CachePopulateBytes
    pub fn get_cache_populate_bytes(&self) -> Option<&u64> {
        self.cache_populate_bytes.as_ref()
    }

    /// Sets the value of CachePopulateBytesPersec
    pub fn set_cache_populate_bytes_persec(&mut self, value: u64) {
        self.cache_populate_bytes_persec = Some(value);
    }

    /// Gets the value of CachePopulateBytesPersec
    pub fn get_cache_populate_bytes_persec(&self) -> Option<&u64> {
        self.cache_populate_bytes_persec.as_ref()
    }

    /// Sets the value of CacheWriteBytes
    pub fn set_cache_write_bytes(&mut self, value: u64) {
        self.cache_write_bytes = Some(value);
    }

    /// Gets the value of CacheWriteBytes
    pub fn get_cache_write_bytes(&self) -> Option<&u64> {
        self.cache_write_bytes.as_ref()
    }

    /// Sets the value of CacheWriteBytesPersec
    pub fn set_cache_write_bytes_persec(&mut self, value: u64) {
        self.cache_write_bytes_persec = Some(value);
    }

    /// Gets the value of CacheWriteBytesPersec
    pub fn get_cache_write_bytes_persec(&self) -> Option<&u64> {
        self.cache_write_bytes_persec.as_ref()
    }

    /// Sets the value of CacheWrites
    pub fn set_cache_writes(&mut self, value: u64) {
        self.cache_writes = Some(value);
    }

    /// Gets the value of CacheWrites
    pub fn get_cache_writes(&self) -> Option<&u64> {
        self.cache_writes.as_ref()
    }

    /// Sets the value of CacheWritesPersec
    pub fn set_cache_writes_persec(&mut self, value: u64) {
        self.cache_writes_persec = Some(value);
    }

    /// Gets the value of CacheWritesPersec
    pub fn get_cache_writes_persec(&self) -> Option<&u64> {
        self.cache_writes_persec.as_ref()
    }

    /// Sets the value of DestageBytes
    pub fn set_destage_bytes(&mut self, value: u64) {
        self.destage_bytes = Some(value);
    }

    /// Gets the value of DestageBytes
    pub fn get_destage_bytes(&self) -> Option<&u64> {
        self.destage_bytes.as_ref()
    }

    /// Sets the value of DestageBytesPersec
    pub fn set_destage_bytes_persec(&mut self, value: u64) {
        self.destage_bytes_persec = Some(value);
    }

    /// Gets the value of DestageBytesPersec
    pub fn get_destage_bytes_persec(&self) -> Option<&u64> {
        self.destage_bytes_persec.as_ref()
    }

    /// Sets the value of DestageTransfers
    pub fn set_destage_transfers(&mut self, value: u64) {
        self.destage_transfers = Some(value);
    }

    /// Gets the value of DestageTransfers
    pub fn get_destage_transfers(&self) -> Option<&u64> {
        self.destage_transfers.as_ref()
    }

    /// Sets the value of DestageTransfersPersec
    pub fn set_destage_transfers_persec(&mut self, value: u64) {
        self.destage_transfers_persec = Some(value);
    }

    /// Gets the value of DestageTransfersPersec
    pub fn get_destage_transfers_persec(&self) -> Option<&u64> {
        self.destage_transfers_persec.as_ref()
    }

    /// Sets the value of DirectReadBytes
    pub fn set_direct_read_bytes(&mut self, value: u64) {
        self.direct_read_bytes = Some(value);
    }

    /// Gets the value of DirectReadBytes
    pub fn get_direct_read_bytes(&self) -> Option<&u64> {
        self.direct_read_bytes.as_ref()
    }

    /// Sets the value of DirectReadBytesPersec
    pub fn set_direct_read_bytes_persec(&mut self, value: u64) {
        self.direct_read_bytes_persec = Some(value);
    }

    /// Gets the value of DirectReadBytesPersec
    pub fn get_direct_read_bytes_persec(&self) -> Option<&u64> {
        self.direct_read_bytes_persec.as_ref()
    }

    /// Sets the value of DirectReads
    pub fn set_direct_reads(&mut self, value: u64) {
        self.direct_reads = Some(value);
    }

    /// Gets the value of DirectReads
    pub fn get_direct_reads(&self) -> Option<&u64> {
        self.direct_reads.as_ref()
    }

    /// Sets the value of DirectReadsPersec
    pub fn set_direct_reads_persec(&mut self, value: u64) {
        self.direct_reads_persec = Some(value);
    }

    /// Gets the value of DirectReadsPersec
    pub fn get_direct_reads_persec(&self) -> Option<&u64> {
        self.direct_reads_persec.as_ref()
    }

    /// Sets the value of DirectWriteBytes
    pub fn set_direct_write_bytes(&mut self, value: u64) {
        self.direct_write_bytes = Some(value);
    }

    /// Gets the value of DirectWriteBytes
    pub fn get_direct_write_bytes(&self) -> Option<&u64> {
        self.direct_write_bytes.as_ref()
    }

    /// Sets the value of DirectWriteBytesPersec
    pub fn set_direct_write_bytes_persec(&mut self, value: u64) {
        self.direct_write_bytes_persec = Some(value);
    }

    /// Gets the value of DirectWriteBytesPersec
    pub fn get_direct_write_bytes_persec(&self) -> Option<&u64> {
        self.direct_write_bytes_persec.as_ref()
    }

    /// Sets the value of DirectWrites
    pub fn set_direct_writes(&mut self, value: u64) {
        self.direct_writes = Some(value);
    }

    /// Gets the value of DirectWrites
    pub fn get_direct_writes(&self) -> Option<&u64> {
        self.direct_writes.as_ref()
    }

    /// Sets the value of DirectWritesPersec
    pub fn set_direct_writes_persec(&mut self, value: u64) {
        self.direct_writes_persec = Some(value);
    }

    /// Gets the value of DirectWritesPersec
    pub fn get_direct_writes_persec(&self) -> Option<&u64> {
        self.direct_writes_persec.as_ref()
    }

    /// Sets the value of DirtyReadBytes
    pub fn set_dirty_read_bytes(&mut self, value: u64) {
        self.dirty_read_bytes = Some(value);
    }

    /// Gets the value of DirtyReadBytes
    pub fn get_dirty_read_bytes(&self) -> Option<&u64> {
        self.dirty_read_bytes.as_ref()
    }

    /// Sets the value of DirtyReadBytesPersec
    pub fn set_dirty_read_bytes_persec(&mut self, value: u64) {
        self.dirty_read_bytes_persec = Some(value);
    }

    /// Gets the value of DirtyReadBytesPersec
    pub fn get_dirty_read_bytes_persec(&self) -> Option<&u64> {
        self.dirty_read_bytes_persec.as_ref()
    }

    /// Sets the value of DirtySlots
    pub fn set_dirty_slots(&mut self, value: u64) {
        self.dirty_slots = Some(value);
    }

    /// Gets the value of DirtySlots
    pub fn get_dirty_slots(&self) -> Option<&u64> {
        self.dirty_slots.as_ref()
    }

    /// Sets the value of DirtySlotsExpands
    pub fn set_dirty_slots_expands(&mut self, value: u64) {
        self.dirty_slots_expands = Some(value);
    }

    /// Gets the value of DirtySlotsExpands
    pub fn get_dirty_slots_expands(&self) -> Option<&u64> {
        self.dirty_slots_expands.as_ref()
    }

    /// Sets the value of DirtySlotsExpandsPersec
    pub fn set_dirty_slots_expands_persec(&mut self, value: u64) {
        self.dirty_slots_expands_persec = Some(value);
    }

    /// Gets the value of DirtySlotsExpandsPersec
    pub fn get_dirty_slots_expands_persec(&self) -> Option<&u64> {
        self.dirty_slots_expands_persec.as_ref()
    }

    /// Sets the value of DiskBytes
    pub fn set_disk_bytes(&mut self, value: u64) {
        self.disk_bytes = Some(value);
    }

    /// Gets the value of DiskBytes
    pub fn get_disk_bytes(&self) -> Option<&u64> {
        self.disk_bytes.as_ref()
    }

    /// Sets the value of DiskBytesPersec
    pub fn set_disk_bytes_persec(&mut self, value: u64) {
        self.disk_bytes_persec = Some(value);
    }

    /// Gets the value of DiskBytesPersec
    pub fn get_disk_bytes_persec(&self) -> Option<&u64> {
        self.disk_bytes_persec.as_ref()
    }

    /// Sets the value of DiskReadBytes
    pub fn set_disk_read_bytes(&mut self, value: u64) {
        self.disk_read_bytes = Some(value);
    }

    /// Gets the value of DiskReadBytes
    pub fn get_disk_read_bytes(&self) -> Option<&u64> {
        self.disk_read_bytes.as_ref()
    }

    /// Sets the value of DiskReadBytesPersec
    pub fn set_disk_read_bytes_persec(&mut self, value: u64) {
        self.disk_read_bytes_persec = Some(value);
    }

    /// Gets the value of DiskReadBytesPersec
    pub fn get_disk_read_bytes_persec(&self) -> Option<&u64> {
        self.disk_read_bytes_persec.as_ref()
    }

    /// Sets the value of DiskReads
    pub fn set_disk_reads(&mut self, value: u64) {
        self.disk_reads = Some(value);
    }

    /// Gets the value of DiskReads
    pub fn get_disk_reads(&self) -> Option<&u64> {
        self.disk_reads.as_ref()
    }

    /// Sets the value of DiskReadsPersec
    pub fn set_disk_reads_persec(&mut self, value: u64) {
        self.disk_reads_persec = Some(value);
    }

    /// Gets the value of DiskReadsPersec
    pub fn get_disk_reads_persec(&self) -> Option<&u64> {
        self.disk_reads_persec.as_ref()
    }

    /// Sets the value of DiskTransfers
    pub fn set_disk_transfers(&mut self, value: u64) {
        self.disk_transfers = Some(value);
    }

    /// Gets the value of DiskTransfers
    pub fn get_disk_transfers(&self) -> Option<&u64> {
        self.disk_transfers.as_ref()
    }

    /// Sets the value of DiskTransfersPersec
    pub fn set_disk_transfers_persec(&mut self, value: u64) {
        self.disk_transfers_persec = Some(value);
    }

    /// Gets the value of DiskTransfersPersec
    pub fn get_disk_transfers_persec(&self) -> Option<&u64> {
        self.disk_transfers_persec.as_ref()
    }

    /// Sets the value of DiskWriteBytes
    pub fn set_disk_write_bytes(&mut self, value: u64) {
        self.disk_write_bytes = Some(value);
    }

    /// Gets the value of DiskWriteBytes
    pub fn get_disk_write_bytes(&self) -> Option<&u64> {
        self.disk_write_bytes.as_ref()
    }

    /// Sets the value of DiskWriteBytesPersec
    pub fn set_disk_write_bytes_persec(&mut self, value: u64) {
        self.disk_write_bytes_persec = Some(value);
    }

    /// Gets the value of DiskWriteBytesPersec
    pub fn get_disk_write_bytes_persec(&self) -> Option<&u64> {
        self.disk_write_bytes_persec.as_ref()
    }

    /// Sets the value of DiskWrites
    pub fn set_disk_writes(&mut self, value: u64) {
        self.disk_writes = Some(value);
    }

    /// Gets the value of DiskWrites
    pub fn get_disk_writes(&self) -> Option<&u64> {
        self.disk_writes.as_ref()
    }

    /// Sets the value of DiskWritesPersec
    pub fn set_disk_writes_persec(&mut self, value: u64) {
        self.disk_writes_persec = Some(value);
    }

    /// Gets the value of DiskWritesPersec
    pub fn get_disk_writes_persec(&self) -> Option<&u64> {
        self.disk_writes_persec.as_ref()
    }

    /// Sets the value of MissingSlots
    pub fn set_missing_slots(&mut self, value: u64) {
        self.missing_slots = Some(value);
    }

    /// Gets the value of MissingSlots
    pub fn get_missing_slots(&self) -> Option<&u64> {
        self.missing_slots.as_ref()
    }

    /// Sets the value of RateDiskCacheReads
    pub fn set_rate_disk_cache_reads(&mut self, value: u64) {
        self.rate_disk_cache_reads = Some(value);
    }

    /// Gets the value of RateDiskCacheReads
    pub fn get_rate_disk_cache_reads(&self) -> Option<&u64> {
        self.rate_disk_cache_reads.as_ref()
    }

    /// Sets the value of RateDiskCacheWrites
    pub fn set_rate_disk_cache_writes(&mut self, value: u64) {
        self.rate_disk_cache_writes = Some(value);
    }

    /// Gets the value of RateDiskCacheWrites
    pub fn get_rate_disk_cache_writes(&self) -> Option<&u64> {
        self.rate_disk_cache_writes.as_ref()
    }

    /// Sets the value of ReadErrorsMedia
    pub fn set_read_errors_media(&mut self, value: u64) {
        self.read_errors_media = Some(value);
    }

    /// Gets the value of ReadErrorsMedia
    pub fn get_read_errors_media(&self) -> Option<&u64> {
        self.read_errors_media.as_ref()
    }

    /// Sets the value of ReadErrorsTimeout
    pub fn set_read_errors_timeout(&mut self, value: u64) {
        self.read_errors_timeout = Some(value);
    }

    /// Gets the value of ReadErrorsTimeout
    pub fn get_read_errors_timeout(&self) -> Option<&u64> {
        self.read_errors_timeout.as_ref()
    }

    /// Sets the value of ReadErrorsTotal
    pub fn set_read_errors_total(&mut self, value: u64) {
        self.read_errors_total = Some(value);
    }

    /// Gets the value of ReadErrorsTotal
    pub fn get_read_errors_total(&self) -> Option<&u64> {
        self.read_errors_total.as_ref()
    }

    /// Sets the value of WriteErrorsMedia
    pub fn set_write_errors_media(&mut self, value: u64) {
        self.write_errors_media = Some(value);
    }

    /// Gets the value of WriteErrorsMedia
    pub fn get_write_errors_media(&self) -> Option<&u64> {
        self.write_errors_media.as_ref()
    }

    /// Sets the value of WriteErrorsTimeout
    pub fn set_write_errors_timeout(&mut self, value: u64) {
        self.write_errors_timeout = Some(value);
    }

    /// Gets the value of WriteErrorsTimeout
    pub fn get_write_errors_timeout(&self) -> Option<&u64> {
        self.write_errors_timeout.as_ref()
    }

    /// Sets the value of WriteErrorsTotal
    pub fn set_write_errors_total(&mut self, value: u64) {
        self.write_errors_total = Some(value);
    }

    /// Gets the value of WriteErrorsTotal
    pub fn get_write_errors_total(&self) -> Option<&u64> {
        self.write_errors_total.as_ref()
    }
}

