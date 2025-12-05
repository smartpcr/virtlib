// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_StorageSpacesVirtualDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_StorageSpacesVirtualDisk {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "VirtualDiskActive")]
    pub virtual_disk_active: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskActiveBytes")]
    pub virtual_disk_active_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskFailedReplacementBytes")]
    pub virtual_disk_failed_replacement_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskFailedReplacementCount")]
    pub virtual_disk_failed_replacement_count: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskMissing")]
    pub virtual_disk_missing: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskMissingBytes")]
    pub virtual_disk_missing_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskNeedReallocation")]
    pub virtual_disk_need_reallocation: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskNeedReallocationBytes")]
    pub virtual_disk_need_reallocation_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskNeedRegeneration")]
    pub virtual_disk_need_regeneration: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskNeedRegenerationBytes")]
    pub virtual_disk_need_regeneration_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskPendingDeletion")]
    pub virtual_disk_pending_deletion: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskPendingDeletionBytes")]
    pub virtual_disk_pending_deletion_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskReasonFailure")]
    pub virtual_disk_reason_failure: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskReasonFailureBytes")]
    pub virtual_disk_reason_failure_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskReasonHardwareError")]
    pub virtual_disk_reason_hardware_error: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskReasonHardwareErrorBytes")]
    pub virtual_disk_reason_hardware_error_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskReasonIoError")]
    pub virtual_disk_reason_io_error: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskReasonIoErrorBytes")]
    pub virtual_disk_reason_io_error_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskReasonMissing")]
    pub virtual_disk_reason_missing: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskReasonMissingBytes")]
    pub virtual_disk_reason_missing_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskReasonNew")]
    pub virtual_disk_reason_new: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskReasonNewBytes")]
    pub virtual_disk_reason_new_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskReasonRegenReadError")]
    pub virtual_disk_reason_regen_read_error: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskReasonRegenReadErrorBytes")]
    pub virtual_disk_reason_regen_read_error_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskReasonRegenWriteError")]
    pub virtual_disk_reason_regen_write_error: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskReasonRegenWriteErrorBytes")]
    pub virtual_disk_reason_regen_write_error_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskReasonRetired")]
    pub virtual_disk_reason_retired: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskReasonRetiredBytes")]
    pub virtual_disk_reason_retired_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskRebalanceReplacementBytes")]
    pub virtual_disk_rebalance_replacement_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskRebalanceReplacementCount")]
    pub virtual_disk_rebalance_replacement_count: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskRegenerating")]
    pub virtual_disk_regenerating: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskRegeneratingBytes")]
    pub virtual_disk_regenerating_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskRegenerationOutstandingBytes")]
    pub virtual_disk_regeneration_outstanding_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskRegenerationProcessedBytes")]
    pub virtual_disk_regeneration_processed_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskRegenerationSkippedBytes")]
    pub virtual_disk_regeneration_skipped_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskRegenerationTotalBytes")]
    pub virtual_disk_regeneration_total_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskRepairNeedPhase2Count")]
    pub virtual_disk_repair_need_phase2_count: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskRepairNeedPhase6Count")]
    pub virtual_disk_repair_need_phase6_count: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskRepairPhase1Count")]
    pub virtual_disk_repair_phase1_count: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskRepairPhase1Status")]
    pub virtual_disk_repair_phase1_status: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskRepairPhase2Count")]
    pub virtual_disk_repair_phase2_count: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskRepairPhase2Status")]
    pub virtual_disk_repair_phase2_status: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskRepairPhase3Count")]
    pub virtual_disk_repair_phase3_count: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskRepairPhase3Status")]
    pub virtual_disk_repair_phase3_status: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskRepairPhase4Count")]
    pub virtual_disk_repair_phase4_count: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskRepairPhase4Status")]
    pub virtual_disk_repair_phase4_status: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskRepairPhase5Count")]
    pub virtual_disk_repair_phase5_count: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskRepairPhase5Status")]
    pub virtual_disk_repair_phase5_status: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskRepairPhase6Count")]
    pub virtual_disk_repair_phase6_count: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskRepairPhase6Status")]
    pub virtual_disk_repair_phase6_status: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskRepairReplacementBytes")]
    pub virtual_disk_repair_replacement_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskRepairReplacementCount")]
    pub virtual_disk_repair_replacement_count: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskScopeRegenerationBytes")]
    pub virtual_disk_scope_regeneration_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskScopeRegenerationCount")]
    pub virtual_disk_scope_regeneration_count: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskScrubBytesPersec")]
    pub virtual_disk_scrub_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskScrubLatencyms")]
    pub virtual_disk_scrub_latencyms: Option<u32>,

/// 
    #[serde(rename = "VirtualDiskScrubRepairedBytesPersec")]
    pub virtual_disk_scrub_repaired_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskStale")]
    pub virtual_disk_stale: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskStaleBytes")]
    pub virtual_disk_stale_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskTotal")]
    pub virtual_disk_total: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskTotalBytes")]
    pub virtual_disk_total_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskUnmappedBytes")]
    pub virtual_disk_unmapped_bytes: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskUnmappedCount")]
    pub virtual_disk_unmapped_count: Option<u64>,
}

impl Win32_PerfFormattedData_Counters_StorageSpacesVirtualDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            virtual_disk_active: None,
            virtual_disk_active_bytes: None,
            virtual_disk_failed_replacement_bytes: None,
            virtual_disk_failed_replacement_count: None,
            virtual_disk_missing: None,
            virtual_disk_missing_bytes: None,
            virtual_disk_need_reallocation: None,
            virtual_disk_need_reallocation_bytes: None,
            virtual_disk_need_regeneration: None,
            virtual_disk_need_regeneration_bytes: None,
            virtual_disk_pending_deletion: None,
            virtual_disk_pending_deletion_bytes: None,
            virtual_disk_reason_failure: None,
            virtual_disk_reason_failure_bytes: None,
            virtual_disk_reason_hardware_error: None,
            virtual_disk_reason_hardware_error_bytes: None,
            virtual_disk_reason_io_error: None,
            virtual_disk_reason_io_error_bytes: None,
            virtual_disk_reason_missing: None,
            virtual_disk_reason_missing_bytes: None,
            virtual_disk_reason_new: None,
            virtual_disk_reason_new_bytes: None,
            virtual_disk_reason_regen_read_error: None,
            virtual_disk_reason_regen_read_error_bytes: None,
            virtual_disk_reason_regen_write_error: None,
            virtual_disk_reason_regen_write_error_bytes: None,
            virtual_disk_reason_retired: None,
            virtual_disk_reason_retired_bytes: None,
            virtual_disk_rebalance_replacement_bytes: None,
            virtual_disk_rebalance_replacement_count: None,
            virtual_disk_regenerating: None,
            virtual_disk_regenerating_bytes: None,
            virtual_disk_regeneration_outstanding_bytes: None,
            virtual_disk_regeneration_processed_bytes: None,
            virtual_disk_regeneration_skipped_bytes: None,
            virtual_disk_regeneration_total_bytes: None,
            virtual_disk_repair_need_phase2_count: None,
            virtual_disk_repair_need_phase6_count: None,
            virtual_disk_repair_phase1_count: None,
            virtual_disk_repair_phase1_status: None,
            virtual_disk_repair_phase2_count: None,
            virtual_disk_repair_phase2_status: None,
            virtual_disk_repair_phase3_count: None,
            virtual_disk_repair_phase3_status: None,
            virtual_disk_repair_phase4_count: None,
            virtual_disk_repair_phase4_status: None,
            virtual_disk_repair_phase5_count: None,
            virtual_disk_repair_phase5_status: None,
            virtual_disk_repair_phase6_count: None,
            virtual_disk_repair_phase6_status: None,
            virtual_disk_repair_replacement_bytes: None,
            virtual_disk_repair_replacement_count: None,
            virtual_disk_scope_regeneration_bytes: None,
            virtual_disk_scope_regeneration_count: None,
            virtual_disk_scrub_bytes_persec: None,
            virtual_disk_scrub_latencyms: None,
            virtual_disk_scrub_repaired_bytes_persec: None,
            virtual_disk_stale: None,
            virtual_disk_stale_bytes: None,
            virtual_disk_total: None,
            virtual_disk_total_bytes: None,
            virtual_disk_unmapped_bytes: None,
            virtual_disk_unmapped_count: None,
        }
    }


    /// Sets the value of VirtualDiskActive
    pub fn set_virtual_disk_active(&mut self, value: u64) {
        self.virtual_disk_active = Some(value);
    }

    /// Gets the value of VirtualDiskActive
    pub fn get_virtual_disk_active(&self) -> Option<&u64> {
        self.virtual_disk_active.as_ref()
    }

    /// Sets the value of VirtualDiskActiveBytes
    pub fn set_virtual_disk_active_bytes(&mut self, value: u64) {
        self.virtual_disk_active_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskActiveBytes
    pub fn get_virtual_disk_active_bytes(&self) -> Option<&u64> {
        self.virtual_disk_active_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskFailedReplacementBytes
    pub fn set_virtual_disk_failed_replacement_bytes(&mut self, value: u64) {
        self.virtual_disk_failed_replacement_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskFailedReplacementBytes
    pub fn get_virtual_disk_failed_replacement_bytes(&self) -> Option<&u64> {
        self.virtual_disk_failed_replacement_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskFailedReplacementCount
    pub fn set_virtual_disk_failed_replacement_count(&mut self, value: u64) {
        self.virtual_disk_failed_replacement_count = Some(value);
    }

    /// Gets the value of VirtualDiskFailedReplacementCount
    pub fn get_virtual_disk_failed_replacement_count(&self) -> Option<&u64> {
        self.virtual_disk_failed_replacement_count.as_ref()
    }

    /// Sets the value of VirtualDiskMissing
    pub fn set_virtual_disk_missing(&mut self, value: u64) {
        self.virtual_disk_missing = Some(value);
    }

    /// Gets the value of VirtualDiskMissing
    pub fn get_virtual_disk_missing(&self) -> Option<&u64> {
        self.virtual_disk_missing.as_ref()
    }

    /// Sets the value of VirtualDiskMissingBytes
    pub fn set_virtual_disk_missing_bytes(&mut self, value: u64) {
        self.virtual_disk_missing_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskMissingBytes
    pub fn get_virtual_disk_missing_bytes(&self) -> Option<&u64> {
        self.virtual_disk_missing_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskNeedReallocation
    pub fn set_virtual_disk_need_reallocation(&mut self, value: u64) {
        self.virtual_disk_need_reallocation = Some(value);
    }

    /// Gets the value of VirtualDiskNeedReallocation
    pub fn get_virtual_disk_need_reallocation(&self) -> Option<&u64> {
        self.virtual_disk_need_reallocation.as_ref()
    }

    /// Sets the value of VirtualDiskNeedReallocationBytes
    pub fn set_virtual_disk_need_reallocation_bytes(&mut self, value: u64) {
        self.virtual_disk_need_reallocation_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskNeedReallocationBytes
    pub fn get_virtual_disk_need_reallocation_bytes(&self) -> Option<&u64> {
        self.virtual_disk_need_reallocation_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskNeedRegeneration
    pub fn set_virtual_disk_need_regeneration(&mut self, value: u64) {
        self.virtual_disk_need_regeneration = Some(value);
    }

    /// Gets the value of VirtualDiskNeedRegeneration
    pub fn get_virtual_disk_need_regeneration(&self) -> Option<&u64> {
        self.virtual_disk_need_regeneration.as_ref()
    }

    /// Sets the value of VirtualDiskNeedRegenerationBytes
    pub fn set_virtual_disk_need_regeneration_bytes(&mut self, value: u64) {
        self.virtual_disk_need_regeneration_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskNeedRegenerationBytes
    pub fn get_virtual_disk_need_regeneration_bytes(&self) -> Option<&u64> {
        self.virtual_disk_need_regeneration_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskPendingDeletion
    pub fn set_virtual_disk_pending_deletion(&mut self, value: u64) {
        self.virtual_disk_pending_deletion = Some(value);
    }

    /// Gets the value of VirtualDiskPendingDeletion
    pub fn get_virtual_disk_pending_deletion(&self) -> Option<&u64> {
        self.virtual_disk_pending_deletion.as_ref()
    }

    /// Sets the value of VirtualDiskPendingDeletionBytes
    pub fn set_virtual_disk_pending_deletion_bytes(&mut self, value: u64) {
        self.virtual_disk_pending_deletion_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskPendingDeletionBytes
    pub fn get_virtual_disk_pending_deletion_bytes(&self) -> Option<&u64> {
        self.virtual_disk_pending_deletion_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskReasonFailure
    pub fn set_virtual_disk_reason_failure(&mut self, value: u64) {
        self.virtual_disk_reason_failure = Some(value);
    }

    /// Gets the value of VirtualDiskReasonFailure
    pub fn get_virtual_disk_reason_failure(&self) -> Option<&u64> {
        self.virtual_disk_reason_failure.as_ref()
    }

    /// Sets the value of VirtualDiskReasonFailureBytes
    pub fn set_virtual_disk_reason_failure_bytes(&mut self, value: u64) {
        self.virtual_disk_reason_failure_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskReasonFailureBytes
    pub fn get_virtual_disk_reason_failure_bytes(&self) -> Option<&u64> {
        self.virtual_disk_reason_failure_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskReasonHardwareError
    pub fn set_virtual_disk_reason_hardware_error(&mut self, value: u64) {
        self.virtual_disk_reason_hardware_error = Some(value);
    }

    /// Gets the value of VirtualDiskReasonHardwareError
    pub fn get_virtual_disk_reason_hardware_error(&self) -> Option<&u64> {
        self.virtual_disk_reason_hardware_error.as_ref()
    }

    /// Sets the value of VirtualDiskReasonHardwareErrorBytes
    pub fn set_virtual_disk_reason_hardware_error_bytes(&mut self, value: u64) {
        self.virtual_disk_reason_hardware_error_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskReasonHardwareErrorBytes
    pub fn get_virtual_disk_reason_hardware_error_bytes(&self) -> Option<&u64> {
        self.virtual_disk_reason_hardware_error_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskReasonIoError
    pub fn set_virtual_disk_reason_io_error(&mut self, value: u64) {
        self.virtual_disk_reason_io_error = Some(value);
    }

    /// Gets the value of VirtualDiskReasonIoError
    pub fn get_virtual_disk_reason_io_error(&self) -> Option<&u64> {
        self.virtual_disk_reason_io_error.as_ref()
    }

    /// Sets the value of VirtualDiskReasonIoErrorBytes
    pub fn set_virtual_disk_reason_io_error_bytes(&mut self, value: u64) {
        self.virtual_disk_reason_io_error_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskReasonIoErrorBytes
    pub fn get_virtual_disk_reason_io_error_bytes(&self) -> Option<&u64> {
        self.virtual_disk_reason_io_error_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskReasonMissing
    pub fn set_virtual_disk_reason_missing(&mut self, value: u64) {
        self.virtual_disk_reason_missing = Some(value);
    }

    /// Gets the value of VirtualDiskReasonMissing
    pub fn get_virtual_disk_reason_missing(&self) -> Option<&u64> {
        self.virtual_disk_reason_missing.as_ref()
    }

    /// Sets the value of VirtualDiskReasonMissingBytes
    pub fn set_virtual_disk_reason_missing_bytes(&mut self, value: u64) {
        self.virtual_disk_reason_missing_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskReasonMissingBytes
    pub fn get_virtual_disk_reason_missing_bytes(&self) -> Option<&u64> {
        self.virtual_disk_reason_missing_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskReasonNew
    pub fn set_virtual_disk_reason_new(&mut self, value: u64) {
        self.virtual_disk_reason_new = Some(value);
    }

    /// Gets the value of VirtualDiskReasonNew
    pub fn get_virtual_disk_reason_new(&self) -> Option<&u64> {
        self.virtual_disk_reason_new.as_ref()
    }

    /// Sets the value of VirtualDiskReasonNewBytes
    pub fn set_virtual_disk_reason_new_bytes(&mut self, value: u64) {
        self.virtual_disk_reason_new_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskReasonNewBytes
    pub fn get_virtual_disk_reason_new_bytes(&self) -> Option<&u64> {
        self.virtual_disk_reason_new_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskReasonRegenReadError
    pub fn set_virtual_disk_reason_regen_read_error(&mut self, value: u64) {
        self.virtual_disk_reason_regen_read_error = Some(value);
    }

    /// Gets the value of VirtualDiskReasonRegenReadError
    pub fn get_virtual_disk_reason_regen_read_error(&self) -> Option<&u64> {
        self.virtual_disk_reason_regen_read_error.as_ref()
    }

    /// Sets the value of VirtualDiskReasonRegenReadErrorBytes
    pub fn set_virtual_disk_reason_regen_read_error_bytes(&mut self, value: u64) {
        self.virtual_disk_reason_regen_read_error_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskReasonRegenReadErrorBytes
    pub fn get_virtual_disk_reason_regen_read_error_bytes(&self) -> Option<&u64> {
        self.virtual_disk_reason_regen_read_error_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskReasonRegenWriteError
    pub fn set_virtual_disk_reason_regen_write_error(&mut self, value: u64) {
        self.virtual_disk_reason_regen_write_error = Some(value);
    }

    /// Gets the value of VirtualDiskReasonRegenWriteError
    pub fn get_virtual_disk_reason_regen_write_error(&self) -> Option<&u64> {
        self.virtual_disk_reason_regen_write_error.as_ref()
    }

    /// Sets the value of VirtualDiskReasonRegenWriteErrorBytes
    pub fn set_virtual_disk_reason_regen_write_error_bytes(&mut self, value: u64) {
        self.virtual_disk_reason_regen_write_error_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskReasonRegenWriteErrorBytes
    pub fn get_virtual_disk_reason_regen_write_error_bytes(&self) -> Option<&u64> {
        self.virtual_disk_reason_regen_write_error_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskReasonRetired
    pub fn set_virtual_disk_reason_retired(&mut self, value: u64) {
        self.virtual_disk_reason_retired = Some(value);
    }

    /// Gets the value of VirtualDiskReasonRetired
    pub fn get_virtual_disk_reason_retired(&self) -> Option<&u64> {
        self.virtual_disk_reason_retired.as_ref()
    }

    /// Sets the value of VirtualDiskReasonRetiredBytes
    pub fn set_virtual_disk_reason_retired_bytes(&mut self, value: u64) {
        self.virtual_disk_reason_retired_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskReasonRetiredBytes
    pub fn get_virtual_disk_reason_retired_bytes(&self) -> Option<&u64> {
        self.virtual_disk_reason_retired_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskRebalanceReplacementBytes
    pub fn set_virtual_disk_rebalance_replacement_bytes(&mut self, value: u64) {
        self.virtual_disk_rebalance_replacement_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskRebalanceReplacementBytes
    pub fn get_virtual_disk_rebalance_replacement_bytes(&self) -> Option<&u64> {
        self.virtual_disk_rebalance_replacement_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskRebalanceReplacementCount
    pub fn set_virtual_disk_rebalance_replacement_count(&mut self, value: u64) {
        self.virtual_disk_rebalance_replacement_count = Some(value);
    }

    /// Gets the value of VirtualDiskRebalanceReplacementCount
    pub fn get_virtual_disk_rebalance_replacement_count(&self) -> Option<&u64> {
        self.virtual_disk_rebalance_replacement_count.as_ref()
    }

    /// Sets the value of VirtualDiskRegenerating
    pub fn set_virtual_disk_regenerating(&mut self, value: u64) {
        self.virtual_disk_regenerating = Some(value);
    }

    /// Gets the value of VirtualDiskRegenerating
    pub fn get_virtual_disk_regenerating(&self) -> Option<&u64> {
        self.virtual_disk_regenerating.as_ref()
    }

    /// Sets the value of VirtualDiskRegeneratingBytes
    pub fn set_virtual_disk_regenerating_bytes(&mut self, value: u64) {
        self.virtual_disk_regenerating_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskRegeneratingBytes
    pub fn get_virtual_disk_regenerating_bytes(&self) -> Option<&u64> {
        self.virtual_disk_regenerating_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskRegenerationOutstandingBytes
    pub fn set_virtual_disk_regeneration_outstanding_bytes(&mut self, value: u64) {
        self.virtual_disk_regeneration_outstanding_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskRegenerationOutstandingBytes
    pub fn get_virtual_disk_regeneration_outstanding_bytes(&self) -> Option<&u64> {
        self.virtual_disk_regeneration_outstanding_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskRegenerationProcessedBytes
    pub fn set_virtual_disk_regeneration_processed_bytes(&mut self, value: u64) {
        self.virtual_disk_regeneration_processed_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskRegenerationProcessedBytes
    pub fn get_virtual_disk_regeneration_processed_bytes(&self) -> Option<&u64> {
        self.virtual_disk_regeneration_processed_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskRegenerationSkippedBytes
    pub fn set_virtual_disk_regeneration_skipped_bytes(&mut self, value: u64) {
        self.virtual_disk_regeneration_skipped_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskRegenerationSkippedBytes
    pub fn get_virtual_disk_regeneration_skipped_bytes(&self) -> Option<&u64> {
        self.virtual_disk_regeneration_skipped_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskRegenerationTotalBytes
    pub fn set_virtual_disk_regeneration_total_bytes(&mut self, value: u64) {
        self.virtual_disk_regeneration_total_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskRegenerationTotalBytes
    pub fn get_virtual_disk_regeneration_total_bytes(&self) -> Option<&u64> {
        self.virtual_disk_regeneration_total_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskRepairNeedPhase2Count
    pub fn set_virtual_disk_repair_need_phase2_count(&mut self, value: u64) {
        self.virtual_disk_repair_need_phase2_count = Some(value);
    }

    /// Gets the value of VirtualDiskRepairNeedPhase2Count
    pub fn get_virtual_disk_repair_need_phase2_count(&self) -> Option<&u64> {
        self.virtual_disk_repair_need_phase2_count.as_ref()
    }

    /// Sets the value of VirtualDiskRepairNeedPhase6Count
    pub fn set_virtual_disk_repair_need_phase6_count(&mut self, value: u64) {
        self.virtual_disk_repair_need_phase6_count = Some(value);
    }

    /// Gets the value of VirtualDiskRepairNeedPhase6Count
    pub fn get_virtual_disk_repair_need_phase6_count(&self) -> Option<&u64> {
        self.virtual_disk_repair_need_phase6_count.as_ref()
    }

    /// Sets the value of VirtualDiskRepairPhase1Count
    pub fn set_virtual_disk_repair_phase1_count(&mut self, value: u64) {
        self.virtual_disk_repair_phase1_count = Some(value);
    }

    /// Gets the value of VirtualDiskRepairPhase1Count
    pub fn get_virtual_disk_repair_phase1_count(&self) -> Option<&u64> {
        self.virtual_disk_repair_phase1_count.as_ref()
    }

    /// Sets the value of VirtualDiskRepairPhase1Status
    pub fn set_virtual_disk_repair_phase1_status(&mut self, value: u64) {
        self.virtual_disk_repair_phase1_status = Some(value);
    }

    /// Gets the value of VirtualDiskRepairPhase1Status
    pub fn get_virtual_disk_repair_phase1_status(&self) -> Option<&u64> {
        self.virtual_disk_repair_phase1_status.as_ref()
    }

    /// Sets the value of VirtualDiskRepairPhase2Count
    pub fn set_virtual_disk_repair_phase2_count(&mut self, value: u64) {
        self.virtual_disk_repair_phase2_count = Some(value);
    }

    /// Gets the value of VirtualDiskRepairPhase2Count
    pub fn get_virtual_disk_repair_phase2_count(&self) -> Option<&u64> {
        self.virtual_disk_repair_phase2_count.as_ref()
    }

    /// Sets the value of VirtualDiskRepairPhase2Status
    pub fn set_virtual_disk_repair_phase2_status(&mut self, value: u64) {
        self.virtual_disk_repair_phase2_status = Some(value);
    }

    /// Gets the value of VirtualDiskRepairPhase2Status
    pub fn get_virtual_disk_repair_phase2_status(&self) -> Option<&u64> {
        self.virtual_disk_repair_phase2_status.as_ref()
    }

    /// Sets the value of VirtualDiskRepairPhase3Count
    pub fn set_virtual_disk_repair_phase3_count(&mut self, value: u64) {
        self.virtual_disk_repair_phase3_count = Some(value);
    }

    /// Gets the value of VirtualDiskRepairPhase3Count
    pub fn get_virtual_disk_repair_phase3_count(&self) -> Option<&u64> {
        self.virtual_disk_repair_phase3_count.as_ref()
    }

    /// Sets the value of VirtualDiskRepairPhase3Status
    pub fn set_virtual_disk_repair_phase3_status(&mut self, value: u64) {
        self.virtual_disk_repair_phase3_status = Some(value);
    }

    /// Gets the value of VirtualDiskRepairPhase3Status
    pub fn get_virtual_disk_repair_phase3_status(&self) -> Option<&u64> {
        self.virtual_disk_repair_phase3_status.as_ref()
    }

    /// Sets the value of VirtualDiskRepairPhase4Count
    pub fn set_virtual_disk_repair_phase4_count(&mut self, value: u64) {
        self.virtual_disk_repair_phase4_count = Some(value);
    }

    /// Gets the value of VirtualDiskRepairPhase4Count
    pub fn get_virtual_disk_repair_phase4_count(&self) -> Option<&u64> {
        self.virtual_disk_repair_phase4_count.as_ref()
    }

    /// Sets the value of VirtualDiskRepairPhase4Status
    pub fn set_virtual_disk_repair_phase4_status(&mut self, value: u64) {
        self.virtual_disk_repair_phase4_status = Some(value);
    }

    /// Gets the value of VirtualDiskRepairPhase4Status
    pub fn get_virtual_disk_repair_phase4_status(&self) -> Option<&u64> {
        self.virtual_disk_repair_phase4_status.as_ref()
    }

    /// Sets the value of VirtualDiskRepairPhase5Count
    pub fn set_virtual_disk_repair_phase5_count(&mut self, value: u64) {
        self.virtual_disk_repair_phase5_count = Some(value);
    }

    /// Gets the value of VirtualDiskRepairPhase5Count
    pub fn get_virtual_disk_repair_phase5_count(&self) -> Option<&u64> {
        self.virtual_disk_repair_phase5_count.as_ref()
    }

    /// Sets the value of VirtualDiskRepairPhase5Status
    pub fn set_virtual_disk_repair_phase5_status(&mut self, value: u64) {
        self.virtual_disk_repair_phase5_status = Some(value);
    }

    /// Gets the value of VirtualDiskRepairPhase5Status
    pub fn get_virtual_disk_repair_phase5_status(&self) -> Option<&u64> {
        self.virtual_disk_repair_phase5_status.as_ref()
    }

    /// Sets the value of VirtualDiskRepairPhase6Count
    pub fn set_virtual_disk_repair_phase6_count(&mut self, value: u64) {
        self.virtual_disk_repair_phase6_count = Some(value);
    }

    /// Gets the value of VirtualDiskRepairPhase6Count
    pub fn get_virtual_disk_repair_phase6_count(&self) -> Option<&u64> {
        self.virtual_disk_repair_phase6_count.as_ref()
    }

    /// Sets the value of VirtualDiskRepairPhase6Status
    pub fn set_virtual_disk_repair_phase6_status(&mut self, value: u64) {
        self.virtual_disk_repair_phase6_status = Some(value);
    }

    /// Gets the value of VirtualDiskRepairPhase6Status
    pub fn get_virtual_disk_repair_phase6_status(&self) -> Option<&u64> {
        self.virtual_disk_repair_phase6_status.as_ref()
    }

    /// Sets the value of VirtualDiskRepairReplacementBytes
    pub fn set_virtual_disk_repair_replacement_bytes(&mut self, value: u64) {
        self.virtual_disk_repair_replacement_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskRepairReplacementBytes
    pub fn get_virtual_disk_repair_replacement_bytes(&self) -> Option<&u64> {
        self.virtual_disk_repair_replacement_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskRepairReplacementCount
    pub fn set_virtual_disk_repair_replacement_count(&mut self, value: u64) {
        self.virtual_disk_repair_replacement_count = Some(value);
    }

    /// Gets the value of VirtualDiskRepairReplacementCount
    pub fn get_virtual_disk_repair_replacement_count(&self) -> Option<&u64> {
        self.virtual_disk_repair_replacement_count.as_ref()
    }

    /// Sets the value of VirtualDiskScopeRegenerationBytes
    pub fn set_virtual_disk_scope_regeneration_bytes(&mut self, value: u64) {
        self.virtual_disk_scope_regeneration_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskScopeRegenerationBytes
    pub fn get_virtual_disk_scope_regeneration_bytes(&self) -> Option<&u64> {
        self.virtual_disk_scope_regeneration_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskScopeRegenerationCount
    pub fn set_virtual_disk_scope_regeneration_count(&mut self, value: u64) {
        self.virtual_disk_scope_regeneration_count = Some(value);
    }

    /// Gets the value of VirtualDiskScopeRegenerationCount
    pub fn get_virtual_disk_scope_regeneration_count(&self) -> Option<&u64> {
        self.virtual_disk_scope_regeneration_count.as_ref()
    }

    /// Sets the value of VirtualDiskScrubBytesPersec
    pub fn set_virtual_disk_scrub_bytes_persec(&mut self, value: u64) {
        self.virtual_disk_scrub_bytes_persec = Some(value);
    }

    /// Gets the value of VirtualDiskScrubBytesPersec
    pub fn get_virtual_disk_scrub_bytes_persec(&self) -> Option<&u64> {
        self.virtual_disk_scrub_bytes_persec.as_ref()
    }

    /// Sets the value of VirtualDiskScrubLatencyms
    pub fn set_virtual_disk_scrub_latencyms(&mut self, value: u32) {
        self.virtual_disk_scrub_latencyms = Some(value);
    }

    /// Gets the value of VirtualDiskScrubLatencyms
    pub fn get_virtual_disk_scrub_latencyms(&self) -> Option<&u32> {
        self.virtual_disk_scrub_latencyms.as_ref()
    }

    /// Sets the value of VirtualDiskScrubRepairedBytesPersec
    pub fn set_virtual_disk_scrub_repaired_bytes_persec(&mut self, value: u64) {
        self.virtual_disk_scrub_repaired_bytes_persec = Some(value);
    }

    /// Gets the value of VirtualDiskScrubRepairedBytesPersec
    pub fn get_virtual_disk_scrub_repaired_bytes_persec(&self) -> Option<&u64> {
        self.virtual_disk_scrub_repaired_bytes_persec.as_ref()
    }

    /// Sets the value of VirtualDiskStale
    pub fn set_virtual_disk_stale(&mut self, value: u64) {
        self.virtual_disk_stale = Some(value);
    }

    /// Gets the value of VirtualDiskStale
    pub fn get_virtual_disk_stale(&self) -> Option<&u64> {
        self.virtual_disk_stale.as_ref()
    }

    /// Sets the value of VirtualDiskStaleBytes
    pub fn set_virtual_disk_stale_bytes(&mut self, value: u64) {
        self.virtual_disk_stale_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskStaleBytes
    pub fn get_virtual_disk_stale_bytes(&self) -> Option<&u64> {
        self.virtual_disk_stale_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskTotal
    pub fn set_virtual_disk_total(&mut self, value: u64) {
        self.virtual_disk_total = Some(value);
    }

    /// Gets the value of VirtualDiskTotal
    pub fn get_virtual_disk_total(&self) -> Option<&u64> {
        self.virtual_disk_total.as_ref()
    }

    /// Sets the value of VirtualDiskTotalBytes
    pub fn set_virtual_disk_total_bytes(&mut self, value: u64) {
        self.virtual_disk_total_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskTotalBytes
    pub fn get_virtual_disk_total_bytes(&self) -> Option<&u64> {
        self.virtual_disk_total_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskUnmappedBytes
    pub fn set_virtual_disk_unmapped_bytes(&mut self, value: u64) {
        self.virtual_disk_unmapped_bytes = Some(value);
    }

    /// Gets the value of VirtualDiskUnmappedBytes
    pub fn get_virtual_disk_unmapped_bytes(&self) -> Option<&u64> {
        self.virtual_disk_unmapped_bytes.as_ref()
    }

    /// Sets the value of VirtualDiskUnmappedCount
    pub fn set_virtual_disk_unmapped_count(&mut self, value: u64) {
        self.virtual_disk_unmapped_count = Some(value);
    }

    /// Gets the value of VirtualDiskUnmappedCount
    pub fn get_virtual_disk_unmapped_count(&self) -> Option<&u64> {
        self.virtual_disk_unmapped_count.as_ref()
    }
}

