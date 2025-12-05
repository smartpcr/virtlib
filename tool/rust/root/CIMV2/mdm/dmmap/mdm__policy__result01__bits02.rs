// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_BITS02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_BITS02 {

/// 
    #[serde(rename = "BandwidthThrottlingEndTime")]
    pub bandwidth_throttling_end_time: Option<i32>,

/// 
    #[serde(rename = "BandwidthThrottlingStartTime")]
    pub bandwidth_throttling_start_time: Option<i32>,

/// 
    #[serde(rename = "BandwidthThrottlingTransferRate")]
    pub bandwidth_throttling_transfer_rate: Option<i32>,

/// 
    #[serde(rename = "CostedNetworkBehaviorBackgroundPriority")]
    pub costed_network_behavior_background_priority: Option<i32>,

/// 
    #[serde(rename = "CostedNetworkBehaviorForegroundPriority")]
    pub costed_network_behavior_foreground_priority: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "JobInactivityTimeout")]
    pub job_inactivity_timeout: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_Result01_BITS02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            bandwidth_throttling_end_time: None,
            bandwidth_throttling_start_time: None,
            bandwidth_throttling_transfer_rate: None,
            costed_network_behavior_background_priority: None,
            costed_network_behavior_foreground_priority: None,
            instance_id: None,
            job_inactivity_timeout: None,
            parent_id: None,
        }
    }


    /// Sets the value of BandwidthThrottlingEndTime
    pub fn set_bandwidth_throttling_end_time(&mut self, value: i32) {
        self.bandwidth_throttling_end_time = Some(value);
    }

    /// Gets the value of BandwidthThrottlingEndTime
    pub fn get_bandwidth_throttling_end_time(&self) -> Option<&i32> {
        self.bandwidth_throttling_end_time.as_ref()
    }

    /// Sets the value of BandwidthThrottlingStartTime
    pub fn set_bandwidth_throttling_start_time(&mut self, value: i32) {
        self.bandwidth_throttling_start_time = Some(value);
    }

    /// Gets the value of BandwidthThrottlingStartTime
    pub fn get_bandwidth_throttling_start_time(&self) -> Option<&i32> {
        self.bandwidth_throttling_start_time.as_ref()
    }

    /// Sets the value of BandwidthThrottlingTransferRate
    pub fn set_bandwidth_throttling_transfer_rate(&mut self, value: i32) {
        self.bandwidth_throttling_transfer_rate = Some(value);
    }

    /// Gets the value of BandwidthThrottlingTransferRate
    pub fn get_bandwidth_throttling_transfer_rate(&self) -> Option<&i32> {
        self.bandwidth_throttling_transfer_rate.as_ref()
    }

    /// Sets the value of CostedNetworkBehaviorBackgroundPriority
    pub fn set_costed_network_behavior_background_priority(&mut self, value: i32) {
        self.costed_network_behavior_background_priority = Some(value);
    }

    /// Gets the value of CostedNetworkBehaviorBackgroundPriority
    pub fn get_costed_network_behavior_background_priority(&self) -> Option<&i32> {
        self.costed_network_behavior_background_priority.as_ref()
    }

    /// Sets the value of CostedNetworkBehaviorForegroundPriority
    pub fn set_costed_network_behavior_foreground_priority(&mut self, value: i32) {
        self.costed_network_behavior_foreground_priority = Some(value);
    }

    /// Gets the value of CostedNetworkBehaviorForegroundPriority
    pub fn get_costed_network_behavior_foreground_priority(&self) -> Option<&i32> {
        self.costed_network_behavior_foreground_priority.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of JobInactivityTimeout
    pub fn set_job_inactivity_timeout(&mut self, value: i32) {
        self.job_inactivity_timeout = Some(value);
    }

    /// Gets the value of JobInactivityTimeout
    pub fn get_job_inactivity_timeout(&self) -> Option<&i32> {
        self.job_inactivity_timeout.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }
}

