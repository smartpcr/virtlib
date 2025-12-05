// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_ESENT_DatabaseTableClasses struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_ESENT_DatabaseTableClasses {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "DatabaseCacheMissAttachedAverageLatency")]
    pub database_cache_miss_attached_average_latency: Option<u32>,

/// 
    #[serde(rename = "DatabaseCacheMissAttachedAverageLatency_Base")]
    pub database_cache_miss_attached_average_latency__base: Option<u32>,

/// 
    #[serde(rename = "DatabaseCacheMissesPersec")]
    pub database_cache_misses_persec: Option<u32>,

/// 
    #[serde(rename = "DatabaseCachePercentHit")]
    pub database_cache_percent_hit: Option<u32>,

/// 
    #[serde(rename = "DatabaseCachePercentHit_Base")]
    pub database_cache_percent_hit__base: Option<u32>,

/// 
    #[serde(rename = "DatabaseCachePercentHitUnique")]
    pub database_cache_percent_hit_unique: Option<u32>,

/// 
    #[serde(rename = "DatabaseCachePercentHitUnique_Base")]
    pub database_cache_percent_hit_unique__base: Option<u32>,

/// 
    #[serde(rename = "DatabaseCacheRequestsPersec")]
    pub database_cache_requests_persec: Option<u32>,

/// 
    #[serde(rename = "DatabaseCacheRequestsPersecUnique")]
    pub database_cache_requests_persec_unique: Option<u32>,

/// 
    #[serde(rename = "DatabaseCacheSize")]
    pub database_cache_size: Option<u64>,

/// 
    #[serde(rename = "DatabaseCacheSizeMB")]
    pub database_cache_size_mb: Option<u64>,
}

impl Win32_PerfRawData_ESENT_DatabaseTableClasses {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            database_cache_miss_attached_average_latency: None,
            database_cache_miss_attached_average_latency__base: None,
            database_cache_misses_persec: None,
            database_cache_percent_hit: None,
            database_cache_percent_hit__base: None,
            database_cache_percent_hit_unique: None,
            database_cache_percent_hit_unique__base: None,
            database_cache_requests_persec: None,
            database_cache_requests_persec_unique: None,
            database_cache_size: None,
            database_cache_size_mb: None,
        }
    }


    /// Sets the value of DatabaseCacheMissAttachedAverageLatency
    pub fn set_database_cache_miss_attached_average_latency(&mut self, value: u32) {
        self.database_cache_miss_attached_average_latency = Some(value);
    }

    /// Gets the value of DatabaseCacheMissAttachedAverageLatency
    pub fn get_database_cache_miss_attached_average_latency(&self) -> Option<&u32> {
        self.database_cache_miss_attached_average_latency.as_ref()
    }

    /// Sets the value of DatabaseCacheMissAttachedAverageLatency_Base
    pub fn set_database_cache_miss_attached_average_latency__base(&mut self, value: u32) {
        self.database_cache_miss_attached_average_latency__base = Some(value);
    }

    /// Gets the value of DatabaseCacheMissAttachedAverageLatency_Base
    pub fn get_database_cache_miss_attached_average_latency__base(&self) -> Option<&u32> {
        self.database_cache_miss_attached_average_latency__base.as_ref()
    }

    /// Sets the value of DatabaseCacheMissesPersec
    pub fn set_database_cache_misses_persec(&mut self, value: u32) {
        self.database_cache_misses_persec = Some(value);
    }

    /// Gets the value of DatabaseCacheMissesPersec
    pub fn get_database_cache_misses_persec(&self) -> Option<&u32> {
        self.database_cache_misses_persec.as_ref()
    }

    /// Sets the value of DatabaseCachePercentHit
    pub fn set_database_cache_percent_hit(&mut self, value: u32) {
        self.database_cache_percent_hit = Some(value);
    }

    /// Gets the value of DatabaseCachePercentHit
    pub fn get_database_cache_percent_hit(&self) -> Option<&u32> {
        self.database_cache_percent_hit.as_ref()
    }

    /// Sets the value of DatabaseCachePercentHit_Base
    pub fn set_database_cache_percent_hit__base(&mut self, value: u32) {
        self.database_cache_percent_hit__base = Some(value);
    }

    /// Gets the value of DatabaseCachePercentHit_Base
    pub fn get_database_cache_percent_hit__base(&self) -> Option<&u32> {
        self.database_cache_percent_hit__base.as_ref()
    }

    /// Sets the value of DatabaseCachePercentHitUnique
    pub fn set_database_cache_percent_hit_unique(&mut self, value: u32) {
        self.database_cache_percent_hit_unique = Some(value);
    }

    /// Gets the value of DatabaseCachePercentHitUnique
    pub fn get_database_cache_percent_hit_unique(&self) -> Option<&u32> {
        self.database_cache_percent_hit_unique.as_ref()
    }

    /// Sets the value of DatabaseCachePercentHitUnique_Base
    pub fn set_database_cache_percent_hit_unique__base(&mut self, value: u32) {
        self.database_cache_percent_hit_unique__base = Some(value);
    }

    /// Gets the value of DatabaseCachePercentHitUnique_Base
    pub fn get_database_cache_percent_hit_unique__base(&self) -> Option<&u32> {
        self.database_cache_percent_hit_unique__base.as_ref()
    }

    /// Sets the value of DatabaseCacheRequestsPersec
    pub fn set_database_cache_requests_persec(&mut self, value: u32) {
        self.database_cache_requests_persec = Some(value);
    }

    /// Gets the value of DatabaseCacheRequestsPersec
    pub fn get_database_cache_requests_persec(&self) -> Option<&u32> {
        self.database_cache_requests_persec.as_ref()
    }

    /// Sets the value of DatabaseCacheRequestsPersecUnique
    pub fn set_database_cache_requests_persec_unique(&mut self, value: u32) {
        self.database_cache_requests_persec_unique = Some(value);
    }

    /// Gets the value of DatabaseCacheRequestsPersecUnique
    pub fn get_database_cache_requests_persec_unique(&self) -> Option<&u32> {
        self.database_cache_requests_persec_unique.as_ref()
    }

    /// Sets the value of DatabaseCacheSize
    pub fn set_database_cache_size(&mut self, value: u64) {
        self.database_cache_size = Some(value);
    }

    /// Gets the value of DatabaseCacheSize
    pub fn get_database_cache_size(&self) -> Option<&u64> {
        self.database_cache_size.as_ref()
    }

    /// Sets the value of DatabaseCacheSizeMB
    pub fn set_database_cache_size_mb(&mut self, value: u64) {
        self.database_cache_size_mb = Some(value);
    }

    /// Gets the value of DatabaseCacheSizeMB
    pub fn get_database_cache_size_mb(&self) -> Option<&u64> {
        self.database_cache_size_mb.as_ref()
    }
}

