// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageQoSVolume struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageQoSVolume {

/// 
    #[serde(rename = "Bandwidth")]
    pub bandwidth: Option<u64>,

/// 
    #[serde(rename = "BandwidthLimit")]
    pub bandwidth_limit: Option<u64>,

/// 
    #[serde(rename = "Interval")]
    pub interval: Option<u64>,

/// 
    #[serde(rename = "IOPS")]
    pub iops: Option<u64>,

/// 
    #[serde(rename = "Latency")]
    pub latency: Option<u64>,

/// 
    #[serde(rename = "Limit")]
    pub limit: Option<u64>,

/// 
    #[serde(rename = "Mountpoint")]
    pub mountpoint: Option<String>,

/// 
    #[serde(rename = "Reservation")]
    pub reservation: Option<u64>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<u16>,

/// 
    #[serde(rename = "TimeStamp")]
    pub time_stamp: Option<u64>,

/// 
    #[serde(rename = "VolumeId")]
    pub volume_id: Option<String>,
}

impl MSFT_StorageQoSVolume {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            bandwidth: None,
            bandwidth_limit: None,
            interval: None,
            iops: None,
            latency: None,
            limit: None,
            mountpoint: None,
            reservation: None,
            status: None,
            time_stamp: None,
            volume_id: None,
        }
    }


    /// Sets the value of Bandwidth
    pub fn set_bandwidth(&mut self, value: u64) {
        self.bandwidth = Some(value);
    }

    /// Gets the value of Bandwidth
    pub fn get_bandwidth(&self) -> Option<&u64> {
        self.bandwidth.as_ref()
    }

    /// Sets the value of BandwidthLimit
    pub fn set_bandwidth_limit(&mut self, value: u64) {
        self.bandwidth_limit = Some(value);
    }

    /// Gets the value of BandwidthLimit
    pub fn get_bandwidth_limit(&self) -> Option<&u64> {
        self.bandwidth_limit.as_ref()
    }

    /// Sets the value of Interval
    pub fn set_interval(&mut self, value: u64) {
        self.interval = Some(value);
    }

    /// Gets the value of Interval
    pub fn get_interval(&self) -> Option<&u64> {
        self.interval.as_ref()
    }

    /// Sets the value of IOPS
    pub fn set_iops(&mut self, value: u64) {
        self.iops = Some(value);
    }

    /// Gets the value of IOPS
    pub fn get_iops(&self) -> Option<&u64> {
        self.iops.as_ref()
    }

    /// Sets the value of Latency
    pub fn set_latency(&mut self, value: u64) {
        self.latency = Some(value);
    }

    /// Gets the value of Latency
    pub fn get_latency(&self) -> Option<&u64> {
        self.latency.as_ref()
    }

    /// Sets the value of Limit
    pub fn set_limit(&mut self, value: u64) {
        self.limit = Some(value);
    }

    /// Gets the value of Limit
    pub fn get_limit(&self) -> Option<&u64> {
        self.limit.as_ref()
    }

    /// Sets the value of Mountpoint
    pub fn set_mountpoint(&mut self, value: String) {
        self.mountpoint = Some(value);
    }

    /// Gets the value of Mountpoint
    pub fn get_mountpoint(&self) -> Option<&String> {
        self.mountpoint.as_ref()
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

