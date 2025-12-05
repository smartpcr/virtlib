// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_80211_Statistics struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_80211_Statistics {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "ACKFailureCount")]
    pub ackfailure_count: Option<u64>,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "FailedCount")]
    pub failed_count: Option<u64>,

/// 
    #[serde(rename = "FCSErrorCount")]
    pub fcserror_count: Option<u64>,

/// 
    #[serde(rename = "FrameDuplicateCount")]
    pub frame_duplicate_count: Option<u64>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "MulticastReceivedFrameCount")]
    pub multicast_received_frame_count: Option<u64>,

/// 
    #[serde(rename = "MulticastTransmittedFrameCount")]
    pub multicast_transmitted_frame_count: Option<u64>,

/// 
    #[serde(rename = "MultipleRetryCount")]
    pub multiple_retry_count: Option<u64>,

/// 
    #[serde(rename = "ReceivedFragmentCount")]
    pub received_fragment_count: Option<u64>,

/// 
    #[serde(rename = "RetryCount")]
    pub retry_count: Option<u64>,

/// 
    #[serde(rename = "RTSFailureCount")]
    pub rtsfailure_count: Option<u64>,

/// 
    #[serde(rename = "RTSSuccessCount")]
    pub rtssuccess_count: Option<u64>,

/// 
    #[serde(rename = "StatisticsLength")]
    pub statistics_length: Option<u32>,

/// 
    #[serde(rename = "TransmittedFragmentCount")]
    pub transmitted_fragment_count: Option<u64>,
}

impl MSNdis_80211_Statistics {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            ackfailure_count: None,
            active: None,
            failed_count: None,
            fcserror_count: None,
            frame_duplicate_count: None,
            instance_name: None,
            multicast_received_frame_count: None,
            multicast_transmitted_frame_count: None,
            multiple_retry_count: None,
            received_fragment_count: None,
            retry_count: None,
            rtsfailure_count: None,
            rtssuccess_count: None,
            statistics_length: None,
            transmitted_fragment_count: None,
        }
    }


    /// Sets the value of ACKFailureCount
    pub fn set_ackfailure_count(&mut self, value: u64) {
        self.ackfailure_count = Some(value);
    }

    /// Gets the value of ACKFailureCount
    pub fn get_ackfailure_count(&self) -> Option<&u64> {
        self.ackfailure_count.as_ref()
    }

    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of FailedCount
    pub fn set_failed_count(&mut self, value: u64) {
        self.failed_count = Some(value);
    }

    /// Gets the value of FailedCount
    pub fn get_failed_count(&self) -> Option<&u64> {
        self.failed_count.as_ref()
    }

    /// Sets the value of FCSErrorCount
    pub fn set_fcserror_count(&mut self, value: u64) {
        self.fcserror_count = Some(value);
    }

    /// Gets the value of FCSErrorCount
    pub fn get_fcserror_count(&self) -> Option<&u64> {
        self.fcserror_count.as_ref()
    }

    /// Sets the value of FrameDuplicateCount
    pub fn set_frame_duplicate_count(&mut self, value: u64) {
        self.frame_duplicate_count = Some(value);
    }

    /// Gets the value of FrameDuplicateCount
    pub fn get_frame_duplicate_count(&self) -> Option<&u64> {
        self.frame_duplicate_count.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of MulticastReceivedFrameCount
    pub fn set_multicast_received_frame_count(&mut self, value: u64) {
        self.multicast_received_frame_count = Some(value);
    }

    /// Gets the value of MulticastReceivedFrameCount
    pub fn get_multicast_received_frame_count(&self) -> Option<&u64> {
        self.multicast_received_frame_count.as_ref()
    }

    /// Sets the value of MulticastTransmittedFrameCount
    pub fn set_multicast_transmitted_frame_count(&mut self, value: u64) {
        self.multicast_transmitted_frame_count = Some(value);
    }

    /// Gets the value of MulticastTransmittedFrameCount
    pub fn get_multicast_transmitted_frame_count(&self) -> Option<&u64> {
        self.multicast_transmitted_frame_count.as_ref()
    }

    /// Sets the value of MultipleRetryCount
    pub fn set_multiple_retry_count(&mut self, value: u64) {
        self.multiple_retry_count = Some(value);
    }

    /// Gets the value of MultipleRetryCount
    pub fn get_multiple_retry_count(&self) -> Option<&u64> {
        self.multiple_retry_count.as_ref()
    }

    /// Sets the value of ReceivedFragmentCount
    pub fn set_received_fragment_count(&mut self, value: u64) {
        self.received_fragment_count = Some(value);
    }

    /// Gets the value of ReceivedFragmentCount
    pub fn get_received_fragment_count(&self) -> Option<&u64> {
        self.received_fragment_count.as_ref()
    }

    /// Sets the value of RetryCount
    pub fn set_retry_count(&mut self, value: u64) {
        self.retry_count = Some(value);
    }

    /// Gets the value of RetryCount
    pub fn get_retry_count(&self) -> Option<&u64> {
        self.retry_count.as_ref()
    }

    /// Sets the value of RTSFailureCount
    pub fn set_rtsfailure_count(&mut self, value: u64) {
        self.rtsfailure_count = Some(value);
    }

    /// Gets the value of RTSFailureCount
    pub fn get_rtsfailure_count(&self) -> Option<&u64> {
        self.rtsfailure_count.as_ref()
    }

    /// Sets the value of RTSSuccessCount
    pub fn set_rtssuccess_count(&mut self, value: u64) {
        self.rtssuccess_count = Some(value);
    }

    /// Gets the value of RTSSuccessCount
    pub fn get_rtssuccess_count(&self) -> Option<&u64> {
        self.rtssuccess_count.as_ref()
    }

    /// Sets the value of StatisticsLength
    pub fn set_statistics_length(&mut self, value: u32) {
        self.statistics_length = Some(value);
    }

    /// Gets the value of StatisticsLength
    pub fn get_statistics_length(&self) -> Option<&u32> {
        self.statistics_length.as_ref()
    }

    /// Sets the value of TransmittedFragmentCount
    pub fn set_transmitted_fragment_count(&mut self, value: u64) {
        self.transmitted_fragment_count = Some(value);
    }

    /// Gets the value of TransmittedFragmentCount
    pub fn get_transmitted_fragment_count(&self) -> Option<&u64> {
        self.transmitted_fragment_count.as_ref()
    }
}

