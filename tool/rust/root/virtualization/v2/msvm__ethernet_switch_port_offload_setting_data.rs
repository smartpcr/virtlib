// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_EthernetSwitchPortOffloadSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_EthernetSwitchPortOffloadSettingData {
    #[serde(flatten)]
    pub base: Msvm_EthernetSwitchPortFeatureSettingData,

/// 
    #[serde(rename = "IOVInterruptModeration")]
    pub iovinterrupt_moderation: Option<EthernetSwitchPortOffloadSettingData_IOVInterruptModeration>,

/// 
    #[serde(rename = "IOVOffloadWeight")]
    pub iovoffload_weight: Option<u32>,

/// 
    #[serde(rename = "IOVQueuePairsRequested")]
    pub iovqueue_pairs_requested: Option<u32>,

/// 
    #[serde(rename = "IPSecOffloadLimit")]
    pub ipsec_offload_limit: Option<u32>,

/// 
    #[serde(rename = "PacketDirectModerationCount")]
    pub packet_direct_moderation_count: Option<u32>,

/// 
    #[serde(rename = "PacketDirectModerationInterval")]
    pub packet_direct_moderation_interval: Option<u32>,

/// 
    #[serde(rename = "PacketDirectNumProcs")]
    pub packet_direct_num_procs: Option<u32>,

/// 
    #[serde(rename = "RscEnabled")]
    pub rsc_enabled: Option<bool>,

/// 
    #[serde(rename = "VmmqEnabled")]
    pub vmmq_enabled: Option<bool>,

/// 
    #[serde(rename = "VmmqQueuePairs")]
    pub vmmq_queue_pairs: Option<u32>,

/// 
    #[serde(rename = "VMQOffloadWeight")]
    pub vmqoffload_weight: Option<u32>,

/// 
    #[serde(rename = "VrssEnabled")]
    pub vrss_enabled: Option<bool>,

/// 
    #[serde(rename = "VrssExcludePrimaryProcessor")]
    pub vrss_exclude_primary_processor: Option<bool>,

/// 
    #[serde(rename = "VrssIndependentHostSpreading")]
    pub vrss_independent_host_spreading: Option<bool>,

/// 
    #[serde(rename = "VrssMinQueuePairs")]
    pub vrss_min_queue_pairs: Option<u32>,

/// 
    #[serde(rename = "VrssQueueSchedulingMode")]
    pub vrss_queue_scheduling_mode: Option<u32>,

/// 
    #[serde(rename = "VrssVmbusChannelAffinityPolicy")]
    pub vrss_vmbus_channel_affinity_policy: Option<u32>,
}

impl Msvm_EthernetSwitchPortOffloadSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_EthernetSwitchPortFeatureSettingData::new(),
            iovinterrupt_moderation: None,
            iovoffload_weight: None,
            iovqueue_pairs_requested: None,
            ipsec_offload_limit: None,
            packet_direct_moderation_count: None,
            packet_direct_moderation_interval: None,
            packet_direct_num_procs: None,
            rsc_enabled: None,
            vmmq_enabled: None,
            vmmq_queue_pairs: None,
            vmqoffload_weight: None,
            vrss_enabled: None,
            vrss_exclude_primary_processor: None,
            vrss_independent_host_spreading: None,
            vrss_min_queue_pairs: None,
            vrss_queue_scheduling_mode: None,
            vrss_vmbus_channel_affinity_policy: None,
        }
    }


    /// Sets the value of IOVInterruptModeration
    pub fn set_iovinterrupt_moderation(&mut self, value: EthernetSwitchPortOffloadSettingData_IOVInterruptModeration) {
        self.iovinterrupt_moderation = Some(value);
    }

    /// Gets the value of IOVInterruptModeration
    pub fn get_iovinterrupt_moderation(&self) -> Option<&EthernetSwitchPortOffloadSettingData_IOVInterruptModeration> {
        self.iovinterrupt_moderation.as_ref()
    }

    /// Sets the value of IOVOffloadWeight
    pub fn set_iovoffload_weight(&mut self, value: u32) {
        self.iovoffload_weight = Some(value);
    }

    /// Gets the value of IOVOffloadWeight
    pub fn get_iovoffload_weight(&self) -> Option<&u32> {
        self.iovoffload_weight.as_ref()
    }

    /// Sets the value of IOVQueuePairsRequested
    pub fn set_iovqueue_pairs_requested(&mut self, value: u32) {
        self.iovqueue_pairs_requested = Some(value);
    }

    /// Gets the value of IOVQueuePairsRequested
    pub fn get_iovqueue_pairs_requested(&self) -> Option<&u32> {
        self.iovqueue_pairs_requested.as_ref()
    }

    /// Sets the value of IPSecOffloadLimit
    pub fn set_ipsec_offload_limit(&mut self, value: u32) {
        self.ipsec_offload_limit = Some(value);
    }

    /// Gets the value of IPSecOffloadLimit
    pub fn get_ipsec_offload_limit(&self) -> Option<&u32> {
        self.ipsec_offload_limit.as_ref()
    }

    /// Sets the value of PacketDirectModerationCount
    pub fn set_packet_direct_moderation_count(&mut self, value: u32) {
        self.packet_direct_moderation_count = Some(value);
    }

    /// Gets the value of PacketDirectModerationCount
    pub fn get_packet_direct_moderation_count(&self) -> Option<&u32> {
        self.packet_direct_moderation_count.as_ref()
    }

    /// Sets the value of PacketDirectModerationInterval
    pub fn set_packet_direct_moderation_interval(&mut self, value: u32) {
        self.packet_direct_moderation_interval = Some(value);
    }

    /// Gets the value of PacketDirectModerationInterval
    pub fn get_packet_direct_moderation_interval(&self) -> Option<&u32> {
        self.packet_direct_moderation_interval.as_ref()
    }

    /// Sets the value of PacketDirectNumProcs
    pub fn set_packet_direct_num_procs(&mut self, value: u32) {
        self.packet_direct_num_procs = Some(value);
    }

    /// Gets the value of PacketDirectNumProcs
    pub fn get_packet_direct_num_procs(&self) -> Option<&u32> {
        self.packet_direct_num_procs.as_ref()
    }

    /// Sets the value of RscEnabled
    pub fn set_rsc_enabled(&mut self, value: bool) {
        self.rsc_enabled = Some(value);
    }

    /// Gets the value of RscEnabled
    pub fn get_rsc_enabled(&self) -> Option<&bool> {
        self.rsc_enabled.as_ref()
    }

    /// Sets the value of VmmqEnabled
    pub fn set_vmmq_enabled(&mut self, value: bool) {
        self.vmmq_enabled = Some(value);
    }

    /// Gets the value of VmmqEnabled
    pub fn get_vmmq_enabled(&self) -> Option<&bool> {
        self.vmmq_enabled.as_ref()
    }

    /// Sets the value of VmmqQueuePairs
    pub fn set_vmmq_queue_pairs(&mut self, value: u32) {
        self.vmmq_queue_pairs = Some(value);
    }

    /// Gets the value of VmmqQueuePairs
    pub fn get_vmmq_queue_pairs(&self) -> Option<&u32> {
        self.vmmq_queue_pairs.as_ref()
    }

    /// Sets the value of VMQOffloadWeight
    pub fn set_vmqoffload_weight(&mut self, value: u32) {
        self.vmqoffload_weight = Some(value);
    }

    /// Gets the value of VMQOffloadWeight
    pub fn get_vmqoffload_weight(&self) -> Option<&u32> {
        self.vmqoffload_weight.as_ref()
    }

    /// Sets the value of VrssEnabled
    pub fn set_vrss_enabled(&mut self, value: bool) {
        self.vrss_enabled = Some(value);
    }

    /// Gets the value of VrssEnabled
    pub fn get_vrss_enabled(&self) -> Option<&bool> {
        self.vrss_enabled.as_ref()
    }

    /// Sets the value of VrssExcludePrimaryProcessor
    pub fn set_vrss_exclude_primary_processor(&mut self, value: bool) {
        self.vrss_exclude_primary_processor = Some(value);
    }

    /// Gets the value of VrssExcludePrimaryProcessor
    pub fn get_vrss_exclude_primary_processor(&self) -> Option<&bool> {
        self.vrss_exclude_primary_processor.as_ref()
    }

    /// Sets the value of VrssIndependentHostSpreading
    pub fn set_vrss_independent_host_spreading(&mut self, value: bool) {
        self.vrss_independent_host_spreading = Some(value);
    }

    /// Gets the value of VrssIndependentHostSpreading
    pub fn get_vrss_independent_host_spreading(&self) -> Option<&bool> {
        self.vrss_independent_host_spreading.as_ref()
    }

    /// Sets the value of VrssMinQueuePairs
    pub fn set_vrss_min_queue_pairs(&mut self, value: u32) {
        self.vrss_min_queue_pairs = Some(value);
    }

    /// Gets the value of VrssMinQueuePairs
    pub fn get_vrss_min_queue_pairs(&self) -> Option<&u32> {
        self.vrss_min_queue_pairs.as_ref()
    }

    /// Sets the value of VrssQueueSchedulingMode
    pub fn set_vrss_queue_scheduling_mode(&mut self, value: u32) {
        self.vrss_queue_scheduling_mode = Some(value);
    }

    /// Gets the value of VrssQueueSchedulingMode
    pub fn get_vrss_queue_scheduling_mode(&self) -> Option<&u32> {
        self.vrss_queue_scheduling_mode.as_ref()
    }

    /// Sets the value of VrssVmbusChannelAffinityPolicy
    pub fn set_vrss_vmbus_channel_affinity_policy(&mut self, value: u32) {
        self.vrss_vmbus_channel_affinity_policy = Some(value);
    }

    /// Gets the value of VrssVmbusChannelAffinityPolicy
    pub fn get_vrss_vmbus_channel_affinity_policy(&self) -> Option<&u32> {
        self.vrss_vmbus_channel_affinity_policy.as_ref()
    }
}

impl Msvm_EthernetSwitchPortOffloadSettingData {
    /// Gets the related Msvm_EthernetPortAllocationSettingData object(s)
    pub fn get_related__ethernet_port_allocation_setting_data(&self) -> Result<Msvm_EthernetPortAllocationSettingData, WmiError> {
        self.get_related("Msvm_EthernetPortAllocationSettingData")
    }

}

