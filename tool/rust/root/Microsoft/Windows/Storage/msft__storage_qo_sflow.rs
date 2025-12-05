// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageQoSFlow struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageQoSFlow {

/// 
    #[serde(rename = "BandwidthLimit")]
    pub bandwidth_limit: Option<u64>,

/// 
    #[serde(rename = "FilePath")]
    pub file_path: Option<String>,

/// 
    #[serde(rename = "FlowId")]
    pub flow_id: Option<String>,

/// 
    #[serde(rename = "InitiatorBandwidth")]
    pub initiator_bandwidth: Option<u64>,

/// 
    #[serde(rename = "InitiatorId")]
    pub initiator_id: Option<String>,

/// 
    #[serde(rename = "InitiatorIOPS")]
    pub initiator_iops: Option<u64>,

/// 
    #[serde(rename = "InitiatorLatency")]
    pub initiator_latency: Option<u64>,

/// 
    #[serde(rename = "InitiatorName")]
    pub initiator_name: Option<String>,

/// 
    #[serde(rename = "InitiatorNodeName")]
    pub initiator_node_name: Option<String>,

/// 
    #[serde(rename = "Interval")]
    pub interval: Option<u64>,

/// 
    #[serde(rename = "Limit")]
    pub limit: Option<u64>,

/// 
    #[serde(rename = "PolicyId")]
    pub policy_id: Option<String>,

/// 
    #[serde(rename = "Reservation")]
    pub reservation: Option<u64>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<u16>,

/// 
    #[serde(rename = "StorageNodeBandwidth")]
    pub storage_node_bandwidth: Option<u64>,

/// 
    #[serde(rename = "StorageNodeIOPS")]
    pub storage_node_iops: Option<u64>,

/// 
    #[serde(rename = "StorageNodeLatency")]
    pub storage_node_latency: Option<u64>,

/// 
    #[serde(rename = "StorageNodeName")]
    pub storage_node_name: Option<String>,

/// 
    #[serde(rename = "TimeStamp")]
    pub time_stamp: Option<u64>,

/// 
    #[serde(rename = "VolumeId")]
    pub volume_id: Option<String>,
}

impl MSFT_StorageQoSFlow {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            bandwidth_limit: None,
            file_path: None,
            flow_id: None,
            initiator_bandwidth: None,
            initiator_id: None,
            initiator_iops: None,
            initiator_latency: None,
            initiator_name: None,
            initiator_node_name: None,
            interval: None,
            limit: None,
            policy_id: None,
            reservation: None,
            status: None,
            storage_node_bandwidth: None,
            storage_node_iops: None,
            storage_node_latency: None,
            storage_node_name: None,
            time_stamp: None,
            volume_id: None,
        }
    }


    /// Sets the value of BandwidthLimit
    pub fn set_bandwidth_limit(&mut self, value: u64) {
        self.bandwidth_limit = Some(value);
    }

    /// Gets the value of BandwidthLimit
    pub fn get_bandwidth_limit(&self) -> Option<&u64> {
        self.bandwidth_limit.as_ref()
    }

    /// Sets the value of FilePath
    pub fn set_file_path(&mut self, value: String) {
        self.file_path = Some(value);
    }

    /// Gets the value of FilePath
    pub fn get_file_path(&self) -> Option<&String> {
        self.file_path.as_ref()
    }

    /// Sets the value of FlowId
    pub fn set_flow_id(&mut self, value: String) {
        self.flow_id = Some(value);
    }

    /// Gets the value of FlowId
    pub fn get_flow_id(&self) -> Option<&String> {
        self.flow_id.as_ref()
    }

    /// Sets the value of InitiatorBandwidth
    pub fn set_initiator_bandwidth(&mut self, value: u64) {
        self.initiator_bandwidth = Some(value);
    }

    /// Gets the value of InitiatorBandwidth
    pub fn get_initiator_bandwidth(&self) -> Option<&u64> {
        self.initiator_bandwidth.as_ref()
    }

    /// Sets the value of InitiatorId
    pub fn set_initiator_id(&mut self, value: String) {
        self.initiator_id = Some(value);
    }

    /// Gets the value of InitiatorId
    pub fn get_initiator_id(&self) -> Option<&String> {
        self.initiator_id.as_ref()
    }

    /// Sets the value of InitiatorIOPS
    pub fn set_initiator_iops(&mut self, value: u64) {
        self.initiator_iops = Some(value);
    }

    /// Gets the value of InitiatorIOPS
    pub fn get_initiator_iops(&self) -> Option<&u64> {
        self.initiator_iops.as_ref()
    }

    /// Sets the value of InitiatorLatency
    pub fn set_initiator_latency(&mut self, value: u64) {
        self.initiator_latency = Some(value);
    }

    /// Gets the value of InitiatorLatency
    pub fn get_initiator_latency(&self) -> Option<&u64> {
        self.initiator_latency.as_ref()
    }

    /// Sets the value of InitiatorName
    pub fn set_initiator_name(&mut self, value: String) {
        self.initiator_name = Some(value);
    }

    /// Gets the value of InitiatorName
    pub fn get_initiator_name(&self) -> Option<&String> {
        self.initiator_name.as_ref()
    }

    /// Sets the value of InitiatorNodeName
    pub fn set_initiator_node_name(&mut self, value: String) {
        self.initiator_node_name = Some(value);
    }

    /// Gets the value of InitiatorNodeName
    pub fn get_initiator_node_name(&self) -> Option<&String> {
        self.initiator_node_name.as_ref()
    }

    /// Sets the value of Interval
    pub fn set_interval(&mut self, value: u64) {
        self.interval = Some(value);
    }

    /// Gets the value of Interval
    pub fn get_interval(&self) -> Option<&u64> {
        self.interval.as_ref()
    }

    /// Sets the value of Limit
    pub fn set_limit(&mut self, value: u64) {
        self.limit = Some(value);
    }

    /// Gets the value of Limit
    pub fn get_limit(&self) -> Option<&u64> {
        self.limit.as_ref()
    }

    /// Sets the value of PolicyId
    pub fn set_policy_id(&mut self, value: String) {
        self.policy_id = Some(value);
    }

    /// Gets the value of PolicyId
    pub fn get_policy_id(&self) -> Option<&String> {
        self.policy_id.as_ref()
    }

    /// Sets the value of Reservation
    pub fn set_reservation(&mut self, value: u64) {
        self.reservation = Some(value);
    }

    /// Gets the value of Reservation
    pub fn get_reservation(&self) -> Option<&u64> {
        self.reservation.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u16) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u16> {
        self.status.as_ref()
    }

    /// Sets the value of StorageNodeBandwidth
    pub fn set_storage_node_bandwidth(&mut self, value: u64) {
        self.storage_node_bandwidth = Some(value);
    }

    /// Gets the value of StorageNodeBandwidth
    pub fn get_storage_node_bandwidth(&self) -> Option<&u64> {
        self.storage_node_bandwidth.as_ref()
    }

    /// Sets the value of StorageNodeIOPS
    pub fn set_storage_node_iops(&mut self, value: u64) {
        self.storage_node_iops = Some(value);
    }

    /// Gets the value of StorageNodeIOPS
    pub fn get_storage_node_iops(&self) -> Option<&u64> {
        self.storage_node_iops.as_ref()
    }

    /// Sets the value of StorageNodeLatency
    pub fn set_storage_node_latency(&mut self, value: u64) {
        self.storage_node_latency = Some(value);
    }

    /// Gets the value of StorageNodeLatency
    pub fn get_storage_node_latency(&self) -> Option<&u64> {
        self.storage_node_latency.as_ref()
    }

    /// Sets the value of StorageNodeName
    pub fn set_storage_node_name(&mut self, value: String) {
        self.storage_node_name = Some(value);
    }

    /// Gets the value of StorageNodeName
    pub fn get_storage_node_name(&self) -> Option<&String> {
        self.storage_node_name.as_ref()
    }

    /// Sets the value of TimeStamp
    pub fn set_time_stamp(&mut self, value: u64) {
        self.time_stamp = Some(value);
    }

    /// Gets the value of TimeStamp
    pub fn get_time_stamp(&self) -> Option<&u64> {
        self.time_stamp.as_ref()
    }

    /// Sets the value of VolumeId
    pub fn set_volume_id(&mut self, value: String) {
        self.volume_id = Some(value);
    }

    /// Gets the value of VolumeId
    pub fn get_volume_id(&self) -> Option<&String> {
        self.volume_id.as_ref()
    }
}

