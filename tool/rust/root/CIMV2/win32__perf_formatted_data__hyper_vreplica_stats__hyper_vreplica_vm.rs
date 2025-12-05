// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_HyperVReplicaStats_HyperVReplicaVM struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_HyperVReplicaStats_HyperVReplicaVM {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "AverageReplicationLatency")]
    pub average_replication_latency: Option<u64>,

/// 
    #[serde(rename = "AverageReplicationSize")]
    pub average_replication_size: Option<u64>,

/// 
    #[serde(rename = "CompressionEfficiency")]
    pub compression_efficiency: Option<u64>,

/// 
    #[serde(rename = "LastReplicationSize")]
    pub last_replication_size: Option<u64>,

/// 
    #[serde(rename = "NetworkBytesRecv")]
    pub network_bytes_recv: Option<u64>,

/// 
    #[serde(rename = "NetworkBytesSent")]
    pub network_bytes_sent: Option<u64>,

/// 
    #[serde(rename = "ReplicationCount")]
    pub replication_count: Option<u32>,

/// 
    #[serde(rename = "ReplicationLatency")]
    pub replication_latency: Option<u64>,

/// 
    #[serde(rename = "ResynchronizedBytes")]
    pub resynchronized_bytes: Option<u64>,
}

impl Win32_PerfFormattedData_HyperVReplicaStats_HyperVReplicaVM {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            average_replication_latency: None,
            average_replication_size: None,
            compression_efficiency: None,
            last_replication_size: None,
            network_bytes_recv: None,
            network_bytes_sent: None,
            replication_count: None,
            replication_latency: None,
            resynchronized_bytes: None,
        }
    }


    /// Sets the value of AverageReplicationLatency
    pub fn set_average_replication_latency(&mut self, value: u64) {
        self.average_replication_latency = Some(value);
    }

    /// Gets the value of AverageReplicationLatency
    pub fn get_average_replication_latency(&self) -> Option<&u64> {
        self.average_replication_latency.as_ref()
    }

    /// Sets the value of AverageReplicationSize
    pub fn set_average_replication_size(&mut self, value: u64) {
        self.average_replication_size = Some(value);
    }

    /// Gets the value of AverageReplicationSize
    pub fn get_average_replication_size(&self) -> Option<&u64> {
        self.average_replication_size.as_ref()
    }

    /// Sets the value of CompressionEfficiency
    pub fn set_compression_efficiency(&mut self, value: u64) {
        self.compression_efficiency = Some(value);
    }

    /// Gets the value of CompressionEfficiency
    pub fn get_compression_efficiency(&self) -> Option<&u64> {
        self.compression_efficiency.as_ref()
    }

    /// Sets the value of LastReplicationSize
    pub fn set_last_replication_size(&mut self, value: u64) {
        self.last_replication_size = Some(value);
    }

    /// Gets the value of LastReplicationSize
    pub fn get_last_replication_size(&self) -> Option<&u64> {
        self.last_replication_size.as_ref()
    }

    /// Sets the value of NetworkBytesRecv
    pub fn set_network_bytes_recv(&mut self, value: u64) {
        self.network_bytes_recv = Some(value);
    }

    /// Gets the value of NetworkBytesRecv
    pub fn get_network_bytes_recv(&self) -> Option<&u64> {
        self.network_bytes_recv.as_ref()
    }

    /// Sets the value of NetworkBytesSent
    pub fn set_network_bytes_sent(&mut self, value: u64) {
        self.network_bytes_sent = Some(value);
    }

    /// Gets the value of NetworkBytesSent
    pub fn get_network_bytes_sent(&self) -> Option<&u64> {
        self.network_bytes_sent.as_ref()
    }

    /// Sets the value of ReplicationCount
    pub fn set_replication_count(&mut self, value: u32) {
        self.replication_count = Some(value);
    }

    /// Gets the value of ReplicationCount
    pub fn get_replication_count(&self) -> Option<&u32> {
        self.replication_count.as_ref()
    }

    /// Sets the value of ReplicationLatency
    pub fn set_replication_latency(&mut self, value: u64) {
        self.replication_latency = Some(value);
    }

    /// Gets the value of ReplicationLatency
    pub fn get_replication_latency(&self) -> Option<&u64> {
        self.replication_latency.as_ref()
    }

    /// Sets the value of ResynchronizedBytes
    pub fn set_resynchronized_bytes(&mut self, value: u64) {
        self.resynchronized_bytes = Some(value);
    }

    /// Gets the value of ResynchronizedBytes
    pub fn get_resynchronized_bytes(&self) -> Option<&u64> {
        self.resynchronized_bytes.as_ref()
    }
}

