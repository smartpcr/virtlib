// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_StorageQoSFilterFlow struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_StorageQoSFilterFlow {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "AvgBandwidth")]
    pub avg_bandwidth: Option<u64>,

/// 
    #[serde(rename = "AvgDeviceQueueLength")]
    pub avg_device_queue_length: Option<u64>,

/// 
    #[serde(rename = "AvgIOQuotaReplenishmentOperationsPersec")]
    pub avg_ioquota_replenishment_operations_persec: Option<u64>,

/// 
    #[serde(rename = "AvgNormalizedIOPS")]
    pub avg_normalized_iops: Option<u64>,

/// 
    #[serde(rename = "AvgSchedulerQueueLength")]
    pub avg_scheduler_queue_length: Option<u64>,

/// 
    #[serde(rename = "MaximumBandwidth")]
    pub maximum_bandwidth: Option<u64>,

/// 
    #[serde(rename = "NormalizedMaximumIORate")]
    pub normalized_maximum_iorate: Option<u64>,

/// 
    #[serde(rename = "NormalizedMinimumIORate")]
    pub normalized_minimum_iorate: Option<u64>,

/// 
    #[serde(rename = "TotalBandwidthquotaIncrementPersec")]
    pub total_bandwidthquota_increment_persec: Option<u64>,

/// 
    #[serde(rename = "TotalNormalizedIOQuotaIncrement")]
    pub total_normalized_ioquota_increment: Option<u64>,
}

impl Win32_PerfFormattedData_Counters_StorageQoSFilterFlow {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            avg_bandwidth: None,
            avg_device_queue_length: None,
            avg_ioquota_replenishment_operations_persec: None,
            avg_normalized_iops: None,
            avg_scheduler_queue_length: None,
            maximum_bandwidth: None,
            normalized_maximum_iorate: None,
            normalized_minimum_iorate: None,
            total_bandwidthquota_increment_persec: None,
            total_normalized_ioquota_increment: None,
        }
    }


    /// Sets the value of AvgBandwidth
    pub fn set_avg_bandwidth(&mut self, value: u64) {
        self.avg_bandwidth = Some(value);
    }

    /// Gets the value of AvgBandwidth
    pub fn get_avg_bandwidth(&self) -> Option<&u64> {
        self.avg_bandwidth.as_ref()
    }

    /// Sets the value of AvgDeviceQueueLength
    pub fn set_avg_device_queue_length(&mut self, value: u64) {
        self.avg_device_queue_length = Some(value);
    }

    /// Gets the value of AvgDeviceQueueLength
    pub fn get_avg_device_queue_length(&self) -> Option<&u64> {
        self.avg_device_queue_length.as_ref()
    }

    /// Sets the value of AvgIOQuotaReplenishmentOperationsPersec
    pub fn set_avg_ioquota_replenishment_operations_persec(&mut self, value: u64) {
        self.avg_ioquota_replenishment_operations_persec = Some(value);
    }

    /// Gets the value of AvgIOQuotaReplenishmentOperationsPersec
    pub fn get_avg_ioquota_replenishment_operations_persec(&self) -> Option<&u64> {
        self.avg_ioquota_replenishment_operations_persec.as_ref()
    }

    /// Sets the value of AvgNormalizedIOPS
    pub fn set_avg_normalized_iops(&mut self, value: u64) {
        self.avg_normalized_iops = Some(value);
    }

    /// Gets the value of AvgNormalizedIOPS
    pub fn get_avg_normalized_iops(&self) -> Option<&u64> {
        self.avg_normalized_iops.as_ref()
    }

    /// Sets the value of AvgSchedulerQueueLength
    pub fn set_avg_scheduler_queue_length(&mut self, value: u64) {
        self.avg_scheduler_queue_length = Some(value);
    }

    /// Gets the value of AvgSchedulerQueueLength
    pub fn get_avg_scheduler_queue_length(&self) -> Option<&u64> {
        self.avg_scheduler_queue_length.as_ref()
    }

    /// Sets the value of MaximumBandwidth
    pub fn set_maximum_bandwidth(&mut self, value: u64) {
        self.maximum_bandwidth = Some(value);
    }

    /// Gets the value of MaximumBandwidth
    pub fn get_maximum_bandwidth(&self) -> Option<&u64> {
        self.maximum_bandwidth.as_ref()
    }

    /// Sets the value of NormalizedMaximumIORate
    pub fn set_normalized_maximum_iorate(&mut self, value: u64) {
        self.normalized_maximum_iorate = Some(value);
    }

    /// Gets the value of NormalizedMaximumIORate
    pub fn get_normalized_maximum_iorate(&self) -> Option<&u64> {
        self.normalized_maximum_iorate.as_ref()
    }

    /// Sets the value of NormalizedMinimumIORate
    pub fn set_normalized_minimum_iorate(&mut self, value: u64) {
        self.normalized_minimum_iorate = Some(value);
    }

    /// Gets the value of NormalizedMinimumIORate
    pub fn get_normalized_minimum_iorate(&self) -> Option<&u64> {
        self.normalized_minimum_iorate.as_ref()
    }

    /// Sets the value of TotalBandwidthquotaIncrementPersec
    pub fn set_total_bandwidthquota_increment_persec(&mut self, value: u64) {
        self.total_bandwidthquota_increment_persec = Some(value);
    }

    /// Gets the value of TotalBandwidthquotaIncrementPersec
    pub fn get_total_bandwidthquota_increment_persec(&self) -> Option<&u64> {
        self.total_bandwidthquota_increment_persec.as_ref()
    }

    /// Sets the value of TotalNormalizedIOQuotaIncrement
    pub fn set_total_normalized_ioquota_increment(&mut self, value: u64) {
        self.total_normalized_ioquota_increment = Some(value);
    }

    /// Gets the value of TotalNormalizedIOQuotaIncrement
    pub fn get_total_normalized_ioquota_increment(&self) -> Option<&u64> {
        self.total_normalized_ioquota_increment.as_ref()
    }
}

