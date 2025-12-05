// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DedupProperties struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DedupProperties {

/// 
    #[serde(rename = "DedupMode")]
    pub dedup_mode: Option<u32>,

/// 
    #[serde(rename = "InPolicyFilesCount")]
    pub in_policy_files_count: Option<u64>,

/// 
    #[serde(rename = "InPolicyFilesSize")]
    pub in_policy_files_size: Option<u64>,

/// 
    #[serde(rename = "OptimizedFilesCount")]
    pub optimized_files_count: Option<u64>,

/// 
    #[serde(rename = "OptimizedFilesSavingsRate")]
    pub optimized_files_savings_rate: Option<u32>,

/// 
    #[serde(rename = "OptimizedFilesSize")]
    pub optimized_files_size: Option<u64>,

/// 
    #[serde(rename = "ReFSDedupCompressionChunkSize")]
    pub re_fsdedup_compression_chunk_size: Option<u32>,

/// 
    #[serde(rename = "ReFSDedupCompressionFormat")]
    pub re_fsdedup_compression_format: Option<u32>,

/// 
    #[serde(rename = "ReFSDedupCompressionInProgress")]
    pub re_fsdedup_compression_in_progress: Option<bool>,

/// 
    #[serde(rename = "ReFSDedupCompressionLevel")]
    pub re_fsdedup_compression_level: Option<u16>,

/// 
    #[serde(rename = "ReFSDedupLastRunDuration")]
    pub re_fsdedup_last_run_duration: Option<String>,

/// 
    #[serde(rename = "ReFSDedupLastRunStatus")]
    pub re_fsdedup_last_run_status: Option<u64>,

/// 
    #[serde(rename = "ReFSDedupLastRunTime")]
    pub re_fsdedup_last_run_time: Option<String>,

/// 
    #[serde(rename = "ReFSDedupMode")]
    pub re_fsdedup_mode: Option<u32>,

/// 
    #[serde(rename = "ReFSDedupNextRunTime")]
    pub re_fsdedup_next_run_time: Option<String>,

/// 
    #[serde(rename = "ReFSDedupPercentComplete")]
    pub re_fsdedup_percent_complete: Option<f64>,

/// 
    #[serde(rename = "ReFSDedupProcessedOnLastRun")]
    pub re_fsdedup_processed_on_last_run: Option<u64>,

/// 
    #[serde(rename = "ReFSDedupRunning")]
    pub re_fsdedup_running: Option<bool>,

/// 
    #[serde(rename = "ReFSDedupSavingsSize")]
    pub re_fsdedup_savings_size: Option<u64>,

/// 
    #[serde(rename = "ReFSDedupSavingsSizeOnLastRun")]
    pub re_fsdedup_savings_size_on_last_run: Option<u64>,

/// 
    #[serde(rename = "ReFSDedupVolSize")]
    pub re_fsdedup_vol_size: Option<u64>,

/// 
    #[serde(rename = "ReFSDedupVolumeClusterSizeBytes")]
    pub re_fsdedup_volume_cluster_size_bytes: Option<u32>,

/// 
    #[serde(rename = "ReFSDedupVolumeTotalAllocatedClusters")]
    pub re_fsdedup_volume_total_allocated_clusters: Option<u64>,

/// 
    #[serde(rename = "ReFSDedupVolumeTotalAllocatedCompressibleClusters")]
    pub re_fsdedup_volume_total_allocated_compressible_clusters: Option<u64>,

/// 
    #[serde(rename = "ReFSDedupVolumeTotalClusters")]
    pub re_fsdedup_volume_total_clusters: Option<u64>,

/// 
    #[serde(rename = "ReFSDedupVolumeTotalCompressedClusters")]
    pub re_fsdedup_volume_total_compressed_clusters: Option<u64>,

/// 
    #[serde(rename = "ReFSDedupVolumeTotalCompressionSavings")]
    pub re_fsdedup_volume_total_compression_savings: Option<u64>,

/// 
    #[serde(rename = "ReFSDedupVolumeTotalInUseCompressibleClusters")]
    pub re_fsdedup_volume_total_in_use_compressible_clusters: Option<u64>,

/// 
    #[serde(rename = "SavingsRate")]
    pub savings_rate: Option<u32>,

/// 
    #[serde(rename = "SavingsSize")]
    pub savings_size: Option<u64>,

/// 
    #[serde(rename = "UnoptimizedSize")]
    pub unoptimized_size: Option<u64>,
}

impl MSFT_DedupProperties {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            dedup_mode: None,
            in_policy_files_count: None,
            in_policy_files_size: None,
            optimized_files_count: None,
            optimized_files_savings_rate: None,
            optimized_files_size: None,
            re_fsdedup_compression_chunk_size: None,
            re_fsdedup_compression_format: None,
            re_fsdedup_compression_in_progress: None,
            re_fsdedup_compression_level: None,
            re_fsdedup_last_run_duration: None,
            re_fsdedup_last_run_status: None,
            re_fsdedup_last_run_time: None,
            re_fsdedup_mode: None,
            re_fsdedup_next_run_time: None,
            re_fsdedup_percent_complete: None,
            re_fsdedup_processed_on_last_run: None,
            re_fsdedup_running: None,
            re_fsdedup_savings_size: None,
            re_fsdedup_savings_size_on_last_run: None,
            re_fsdedup_vol_size: None,
            re_fsdedup_volume_cluster_size_bytes: None,
            re_fsdedup_volume_total_allocated_clusters: None,
            re_fsdedup_volume_total_allocated_compressible_clusters: None,
            re_fsdedup_volume_total_clusters: None,
            re_fsdedup_volume_total_compressed_clusters: None,
            re_fsdedup_volume_total_compression_savings: None,
            re_fsdedup_volume_total_in_use_compressible_clusters: None,
            savings_rate: None,
            savings_size: None,
            unoptimized_size: None,
        }
    }


    /// Sets the value of DedupMode
    pub fn set_dedup_mode(&mut self, value: u32) {
        self.dedup_mode = Some(value);
    }

    /// Gets the value of DedupMode
    pub fn get_dedup_mode(&self) -> Option<&u32> {
        self.dedup_mode.as_ref()
    }

    /// Sets the value of InPolicyFilesCount
    pub fn set_in_policy_files_count(&mut self, value: u64) {
        self.in_policy_files_count = Some(value);
    }

    /// Gets the value of InPolicyFilesCount
    pub fn get_in_policy_files_count(&self) -> Option<&u64> {
        self.in_policy_files_count.as_ref()
    }

    /// Sets the value of InPolicyFilesSize
    pub fn set_in_policy_files_size(&mut self, value: u64) {
        self.in_policy_files_size = Some(value);
    }

    /// Gets the value of InPolicyFilesSize
    pub fn get_in_policy_files_size(&self) -> Option<&u64> {
        self.in_policy_files_size.as_ref()
    }

    /// Sets the value of OptimizedFilesCount
    pub fn set_optimized_files_count(&mut self, value: u64) {
        self.optimized_files_count = Some(value);
    }

    /// Gets the value of OptimizedFilesCount
    pub fn get_optimized_files_count(&self) -> Option<&u64> {
        self.optimized_files_count.as_ref()
    }

    /// Sets the value of OptimizedFilesSavingsRate
    pub fn set_optimized_files_savings_rate(&mut self, value: u32) {
        self.optimized_files_savings_rate = Some(value);
    }

    /// Gets the value of OptimizedFilesSavingsRate
    pub fn get_optimized_files_savings_rate(&self) -> Option<&u32> {
        self.optimized_files_savings_rate.as_ref()
    }

    /// Sets the value of OptimizedFilesSize
    pub fn set_optimized_files_size(&mut self, value: u64) {
        self.optimized_files_size = Some(value);
    }

    /// Gets the value of OptimizedFilesSize
    pub fn get_optimized_files_size(&self) -> Option<&u64> {
        self.optimized_files_size.as_ref()
    }

    /// Sets the value of ReFSDedupCompressionChunkSize
    pub fn set_re_fsdedup_compression_chunk_size(&mut self, value: u32) {
        self.re_fsdedup_compression_chunk_size = Some(value);
    }

    /// Gets the value of ReFSDedupCompressionChunkSize
    pub fn get_re_fsdedup_compression_chunk_size(&self) -> Option<&u32> {
        self.re_fsdedup_compression_chunk_size.as_ref()
    }

    /// Sets the value of ReFSDedupCompressionFormat
    pub fn set_re_fsdedup_compression_format(&mut self, value: u32) {
        self.re_fsdedup_compression_format = Some(value);
    }

    /// Gets the value of ReFSDedupCompressionFormat
    pub fn get_re_fsdedup_compression_format(&self) -> Option<&u32> {
        self.re_fsdedup_compression_format.as_ref()
    }

    /// Sets the value of ReFSDedupCompressionInProgress
    pub fn set_re_fsdedup_compression_in_progress(&mut self, value: bool) {
        self.re_fsdedup_compression_in_progress = Some(value);
    }

    /// Gets the value of ReFSDedupCompressionInProgress
    pub fn get_re_fsdedup_compression_in_progress(&self) -> Option<&bool> {
        self.re_fsdedup_compression_in_progress.as_ref()
    }

    /// Sets the value of ReFSDedupCompressionLevel
    pub fn set_re_fsdedup_compression_level(&mut self, value: u16) {
        self.re_fsdedup_compression_level = Some(value);
    }

    /// Gets the value of ReFSDedupCompressionLevel
    pub fn get_re_fsdedup_compression_level(&self) -> Option<&u16> {
        self.re_fsdedup_compression_level.as_ref()
    }

    /// Sets the value of ReFSDedupLastRunDuration
    pub fn set_re_fsdedup_last_run_duration(&mut self, value: String) {
        self.re_fsdedup_last_run_duration = Some(value);
    }

    /// Gets the value of ReFSDedupLastRunDuration
    pub fn get_re_fsdedup_last_run_duration(&self) -> Option<&String> {
        self.re_fsdedup_last_run_duration.as_ref()
    }

    /// Sets the value of ReFSDedupLastRunStatus
    pub fn set_re_fsdedup_last_run_status(&mut self, value: u64) {
        self.re_fsdedup_last_run_status = Some(value);
    }

    /// Gets the value of ReFSDedupLastRunStatus
    pub fn get_re_fsdedup_last_run_status(&self) -> Option<&u64> {
        self.re_fsdedup_last_run_status.as_ref()
    }

    /// Sets the value of ReFSDedupLastRunTime
    pub fn set_re_fsdedup_last_run_time(&mut self, value: String) {
        self.re_fsdedup_last_run_time = Some(value);
    }

    /// Gets the value of ReFSDedupLastRunTime
    pub fn get_re_fsdedup_last_run_time(&self) -> Option<&String> {
        self.re_fsdedup_last_run_time.as_ref()
    }

    /// Sets the value of ReFSDedupMode
    pub fn set_re_fsdedup_mode(&mut self, value: u32) {
        self.re_fsdedup_mode = Some(value);
    }

    /// Gets the value of ReFSDedupMode
    pub fn get_re_fsdedup_mode(&self) -> Option<&u32> {
        self.re_fsdedup_mode.as_ref()
    }

    /// Sets the value of ReFSDedupNextRunTime
    pub fn set_re_fsdedup_next_run_time(&mut self, value: String) {
        self.re_fsdedup_next_run_time = Some(value);
    }

    /// Gets the value of ReFSDedupNextRunTime
    pub fn get_re_fsdedup_next_run_time(&self) -> Option<&String> {
        self.re_fsdedup_next_run_time.as_ref()
    }

    /// Sets the value of ReFSDedupPercentComplete
    pub fn set_re_fsdedup_percent_complete(&mut self, value: f64) {
        self.re_fsdedup_percent_complete = Some(value);
    }

    /// Gets the value of ReFSDedupPercentComplete
    pub fn get_re_fsdedup_percent_complete(&self) -> Option<&f64> {
        self.re_fsdedup_percent_complete.as_ref()
    }

    /// Sets the value of ReFSDedupProcessedOnLastRun
    pub fn set_re_fsdedup_processed_on_last_run(&mut self, value: u64) {
        self.re_fsdedup_processed_on_last_run = Some(value);
    }

    /// Gets the value of ReFSDedupProcessedOnLastRun
    pub fn get_re_fsdedup_processed_on_last_run(&self) -> Option<&u64> {
        self.re_fsdedup_processed_on_last_run.as_ref()
    }

    /// Sets the value of ReFSDedupRunning
    pub fn set_re_fsdedup_running(&mut self, value: bool) {
        self.re_fsdedup_running = Some(value);
    }

    /// Gets the value of ReFSDedupRunning
    pub fn get_re_fsdedup_running(&self) -> Option<&bool> {
        self.re_fsdedup_running.as_ref()
    }

    /// Sets the value of ReFSDedupSavingsSize
    pub fn set_re_fsdedup_savings_size(&mut self, value: u64) {
        self.re_fsdedup_savings_size = Some(value);
    }

    /// Gets the value of ReFSDedupSavingsSize
    pub fn get_re_fsdedup_savings_size(&self) -> Option<&u64> {
        self.re_fsdedup_savings_size.as_ref()
    }

    /// Sets the value of ReFSDedupSavingsSizeOnLastRun
    pub fn set_re_fsdedup_savings_size_on_last_run(&mut self, value: u64) {
        self.re_fsdedup_savings_size_on_last_run = Some(value);
    }

    /// Gets the value of ReFSDedupSavingsSizeOnLastRun
    pub fn get_re_fsdedup_savings_size_on_last_run(&self) -> Option<&u64> {
        self.re_fsdedup_savings_size_on_last_run.as_ref()
    }

    /// Sets the value of ReFSDedupVolSize
    pub fn set_re_fsdedup_vol_size(&mut self, value: u64) {
        self.re_fsdedup_vol_size = Some(value);
    }

    /// Gets the value of ReFSDedupVolSize
    pub fn get_re_fsdedup_vol_size(&self) -> Option<&u64> {
        self.re_fsdedup_vol_size.as_ref()
    }

    /// Sets the value of ReFSDedupVolumeClusterSizeBytes
    pub fn set_re_fsdedup_volume_cluster_size_bytes(&mut self, value: u32) {
        self.re_fsdedup_volume_cluster_size_bytes = Some(value);
    }

    /// Gets the value of ReFSDedupVolumeClusterSizeBytes
    pub fn get_re_fsdedup_volume_cluster_size_bytes(&self) -> Option<&u32> {
        self.re_fsdedup_volume_cluster_size_bytes.as_ref()
    }

    /// Sets the value of ReFSDedupVolumeTotalAllocatedClusters
    pub fn set_re_fsdedup_volume_total_allocated_clusters(&mut self, value: u64) {
        self.re_fsdedup_volume_total_allocated_clusters = Some(value);
    }

    /// Gets the value of ReFSDedupVolumeTotalAllocatedClusters
    pub fn get_re_fsdedup_volume_total_allocated_clusters(&self) -> Option<&u64> {
        self.re_fsdedup_volume_total_allocated_clusters.as_ref()
    }

    /// Sets the value of ReFSDedupVolumeTotalAllocatedCompressibleClusters
    pub fn set_re_fsdedup_volume_total_allocated_compressible_clusters(&mut self, value: u64) {
        self.re_fsdedup_volume_total_allocated_compressible_clusters = Some(value);
    }

    /// Gets the value of ReFSDedupVolumeTotalAllocatedCompressibleClusters
    pub fn get_re_fsdedup_volume_total_allocated_compressible_clusters(&self) -> Option<&u64> {
        self.re_fsdedup_volume_total_allocated_compressible_clusters.as_ref()
    }

    /// Sets the value of ReFSDedupVolumeTotalClusters
    pub fn set_re_fsdedup_volume_total_clusters(&mut self, value: u64) {
        self.re_fsdedup_volume_total_clusters = Some(value);
    }

    /// Gets the value of ReFSDedupVolumeTotalClusters
    pub fn get_re_fsdedup_volume_total_clusters(&self) -> Option<&u64> {
        self.re_fsdedup_volume_total_clusters.as_ref()
    }

    /// Sets the value of ReFSDedupVolumeTotalCompressedClusters
    pub fn set_re_fsdedup_volume_total_compressed_clusters(&mut self, value: u64) {
        self.re_fsdedup_volume_total_compressed_clusters = Some(value);
    }

    /// Gets the value of ReFSDedupVolumeTotalCompressedClusters
    pub fn get_re_fsdedup_volume_total_compressed_clusters(&self) -> Option<&u64> {
        self.re_fsdedup_volume_total_compressed_clusters.as_ref()
    }

    /// Sets the value of ReFSDedupVolumeTotalCompressionSavings
    pub fn set_re_fsdedup_volume_total_compression_savings(&mut self, value: u64) {
        self.re_fsdedup_volume_total_compression_savings = Some(value);
    }

    /// Gets the value of ReFSDedupVolumeTotalCompressionSavings
    pub fn get_re_fsdedup_volume_total_compression_savings(&self) -> Option<&u64> {
        self.re_fsdedup_volume_total_compression_savings.as_ref()
    }

    /// Sets the value of ReFSDedupVolumeTotalInUseCompressibleClusters
    pub fn set_re_fsdedup_volume_total_in_use_compressible_clusters(&mut self, value: u64) {
        self.re_fsdedup_volume_total_in_use_compressible_clusters = Some(value);
    }

    /// Gets the value of ReFSDedupVolumeTotalInUseCompressibleClusters
    pub fn get_re_fsdedup_volume_total_in_use_compressible_clusters(&self) -> Option<&u64> {
        self.re_fsdedup_volume_total_in_use_compressible_clusters.as_ref()
    }

    /// Sets the value of SavingsRate
    pub fn set_savings_rate(&mut self, value: u32) {
        self.savings_rate = Some(value);
    }

    /// Gets the value of SavingsRate
    pub fn get_savings_rate(&self) -> Option<&u32> {
        self.savings_rate.as_ref()
    }

    /// Sets the value of SavingsSize
    pub fn set_savings_size(&mut self, value: u64) {
        self.savings_size = Some(value);
    }

    /// Gets the value of SavingsSize
    pub fn get_savings_size(&self) -> Option<&u64> {
        self.savings_size.as_ref()
    }

    /// Sets the value of UnoptimizedSize
    pub fn set_unoptimized_size(&mut self, value: u64) {
        self.unoptimized_size = Some(value);
    }

    /// Gets the value of UnoptimizedSize
    pub fn get_unoptimized_size(&self) -> Option<&u64> {
        self.unoptimized_size.as_ref()
    }
}

