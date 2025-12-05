// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_ESENT_DatabaseDatabases struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_ESENT_DatabaseDatabases {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "DatabaseCachePercentHitUnique")]
    pub database_cache_percent_hit_unique: Option<u32>,

/// 
    #[serde(rename = "DatabaseCacheRequestsPersecUnique")]
    pub database_cache_requests_persec_unique: Option<u32>,

/// 
    #[serde(rename = "DatabaseCacheSizeMB")]
    pub database_cache_size_mb: Option<u64>,

/// 
    #[serde(rename = "IODatabaseReadsAverageLatency")]
    pub iodatabase_reads_average_latency: Option<u32>,

/// 
    #[serde(rename = "IODatabaseReadsPersec")]
    pub iodatabase_reads_persec: Option<u32>,

/// 
    #[serde(rename = "IODatabaseWritesAverageLatency")]
    pub iodatabase_writes_average_latency: Option<u32>,

/// 
    #[serde(rename = "IODatabaseWritesPersec")]
    pub iodatabase_writes_persec: Option<u32>,
}

impl Win32_PerfFormattedData_ESENT_DatabaseDatabases {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            database_cache_percent_hit_unique: None,
            database_cache_requests_persec_unique: None,
            database_cache_size_mb: None,
            iodatabase_reads_average_latency: None,
            iodatabase_reads_persec: None,
            iodatabase_writes_average_latency: None,
            iodatabase_writes_persec: None,
        }
    }


    /// Sets the value of DatabaseCachePercentHitUnique
    pub fn set_database_cache_percent_hit_unique(&mut self, value: u32) {
        self.database_cache_percent_hit_unique = Some(value);
    }

    /// Gets the value of DatabaseCachePercentHitUnique
    pub fn get_database_cache_percent_hit_unique(&self) -> Option<&u32> {
        self.database_cache_percent_hit_unique.as_ref()
    }

    /// Sets the value of DatabaseCacheRequestsPersecUnique
    pub fn set_database_cache_requests_persec_unique(&mut self, value: u32) {
        self.database_cache_requests_persec_unique = Some(value);
    }

    /// Gets the value of DatabaseCacheRequestsPersecUnique
    pub fn get_database_cache_requests_persec_unique(&self) -> Option<&u32> {
        self.database_cache_requests_persec_unique.as_ref()
    }

    /// Sets the value of DatabaseCacheSizeMB
    pub fn set_database_cache_size_mb(&mut self, value: u64) {
        self.database_cache_size_mb = Some(value);
    }

    /// Gets the value of DatabaseCacheSizeMB
    pub fn get_database_cache_size_mb(&self) -> Option<&u64> {
        self.database_cache_size_mb.as_ref()
    }

    /// Sets the value of IODatabaseReadsAverageLatency
    pub fn set_iodatabase_reads_average_latency(&mut self, value: u32) {
        self.iodatabase_reads_average_latency = Some(value);
    }

    /// Gets the value of IODatabaseReadsAverageLatency
    pub fn get_iodatabase_reads_average_latency(&self) -> Option<&u32> {
        self.iodatabase_reads_average_latency.as_ref()
    }

    /// Sets the value of IODatabaseReadsPersec
    pub fn set_iodatabase_reads_persec(&mut self, value: u32) {
        self.iodatabase_reads_persec = Some(value);
    }

    /// Gets the value of IODatabaseReadsPersec
    pub fn get_iodatabase_reads_persec(&self) -> Option<&u32> {
        self.iodatabase_reads_persec.as_ref()
    }

    /// Sets the value of IODatabaseWritesAverageLatency
    pub fn set_iodatabase_writes_average_latency(&mut self, value: u32) {
        self.iodatabase_writes_average_latency = Some(value);
    }

    /// Gets the value of IODatabaseWritesAverageLatency
    pub fn get_iodatabase_writes_average_latency(&self) -> Option<&u32> {
        self.iodatabase_writes_average_latency.as_ref()
    }

    /// Sets the value of IODatabaseWritesPersec
    pub fn set_iodatabase_writes_persec(&mut self, value: u32) {
        self.iodatabase_writes_persec = Some(value);
    }

    /// Gets the value of IODatabaseWritesPersec
    pub fn get_iodatabase_writes_persec(&self) -> Option<&u32> {
        self.iodatabase_writes_persec.as_ref()
    }
}

