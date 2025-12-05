// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_ESENT_Database struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_ESENT_Database {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "DatabaseCacheMemoryCommitted")]
    pub database_cache_memory_committed: Option<u64>,

/// 
    #[serde(rename = "DatabaseCacheMemoryCommittedMB")]
    pub database_cache_memory_committed_mb: Option<u64>,

/// 
    #[serde(rename = "DatabaseCacheMemoryReserved")]
    pub database_cache_memory_reserved: Option<u64>,

/// 
    #[serde(rename = "DatabaseCacheMemoryReservedMB")]
    pub database_cache_memory_reserved_mb: Option<u64>,

/// 
    #[serde(rename = "DatabaseCacheMissAttachedAverageLatency")]
    pub database_cache_miss_attached_average_latency: Option<u32>,

/// 
    #[serde(rename = "DatabaseCacheMissesPersec")]
    pub database_cache_misses_persec: Option<u32>,

/// 
    #[serde(rename = "DatabaseCachePercentDehydrated")]
    pub database_cache_percent_dehydrated: Option<u32>,

/// 
    #[serde(rename = "DatabaseCachePercentHit")]
    pub database_cache_percent_hit: Option<u32>,

/// 
    #[serde(rename = "DatabaseCachePercentHitUnique")]
    pub database_cache_percent_hit_unique: Option<u32>,

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
    #[serde(rename = "DatabaseCacheSizeEffective")]
    pub database_cache_size_effective: Option<u64>,

/// 
    #[serde(rename = "DatabaseCacheSizeEffectiveMB")]
    pub database_cache_size_effective_mb: Option<u64>,

/// 
    #[serde(rename = "DatabaseCacheSizeMB")]
    pub database_cache_size_mb: Option<u64>,

/// 
    #[serde(rename = "DatabaseCacheSizeResident")]
    pub database_cache_size_resident: Option<u64>,

/// 
    #[serde(rename = "DatabaseCacheSizeResidentMB")]
    pub database_cache_size_resident_mb: Option<u64>,

/// 
    #[serde(rename = "DatabaseMaintenanceDuration")]
    pub database_maintenance_duration: Option<u32>,

/// 
    #[serde(rename = "DatabasePageEvictionsPersec")]
    pub database_page_evictions_persec: Option<u32>,

/// 
    #[serde(rename = "DatabasePageFaultsPersec")]
    pub database_page_faults_persec: Option<u32>,

/// 
    #[serde(rename = "DatabasePageFaultStallsPersec")]
    pub database_page_fault_stalls_persec: Option<u32>,

/// 
    #[serde(rename = "DefragmentationTasks")]
    pub defragmentation_tasks: Option<u32>,

/// 
    #[serde(rename = "DefragmentationTasksPending")]
    pub defragmentation_tasks_pending: Option<u32>,

/// 
    #[serde(rename = "IODatabaseReadsAttachedAverageLatency")]
    pub iodatabase_reads_attached_average_latency: Option<u32>,

/// 
    #[serde(rename = "IODatabaseReadsAttachedPersec")]
    pub iodatabase_reads_attached_persec: Option<u32>,

/// 
    #[serde(rename = "IODatabaseReadsAverageLatency")]
    pub iodatabase_reads_average_latency: Option<u32>,

/// 
    #[serde(rename = "IODatabaseReadsPersec")]
    pub iodatabase_reads_persec: Option<u32>,

/// 
    #[serde(rename = "IODatabaseReadsRecoveryAverageLatency")]
    pub iodatabase_reads_recovery_average_latency: Option<u32>,

/// 
    #[serde(rename = "IODatabaseReadsRecoveryPersec")]
    pub iodatabase_reads_recovery_persec: Option<u32>,

/// 
    #[serde(rename = "IODatabaseWritesAttachedAverageLatency")]
    pub iodatabase_writes_attached_average_latency: Option<u32>,

/// 
    #[serde(rename = "IODatabaseWritesAttachedPersec")]
    pub iodatabase_writes_attached_persec: Option<u32>,

/// 
    #[serde(rename = "IODatabaseWritesAverageLatency")]
    pub iodatabase_writes_average_latency: Option<u32>,

/// 
    #[serde(rename = "IODatabaseWritesPersec")]
    pub iodatabase_writes_persec: Option<u32>,

/// 
    #[serde(rename = "IODatabaseWritesRecoveryAverageLatency")]
    pub iodatabase_writes_recovery_average_latency: Option<u32>,

/// 
    #[serde(rename = "IODatabaseWritesRecoveryPersec")]
    pub iodatabase_writes_recovery_persec: Option<u32>,

/// 
    #[serde(rename = "IOFlushMapWritesAverageLatency")]
    pub ioflush_map_writes_average_latency: Option<u32>,

/// 
    #[serde(rename = "IOFlushMapWritesPersec")]
    pub ioflush_map_writes_persec: Option<u32>,

/// 
    #[serde(rename = "IOLogReadsAverageLatency")]
    pub iolog_reads_average_latency: Option<u32>,

/// 
    #[serde(rename = "IOLogReadsPersec")]
    pub iolog_reads_persec: Option<u32>,

/// 
    #[serde(rename = "IOLogWritesAverageLatency")]
    pub iolog_writes_average_latency: Option<u32>,

/// 
    #[serde(rename = "IOLogWritesPersec")]
    pub iolog_writes_persec: Option<u32>,

/// 
    #[serde(rename = "LogBytesGeneratedPersec")]
    pub log_bytes_generated_persec: Option<u32>,

/// 
    #[serde(rename = "LogBytesWritePersec")]
    pub log_bytes_write_persec: Option<u32>,

/// 
    #[serde(rename = "LogRecordStallsPersec")]
    pub log_record_stalls_persec: Option<u32>,

/// 
    #[serde(rename = "LogThreadsWaiting")]
    pub log_threads_waiting: Option<u32>,

/// 
    #[serde(rename = "LogWritesPersec")]
    pub log_writes_persec: Option<u32>,

/// 
    #[serde(rename = "SessionsInUse")]
    pub sessions_in_use: Option<u32>,

/// 
    #[serde(rename = "SessionsPercentUsed")]
    pub sessions_percent_used: Option<u32>,

/// 
    #[serde(rename = "TableClosesPersec")]
    pub table_closes_persec: Option<u32>,

/// 
    #[serde(rename = "TableOpenCacheHitsPersec")]
    pub table_open_cache_hits_persec: Option<u32>,

/// 
    #[serde(rename = "TableOpenCacheMissesPersec")]
    pub table_open_cache_misses_persec: Option<u32>,

/// 
    #[serde(rename = "TableOpenCachePercentHit")]
    pub table_open_cache_percent_hit: Option<u32>,

/// 
    #[serde(rename = "TableOpensPersec")]
    pub table_opens_persec: Option<u32>,

/// 
    #[serde(rename = "TablesOpen")]
    pub tables_open: Option<u32>,

/// 
    #[serde(rename = "VersionBucketsAllocated")]
    pub version_buckets_allocated: Option<u32>,
}

impl Win32_PerfFormattedData_ESENT_Database {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            database_cache_memory_committed: None,
            database_cache_memory_committed_mb: None,
            database_cache_memory_reserved: None,
            database_cache_memory_reserved_mb: None,
            database_cache_miss_attached_average_latency: None,
            database_cache_misses_persec: None,
            database_cache_percent_dehydrated: None,
            database_cache_percent_hit: None,
            database_cache_percent_hit_unique: None,
            database_cache_requests_persec: None,
            database_cache_requests_persec_unique: None,
            database_cache_size: None,
            database_cache_size_effective: None,
            database_cache_size_effective_mb: None,
            database_cache_size_mb: None,
            database_cache_size_resident: None,
            database_cache_size_resident_mb: None,
            database_maintenance_duration: None,
            database_page_evictions_persec: None,
            database_page_faults_persec: None,
            database_page_fault_stalls_persec: None,
            defragmentation_tasks: None,
            defragmentation_tasks_pending: None,
            iodatabase_reads_attached_average_latency: None,
            iodatabase_reads_attached_persec: None,
            iodatabase_reads_average_latency: None,
            iodatabase_reads_persec: None,
            iodatabase_reads_recovery_average_latency: None,
            iodatabase_reads_recovery_persec: None,
            iodatabase_writes_attached_average_latency: None,
            iodatabase_writes_attached_persec: None,
            iodatabase_writes_average_latency: None,
            iodatabase_writes_persec: None,
            iodatabase_writes_recovery_average_latency: None,
            iodatabase_writes_recovery_persec: None,
            ioflush_map_writes_average_latency: None,
            ioflush_map_writes_persec: None,
            iolog_reads_average_latency: None,
            iolog_reads_persec: None,
            iolog_writes_average_latency: None,
            iolog_writes_persec: None,
            log_bytes_generated_persec: None,
            log_bytes_write_persec: None,
            log_record_stalls_persec: None,
            log_threads_waiting: None,
            log_writes_persec: None,
            sessions_in_use: None,
            sessions_percent_used: None,
            table_closes_persec: None,
            table_open_cache_hits_persec: None,
            table_open_cache_misses_persec: None,
            table_open_cache_percent_hit: None,
            table_opens_persec: None,
            tables_open: None,
            version_buckets_allocated: None,
        }
    }


    /// Sets the value of DatabaseCacheMemoryCommitted
    pub fn set_database_cache_memory_committed(&mut self, value: u64) {
        self.database_cache_memory_committed = Some(value);
    }

    /// Gets the value of DatabaseCacheMemoryCommitted
    pub fn get_database_cache_memory_committed(&self) -> Option<&u64> {
        self.database_cache_memory_committed.as_ref()
    }

    /// Sets the value of DatabaseCacheMemoryCommittedMB
    pub fn set_database_cache_memory_committed_mb(&mut self, value: u64) {
        self.database_cache_memory_committed_mb = Some(value);
    }

    /// Gets the value of DatabaseCacheMemoryCommittedMB
    pub fn get_database_cache_memory_committed_mb(&self) -> Option<&u64> {
        self.database_cache_memory_committed_mb.as_ref()
    }

    /// Sets the value of DatabaseCacheMemoryReserved
    pub fn set_database_cache_memory_reserved(&mut self, value: u64) {
        self.database_cache_memory_reserved = Some(value);
    }

    /// Gets the value of DatabaseCacheMemoryReserved
    pub fn get_database_cache_memory_reserved(&self) -> Option<&u64> {
        self.database_cache_memory_reserved.as_ref()
    }

    /// Sets the value of DatabaseCacheMemoryReservedMB
    pub fn set_database_cache_memory_reserved_mb(&mut self, value: u64) {
        self.database_cache_memory_reserved_mb = Some(value);
    }

    /// Gets the value of DatabaseCacheMemoryReservedMB
    pub fn get_database_cache_memory_reserved_mb(&self) -> Option<&u64> {
        self.database_cache_memory_reserved_mb.as_ref()
    }

    /// Sets the value of DatabaseCacheMissAttachedAverageLatency
    pub fn set_database_cache_miss_attached_average_latency(&mut self, value: u32) {
        self.database_cache_miss_attached_average_latency = Some(value);
    }

    /// Gets the value of DatabaseCacheMissAttachedAverageLatency
    pub fn get_database_cache_miss_attached_average_latency(&self) -> Option<&u32> {
        self.database_cache_miss_attached_average_latency.as_ref()
    }

    /// Sets the value of DatabaseCacheMissesPersec
    pub fn set_database_cache_misses_persec(&mut self, value: u32) {
        self.database_cache_misses_persec = Some(value);
    }

    /// Gets the value of DatabaseCacheMissesPersec
    pub fn get_database_cache_misses_persec(&self) -> Option<&u32> {
        self.database_cache_misses_persec.as_ref()
    }

    /// Sets the value of DatabaseCachePercentDehydrated
    pub fn set_database_cache_percent_dehydrated(&mut self, value: u32) {
        self.database_cache_percent_dehydrated = Some(value);
    }

    /// Gets the value of DatabaseCachePercentDehydrated
    pub fn get_database_cache_percent_dehydrated(&self) -> Option<&u32> {
        self.database_cache_percent_dehydrated.as_ref()
    }

    /// Sets the value of DatabaseCachePercentHit
    pub fn set_database_cache_percent_hit(&mut self, value: u32) {
        self.database_cache_percent_hit = Some(value);
    }

    /// Gets the value of DatabaseCachePercentHit
    pub fn get_database_cache_percent_hit(&self) -> Option<&u32> {
        self.database_cache_percent_hit.as_ref()
    }

    /// Sets the value of DatabaseCachePercentHitUnique
    pub fn set_database_cache_percent_hit_unique(&mut self, value: u32) {
        self.database_cache_percent_hit_unique = Some(value);
    }

    /// Gets the value of DatabaseCachePercentHitUnique
    pub fn get_database_cache_percent_hit_unique(&self) -> Option<&u32> {
        self.database_cache_percent_hit_unique.as_ref()
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

    /// Sets the value of DatabaseCacheSizeEffective
    pub fn set_database_cache_size_effective(&mut self, value: u64) {
        self.database_cache_size_effective = Some(value);
    }

    /// Gets the value of DatabaseCacheSizeEffective
    pub fn get_database_cache_size_effective(&self) -> Option<&u64> {
        self.database_cache_size_effective.as_ref()
    }

    /// Sets the value of DatabaseCacheSizeEffectiveMB
    pub fn set_database_cache_size_effective_mb(&mut self, value: u64) {
        self.database_cache_size_effective_mb = Some(value);
    }

    /// Gets the value of DatabaseCacheSizeEffectiveMB
    pub fn get_database_cache_size_effective_mb(&self) -> Option<&u64> {
        self.database_cache_size_effective_mb.as_ref()
    }

    /// Sets the value of DatabaseCacheSizeMB
    pub fn set_database_cache_size_mb(&mut self, value: u64) {
        self.database_cache_size_mb = Some(value);
    }

    /// Gets the value of DatabaseCacheSizeMB
    pub fn get_database_cache_size_mb(&self) -> Option<&u64> {
        self.database_cache_size_mb.as_ref()
    }

    /// Sets the value of DatabaseCacheSizeResident
    pub fn set_database_cache_size_resident(&mut self, value: u64) {
        self.database_cache_size_resident = Some(value);
    }

    /// Gets the value of DatabaseCacheSizeResident
    pub fn get_database_cache_size_resident(&self) -> Option<&u64> {
        self.database_cache_size_resident.as_ref()
    }

    /// Sets the value of DatabaseCacheSizeResidentMB
    pub fn set_database_cache_size_resident_mb(&mut self, value: u64) {
        self.database_cache_size_resident_mb = Some(value);
    }

    /// Gets the value of DatabaseCacheSizeResidentMB
    pub fn get_database_cache_size_resident_mb(&self) -> Option<&u64> {
        self.database_cache_size_resident_mb.as_ref()
    }

    /// Sets the value of DatabaseMaintenanceDuration
    pub fn set_database_maintenance_duration(&mut self, value: u32) {
        self.database_maintenance_duration = Some(value);
    }

    /// Gets the value of DatabaseMaintenanceDuration
    pub fn get_database_maintenance_duration(&self) -> Option<&u32> {
        self.database_maintenance_duration.as_ref()
    }

    /// Sets the value of DatabasePageEvictionsPersec
    pub fn set_database_page_evictions_persec(&mut self, value: u32) {
        self.database_page_evictions_persec = Some(value);
    }

    /// Gets the value of DatabasePageEvictionsPersec
    pub fn get_database_page_evictions_persec(&self) -> Option<&u32> {
        self.database_page_evictions_persec.as_ref()
    }

    /// Sets the value of DatabasePageFaultsPersec
    pub fn set_database_page_faults_persec(&mut self, value: u32) {
        self.database_page_faults_persec = Some(value);
    }

    /// Gets the value of DatabasePageFaultsPersec
    pub fn get_database_page_faults_persec(&self) -> Option<&u32> {
        self.database_page_faults_persec.as_ref()
    }

    /// Sets the value of DatabasePageFaultStallsPersec
    pub fn set_database_page_fault_stalls_persec(&mut self, value: u32) {
        self.database_page_fault_stalls_persec = Some(value);
    }

    /// Gets the value of DatabasePageFaultStallsPersec
    pub fn get_database_page_fault_stalls_persec(&self) -> Option<&u32> {
        self.database_page_fault_stalls_persec.as_ref()
    }

    /// Sets the value of DefragmentationTasks
    pub fn set_defragmentation_tasks(&mut self, value: u32) {
        self.defragmentation_tasks = Some(value);
    }

    /// Gets the value of DefragmentationTasks
    pub fn get_defragmentation_tasks(&self) -> Option<&u32> {
        self.defragmentation_tasks.as_ref()
    }

    /// Sets the value of DefragmentationTasksPending
    pub fn set_defragmentation_tasks_pending(&mut self, value: u32) {
        self.defragmentation_tasks_pending = Some(value);
    }

    /// Gets the value of DefragmentationTasksPending
    pub fn get_defragmentation_tasks_pending(&self) -> Option<&u32> {
        self.defragmentation_tasks_pending.as_ref()
    }

    /// Sets the value of IODatabaseReadsAttachedAverageLatency
    pub fn set_iodatabase_reads_attached_average_latency(&mut self, value: u32) {
        self.iodatabase_reads_attached_average_latency = Some(value);
    }

    /// Gets the value of IODatabaseReadsAttachedAverageLatency
    pub fn get_iodatabase_reads_attached_average_latency(&self) -> Option<&u32> {
        self.iodatabase_reads_attached_average_latency.as_ref()
    }

    /// Sets the value of IODatabaseReadsAttachedPersec
    pub fn set_iodatabase_reads_attached_persec(&mut self, value: u32) {
        self.iodatabase_reads_attached_persec = Some(value);
    }

    /// Gets the value of IODatabaseReadsAttachedPersec
    pub fn get_iodatabase_reads_attached_persec(&self) -> Option<&u32> {
        self.iodatabase_reads_attached_persec.as_ref()
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

    /// Sets the value of IODatabaseReadsRecoveryAverageLatency
    pub fn set_iodatabase_reads_recovery_average_latency(&mut self, value: u32) {
        self.iodatabase_reads_recovery_average_latency = Some(value);
    }

    /// Gets the value of IODatabaseReadsRecoveryAverageLatency
    pub fn get_iodatabase_reads_recovery_average_latency(&self) -> Option<&u32> {
        self.iodatabase_reads_recovery_average_latency.as_ref()
    }

    /// Sets the value of IODatabaseReadsRecoveryPersec
    pub fn set_iodatabase_reads_recovery_persec(&mut self, value: u32) {
        self.iodatabase_reads_recovery_persec = Some(value);
    }

    /// Gets the value of IODatabaseReadsRecoveryPersec
    pub fn get_iodatabase_reads_recovery_persec(&self) -> Option<&u32> {
        self.iodatabase_reads_recovery_persec.as_ref()
    }

    /// Sets the value of IODatabaseWritesAttachedAverageLatency
    pub fn set_iodatabase_writes_attached_average_latency(&mut self, value: u32) {
        self.iodatabase_writes_attached_average_latency = Some(value);
    }

    /// Gets the value of IODatabaseWritesAttachedAverageLatency
    pub fn get_iodatabase_writes_attached_average_latency(&self) -> Option<&u32> {
        self.iodatabase_writes_attached_average_latency.as_ref()
    }

    /// Sets the value of IODatabaseWritesAttachedPersec
    pub fn set_iodatabase_writes_attached_persec(&mut self, value: u32) {
        self.iodatabase_writes_attached_persec = Some(value);
    }

    /// Gets the value of IODatabaseWritesAttachedPersec
    pub fn get_iodatabase_writes_attached_persec(&self) -> Option<&u32> {
        self.iodatabase_writes_attached_persec.as_ref()
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

    /// Sets the value of IODatabaseWritesRecoveryAverageLatency
    pub fn set_iodatabase_writes_recovery_average_latency(&mut self, value: u32) {
        self.iodatabase_writes_recovery_average_latency = Some(value);
    }

    /// Gets the value of IODatabaseWritesRecoveryAverageLatency
    pub fn get_iodatabase_writes_recovery_average_latency(&self) -> Option<&u32> {
        self.iodatabase_writes_recovery_average_latency.as_ref()
    }

    /// Sets the value of IODatabaseWritesRecoveryPersec
    pub fn set_iodatabase_writes_recovery_persec(&mut self, value: u32) {
        self.iodatabase_writes_recovery_persec = Some(value);
    }

    /// Gets the value of IODatabaseWritesRecoveryPersec
    pub fn get_iodatabase_writes_recovery_persec(&self) -> Option<&u32> {
        self.iodatabase_writes_recovery_persec.as_ref()
    }

    /// Sets the value of IOFlushMapWritesAverageLatency
    pub fn set_ioflush_map_writes_average_latency(&mut self, value: u32) {
        self.ioflush_map_writes_average_latency = Some(value);
    }

    /// Gets the value of IOFlushMapWritesAverageLatency
    pub fn get_ioflush_map_writes_average_latency(&self) -> Option<&u32> {
        self.ioflush_map_writes_average_latency.as_ref()
    }

    /// Sets the value of IOFlushMapWritesPersec
    pub fn set_ioflush_map_writes_persec(&mut self, value: u32) {
        self.ioflush_map_writes_persec = Some(value);
    }

    /// Gets the value of IOFlushMapWritesPersec
    pub fn get_ioflush_map_writes_persec(&self) -> Option<&u32> {
        self.ioflush_map_writes_persec.as_ref()
    }

    /// Sets the value of IOLogReadsAverageLatency
    pub fn set_iolog_reads_average_latency(&mut self, value: u32) {
        self.iolog_reads_average_latency = Some(value);
    }

    /// Gets the value of IOLogReadsAverageLatency
    pub fn get_iolog_reads_average_latency(&self) -> Option<&u32> {
        self.iolog_reads_average_latency.as_ref()
    }

    /// Sets the value of IOLogReadsPersec
    pub fn set_iolog_reads_persec(&mut self, value: u32) {
        self.iolog_reads_persec = Some(value);
    }

    /// Gets the value of IOLogReadsPersec
    pub fn get_iolog_reads_persec(&self) -> Option<&u32> {
        self.iolog_reads_persec.as_ref()
    }

    /// Sets the value of IOLogWritesAverageLatency
    pub fn set_iolog_writes_average_latency(&mut self, value: u32) {
        self.iolog_writes_average_latency = Some(value);
    }

    /// Gets the value of IOLogWritesAverageLatency
    pub fn get_iolog_writes_average_latency(&self) -> Option<&u32> {
        self.iolog_writes_average_latency.as_ref()
    }

    /// Sets the value of IOLogWritesPersec
    pub fn set_iolog_writes_persec(&mut self, value: u32) {
        self.iolog_writes_persec = Some(value);
    }

    /// Gets the value of IOLogWritesPersec
    pub fn get_iolog_writes_persec(&self) -> Option<&u32> {
        self.iolog_writes_persec.as_ref()
    }

    /// Sets the value of LogBytesGeneratedPersec
    pub fn set_log_bytes_generated_persec(&mut self, value: u32) {
        self.log_bytes_generated_persec = Some(value);
    }

    /// Gets the value of LogBytesGeneratedPersec
    pub fn get_log_bytes_generated_persec(&self) -> Option<&u32> {
        self.log_bytes_generated_persec.as_ref()
    }

    /// Sets the value of LogBytesWritePersec
    pub fn set_log_bytes_write_persec(&mut self, value: u32) {
        self.log_bytes_write_persec = Some(value);
    }

    /// Gets the value of LogBytesWritePersec
    pub fn get_log_bytes_write_persec(&self) -> Option<&u32> {
        self.log_bytes_write_persec.as_ref()
    }

    /// Sets the value of LogRecordStallsPersec
    pub fn set_log_record_stalls_persec(&mut self, value: u32) {
        self.log_record_stalls_persec = Some(value);
    }

    /// Gets the value of LogRecordStallsPersec
    pub fn get_log_record_stalls_persec(&self) -> Option<&u32> {
        self.log_record_stalls_persec.as_ref()
    }

    /// Sets the value of LogThreadsWaiting
    pub fn set_log_threads_waiting(&mut self, value: u32) {
        self.log_threads_waiting = Some(value);
    }

    /// Gets the value of LogThreadsWaiting
    pub fn get_log_threads_waiting(&self) -> Option<&u32> {
        self.log_threads_waiting.as_ref()
    }

    /// Sets the value of LogWritesPersec
    pub fn set_log_writes_persec(&mut self, value: u32) {
        self.log_writes_persec = Some(value);
    }

    /// Gets the value of LogWritesPersec
    pub fn get_log_writes_persec(&self) -> Option<&u32> {
        self.log_writes_persec.as_ref()
    }

    /// Sets the value of SessionsInUse
    pub fn set_sessions_in_use(&mut self, value: u32) {
        self.sessions_in_use = Some(value);
    }

    /// Gets the value of SessionsInUse
    pub fn get_sessions_in_use(&self) -> Option<&u32> {
        self.sessions_in_use.as_ref()
    }

    /// Sets the value of SessionsPercentUsed
    pub fn set_sessions_percent_used(&mut self, value: u32) {
        self.sessions_percent_used = Some(value);
    }

    /// Gets the value of SessionsPercentUsed
    pub fn get_sessions_percent_used(&self) -> Option<&u32> {
        self.sessions_percent_used.as_ref()
    }

    /// Sets the value of TableClosesPersec
    pub fn set_table_closes_persec(&mut self, value: u32) {
        self.table_closes_persec = Some(value);
    }

    /// Gets the value of TableClosesPersec
    pub fn get_table_closes_persec(&self) -> Option<&u32> {
        self.table_closes_persec.as_ref()
    }

    /// Sets the value of TableOpenCacheHitsPersec
    pub fn set_table_open_cache_hits_persec(&mut self, value: u32) {
        self.table_open_cache_hits_persec = Some(value);
    }

    /// Gets the value of TableOpenCacheHitsPersec
    pub fn get_table_open_cache_hits_persec(&self) -> Option<&u32> {
        self.table_open_cache_hits_persec.as_ref()
    }

    /// Sets the value of TableOpenCacheMissesPersec
    pub fn set_table_open_cache_misses_persec(&mut self, value: u32) {
        self.table_open_cache_misses_persec = Some(value);
    }

    /// Gets the value of TableOpenCacheMissesPersec
    pub fn get_table_open_cache_misses_persec(&self) -> Option<&u32> {
        self.table_open_cache_misses_persec.as_ref()
    }

    /// Sets the value of TableOpenCachePercentHit
    pub fn set_table_open_cache_percent_hit(&mut self, value: u32) {
        self.table_open_cache_percent_hit = Some(value);
    }

    /// Gets the value of TableOpenCachePercentHit
    pub fn get_table_open_cache_percent_hit(&self) -> Option<&u32> {
        self.table_open_cache_percent_hit.as_ref()
    }

    /// Sets the value of TableOpensPersec
    pub fn set_table_opens_persec(&mut self, value: u32) {
        self.table_opens_persec = Some(value);
    }

    /// Gets the value of TableOpensPersec
    pub fn get_table_opens_persec(&self) -> Option<&u32> {
        self.table_opens_persec.as_ref()
    }

    /// Sets the value of TablesOpen
    pub fn set_tables_open(&mut self, value: u32) {
        self.tables_open = Some(value);
    }

    /// Gets the value of TablesOpen
    pub fn get_tables_open(&self) -> Option<&u32> {
        self.tables_open.as_ref()
    }

    /// Sets the value of VersionBucketsAllocated
    pub fn set_version_buckets_allocated(&mut self, value: u32) {
        self.version_buckets_allocated = Some(value);
    }

    /// Gets the value of VersionBucketsAllocated
    pub fn get_version_buckets_allocated(&self) -> Option<&u32> {
        self.version_buckets_allocated.as_ref()
    }
}

