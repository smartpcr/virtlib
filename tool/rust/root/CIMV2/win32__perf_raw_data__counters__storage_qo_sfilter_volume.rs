// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_StorageQoSFilterVolume struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_StorageQoSFilterVolume {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "AllocationQuantum")]
    pub allocation_quantum: Option<u64>,

/// 
    #[serde(rename = "AvgBandwidth")]
    pub avg_bandwidth: Option<u64>,

/// 
    #[serde(rename = "AvgDeviceLatency")]
    pub avg_device_latency: Option<u64>,

/// 
    #[serde(rename = "AvgDeviceQueueLength")]
    pub avg_device_queue_length: Option<u64>,

/// 
    #[serde(rename = "AvgIOCost")]
    pub avg_iocost: Option<u64>,

/// 
    #[serde(rename = "AvgNormalizedIOCost")]
    pub avg_normalized_iocost: Option<u64>,

/// 
    #[serde(rename = "AvgSchedulerQueueLength")]
    pub avg_scheduler_queue_length: Option<u64>,

/// 
    #[serde(rename = "CongestionThreshold")]
    pub congestion_threshold: Option<u64>,

/// 
    #[serde(rename = "DelayedCost")]
    pub delayed_cost: Option<u64>,

/// 
    #[serde(rename = "EstimatedCapacity")]
    pub estimated_capacity: Option<u64>,

/// 
    #[serde(rename = "FlowSwitchCost")]
    pub flow_switch_cost: Option<u64>,

/// 
    #[serde(rename = "IssuedCost")]
    pub issued_cost: Option<u64>,

/// 
    #[serde(rename = "LatencyTarget")]
    pub latency_target: Option<u64>,

/// 
    #[serde(rename = "LowerThreshold")]
    pub lower_threshold: Option<u64>,

/// 
    #[serde(rename = "NormalizedThroughput")]
    pub normalized_throughput: Option<u64>,

/// 
    #[serde(rename = "OverheadCost")]
    pub overhead_cost: Option<u64>,

/// 
    #[serde(rename = "SectorCost")]
    pub sector_cost: Option<u64>,

/// 
    #[serde(rename = "SeekCost")]
    pub seek_cost: Option<u64>,
}

impl Win32_PerfRawData_Counters_StorageQoSFilterVolume {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            allocation_quantum: None,
            avg_bandwidth: None,
            avg_device_latency: None,
            avg_device_queue_length: None,
            avg_iocost: None,
            avg_normalized_iocost: None,
            avg_scheduler_queue_length: None,
            congestion_threshold: None,
            delayed_cost: None,
            estimated_capacity: None,
            flow_switch_cost: None,
            issued_cost: None,
            latency_target: None,
            lower_threshold: None,
            normalized_throughput: None,
            overhead_cost: None,
            sector_cost: None,
            seek_cost: None,
        }
    }


    /// Sets the value of AllocationQuantum
    pub fn set_allocation_quantum(&mut self, value: u64) {
        self.allocation_quantum = Some(value);
    }

    /// Gets the value of AllocationQuantum
    pub fn get_allocation_quantum(&self) -> Option<&u64> {
        self.allocation_quantum.as_ref()
    }

    /// Sets the value of AvgBandwidth
    pub fn set_avg_bandwidth(&mut self, value: u64) {
        self.avg_bandwidth = Some(value);
    }

    /// Gets the value of AvgBandwidth
    pub fn get_avg_bandwidth(&self) -> Option<&u64> {
        self.avg_bandwidth.as_ref()
    }

    /// Sets the value of AvgDeviceLatency
    pub fn set_avg_device_latency(&mut self, value: u64) {
        self.avg_device_latency = Some(value);
    }

    /// Gets the value of AvgDeviceLatency
    pub fn get_avg_device_latency(&self) -> Option<&u64> {
        self.avg_device_latency.as_ref()
    }

    /// Sets the value of AvgDeviceQueueLength
    pub fn set_avg_device_queue_length(&mut self, value: u64) {
        self.avg_device_queue_length = Some(value);
    }

    /// Gets the value of AvgDeviceQueueLength
    pub fn get_avg_device_queue_length(&self) -> Option<&u64> {
        self.avg_device_queue_length.as_ref()
    }

    /// Sets the value of AvgIOCost
    pub fn set_avg_iocost(&mut self, value: u64) {
        self.avg_iocost = Some(value);
    }

    /// Gets the value of AvgIOCost
    pub fn get_avg_iocost(&self) -> Option<&u64> {
        self.avg_iocost.as_ref()
    }

    /// Sets the value of AvgNormalizedIOCost
    pub fn set_avg_normalized_iocost(&mut self, value: u64) {
        self.avg_normalized_iocost = Some(value);
    }

    /// Gets the value of AvgNormalizedIOCost
    pub fn get_avg_normalized_iocost(&self) -> Option<&u64> {
        self.avg_normalized_iocost.as_ref()
    }

    /// Sets the value of AvgSchedulerQueueLength
    pub fn set_avg_scheduler_queue_length(&mut self, value: u64) {
        self.avg_scheduler_queue_length = Some(value);
    }

    /// Gets the value of AvgSchedulerQueueLength
    pub fn get_avg_scheduler_queue_length(&self) -> Option<&u64> {
        self.avg_scheduler_queue_length.as_ref()
    }

    /// Sets the value of CongestionThreshold
    pub fn set_congestion_threshold(&mut self, value: u64) {
        self.congestion_threshold = Some(value);
    }

    /// Gets the value of CongestionThreshold
    pub fn get_congestion_threshold(&self) -> Option<&u64> {
        self.congestion_threshold.as_ref()
    }

    /// Sets the value of DelayedCost
    pub fn set_delayed_cost(&mut self, value: u64) {
        self.delayed_cost = Some(value);
    }

    /// Gets the value of DelayedCost
    pub fn get_delayed_cost(&self) -> Option<&u64> {
        self.delayed_cost.as_ref()
    }

    /// Sets the value of EstimatedCapacity
    pub fn set_estimated_capacity(&mut self, value: u64) {
        self.estimated_capacity = Some(value);
    }

    /// Gets the value of EstimatedCapacity
    pub fn get_estimated_capacity(&self) -> Option<&u64> {
        self.estimated_capacity.as_ref()
    }

    /// Sets the value of FlowSwitchCost
    pub fn set_flow_switch_cost(&mut self, value: u64) {
        self.flow_switch_cost = Some(value);
    }

    /// Gets the value of FlowSwitchCost
    pub fn get_flow_switch_cost(&self) -> Option<&u64> {
        self.flow_switch_cost.as_ref()
    }

    /// Sets the value of IssuedCost
    pub fn set_issued_cost(&mut self, value: u64) {
        self.issued_cost = Some(value);
    }

    /// Gets the value of IssuedCost
    pub fn get_issued_cost(&self) -> Option<&u64> {
        self.issued_cost.as_ref()
    }

    /// Sets the value of LatencyTarget
    pub fn set_latency_target(&mut self, value: u64) {
        self.latency_target = Some(value);
    }

    /// Gets the value of LatencyTarget
    pub fn get_latency_target(&self) -> Option<&u64> {
        self.latency_target.as_ref()
    }

    /// Sets the value of LowerThreshold
    pub fn set_lower_threshold(&mut self, value: u64) {
        self.lower_threshold = Some(value);
    }

    /// Gets the value of LowerThreshold
    pub fn get_lower_threshold(&self) -> Option<&u64> {
        self.lower_threshold.as_ref()
    }

    /// Sets the value of NormalizedThroughput
    pub fn set_normalized_throughput(&mut self, value: u64) {
        self.normalized_throughput = Some(value);
    }

    /// Gets the value of NormalizedThroughput
    pub fn get_normalized_throughput(&self) -> Option<&u64> {
        self.normalized_throughput.as_ref()
    }

    /// Sets the value of OverheadCost
    pub fn set_overhead_cost(&mut self, value: u64) {
        self.overhead_cost = Some(value);
    }

    /// Gets the value of OverheadCost
    pub fn get_overhead_cost(&self) -> Option<&u64> {
        self.overhead_cost.as_ref()
    }

    /// Sets the value of SectorCost
    pub fn set_sector_cost(&mut self, value: u64) {
        self.sector_cost = Some(value);
    }

    /// Gets the value of SectorCost
    pub fn get_sector_cost(&self) -> Option<&u64> {
        self.sector_cost.as_ref()
    }

    /// Sets the value of SeekCost
    pub fn set_seek_cost(&mut self, value: u64) {
        self.seek_cost = Some(value);
    }

    /// Gets the value of SeekCost
    pub fn get_seek_cost(&self) -> Option<&u64> {
        self.seek_cost.as_ref()
    }
}

