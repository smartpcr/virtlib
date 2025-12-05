// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_ClusBfltPerfProvider_ClusterStorageDiskSchedulerHdd struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_ClusBfltPerfProvider_ClusterStorageDiskSchedulerHdd {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "BusyNormalPriRate")]
    pub busy_normal_pri_rate: Option<u64>,

/// 
    #[serde(rename = "BusyRate")]
    pub busy_rate: Option<u64>,

/// 
    #[serde(rename = "ExecutionNormalPriRate")]
    pub execution_normal_pri_rate: Option<u64>,

/// 
    #[serde(rename = "NormalizedTransfersPersec")]
    pub normalized_transfers_persec: Option<u64>,

/// 
    #[serde(rename = "SlowDisk")]
    pub slow_disk: Option<u64>,

/// 
    #[serde(rename = "TimeBetweenIOCompAvgus")]
    pub time_between_iocomp_avgus: Option<u64>,

/// 
    #[serde(rename = "TimeBetweenIOCompHighus")]
    pub time_between_iocomp_highus: Option<u64>,

/// 
    #[serde(rename = "TimeBetweenIOCompLowus")]
    pub time_between_iocomp_lowus: Option<u64>,
}

impl Win32_PerfRawData_ClusBfltPerfProvider_ClusterStorageDiskSchedulerHdd {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            busy_normal_pri_rate: None,
            busy_rate: None,
            execution_normal_pri_rate: None,
            normalized_transfers_persec: None,
            slow_disk: None,
            time_between_iocomp_avgus: None,
            time_between_iocomp_highus: None,
            time_between_iocomp_lowus: None,
        }
    }


    /// Sets the value of BusyNormalPriRate
    pub fn set_busy_normal_pri_rate(&mut self, value: u64) {
        self.busy_normal_pri_rate = Some(value);
    }

    /// Gets the value of BusyNormalPriRate
    pub fn get_busy_normal_pri_rate(&self) -> Option<&u64> {
        self.busy_normal_pri_rate.as_ref()
    }

    /// Sets the value of BusyRate
    pub fn set_busy_rate(&mut self, value: u64) {
        self.busy_rate = Some(value);
    }

    /// Gets the value of BusyRate
    pub fn get_busy_rate(&self) -> Option<&u64> {
        self.busy_rate.as_ref()
    }

    /// Sets the value of ExecutionNormalPriRate
    pub fn set_execution_normal_pri_rate(&mut self, value: u64) {
        self.execution_normal_pri_rate = Some(value);
    }

    /// Gets the value of ExecutionNormalPriRate
    pub fn get_execution_normal_pri_rate(&self) -> Option<&u64> {
        self.execution_normal_pri_rate.as_ref()
    }

    /// Sets the value of NormalizedTransfersPersec
    pub fn set_normalized_transfers_persec(&mut self, value: u64) {
        self.normalized_transfers_persec = Some(value);
    }

    /// Gets the value of NormalizedTransfersPersec
    pub fn get_normalized_transfers_persec(&self) -> Option<&u64> {
        self.normalized_transfers_persec.as_ref()
    }

    /// Sets the value of SlowDisk
    pub fn set_slow_disk(&mut self, value: u64) {
        self.slow_disk = Some(value);
    }

    /// Gets the value of SlowDisk
    pub fn get_slow_disk(&self) -> Option<&u64> {
        self.slow_disk.as_ref()
    }

    /// Sets the value of TimeBetweenIOCompAvgus
    pub fn set_time_between_iocomp_avgus(&mut self, value: u64) {
        self.time_between_iocomp_avgus = Some(value);
    }

    /// Gets the value of TimeBetweenIOCompAvgus
    pub fn get_time_between_iocomp_avgus(&self) -> Option<&u64> {
        self.time_between_iocomp_avgus.as_ref()
    }

    /// Sets the value of TimeBetweenIOCompHighus
    pub fn set_time_between_iocomp_highus(&mut self, value: u64) {
        self.time_between_iocomp_highus = Some(value);
    }

    /// Gets the value of TimeBetweenIOCompHighus
    pub fn get_time_between_iocomp_highus(&self) -> Option<&u64> {
        self.time_between_iocomp_highus.as_ref()
    }

    /// Sets the value of TimeBetweenIOCompLowus
    pub fn set_time_between_iocomp_lowus(&mut self, value: u64) {
        self.time_between_iocomp_lowus = Some(value);
    }

    /// Gets the value of TimeBetweenIOCompLowus
    pub fn get_time_between_iocomp_lowus(&self) -> Option<&u64> {
        self.time_between_iocomp_lowus.as_ref()
    }
}

