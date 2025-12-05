// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_ReplicationStatistics struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_ReplicationStatistics {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "ApplicationConsistentSnapshotFailureCount")]
    pub application_consistent_snapshot_failure_count: Option<u32>,

/// 
    #[serde(rename = "EndStatisticTime")]
    pub end_statistic_time: Option<String>,

/// 
    #[serde(rename = "LastTestFailoverTime")]
    pub last_test_failover_time: Option<String>,

/// 
    #[serde(rename = "MaxReplicationLatency")]
    pub max_replication_latency: Option<u32>,

/// 
    #[serde(rename = "MaxReplicationSize")]
    pub max_replication_size: Option<u64>,

/// 
    #[serde(rename = "NetworkFailureCount")]
    pub network_failure_count: Option<u32>,

/// 
    #[serde(rename = "PendingReplicationSize")]
    pub pending_replication_size: Option<u64>,

/// 
    #[serde(rename = "ReplicationFailureCount")]
    pub replication_failure_count: Option<u32>,

/// 
    #[serde(rename = "ReplicationHealth")]
    pub replication_health: Option<u32>,

/// 
    #[serde(rename = "ReplicationLatency")]
    pub replication_latency: Option<u32>,

/// 
    #[serde(rename = "ReplicationMissCount")]
    pub replication_miss_count: Option<u32>,

/// 
    #[serde(rename = "ReplicationSize")]
    pub replication_size: Option<u64>,

/// 
    #[serde(rename = "ReplicationSuccessCount")]
    pub replication_success_count: Option<u32>,

/// 
    #[serde(rename = "StartStatisticTime")]
    pub start_statistic_time: Option<String>,
}

impl Msvm_ReplicationStatistics {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            application_consistent_snapshot_failure_count: None,
            end_statistic_time: None,
            last_test_failover_time: None,
            max_replication_latency: None,
            max_replication_size: None,
            network_failure_count: None,
            pending_replication_size: None,
            replication_failure_count: None,
            replication_health: None,
            replication_latency: None,
            replication_miss_count: None,
            replication_size: None,
            replication_success_count: None,
            start_statistic_time: None,
        }
    }


    /// Sets the value of ApplicationConsistentSnapshotFailureCount
    pub fn set_application_consistent_snapshot_failure_count(&mut self, value: u32) {
        self.application_consistent_snapshot_failure_count = Some(value);
    }

    /// Gets the value of ApplicationConsistentSnapshotFailureCount
    pub fn get_application_consistent_snapshot_failure_count(&self) -> Option<&u32> {
        self.application_consistent_snapshot_failure_count.as_ref()
    }

    /// Sets the value of EndStatisticTime
    pub fn set_end_statistic_time(&mut self, value: String) {
        self.end_statistic_time = Some(value);
    }

    /// Gets the value of EndStatisticTime
    pub fn get_end_statistic_time(&self) -> Option<&String> {
        self.end_statistic_time.as_ref()
    }

    /// Sets the value of LastTestFailoverTime
    pub fn set_last_test_failover_time(&mut self, value: String) {
        self.last_test_failover_time = Some(value);
    }

    /// Gets the value of LastTestFailoverTime
    pub fn get_last_test_failover_time(&self) -> Option<&String> {
        self.last_test_failover_time.as_ref()
    }

    /// Sets the value of MaxReplicationLatency
    pub fn set_max_replication_latency(&mut self, value: u32) {
        self.max_replication_latency = Some(value);
    }

    /// Gets the value of MaxReplicationLatency
    pub fn get_max_replication_latency(&self) -> Option<&u32> {
        self.max_replication_latency.as_ref()
    }

    /// Sets the value of MaxReplicationSize
    pub fn set_max_replication_size(&mut self, value: u64) {
        self.max_replication_size = Some(value);
    }

    /// Gets the value of MaxReplicationSize
    pub fn get_max_replication_size(&self) -> Option<&u64> {
        self.max_replication_size.as_ref()
    }

    /// Sets the value of NetworkFailureCount
    pub fn set_network_failure_count(&mut self, value: u32) {
        self.network_failure_count = Some(value);
    }

    /// Gets the value of NetworkFailureCount
    pub fn get_network_failure_count(&self) -> Option<&u32> {
        self.network_failure_count.as_ref()
    }

    /// Sets the value of PendingReplicationSize
    pub fn set_pending_replication_size(&mut self, value: u64) {
        self.pending_replication_size = Some(value);
    }

    /// Gets the value of PendingReplicationSize
    pub fn get_pending_replication_size(&self) -> Option<&u64> {
        self.pending_replication_size.as_ref()
    }

    /// Sets the value of ReplicationFailureCount
    pub fn set_replication_failure_count(&mut self, value: u32) {
        self.replication_failure_count = Some(value);
    }

    /// Gets the value of ReplicationFailureCount
    pub fn get_replication_failure_count(&self) -> Option<&u32> {
        self.replication_failure_count.as_ref()
    }

    /// Sets the value of ReplicationHealth
    pub fn set_replication_health(&mut self, value: u32) {
        self.replication_health = Some(value);
    }

    /// Gets the value of ReplicationHealth
    pub fn get_replication_health(&self) -> Option<&u32> {
        self.replication_health.as_ref()
    }

    /// Sets the value of ReplicationLatency
    pub fn set_replication_latency(&mut self, value: u32) {
        self.replication_latency = Some(value);
    }

    /// Gets the value of ReplicationLatency
    pub fn get_replication_latency(&self) -> Option<&u32> {
        self.replication_latency.as_ref()
    }

    /// Sets the value of ReplicationMissCount
    pub fn set_replication_miss_count(&mut self, value: u32) {
        self.replication_miss_count = Some(value);
    }

    /// Gets the value of ReplicationMissCount
    pub fn get_replication_miss_count(&self) -> Option<&u32> {
        self.replication_miss_count.as_ref()
    }

    /// Sets the value of ReplicationSize
    pub fn set_replication_size(&mut self, value: u64) {
        self.replication_size = Some(value);
    }

    /// Gets the value of ReplicationSize
    pub fn get_replication_size(&self) -> Option<&u64> {
        self.replication_size.as_ref()
    }

    /// Sets the value of ReplicationSuccessCount
    pub fn set_replication_success_count(&mut self, value: u32) {
        self.replication_success_count = Some(value);
    }

    /// Gets the value of ReplicationSuccessCount
    pub fn get_replication_success_count(&self) -> Option<&u32> {
        self.replication_success_count.as_ref()
    }

    /// Sets the value of StartStatisticTime
    pub fn set_start_statistic_time(&mut self, value: String) {
        self.start_statistic_time = Some(value);
    }

    /// Gets the value of StartStatisticTime
    pub fn get_start_statistic_time(&self) -> Option<&String> {
        self.start_statistic_time.as_ref()
    }
}

