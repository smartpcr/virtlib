// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_ESENT_DatabaseInstances struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_ESENT_DatabaseInstances {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "DatabaseCacheMissAttachedAverageLatency")]
    pub database_cache_miss_attached_average_latency: Option<u32>,

/// 
    #[serde(rename = "DatabaseCacheMissesPersec")]
    pub database_cache_misses_persec: Option<u32>,

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
    #[serde(rename = "DatabaseCacheSizeMB")]
    pub database_cache_size_mb: Option<u64>,

/// 
    #[serde(rename = "DatabaseMaintenanceDuration")]
    pub database_maintenance_duration: Option<u32>,

/// 
    #[serde(rename = "DatabaseOldestTransaction")]
    pub database_oldest_transaction: Option<u64>,

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
    #[serde(rename = "LogCheckpointDepthasaPercentofTarget")]
    pub log_checkpoint_depthasa_percentof_target: Option<u32>,

/// 
    #[serde(rename = "LogFileCurrentGeneration")]
    pub log_file_current_generation: Option<u32>,

/// 
    #[serde(rename = "LogFilesGenerated")]
    pub log_files_generated: Option<u32>,

/// 
    #[serde(rename = "LogFilesGeneratedPrematurely")]
    pub log_files_generated_prematurely: Option<u32>,

/// 
    #[serde(rename = "LogGenerationCheckpointDepth")]
    pub log_generation_checkpoint_depth: Option<u32>,

/// 
    #[serde(rename = "LogGenerationCheckpointDepthMax")]
    pub log_generation_checkpoint_depth_max: Option<u32>,

/// 
    #[serde(rename = "LogGenerationCheckpointDepthTarget")]
    pub log_generation_checkpoint_depth_target: Option<u32>,

/// 
    #[serde(rename = "LogGenerationLossResiliencyDepth")]
    pub log_generation_loss_resiliency_depth: Option<u32>,

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
    #[serde(rename = "StreamingBackupPagesReadPersec")]
    pub streaming_backup_pages_read_persec: Option<u32>,

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
    #[serde(rename = "Versionbucketsallocated")]
    pub versionbucketsallocated: Option<u32>,
}

impl Win32_PerfFormattedData_ESENT_DatabaseInstances {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            database_cache_miss_attached_average_latency: None,
            database_cache_misses_persec: None,
            database_cache_percent_hit: None,
            database_cache_percent_hit_unique: None,
            database_cache_requests_persec: None,
            database_cache_requests_persec_unique: None,
            database_cache_size_mb: None,
            database_maintenance_duration: None,
            database_oldest_transaction: None,
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
            log_checkpoint_depthasa_percentof_target: None,
            log_file_current_generation: None,
            log_files_generated: None,
            log_files_generated_prematurely: None,
            log_generation_checkpoint_depth: None,
            log_generation_checkpoint_depth_max: None,
            log_generation_checkpoint_depth_target: None,
            log_generation_loss_resiliency_depth: None,
            log_record_stalls_persec: None,
            log_threads_waiting: None,
            log_writes_persec: None,
            sessions_in_use: None,
            sessions_percent_used: None,
            streaming_backup_pages_read_persec: None,
            table_closes_persec: None,
            table_open_cache_hits_persec: None,
            table_open_cache_misses_persec: None,
            table_open_cache_percent_hit: None,
            table_opens_persec: None,
            tables_open: None,
            versionbucketsallocated: None,
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

    /// Sets the value of DatabaseCacheSizeMB
    pub fn set_database_cache_size_mb(&mut self, value: u64) {
        self.database_cache_size_mb = Some(value);
    }

    /// Gets the value of DatabaseCacheSizeMB
    pub fn get_database_cache_size_mb(&self) -> Option<&u64> {
        self.database_cache_size_mb.as_ref()
    }

    /// Sets the value of DatabaseMaintenanceDuration
    pub fn set_database_maintenance_duration(&mut self, value: u32) {
        self.database_maintenance_duration = Some(value);
    }

    /// Gets the value of DatabaseMaintenanceDuration
    pub fn get_database_maintenance_duration(&self) -> Option<&u32> {
        self.database_maintenance_duration.as_ref()
    }

    /// Sets the value of DatabaseOldestTransaction
    pub fn set_database_oldest_transaction(&mut self, value: u64) {
        self.database_oldest_transaction = Some(value);
    }

    /// Gets the value of DatabaseOldestTransaction
    pub fn get_database_oldest_transaction(&self) -> Option<&u64> {
        self.database_oldest_transaction.as_ref()
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

    /// Sets the value of LogCheckpointDepthasaPercentofTarget
    pub fn set_log_checkpoint_depthasa_percentof_target(&mut self, value: u32) {
        self.log_checkpoint_depthasa_percentof_target = Some(value);
    }

    /// Gets the value of LogCheckpointDepthasaPercentofTarget
    pub fn get_log_checkpoint_depthasa_percentof_target(&self) -> Option<&u32> {
        self.log_checkpoint_depthasa_percentof_target.as_ref()
    }

    /// Sets the value of LogFileCurrentGeneration
    pub fn set_log_file_current_generation(&mut self, value: u32) {
        self.log_file_current_generation = Some(value);
    }

    /// Gets the value of LogFileCurrentGeneration
    pub fn get_log_file_current_generation(&self) -> Option<&u32> {
        self.log_file_current_generation.as_ref()
    }

    /// Sets the value of LogFilesGenerated
    pub fn set_log_files_generated(&mut self, value: u32) {
        self.log_files_generated = Some(value);
    }

    /// Gets the value of LogFilesGenerated
    pub fn get_log_files_generated(&self) -> Option<&u32> {
        self.log_files_generated.as_ref()
    }

    /// Sets the value of LogFilesGeneratedPrematurely
    pub fn set_log_files_generated_prematurely(&mut self, value: u32) {
        self.log_files_generated_prematurely = Some(value);
    }

    /// Gets the value of LogFilesGeneratedPrematurely
    pub fn get_log_files_generated_prematurely(&self) -> Option<&u32> {
        self.log_files_generated_prematurely.as_ref()
    }

    /// Sets the value of LogGenerationCheckpointDepth
    pub fn set_log_generation_checkpoint_depth(&mut self, value: u32) {
        self.log_generation_checkpoint_depth = Some(value);
    }

    /// Gets the value of LogGenerationCheckpointDepth
    pub fn get_log_generation_checkpoint_depth(&self) -> Option<&u32> {
        self.log_generation_checkpoint_depth.as_ref()
    }

    /// Sets the value of LogGenerationCheckpointDepthMax
    pub fn set_log_generation_checkpoint_depth_max(&mut self, value: u32) {
        self.log_generation_checkpoint_depth_max = Some(value);
    }

    /// Gets the value of LogGenerationCheckpointDepthMax
    pub fn get_log_generation_checkpoint_depth_max(&self) -> Option<&u32> {
        self.log_generation_checkpoint_depth_max.as_ref()
    }

    /// Sets the value of LogGenerationCheckpointDepthTarget
    pub fn set_log_generation_checkpoint_depth_target(&mut self, value: u32) {
        self.log_generation_checkpoint_depth_target = Some(value);
    }

    /// Gets the value of LogGenerationCheckpointDepthTarget
    pub fn get_log_generation_checkpoint_depth_target(&self) -> Option<&u32> {
        self.log_generation_checkpoint_depth_target.as_ref()
    }

    /// Sets the value of LogGenerationLossResiliencyDepth
    pub fn set_log_generation_loss_resiliency_depth(&mut self, value: u32) {
        self.log_generation_loss_resiliency_depth = Some(value);
    }

    /// Gets the value of LogGenerationLossResiliencyDepth
    pub fn get_log_generation_loss_resiliency_depth(&self) -> Option<&u32> {
        self.log_generation_loss_resiliency_depth.as_ref()
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

    /// Sets the value of StreamingBackupPagesReadPersec
    pub fn set_streaming_backup_pages_read_persec(&mut self, value: u32) {
        self.streaming_backup_pages_read_persec = Some(value);
    }

    /// Gets the value of StreamingBackupPagesReadPersec
    pub fn get_streaming_backup_pages_read_persec(&self) -> Option<&u32> {
        self.streaming_backup_pages_read_persec.as_ref()
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

    /// Sets the value of Versionbucketsallocated
    pub fn set_versionbucketsallocated(&mut self, value: u32) {
        self.versionbucketsallocated = Some(value);
    }

    /// Gets the value of Versionbucketsallocated
    pub fn get_versionbucketsallocated(&self) -> Option<&u32> {
        self.versionbucketsallocated.as_ref()
    }
}

