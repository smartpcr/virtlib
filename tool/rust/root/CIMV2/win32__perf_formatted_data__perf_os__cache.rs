// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_PerfOS_Cache struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_PerfOS_Cache {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "AsyncCopyReadsPersec")]
    pub async_copy_reads_persec: Option<u32>,

/// 
    #[serde(rename = "AsyncDataMapsPersec")]
    pub async_data_maps_persec: Option<u32>,

/// 
    #[serde(rename = "AsyncFastReadsPersec")]
    pub async_fast_reads_persec: Option<u32>,

/// 
    #[serde(rename = "AsyncMDLReadsPersec")]
    pub async_mdlreads_persec: Option<u32>,

/// 
    #[serde(rename = "AsyncPinReadsPersec")]
    pub async_pin_reads_persec: Option<u32>,

/// 
    #[serde(rename = "CopyReadHitsPercent")]
    pub copy_read_hits_percent: Option<u32>,

/// 
    #[serde(rename = "CopyReadsPersec")]
    pub copy_reads_persec: Option<u32>,

/// 
    #[serde(rename = "DataFlushesPersec")]
    pub data_flushes_persec: Option<u32>,

/// 
    #[serde(rename = "DataFlushPagesPersec")]
    pub data_flush_pages_persec: Option<u32>,

/// 
    #[serde(rename = "DataMapHitsPercent")]
    pub data_map_hits_percent: Option<u32>,

/// 
    #[serde(rename = "DataMapPinsPersec")]
    pub data_map_pins_persec: Option<u32>,

/// 
    #[serde(rename = "DataMapsPersec")]
    pub data_maps_persec: Option<u32>,

/// 
    #[serde(rename = "DirtyPages")]
    pub dirty_pages: Option<u64>,

/// 
    #[serde(rename = "DirtyPageThreshold")]
    pub dirty_page_threshold: Option<u64>,

/// 
    #[serde(rename = "FastReadNotPossiblesPersec")]
    pub fast_read_not_possibles_persec: Option<u32>,

/// 
    #[serde(rename = "FastReadResourceMissesPersec")]
    pub fast_read_resource_misses_persec: Option<u32>,

/// 
    #[serde(rename = "FastReadsPersec")]
    pub fast_reads_persec: Option<u32>,

/// 
    #[serde(rename = "LazyWriteFlushesPersec")]
    pub lazy_write_flushes_persec: Option<u32>,

/// 
    #[serde(rename = "LazyWritePagesPersec")]
    pub lazy_write_pages_persec: Option<u32>,

/// 
    #[serde(rename = "MDLReadHitsPercent")]
    pub mdlread_hits_percent: Option<u32>,

/// 
    #[serde(rename = "MDLReadsPersec")]
    pub mdlreads_persec: Option<u32>,

/// 
    #[serde(rename = "PinReadHitsPercent")]
    pub pin_read_hits_percent: Option<u32>,

/// 
    #[serde(rename = "PinReadsPersec")]
    pub pin_reads_persec: Option<u32>,

/// 
    #[serde(rename = "ReadAheadsPersec")]
    pub read_aheads_persec: Option<u32>,

/// 
    #[serde(rename = "SyncCopyReadsPersec")]
    pub sync_copy_reads_persec: Option<u32>,

/// 
    #[serde(rename = "SyncDataMapsPersec")]
    pub sync_data_maps_persec: Option<u32>,

/// 
    #[serde(rename = "SyncFastReadsPersec")]
    pub sync_fast_reads_persec: Option<u32>,

/// 
    #[serde(rename = "SyncMDLReadsPersec")]
    pub sync_mdlreads_persec: Option<u32>,

/// 
    #[serde(rename = "SyncPinReadsPersec")]
    pub sync_pin_reads_persec: Option<u32>,
}

impl Win32_PerfFormattedData_PerfOS_Cache {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            async_copy_reads_persec: None,
            async_data_maps_persec: None,
            async_fast_reads_persec: None,
            async_mdlreads_persec: None,
            async_pin_reads_persec: None,
            copy_read_hits_percent: None,
            copy_reads_persec: None,
            data_flushes_persec: None,
            data_flush_pages_persec: None,
            data_map_hits_percent: None,
            data_map_pins_persec: None,
            data_maps_persec: None,
            dirty_pages: None,
            dirty_page_threshold: None,
            fast_read_not_possibles_persec: None,
            fast_read_resource_misses_persec: None,
            fast_reads_persec: None,
            lazy_write_flushes_persec: None,
            lazy_write_pages_persec: None,
            mdlread_hits_percent: None,
            mdlreads_persec: None,
            pin_read_hits_percent: None,
            pin_reads_persec: None,
            read_aheads_persec: None,
            sync_copy_reads_persec: None,
            sync_data_maps_persec: None,
            sync_fast_reads_persec: None,
            sync_mdlreads_persec: None,
            sync_pin_reads_persec: None,
        }
    }


    /// Sets the value of AsyncCopyReadsPersec
    pub fn set_async_copy_reads_persec(&mut self, value: u32) {
        self.async_copy_reads_persec = Some(value);
    }

    /// Gets the value of AsyncCopyReadsPersec
    pub fn get_async_copy_reads_persec(&self) -> Option<&u32> {
        self.async_copy_reads_persec.as_ref()
    }

    /// Sets the value of AsyncDataMapsPersec
    pub fn set_async_data_maps_persec(&mut self, value: u32) {
        self.async_data_maps_persec = Some(value);
    }

    /// Gets the value of AsyncDataMapsPersec
    pub fn get_async_data_maps_persec(&self) -> Option<&u32> {
        self.async_data_maps_persec.as_ref()
    }

    /// Sets the value of AsyncFastReadsPersec
    pub fn set_async_fast_reads_persec(&mut self, value: u32) {
        self.async_fast_reads_persec = Some(value);
    }

    /// Gets the value of AsyncFastReadsPersec
    pub fn get_async_fast_reads_persec(&self) -> Option<&u32> {
        self.async_fast_reads_persec.as_ref()
    }

    /// Sets the value of AsyncMDLReadsPersec
    pub fn set_async_mdlreads_persec(&mut self, value: u32) {
        self.async_mdlreads_persec = Some(value);
    }

    /// Gets the value of AsyncMDLReadsPersec
    pub fn get_async_mdlreads_persec(&self) -> Option<&u32> {
        self.async_mdlreads_persec.as_ref()
    }

    /// Sets the value of AsyncPinReadsPersec
    pub fn set_async_pin_reads_persec(&mut self, value: u32) {
        self.async_pin_reads_persec = Some(value);
    }

    /// Gets the value of AsyncPinReadsPersec
    pub fn get_async_pin_reads_persec(&self) -> Option<&u32> {
        self.async_pin_reads_persec.as_ref()
    }

    /// Sets the value of CopyReadHitsPercent
    pub fn set_copy_read_hits_percent(&mut self, value: u32) {
        self.copy_read_hits_percent = Some(value);
    }

    /// Gets the value of CopyReadHitsPercent
    pub fn get_copy_read_hits_percent(&self) -> Option<&u32> {
        self.copy_read_hits_percent.as_ref()
    }

    /// Sets the value of CopyReadsPersec
    pub fn set_copy_reads_persec(&mut self, value: u32) {
        self.copy_reads_persec = Some(value);
    }

    /// Gets the value of CopyReadsPersec
    pub fn get_copy_reads_persec(&self) -> Option<&u32> {
        self.copy_reads_persec.as_ref()
    }

    /// Sets the value of DataFlushesPersec
    pub fn set_data_flushes_persec(&mut self, value: u32) {
        self.data_flushes_persec = Some(value);
    }

    /// Gets the value of DataFlushesPersec
    pub fn get_data_flushes_persec(&self) -> Option<&u32> {
        self.data_flushes_persec.as_ref()
    }

    /// Sets the value of DataFlushPagesPersec
    pub fn set_data_flush_pages_persec(&mut self, value: u32) {
        self.data_flush_pages_persec = Some(value);
    }

    /// Gets the value of DataFlushPagesPersec
    pub fn get_data_flush_pages_persec(&self) -> Option<&u32> {
        self.data_flush_pages_persec.as_ref()
    }

    /// Sets the value of DataMapHitsPercent
    pub fn set_data_map_hits_percent(&mut self, value: u32) {
        self.data_map_hits_percent = Some(value);
    }

    /// Gets the value of DataMapHitsPercent
    pub fn get_data_map_hits_percent(&self) -> Option<&u32> {
        self.data_map_hits_percent.as_ref()
    }

    /// Sets the value of DataMapPinsPersec
    pub fn set_data_map_pins_persec(&mut self, value: u32) {
        self.data_map_pins_persec = Some(value);
    }

    /// Gets the value of DataMapPinsPersec
    pub fn get_data_map_pins_persec(&self) -> Option<&u32> {
        self.data_map_pins_persec.as_ref()
    }

    /// Sets the value of DataMapsPersec
    pub fn set_data_maps_persec(&mut self, value: u32) {
        self.data_maps_persec = Some(value);
    }

    /// Gets the value of DataMapsPersec
    pub fn get_data_maps_persec(&self) -> Option<&u32> {
        self.data_maps_persec.as_ref()
    }

    /// Sets the value of DirtyPages
    pub fn set_dirty_pages(&mut self, value: u64) {
        self.dirty_pages = Some(value);
    }

    /// Gets the value of DirtyPages
    pub fn get_dirty_pages(&self) -> Option<&u64> {
        self.dirty_pages.as_ref()
    }

    /// Sets the value of DirtyPageThreshold
    pub fn set_dirty_page_threshold(&mut self, value: u64) {
        self.dirty_page_threshold = Some(value);
    }

    /// Gets the value of DirtyPageThreshold
    pub fn get_dirty_page_threshold(&self) -> Option<&u64> {
        self.dirty_page_threshold.as_ref()
    }

    /// Sets the value of FastReadNotPossiblesPersec
    pub fn set_fast_read_not_possibles_persec(&mut self, value: u32) {
        self.fast_read_not_possibles_persec = Some(value);
    }

    /// Gets the value of FastReadNotPossiblesPersec
    pub fn get_fast_read_not_possibles_persec(&self) -> Option<&u32> {
        self.fast_read_not_possibles_persec.as_ref()
    }

    /// Sets the value of FastReadResourceMissesPersec
    pub fn set_fast_read_resource_misses_persec(&mut self, value: u32) {
        self.fast_read_resource_misses_persec = Some(value);
    }

    /// Gets the value of FastReadResourceMissesPersec
    pub fn get_fast_read_resource_misses_persec(&self) -> Option<&u32> {
        self.fast_read_resource_misses_persec.as_ref()
    }

    /// Sets the value of FastReadsPersec
    pub fn set_fast_reads_persec(&mut self, value: u32) {
        self.fast_reads_persec = Some(value);
    }

    /// Gets the value of FastReadsPersec
    pub fn get_fast_reads_persec(&self) -> Option<&u32> {
        self.fast_reads_persec.as_ref()
    }

    /// Sets the value of LazyWriteFlushesPersec
    pub fn set_lazy_write_flushes_persec(&mut self, value: u32) {
        self.lazy_write_flushes_persec = Some(value);
    }

    /// Gets the value of LazyWriteFlushesPersec
    pub fn get_lazy_write_flushes_persec(&self) -> Option<&u32> {
        self.lazy_write_flushes_persec.as_ref()
    }

    /// Sets the value of LazyWritePagesPersec
    pub fn set_lazy_write_pages_persec(&mut self, value: u32) {
        self.lazy_write_pages_persec = Some(value);
    }

    /// Gets the value of LazyWritePagesPersec
    pub fn get_lazy_write_pages_persec(&self) -> Option<&u32> {
        self.lazy_write_pages_persec.as_ref()
    }

    /// Sets the value of MDLReadHitsPercent
    pub fn set_mdlread_hits_percent(&mut self, value: u32) {
        self.mdlread_hits_percent = Some(value);
    }

    /// Gets the value of MDLReadHitsPercent
    pub fn get_mdlread_hits_percent(&self) -> Option<&u32> {
        self.mdlread_hits_percent.as_ref()
    }

    /// Sets the value of MDLReadsPersec
    pub fn set_mdlreads_persec(&mut self, value: u32) {
        self.mdlreads_persec = Some(value);
    }

    /// Gets the value of MDLReadsPersec
    pub fn get_mdlreads_persec(&self) -> Option<&u32> {
        self.mdlreads_persec.as_ref()
    }

    /// Sets the value of PinReadHitsPercent
    pub fn set_pin_read_hits_percent(&mut self, value: u32) {
        self.pin_read_hits_percent = Some(value);
    }

    /// Gets the value of PinReadHitsPercent
    pub fn get_pin_read_hits_percent(&self) -> Option<&u32> {
        self.pin_read_hits_percent.as_ref()
    }

    /// Sets the value of PinReadsPersec
    pub fn set_pin_reads_persec(&mut self, value: u32) {
        self.pin_reads_persec = Some(value);
    }

    /// Gets the value of PinReadsPersec
    pub fn get_pin_reads_persec(&self) -> Option<&u32> {
        self.pin_reads_persec.as_ref()
    }

    /// Sets the value of ReadAheadsPersec
    pub fn set_read_aheads_persec(&mut self, value: u32) {
        self.read_aheads_persec = Some(value);
    }

    /// Gets the value of ReadAheadsPersec
    pub fn get_read_aheads_persec(&self) -> Option<&u32> {
        self.read_aheads_persec.as_ref()
    }

    /// Sets the value of SyncCopyReadsPersec
    pub fn set_sync_copy_reads_persec(&mut self, value: u32) {
        self.sync_copy_reads_persec = Some(value);
    }

    /// Gets the value of SyncCopyReadsPersec
    pub fn get_sync_copy_reads_persec(&self) -> Option<&u32> {
        self.sync_copy_reads_persec.as_ref()
    }

    /// Sets the value of SyncDataMapsPersec
    pub fn set_sync_data_maps_persec(&mut self, value: u32) {
        self.sync_data_maps_persec = Some(value);
    }

    /// Gets the value of SyncDataMapsPersec
    pub fn get_sync_data_maps_persec(&self) -> Option<&u32> {
        self.sync_data_maps_persec.as_ref()
    }

    /// Sets the value of SyncFastReadsPersec
    pub fn set_sync_fast_reads_persec(&mut self, value: u32) {
        self.sync_fast_reads_persec = Some(value);
    }

    /// Gets the value of SyncFastReadsPersec
    pub fn get_sync_fast_reads_persec(&self) -> Option<&u32> {
        self.sync_fast_reads_persec.as_ref()
    }

    /// Sets the value of SyncMDLReadsPersec
    pub fn set_sync_mdlreads_persec(&mut self, value: u32) {
        self.sync_mdlreads_persec = Some(value);
    }

    /// Gets the value of SyncMDLReadsPersec
    pub fn get_sync_mdlreads_persec(&self) -> Option<&u32> {
        self.sync_mdlreads_persec.as_ref()
    }

    /// Sets the value of SyncPinReadsPersec
    pub fn set_sync_pin_reads_persec(&mut self, value: u32) {
        self.sync_pin_reads_persec = Some(value);
    }

    /// Gets the value of SyncPinReadsPersec
    pub fn get_sync_pin_reads_persec(&self) -> Option<&u32> {
        self.sync_pin_reads_persec.as_ref()
    }
}

