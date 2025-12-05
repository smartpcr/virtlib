// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_EthernetSwitchPortOffloadData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_EthernetSwitchPortOffloadData {
    #[serde(flatten)]
    pub base: Msvm_EthernetPortData,

/// 
    #[serde(rename = "IovOffloadUsage")]
    pub iov_offload_usage: Option<u32>,

/// 
    #[serde(rename = "IovQueuePairUsage")]
    pub iov_queue_pair_usage: Option<u32>,

/// 
    #[serde(rename = "IovVfDataPathActive")]
    pub iov_vf_data_path_active: Option<bool>,

/// 
    #[serde(rename = "IovVfId")]
    pub iov_vf_id: Option<u16>,

/// 
    #[serde(rename = "IpsecCurrentOffloadSaCount")]
    pub ipsec_current_offload_sa_count: Option<u32>,

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
    #[serde(rename = "VMQId")]
    pub vmqid: Option<u32>,

/// 
    #[serde(rename = "VMQOffloadUsage")]
    pub vmqoffload_usage: Option<u32>,

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

impl Msvm_EthernetSwitchPortOffloadData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_EthernetPortData::new(),
            iov_offload_usage: None,
            iov_queue_pair_usage: None,
            iov_vf_data_path_active: None,
            iov_vf_id: None,
            ipsec_current_offload_sa_count: None,
            rsc_enabled: None,
            vmmq_enabled: None,
            vmmq_queue_pairs: None,
            vmqid: None,
            vmqoffload_usage: None,
            vrss_enabled: None,
            vrss_exclude_primary_processor: None,
            vrss_independent_host_spreading: None,
            vrss_min_queue_pairs: None,
            vrss_queue_scheduling_mode: None,
            vrss_vmbus_channel_affinity_policy: None,
        }
    }


    /// Sets the value of IovOffloadUsage
    pub fn set_iov_offload_usage(&mut self, value: u32) {
        self.iov_offload_usage = Some(value);
    }

    /// Gets the value of IovOffloadUsage
    pub fn get_iov_offload_usage(&self) -> Option<&u32> {
        self.iov_offload_usage.as_ref()
    }

    /// Sets the value of IovQueuePairUsage
    pub fn set_iov_queue_pair_usage(&mut self, value: u32) {
        self.iov_queue_pair_usage = Some(value);
    }

    /// Gets the value of IovQueuePairUsage
    pub fn get_iov_queue_pair_usage(&self) -> Option<&u32> {
        self.iov_queue_pair_usage.as_ref()
    }

    /// Sets the value of IovVfDataPathActive
    pub fn set_iov_vf_data_path_active(&mut self, value: bool) {
        self.iov_vf_data_path_active = Some(value);
    }

    /// Gets the value of IovVfDataPathActive
    pub fn get_iov_vf_data_path_active(&self) -> Option<&bool> {
        self.iov_vf_data_path_active.as_ref()
    }

    /// Sets the value of IovVfId
    pub fn set_iov_vf_id(&mut self, value: u16) {
        self.iov_vf_id = Some(value);
    }

    /// Gets the value of IovVfId
    pub fn get_iov_vf_id(&self) -> Option<&u16> {
        self.iov_vf_id.as_ref()
    }

    /// Sets the value of IpsecCurrentOffloadSaCount
    pub fn set_ipsec_current_offload_sa_count(&mut self, value: u32) {
        self.ipsec_current_offload_sa_count = Some(value);
    }

    /// Gets the value of IpsecCurrentOffloadSaCount
    pub fn get_ipsec_current_offload_sa_count(&self) -> Option<&u32> {
        self.ipsec_current_offload_sa_count.as_ref()
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

    /// Sets the value of VMQId
    pub fn set_vmqid(&mut self, value: u32) {
        self.vmqid = Some(value);
    }

    /// Gets the value of VMQId
    pub fn get_vmqid(&self) -> Option<&u32> {
        self.vmqid.as_ref()
    }

    /// Sets the value of VMQOffloadUsage
    pub fn set_vmqoffload_usage(&mut self, value: u32) {
        self.vmqoffload_usage = Some(value);
    }

    /// Gets the value of VMQOffloadUsage
    pub fn get_vmqoffload_usage(&self) -> Option<&u32> {
        self.vmqoffload_usage.as_ref()
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

impl Msvm_EthernetSwitchPortOffloadData {
    /// Gets the related Msvm_EthernetSwitchPort object(s)
    pub fn get_related__ethernet_switch_port(&self) -> Result<Msvm_EthernetSwitchPort, WmiError> {
        self.get_related("Msvm_EthernetSwitchPort")
    }

}

