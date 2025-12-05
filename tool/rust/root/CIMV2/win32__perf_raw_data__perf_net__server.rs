// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_PerfNet_Server struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_PerfNet_Server {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "BlockingRequestsRejected")]
    pub blocking_requests_rejected: Option<u32>,

/// 
    #[serde(rename = "BytesReceivedPersec")]
    pub bytes_received_persec: Option<u64>,

/// 
    #[serde(rename = "BytesTotalPersec")]
    pub bytes_total_persec: Option<u64>,

/// 
    #[serde(rename = "BytesTransmittedPersec")]
    pub bytes_transmitted_persec: Option<u64>,

/// 
    #[serde(rename = "ContextBlocksQueuedPersec")]
    pub context_blocks_queued_persec: Option<u32>,

/// 
    #[serde(rename = "ErrorsAccessPermissions")]
    pub errors_access_permissions: Option<u32>,

/// 
    #[serde(rename = "ErrorsGrantedAccess")]
    pub errors_granted_access: Option<u32>,

/// 
    #[serde(rename = "ErrorsLogon")]
    pub errors_logon: Option<u32>,

/// 
    #[serde(rename = "ErrorsSystem")]
    pub errors_system: Option<u32>,

/// 
    #[serde(rename = "FileDirectorySearches")]
    pub file_directory_searches: Option<u32>,

/// 
    #[serde(rename = "FilesOpen")]
    pub files_open: Option<u32>,

/// 
    #[serde(rename = "FilesOpenedTotal")]
    pub files_opened_total: Option<u32>,

/// 
    #[serde(rename = "LogonPersec")]
    pub logon_persec: Option<u32>,

/// 
    #[serde(rename = "LogonTotal")]
    pub logon_total: Option<u32>,

/// 
    #[serde(rename = "PoolNonpagedBytes")]
    pub pool_nonpaged_bytes: Option<u32>,

/// 
    #[serde(rename = "PoolNonpagedFailures")]
    pub pool_nonpaged_failures: Option<u32>,

/// 
    #[serde(rename = "PoolNonpagedPeak")]
    pub pool_nonpaged_peak: Option<u32>,

/// 
    #[serde(rename = "PoolPagedBytes")]
    pub pool_paged_bytes: Option<u32>,

/// 
    #[serde(rename = "PoolPagedFailures")]
    pub pool_paged_failures: Option<u32>,

/// 
    #[serde(rename = "PoolPagedPeak")]
    pub pool_paged_peak: Option<u32>,

/// 
    #[serde(rename = "ReconnectedDurableHandles")]
    pub reconnected_durable_handles: Option<u32>,

/// 
    #[serde(rename = "ReconnectedResilientHandles")]
    pub reconnected_resilient_handles: Option<u32>,

/// 
    #[serde(rename = "ServerSessions")]
    pub server_sessions: Option<u32>,

/// 
    #[serde(rename = "SessionsErroredOut")]
    pub sessions_errored_out: Option<u32>,

/// 
    #[serde(rename = "SessionsForcedOff")]
    pub sessions_forced_off: Option<u32>,

/// 
    #[serde(rename = "SessionsLoggedOff")]
    pub sessions_logged_off: Option<u32>,

/// 
    #[serde(rename = "SessionsTimedOut")]
    pub sessions_timed_out: Option<u32>,

/// 
    #[serde(rename = "SMBBranchCacheHashBytesSent")]
    pub smbbranch_cache_hash_bytes_sent: Option<u64>,

/// 
    #[serde(rename = "SMBBranchCacheHashGenerationRequests")]
    pub smbbranch_cache_hash_generation_requests: Option<u32>,

/// 
    #[serde(rename = "SMBBranchCacheHashHeaderRequests")]
    pub smbbranch_cache_hash_header_requests: Option<u32>,

/// 
    #[serde(rename = "SMBBranchCacheHashRequestsReceived")]
    pub smbbranch_cache_hash_requests_received: Option<u32>,

/// 
    #[serde(rename = "SMBBranchCacheHashResponsesSent")]
    pub smbbranch_cache_hash_responses_sent: Option<u32>,

/// 
    #[serde(rename = "SMBBranchCacheHashV2BytesSent")]
    pub smbbranch_cache_hash_v2_bytes_sent: Option<u64>,

/// 
    #[serde(rename = "SMBBranchCacheHashV2GenerationRequests")]
    pub smbbranch_cache_hash_v2_generation_requests: Option<u32>,

/// 
    #[serde(rename = "SMBBranchCacheHashV2HeaderRequests")]
    pub smbbranch_cache_hash_v2_header_requests: Option<u32>,

/// 
    #[serde(rename = "SMBBranchCacheHashV2RequestsReceived")]
    pub smbbranch_cache_hash_v2_requests_received: Option<u32>,

/// 
    #[serde(rename = "SMBBranchCacheHashV2RequestsServedFromDedup")]
    pub smbbranch_cache_hash_v2_requests_served_from_dedup: Option<u32>,

/// 
    #[serde(rename = "SMBBranchCacheHashV2ResponsesSent")]
    pub smbbranch_cache_hash_v2_responses_sent: Option<u32>,

/// 
    #[serde(rename = "TotalDurableHandles")]
    pub total_durable_handles: Option<u32>,

/// 
    #[serde(rename = "TotalResilientHandles")]
    pub total_resilient_handles: Option<u32>,

/// 
    #[serde(rename = "WorkItemShortages")]
    pub work_item_shortages: Option<u32>,
}

impl Win32_PerfRawData_PerfNet_Server {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            blocking_requests_rejected: None,
            bytes_received_persec: None,
            bytes_total_persec: None,
            bytes_transmitted_persec: None,
            context_blocks_queued_persec: None,
            errors_access_permissions: None,
            errors_granted_access: None,
            errors_logon: None,
            errors_system: None,
            file_directory_searches: None,
            files_open: None,
            files_opened_total: None,
            logon_persec: None,
            logon_total: None,
            pool_nonpaged_bytes: None,
            pool_nonpaged_failures: None,
            pool_nonpaged_peak: None,
            pool_paged_bytes: None,
            pool_paged_failures: None,
            pool_paged_peak: None,
            reconnected_durable_handles: None,
            reconnected_resilient_handles: None,
            server_sessions: None,
            sessions_errored_out: None,
            sessions_forced_off: None,
            sessions_logged_off: None,
            sessions_timed_out: None,
            smbbranch_cache_hash_bytes_sent: None,
            smbbranch_cache_hash_generation_requests: None,
            smbbranch_cache_hash_header_requests: None,
            smbbranch_cache_hash_requests_received: None,
            smbbranch_cache_hash_responses_sent: None,
            smbbranch_cache_hash_v2_bytes_sent: None,
            smbbranch_cache_hash_v2_generation_requests: None,
            smbbranch_cache_hash_v2_header_requests: None,
            smbbranch_cache_hash_v2_requests_received: None,
            smbbranch_cache_hash_v2_requests_served_from_dedup: None,
            smbbranch_cache_hash_v2_responses_sent: None,
            total_durable_handles: None,
            total_resilient_handles: None,
            work_item_shortages: None,
        }
    }


    /// Sets the value of BlockingRequestsRejected
    pub fn set_blocking_requests_rejected(&mut self, value: u32) {
        self.blocking_requests_rejected = Some(value);
    }

    /// Gets the value of BlockingRequestsRejected
    pub fn get_blocking_requests_rejected(&self) -> Option<&u32> {
        self.blocking_requests_rejected.as_ref()
    }

    /// Sets the value of BytesReceivedPersec
    pub fn set_bytes_received_persec(&mut self, value: u64) {
        self.bytes_received_persec = Some(value);
    }

    /// Gets the value of BytesReceivedPersec
    pub fn get_bytes_received_persec(&self) -> Option<&u64> {
        self.bytes_received_persec.as_ref()
    }

    /// Sets the value of BytesTotalPersec
    pub fn set_bytes_total_persec(&mut self, value: u64) {
        self.bytes_total_persec = Some(value);
    }

    /// Gets the value of BytesTotalPersec
    pub fn get_bytes_total_persec(&self) -> Option<&u64> {
        self.bytes_total_persec.as_ref()
    }

    /// Sets the value of BytesTransmittedPersec
    pub fn set_bytes_transmitted_persec(&mut self, value: u64) {
        self.bytes_transmitted_persec = Some(value);
    }

    /// Gets the value of BytesTransmittedPersec
    pub fn get_bytes_transmitted_persec(&self) -> Option<&u64> {
        self.bytes_transmitted_persec.as_ref()
    }

    /// Sets the value of ContextBlocksQueuedPersec
    pub fn set_context_blocks_queued_persec(&mut self, value: u32) {
        self.context_blocks_queued_persec = Some(value);
    }

    /// Gets the value of ContextBlocksQueuedPersec
    pub fn get_context_blocks_queued_persec(&self) -> Option<&u32> {
        self.context_blocks_queued_persec.as_ref()
    }

    /// Sets the value of ErrorsAccessPermissions
    pub fn set_errors_access_permissions(&mut self, value: u32) {
        self.errors_access_permissions = Some(value);
    }

    /// Gets the value of ErrorsAccessPermissions
    pub fn get_errors_access_permissions(&self) -> Option<&u32> {
        self.errors_access_permissions.as_ref()
    }

    /// Sets the value of ErrorsGrantedAccess
    pub fn set_errors_granted_access(&mut self, value: u32) {
        self.errors_granted_access = Some(value);
    }

    /// Gets the value of ErrorsGrantedAccess
    pub fn get_errors_granted_access(&self) -> Option<&u32> {
        self.errors_granted_access.as_ref()
    }

    /// Sets the value of ErrorsLogon
    pub fn set_errors_logon(&mut self, value: u32) {
        self.errors_logon = Some(value);
    }

    /// Gets the value of ErrorsLogon
    pub fn get_errors_logon(&self) -> Option<&u32> {
        self.errors_logon.as_ref()
    }

    /// Sets the value of ErrorsSystem
    pub fn set_errors_system(&mut self, value: u32) {
        self.errors_system = Some(value);
    }

    /// Gets the value of ErrorsSystem
    pub fn get_errors_system(&self) -> Option<&u32> {
        self.errors_system.as_ref()
    }

    /// Sets the value of FileDirectorySearches
    pub fn set_file_directory_searches(&mut self, value: u32) {
        self.file_directory_searches = Some(value);
    }

    /// Gets the value of FileDirectorySearches
    pub fn get_file_directory_searches(&self) -> Option<&u32> {
        self.file_directory_searches.as_ref()
    }

    /// Sets the value of FilesOpen
    pub fn set_files_open(&mut self, value: u32) {
        self.files_open = Some(value);
    }

    /// Gets the value of FilesOpen
    pub fn get_files_open(&self) -> Option<&u32> {
        self.files_open.as_ref()
    }

    /// Sets the value of FilesOpenedTotal
    pub fn set_files_opened_total(&mut self, value: u32) {
        self.files_opened_total = Some(value);
    }

    /// Gets the value of FilesOpenedTotal
    pub fn get_files_opened_total(&self) -> Option<&u32> {
        self.files_opened_total.as_ref()
    }

    /// Sets the value of LogonPersec
    pub fn set_logon_persec(&mut self, value: u32) {
        self.logon_persec = Some(value);
    }

    /// Gets the value of LogonPersec
    pub fn get_logon_persec(&self) -> Option<&u32> {
        self.logon_persec.as_ref()
    }

    /// Sets the value of LogonTotal
    pub fn set_logon_total(&mut self, value: u32) {
        self.logon_total = Some(value);
    }

    /// Gets the value of LogonTotal
    pub fn get_logon_total(&self) -> Option<&u32> {
        self.logon_total.as_ref()
    }

    /// Sets the value of PoolNonpagedBytes
    pub fn set_pool_nonpaged_bytes(&mut self, value: u32) {
        self.pool_nonpaged_bytes = Some(value);
    }

    /// Gets the value of PoolNonpagedBytes
    pub fn get_pool_nonpaged_bytes(&self) -> Option<&u32> {
        self.pool_nonpaged_bytes.as_ref()
    }

    /// Sets the value of PoolNonpagedFailures
    pub fn set_pool_nonpaged_failures(&mut self, value: u32) {
        self.pool_nonpaged_failures = Some(value);
    }

    /// Gets the value of PoolNonpagedFailures
    pub fn get_pool_nonpaged_failures(&self) -> Option<&u32> {
        self.pool_nonpaged_failures.as_ref()
    }

    /// Sets the value of PoolNonpagedPeak
    pub fn set_pool_nonpaged_peak(&mut self, value: u32) {
        self.pool_nonpaged_peak = Some(value);
    }

    /// Gets the value of PoolNonpagedPeak
    pub fn get_pool_nonpaged_peak(&self) -> Option<&u32> {
        self.pool_nonpaged_peak.as_ref()
    }

    /// Sets the value of PoolPagedBytes
    pub fn set_pool_paged_bytes(&mut self, value: u32) {
        self.pool_paged_bytes = Some(value);
    }

    /// Gets the value of PoolPagedBytes
    pub fn get_pool_paged_bytes(&self) -> Option<&u32> {
        self.pool_paged_bytes.as_ref()
    }

    /// Sets the value of PoolPagedFailures
    pub fn set_pool_paged_failures(&mut self, value: u32) {
        self.pool_paged_failures = Some(value);
    }

    /// Gets the value of PoolPagedFailures
    pub fn get_pool_paged_failures(&self) -> Option<&u32> {
        self.pool_paged_failures.as_ref()
    }

    /// Sets the value of PoolPagedPeak
    pub fn set_pool_paged_peak(&mut self, value: u32) {
        self.pool_paged_peak = Some(value);
    }

    /// Gets the value of PoolPagedPeak
    pub fn get_pool_paged_peak(&self) -> Option<&u32> {
        self.pool_paged_peak.as_ref()
    }

    /// Sets the value of ReconnectedDurableHandles
    pub fn set_reconnected_durable_handles(&mut self, value: u32) {
        self.reconnected_durable_handles = Some(value);
    }

    /// Gets the value of ReconnectedDurableHandles
    pub fn get_reconnected_durable_handles(&self) -> Option<&u32> {
        self.reconnected_durable_handles.as_ref()
    }

    /// Sets the value of ReconnectedResilientHandles
    pub fn set_reconnected_resilient_handles(&mut self, value: u32) {
        self.reconnected_resilient_handles = Some(value);
    }

    /// Gets the value of ReconnectedResilientHandles
    pub fn get_reconnected_resilient_handles(&self) -> Option<&u32> {
        self.reconnected_resilient_handles.as_ref()
    }

    /// Sets the value of ServerSessions
    pub fn set_server_sessions(&mut self, value: u32) {
        self.server_sessions = Some(value);
    }

    /// Gets the value of ServerSessions
    pub fn get_server_sessions(&self) -> Option<&u32> {
        self.server_sessions.as_ref()
    }

    /// Sets the value of SessionsErroredOut
    pub fn set_sessions_errored_out(&mut self, value: u32) {
        self.sessions_errored_out = Some(value);
    }

    /// Gets the value of SessionsErroredOut
    pub fn get_sessions_errored_out(&self) -> Option<&u32> {
        self.sessions_errored_out.as_ref()
    }

    /// Sets the value of SessionsForcedOff
    pub fn set_sessions_forced_off(&mut self, value: u32) {
        self.sessions_forced_off = Some(value);
    }

    /// Gets the value of SessionsForcedOff
    pub fn get_sessions_forced_off(&self) -> Option<&u32> {
        self.sessions_forced_off.as_ref()
    }

    /// Sets the value of SessionsLoggedOff
    pub fn set_sessions_logged_off(&mut self, value: u32) {
        self.sessions_logged_off = Some(value);
    }

    /// Gets the value of SessionsLoggedOff
    pub fn get_sessions_logged_off(&self) -> Option<&u32> {
        self.sessions_logged_off.as_ref()
    }

    /// Sets the value of SessionsTimedOut
    pub fn set_sessions_timed_out(&mut self, value: u32) {
        self.sessions_timed_out = Some(value);
    }

    /// Gets the value of SessionsTimedOut
    pub fn get_sessions_timed_out(&self) -> Option<&u32> {
        self.sessions_timed_out.as_ref()
    }

    /// Sets the value of SMBBranchCacheHashBytesSent
    pub fn set_smbbranch_cache_hash_bytes_sent(&mut self, value: u64) {
        self.smbbranch_cache_hash_bytes_sent = Some(value);
    }

    /// Gets the value of SMBBranchCacheHashBytesSent
    pub fn get_smbbranch_cache_hash_bytes_sent(&self) -> Option<&u64> {
        self.smbbranch_cache_hash_bytes_sent.as_ref()
    }

    /// Sets the value of SMBBranchCacheHashGenerationRequests
    pub fn set_smbbranch_cache_hash_generation_requests(&mut self, value: u32) {
        self.smbbranch_cache_hash_generation_requests = Some(value);
    }

    /// Gets the value of SMBBranchCacheHashGenerationRequests
    pub fn get_smbbranch_cache_hash_generation_requests(&self) -> Option<&u32> {
        self.smbbranch_cache_hash_generation_requests.as_ref()
    }

    /// Sets the value of SMBBranchCacheHashHeaderRequests
    pub fn set_smbbranch_cache_hash_header_requests(&mut self, value: u32) {
        self.smbbranch_cache_hash_header_requests = Some(value);
    }

    /// Gets the value of SMBBranchCacheHashHeaderRequests
    pub fn get_smbbranch_cache_hash_header_requests(&self) -> Option<&u32> {
        self.smbbranch_cache_hash_header_requests.as_ref()
    }

    /// Sets the value of SMBBranchCacheHashRequestsReceived
    pub fn set_smbbranch_cache_hash_requests_received(&mut self, value: u32) {
        self.smbbranch_cache_hash_requests_received = Some(value);
    }

    /// Gets the value of SMBBranchCacheHashRequestsReceived
    pub fn get_smbbranch_cache_hash_requests_received(&self) -> Option<&u32> {
        self.smbbranch_cache_hash_requests_received.as_ref()
    }

    /// Sets the value of SMBBranchCacheHashResponsesSent
    pub fn set_smbbranch_cache_hash_responses_sent(&mut self, value: u32) {
        self.smbbranch_cache_hash_responses_sent = Some(value);
    }

    /// Gets the value of SMBBranchCacheHashResponsesSent
    pub fn get_smbbranch_cache_hash_responses_sent(&self) -> Option<&u32> {
        self.smbbranch_cache_hash_responses_sent.as_ref()
    }

    /// Sets the value of SMBBranchCacheHashV2BytesSent
    pub fn set_smbbranch_cache_hash_v2_bytes_sent(&mut self, value: u64) {
        self.smbbranch_cache_hash_v2_bytes_sent = Some(value);
    }

    /// Gets the value of SMBBranchCacheHashV2BytesSent
    pub fn get_smbbranch_cache_hash_v2_bytes_sent(&self) -> Option<&u64> {
        self.smbbranch_cache_hash_v2_bytes_sent.as_ref()
    }

    /// Sets the value of SMBBranchCacheHashV2GenerationRequests
    pub fn set_smbbranch_cache_hash_v2_generation_requests(&mut self, value: u32) {
        self.smbbranch_cache_hash_v2_generation_requests = Some(value);
    }

    /// Gets the value of SMBBranchCacheHashV2GenerationRequests
    pub fn get_smbbranch_cache_hash_v2_generation_requests(&self) -> Option<&u32> {
        self.smbbranch_cache_hash_v2_generation_requests.as_ref()
    }

    /// Sets the value of SMBBranchCacheHashV2HeaderRequests
    pub fn set_smbbranch_cache_hash_v2_header_requests(&mut self, value: u32) {
        self.smbbranch_cache_hash_v2_header_requests = Some(value);
    }

    /// Gets the value of SMBBranchCacheHashV2HeaderRequests
    pub fn get_smbbranch_cache_hash_v2_header_requests(&self) -> Option<&u32> {
        self.smbbranch_cache_hash_v2_header_requests.as_ref()
    }

    /// Sets the value of SMBBranchCacheHashV2RequestsReceived
    pub fn set_smbbranch_cache_hash_v2_requests_received(&mut self, value: u32) {
        self.smbbranch_cache_hash_v2_requests_received = Some(value);
    }

    /// Gets the value of SMBBranchCacheHashV2RequestsReceived
    pub fn get_smbbranch_cache_hash_v2_requests_received(&self) -> Option<&u32> {
        self.smbbranch_cache_hash_v2_requests_received.as_ref()
    }

    /// Sets the value of SMBBranchCacheHashV2RequestsServedFromDedup
    pub fn set_smbbranch_cache_hash_v2_requests_served_from_dedup(&mut self, value: u32) {
        self.smbbranch_cache_hash_v2_requests_served_from_dedup = Some(value);
    }

    /// Gets the value of SMBBranchCacheHashV2RequestsServedFromDedup
    pub fn get_smbbranch_cache_hash_v2_requests_served_from_dedup(&self) -> Option<&u32> {
        self.smbbranch_cache_hash_v2_requests_served_from_dedup.as_ref()
    }

    /// Sets the value of SMBBranchCacheHashV2ResponsesSent
    pub fn set_smbbranch_cache_hash_v2_responses_sent(&mut self, value: u32) {
        self.smbbranch_cache_hash_v2_responses_sent = Some(value);
    }

    /// Gets the value of SMBBranchCacheHashV2ResponsesSent
    pub fn get_smbbranch_cache_hash_v2_responses_sent(&self) -> Option<&u32> {
        self.smbbranch_cache_hash_v2_responses_sent.as_ref()
    }

    /// Sets the value of TotalDurableHandles
    pub fn set_total_durable_handles(&mut self, value: u32) {
        self.total_durable_handles = Some(value);
    }

    /// Gets the value of TotalDurableHandles
    pub fn get_total_durable_handles(&self) -> Option<&u32> {
        self.total_durable_handles.as_ref()
    }

    /// Sets the value of TotalResilientHandles
    pub fn set_total_resilient_handles(&mut self, value: u32) {
        self.total_resilient_handles = Some(value);
    }

    /// Gets the value of TotalResilientHandles
    pub fn get_total_resilient_handles(&self) -> Option<&u32> {
        self.total_resilient_handles.as_ref()
    }

    /// Sets the value of WorkItemShortages
    pub fn set_work_item_shortages(&mut self, value: u32) {
        self.work_item_shortages = Some(value);
    }

    /// Gets the value of WorkItemShortages
    pub fn get_work_item_shortages(&self) -> Option<&u32> {
        self.work_item_shortages.as_ref()
    }
}

