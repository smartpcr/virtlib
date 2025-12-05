// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_ClusBfltPerfProvider_ClusterStorageHybridDisks2 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_ClusBfltPerfProvider_ClusterStorageHybridDisks2 {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "CacheReadPopulateL1Bytes")]
    pub cache_read_populate_l1_bytes: Option<u64>,

/// 
    #[serde(rename = "CacheReadPopulateL1BytesPersec")]
    pub cache_read_populate_l1_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "CacheReadPopulateL2Bytes")]
    pub cache_read_populate_l2_bytes: Option<u64>,

/// 
    #[serde(rename = "CacheReadPopulateL2BytesPersec")]
    pub cache_read_populate_l2_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "CacheWritePopulateL0Bytes")]
    pub cache_write_populate_l0_bytes: Option<u64>,

/// 
    #[serde(rename = "CacheWritePopulateL0BytesPersec")]
    pub cache_write_populate_l0_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "CacheWritePopulateL1Bytes")]
    pub cache_write_populate_l1_bytes: Option<u64>,

/// 
    #[serde(rename = "CacheWritePopulateL1BytesPersec")]
    pub cache_write_populate_l1_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "CacheWritePopulateL2Bytes")]
    pub cache_write_populate_l2_bytes: Option<u64>,

/// 
    #[serde(rename = "CacheWritePopulateL2BytesPersec")]
    pub cache_write_populate_l2_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "HeatMapFreeMemory")]
    pub heat_map_free_memory: Option<u64>,

/// 
    #[serde(rename = "HeatMapWindow")]
    pub heat_map_window: Option<u64>,

/// 
    #[serde(rename = "RateDiskVRCReads")]
    pub rate_disk_vrcreads: Option<u64>,

/// 
    #[serde(rename = "RateDiskVRCReads_Base")]
    pub rate_disk_vrcreads__base: Option<u32>,

/// 
    #[serde(rename = "VRCHitReadBytes")]
    pub vrchit_read_bytes: Option<u64>,

/// 
    #[serde(rename = "VRCHitReadBytesPersec")]
    pub vrchit_read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "VRCHitReads")]
    pub vrchit_reads: Option<u64>,

/// 
    #[serde(rename = "VRCHitReadsPersec")]
    pub vrchit_reads_persec: Option<u64>,

/// 
    #[serde(rename = "VRCPopulateBytes")]
    pub vrcpopulate_bytes: Option<u64>,

/// 
    #[serde(rename = "VRCPopulateBytesPersec")]
    pub vrcpopulate_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "VRCPopulates")]
    pub vrcpopulates: Option<u64>,

/// 
    #[serde(rename = "VRCPopulatesPersec")]
    pub vrcpopulates_persec: Option<u64>,
}

impl Win32_PerfRawData_ClusBfltPerfProvider_ClusterStorageHybridDisks2 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            cache_read_populate_l1_bytes: None,
            cache_read_populate_l1_bytes_persec: None,
            cache_read_populate_l2_bytes: None,
            cache_read_populate_l2_bytes_persec: None,
            cache_write_populate_l0_bytes: None,
            cache_write_populate_l0_bytes_persec: None,
            cache_write_populate_l1_bytes: None,
            cache_write_populate_l1_bytes_persec: None,
            cache_write_populate_l2_bytes: None,
            cache_write_populate_l2_bytes_persec: None,
            heat_map_free_memory: None,
            heat_map_window: None,
            rate_disk_vrcreads: None,
            rate_disk_vrcreads__base: None,
            vrchit_read_bytes: None,
            vrchit_read_bytes_persec: None,
            vrchit_reads: None,
            vrchit_reads_persec: None,
            vrcpopulate_bytes: None,
            vrcpopulate_bytes_persec: None,
            vrcpopulates: None,
            vrcpopulates_persec: None,
        }
    }


    /// Sets the value of CacheReadPopulateL1Bytes
    pub fn set_cache_read_populate_l1_bytes(&mut self, value: u64) {
        self.cache_read_populate_l1_bytes = Some(value);
    }

    /// Gets the value of CacheReadPopulateL1Bytes
    pub fn get_cache_read_populate_l1_bytes(&self) -> Option<&u64> {
        self.cache_read_populate_l1_bytes.as_ref()
    }

    /// Sets the value of CacheReadPopulateL1BytesPersec
    pub fn set_cache_read_populate_l1_bytes_persec(&mut self, value: u64) {
        self.cache_read_populate_l1_bytes_persec = Some(value);
    }

    /// Gets the value of CacheReadPopulateL1BytesPersec
    pub fn get_cache_read_populate_l1_bytes_persec(&self) -> Option<&u64> {
        self.cache_read_populate_l1_bytes_persec.as_ref()
    }

    /// Sets the value of CacheReadPopulateL2Bytes
    pub fn set_cache_read_populate_l2_bytes(&mut self, value: u64) {
        self.cache_read_populate_l2_bytes = Some(value);
    }

    /// Gets the value of CacheReadPopulateL2Bytes
    pub fn get_cache_read_populate_l2_bytes(&self) -> Option<&u64> {
        self.cache_read_populate_l2_bytes.as_ref()
    }

    /// Sets the value of CacheReadPopulateL2BytesPersec
    pub fn set_cache_read_populate_l2_bytes_persec(&mut self, value: u64) {
        self.cache_read_populate_l2_bytes_persec = Some(value);
    }

    /// Gets the value of CacheReadPopulateL2BytesPersec
    pub fn get_cache_read_populate_l2_bytes_persec(&self) -> Option<&u64> {
        self.cache_read_populate_l2_bytes_persec.as_ref()
    }

    /// Sets the value of CacheWritePopulateL0Bytes
    pub fn set_cache_write_populate_l0_bytes(&mut self, value: u64) {
        self.cache_write_populate_l0_bytes = Some(value);
    }

    /// Gets the value of CacheWritePopulateL0Bytes
    pub fn get_cache_write_populate_l0_bytes(&self) -> Option<&u64> {
        self.cache_write_populate_l0_bytes.as_ref()
    }

    /// Sets the value of CacheWritePopulateL0BytesPersec
    pub fn set_cache_write_populate_l0_bytes_persec(&mut self, value: u64) {
        self.cache_write_populate_l0_bytes_persec = Some(value);
    }

    /// Gets the value of CacheWritePopulateL0BytesPersec
    pub fn get_cache_write_populate_l0_bytes_persec(&self) -> Option<&u64> {
        self.cache_write_populate_l0_bytes_persec.as_ref()
    }

    /// Sets the value of CacheWritePopulateL1Bytes
    pub fn set_cache_write_populate_l1_bytes(&mut self, value: u64) {
        self.cache_write_populate_l1_bytes = Some(value);
    }

    /// Gets the value of CacheWritePopulateL1Bytes
    pub fn get_cache_write_populate_l1_bytes(&self) -> Option<&u64> {
        self.cache_write_populate_l1_bytes.as_ref()
    }

    /// Sets the value of CacheWritePopulateL1BytesPersec
    pub fn set_cache_write_populate_l1_bytes_persec(&mut self, value: u64) {
        self.cache_write_populate_l1_bytes_persec = Some(value);
    }

    /// Gets the value of CacheWritePopulateL1BytesPersec
    pub fn get_cache_write_populate_l1_bytes_persec(&self) -> Option<&u64> {
        self.cache_write_populate_l1_bytes_persec.as_ref()
    }

    /// Sets the value of CacheWritePopulateL2Bytes
    pub fn set_cache_write_populate_l2_bytes(&mut self, value: u64) {
        self.cache_write_populate_l2_bytes = Some(value);
    }

    /// Gets the value of CacheWritePopulateL2Bytes
    pub fn get_cache_write_populate_l2_bytes(&self) -> Option<&u64> {
        self.cache_write_populate_l2_bytes.as_ref()
    }

    /// Sets the value of CacheWritePopulateL2BytesPersec
    pub fn set_cache_write_populate_l2_bytes_persec(&mut self, value: u64) {
        self.cache_write_populate_l2_bytes_persec = Some(value);
    }

    /// Gets the value of CacheWritePopulateL2BytesPersec
    pub fn get_cache_write_populate_l2_bytes_persec(&self) -> Option<&u64> {
        self.cache_write_populate_l2_bytes_persec.as_ref()
    }

    /// Sets the value of HeatMapFreeMemory
    pub fn set_heat_map_free_memory(&mut self, value: u64) {
        self.heat_map_free_memory = Some(value);
    }

    /// Gets the value of HeatMapFreeMemory
    pub fn get_heat_map_free_memory(&self) -> Option<&u64> {
        self.heat_map_free_memory.as_ref()
    }

    /// Sets the value of HeatMapWindow
    pub fn set_heat_map_window(&mut self, value: u64) {
        self.heat_map_window = Some(value);
    }

    /// Gets the value of HeatMapWindow
    pub fn get_heat_map_window(&self) -> Option<&u64> {
        self.heat_map_window.as_ref()
    }

    /// Sets the value of RateDiskVRCReads
    pub fn set_rate_disk_vrcreads(&mut self, value: u64) {
        self.rate_disk_vrcreads = Some(value);
    }

    /// Gets the value of RateDiskVRCReads
    pub fn get_rate_disk_vrcreads(&self) -> Option<&u64> {
        self.rate_disk_vrcreads.as_ref()
    }

    /// Sets the value of RateDiskVRCReads_Base
    pub fn set_rate_disk_vrcreads__base(&mut self, value: u32) {
        self.rate_disk_vrcreads__base = Some(value);
    }

    /// Gets the value of RateDiskVRCReads_Base
    pub fn get_rate_disk_vrcreads__base(&self) -> Option<&u32> {
        self.rate_disk_vrcreads__base.as_ref()
    }

    /// Sets the value of VRCHitReadBytes
    pub fn set_vrchit_read_bytes(&mut self, value: u64) {
        self.vrchit_read_bytes = Some(value);
    }

    /// Gets the value of VRCHitReadBytes
    pub fn get_vrchit_read_bytes(&self) -> Option<&u64> {
        self.vrchit_read_bytes.as_ref()
    }

    /// Sets the value of VRCHitReadBytesPersec
    pub fn set_vrchit_read_bytes_persec(&mut self, value: u64) {
        self.vrchit_read_bytes_persec = Some(value);
    }

    /// Gets the value of VRCHitReadBytesPersec
    pub fn get_vrchit_read_bytes_persec(&self) -> Option<&u64> {
        self.vrchit_read_bytes_persec.as_ref()
    }

    /// Sets the value of VRCHitReads
    pub fn set_vrchit_reads(&mut self, value: u64) {
        self.vrchit_reads = Some(value);
    }

    /// Gets the value of VRCHitReads
    pub fn get_vrchit_reads(&self) -> Option<&u64> {
        self.vrchit_reads.as_ref()
    }

    /// Sets the value of VRCHitReadsPersec
    pub fn set_vrchit_reads_persec(&mut self, value: u64) {
        self.vrchit_reads_persec = Some(value);
    }

    /// Gets the value of VRCHitReadsPersec
    pub fn get_vrchit_reads_persec(&self) -> Option<&u64> {
        self.vrchit_reads_persec.as_ref()
    }

    /// Sets the value of VRCPopulateBytes
    pub fn set_vrcpopulate_bytes(&mut self, value: u64) {
        self.vrcpopulate_bytes = Some(value);
    }

    /// Gets the value of VRCPopulateBytes
    pub fn get_vrcpopulate_bytes(&self) -> Option<&u64> {
        self.vrcpopulate_bytes.as_ref()
    }

    /// Sets the value of VRCPopulateBytesPersec
    pub fn set_vrcpopulate_bytes_persec(&mut self, value: u64) {
        self.vrcpopulate_bytes_persec = Some(value);
    }

    /// Gets the value of VRCPopulateBytesPersec
    pub fn get_vrcpopulate_bytes_persec(&self) -> Option<&u64> {
        self.vrcpopulate_bytes_persec.as_ref()
    }

    /// Sets the value of VRCPopulates
    pub fn set_vrcpopulates(&mut self, value: u64) {
        self.vrcpopulates = Some(value);
    }

    /// Gets the value of VRCPopulates
    pub fn get_vrcpopulates(&self) -> Option<&u64> {
        self.vrcpopulates.as_ref()
    }

    /// Sets the value of VRCPopulatesPersec
    pub fn set_vrcpopulates_persec(&mut self, value: u64) {
        self.vrcpopulates_persec = Some(value);
    }

    /// Gets the value of VRCPopulatesPersec
    pub fn get_vrcpopulates_persec(&self) -> Option<&u64> {
        self.vrcpopulates_persec.as_ref()
    }
}

