// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_CsvVolumePerfProvider_ClusterCSVVolumeCache struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_CsvVolumePerfProvider_ClusterCSVVolumeCache {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "CacheIOReadBytes")]
    pub cache_ioread_bytes: Option<u64>,

/// 
    #[serde(rename = "CacheIOReadBytesPersec")]
    pub cache_ioread_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "CacheRead")]
    pub cache_read: Option<u64>,

/// 
    #[serde(rename = "CacheReadPerSec")]
    pub cache_read_per_sec: Option<u64>,

/// 
    #[serde(rename = "CacheSizeConfigured")]
    pub cache_size_configured: Option<u64>,

/// 
    #[serde(rename = "CacheSizeCurrent")]
    pub cache_size_current: Option<u64>,

/// 
    #[serde(rename = "CacheState")]
    pub cache_state: Option<u64>,

/// 
    #[serde(rename = "DiskIOReadBytes")]
    pub disk_ioread_bytes: Option<u64>,

/// 
    #[serde(rename = "DiskIOReadBytesPerSec")]
    pub disk_ioread_bytes_per_sec: Option<u64>,

/// 
    #[serde(rename = "DiskIOReads")]
    pub disk_ioreads: Option<u64>,

/// 
    #[serde(rename = "DiskIOReadsPerSec")]
    pub disk_ioreads_per_sec: Option<u64>,

/// 
    #[serde(rename = "IOReadBytes")]
    pub ioread_bytes: Option<u64>,

/// 
    #[serde(rename = "IOReadBytesPerSec")]
    pub ioread_bytes_per_sec: Option<u64>,

/// 
    #[serde(rename = "IOReads")]
    pub ioreads: Option<u64>,

/// 
    #[serde(rename = "IOReadsPerSec")]
    pub ioreads_per_sec: Option<u64>,

/// 
    #[serde(rename = "LRUCacheSizeCurrent")]
    pub lrucache_size_current: Option<u64>,

/// 
    #[serde(rename = "LRUCacheSizeTarget")]
    pub lrucache_size_target: Option<u64>,

/// 
    #[serde(rename = "PartialRead")]
    pub partial_read: Option<u64>,

/// 
    #[serde(rename = "PartialReadPersec")]
    pub partial_read_persec: Option<u64>,

/// 
    #[serde(rename = "PercentCacheValid")]
    pub percent_cache_valid: Option<u64>,

/// 
    #[serde(rename = "ValidCacheSize")]
    pub valid_cache_size: Option<u64>,
}

impl Win32_PerfFormattedData_CsvVolumePerfProvider_ClusterCSVVolumeCache {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            cache_ioread_bytes: None,
            cache_ioread_bytes_persec: None,
            cache_read: None,
            cache_read_per_sec: None,
            cache_size_configured: None,
            cache_size_current: None,
            cache_state: None,
            disk_ioread_bytes: None,
            disk_ioread_bytes_per_sec: None,
            disk_ioreads: None,
            disk_ioreads_per_sec: None,
            ioread_bytes: None,
            ioread_bytes_per_sec: None,
            ioreads: None,
            ioreads_per_sec: None,
            lrucache_size_current: None,
            lrucache_size_target: None,
            partial_read: None,
            partial_read_persec: None,
            percent_cache_valid: None,
            valid_cache_size: None,
        }
    }


    /// Sets the value of CacheIOReadBytes
    pub fn set_cache_ioread_bytes(&mut self, value: u64) {
        self.cache_ioread_bytes = Some(value);
    }

    /// Gets the value of CacheIOReadBytes
    pub fn get_cache_ioread_bytes(&self) -> Option<&u64> {
        self.cache_ioread_bytes.as_ref()
    }

    /// Sets the value of CacheIOReadBytesPersec
    pub fn set_cache_ioread_bytes_persec(&mut self, value: u64) {
        self.cache_ioread_bytes_persec = Some(value);
    }

    /// Gets the value of CacheIOReadBytesPersec
    pub fn get_cache_ioread_bytes_persec(&self) -> Option<&u64> {
        self.cache_ioread_bytes_persec.as_ref()
    }

    /// Sets the value of CacheRead
    pub fn set_cache_read(&mut self, value: u64) {
        self.cache_read = Some(value);
    }

    /// Gets the value of CacheRead
    pub fn get_cache_read(&self) -> Option<&u64> {
        self.cache_read.as_ref()
    }

    /// Sets the value of CacheReadPerSec
    pub fn set_cache_read_per_sec(&mut self, value: u64) {
        self.cache_read_per_sec = Some(value);
    }

    /// Gets the value of CacheReadPerSec
    pub fn get_cache_read_per_sec(&self) -> Option<&u64> {
        self.cache_read_per_sec.as_ref()
    }

    /// Sets the value of CacheSizeConfigured
    pub fn set_cache_size_configured(&mut self, value: u64) {
        self.cache_size_configured = Some(value);
    }

    /// Gets the value of CacheSizeConfigured
    pub fn get_cache_size_configured(&self) -> Option<&u64> {
        self.cache_size_configured.as_ref()
    }

    /// Sets the value of CacheSizeCurrent
    pub fn set_cache_size_current(&mut self, value: u64) {
        self.cache_size_current = Some(value);
    }

    /// Gets the value of CacheSizeCurrent
    pub fn get_cache_size_current(&self) -> Option<&u64> {
        self.cache_size_current.as_ref()
    }

    /// Sets the value of CacheState
    pub fn set_cache_state(&mut self, value: u64) {
        self.cache_state = Some(value);
    }

    /// Gets the value of CacheState
    pub fn get_cache_state(&self) -> Option<&u64> {
        self.cache_state.as_ref()
    }

    /// Sets the value of DiskIOReadBytes
    pub fn set_disk_ioread_bytes(&mut self, value: u64) {
        self.disk_ioread_bytes = Some(value);
    }

    /// Gets the value of DiskIOReadBytes
    pub fn get_disk_ioread_bytes(&self) -> Option<&u64> {
        self.disk_ioread_bytes.as_ref()
    }

    /// Sets the value of DiskIOReadBytesPerSec
    pub fn set_disk_ioread_bytes_per_sec(&mut self, value: u64) {
        self.disk_ioread_bytes_per_sec = Some(value);
    }

    /// Gets the value of DiskIOReadBytesPerSec
    pub fn get_disk_ioread_bytes_per_sec(&self) -> Option<&u64> {
        self.disk_ioread_bytes_per_sec.as_ref()
    }

    /// Sets the value of DiskIOReads
    pub fn set_disk_ioreads(&mut self, value: u64) {
        self.disk_ioreads = Some(value);
    }

    /// Gets the value of DiskIOReads
    pub fn get_disk_ioreads(&self) -> Option<&u64> {
        self.disk_ioreads.as_ref()
    }

    /// Sets the value of DiskIOReadsPerSec
    pub fn set_disk_ioreads_per_sec(&mut self, value: u64) {
        self.disk_ioreads_per_sec = Some(value);
    }

    /// Gets the value of DiskIOReadsPerSec
    pub fn get_disk_ioreads_per_sec(&self) -> Option<&u64> {
        self.disk_ioreads_per_sec.as_ref()
    }

    /// Sets the value of IOReadBytes
    pub fn set_ioread_bytes(&mut self, value: u64) {
        self.ioread_bytes = Some(value);
    }

    /// Gets the value of IOReadBytes
    pub fn get_ioread_bytes(&self) -> Option<&u64> {
        self.ioread_bytes.as_ref()
    }

    /// Sets the value of IOReadBytesPerSec
    pub fn set_ioread_bytes_per_sec(&mut self, value: u64) {
        self.ioread_bytes_per_sec = Some(value);
    }

    /// Gets the value of IOReadBytesPerSec
    pub fn get_ioread_bytes_per_sec(&self) -> Option<&u64> {
        self.ioread_bytes_per_sec.as_ref()
    }

    /// Sets the value of IOReads
    pub fn set_ioreads(&mut self, value: u64) {
        self.ioreads = Some(value);
    }

    /// Gets the value of IOReads
    pub fn get_ioreads(&self) -> Option<&u64> {
        self.ioreads.as_ref()
    }

    /// Sets the value of IOReadsPerSec
    pub fn set_ioreads_per_sec(&mut self, value: u64) {
        self.ioreads_per_sec = Some(value);
    }

    /// Gets the value of IOReadsPerSec
    pub fn get_ioreads_per_sec(&self) -> Option<&u64> {
        self.ioreads_per_sec.as_ref()
    }

    /// Sets the value of LRUCacheSizeCurrent
    pub fn set_lrucache_size_current(&mut self, value: u64) {
        self.lrucache_size_current = Some(value);
    }

    /// Gets the value of LRUCacheSizeCurrent
    pub fn get_lrucache_size_current(&self) -> Option<&u64> {
        self.lrucache_size_current.as_ref()
    }

    /// Sets the value of LRUCacheSizeTarget
    pub fn set_lrucache_size_target(&mut self, value: u64) {
        self.lrucache_size_target = Some(value);
    }

    /// Gets the value of LRUCacheSizeTarget
    pub fn get_lrucache_size_target(&self) -> Option<&u64> {
        self.lrucache_size_target.as_ref()
    }

    /// Sets the value of PartialRead
    pub fn set_partial_read(&mut self, value: u64) {
        self.partial_read = Some(value);
    }

    /// Gets the value of PartialRead
    pub fn get_partial_read(&self) -> Option<&u64> {
        self.partial_read.as_ref()
    }

    /// Sets the value of PartialReadPersec
    pub fn set_partial_read_persec(&mut self, value: u64) {
        self.partial_read_persec = Some(value);
    }

    /// Gets the value of PartialReadPersec
    pub fn get_partial_read_persec(&self) -> Option<&u64> {
        self.partial_read_persec.as_ref()
    }

    /// Sets the value of PercentCacheValid
    pub fn set_percent_cache_valid(&mut self, value: u64) {
        self.percent_cache_valid = Some(value);
    }

    /// Gets the value of PercentCacheValid
    pub fn get_percent_cache_valid(&self) -> Option<&u64> {
        self.percent_cache_valid.as_ref()
    }

    /// Sets the value of ValidCacheSize
    pub fn set_valid_cache_size(&mut self, value: u64) {
        self.valid_cache_size = Some(value);
    }

    /// Gets the value of ValidCacheSize
    pub fn get_valid_cache_size(&self) -> Option<&u64> {
        self.valid_cache_size.as_ref()
    }
}

