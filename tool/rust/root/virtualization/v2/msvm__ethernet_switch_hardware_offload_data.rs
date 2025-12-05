// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_EthernetSwitchHardwareOffloadData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_EthernetSwitchHardwareOffloadData {
    #[serde(flatten)]
    pub base: Msvm_EthernetSwitchData,

/// 
    #[serde(rename = "DefaultQueueVmmqEnabled")]
    pub default_queue_vmmq_enabled: Option<bool>,

/// 
    #[serde(rename = "DefaultQueueVmmqQueuePairs")]
    pub default_queue_vmmq_queue_pairs: Option<u32>,

/// 
    #[serde(rename = "DefaultQueueVrssEnabled")]
    pub default_queue_vrss_enabled: Option<bool>,

/// 
    #[serde(rename = "DefaultQueueVrssExcludePrimaryProcessor")]
    pub default_queue_vrss_exclude_primary_processor: Option<bool>,

/// 
    #[serde(rename = "DefaultQueueVrssIndependentHostSpreading")]
    pub default_queue_vrss_independent_host_spreading: Option<bool>,

/// 
    #[serde(rename = "DefaultQueueVrssMinQueuePairs")]
    pub default_queue_vrss_min_queue_pairs: Option<u32>,

/// 
    #[serde(rename = "DefaultQueueVrssQueueSchedulingMode")]
    pub default_queue_vrss_queue_scheduling_mode: Option<u32>,

/// 
    #[serde(rename = "IovQueuePairCapacity")]
    pub iov_queue_pair_capacity: Option<u32>,

/// 
    #[serde(rename = "IovQueuePairUsage")]
    pub iov_queue_pair_usage: Option<u32>,

/// 
    #[serde(rename = "IovVfCapacity")]
    pub iov_vf_capacity: Option<u32>,

/// 
    #[serde(rename = "IovVfUsage")]
    pub iov_vf_usage: Option<u32>,

/// 
    #[serde(rename = "IPsecSACapacity")]
    pub ipsec_sacapacity: Option<u32>,

/// 
    #[serde(rename = "IPsecSAUsage")]
    pub ipsec_sausage: Option<u32>,

/// 
    #[serde(rename = "PacketDirectInUse")]
    pub packet_direct_in_use: Option<bool>,

/// 
    #[serde(rename = "VmqCapacity")]
    pub vmq_capacity: Option<u32>,

/// 
    #[serde(rename = "VmqUsage")]
    pub vmq_usage: Option<u32>,
}

impl Msvm_EthernetSwitchHardwareOffloadData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_EthernetSwitchData::new(),
            default_queue_vmmq_enabled: None,
            default_queue_vmmq_queue_pairs: None,
            default_queue_vrss_enabled: None,
            default_queue_vrss_exclude_primary_processor: None,
            default_queue_vrss_independent_host_spreading: None,
            default_queue_vrss_min_queue_pairs: None,
            default_queue_vrss_queue_scheduling_mode: None,
            iov_queue_pair_capacity: None,
            iov_queue_pair_usage: None,
            iov_vf_capacity: None,
            iov_vf_usage: None,
            ipsec_sacapacity: None,
            ipsec_sausage: None,
            packet_direct_in_use: None,
            vmq_capacity: None,
            vmq_usage: None,
        }
    }


    /// Sets the value of DefaultQueueVmmqEnabled
    pub fn set_default_queue_vmmq_enabled(&mut self, value: bool) {
        self.default_queue_vmmq_enabled = Some(value);
    }

    /// Gets the value of DefaultQueueVmmqEnabled
    pub fn get_default_queue_vmmq_enabled(&self) -> Option<&bool> {
        self.default_queue_vmmq_enabled.as_ref()
    }

    /// Sets the value of DefaultQueueVmmqQueuePairs
    pub fn set_default_queue_vmmq_queue_pairs(&mut self, value: u32) {
        self.default_queue_vmmq_queue_pairs = Some(value);
    }

    /// Gets the value of DefaultQueueVmmqQueuePairs
    pub fn get_default_queue_vmmq_queue_pairs(&self) -> Option<&u32> {
        self.default_queue_vmmq_queue_pairs.as_ref()
    }

    /// Sets the value of DefaultQueueVrssEnabled
    pub fn set_default_queue_vrss_enabled(&mut self, value: bool) {
        self.default_queue_vrss_enabled = Some(value);
    }

    /// Gets the value of DefaultQueueVrssEnabled
    pub fn get_default_queue_vrss_enabled(&self) -> Option<&bool> {
        self.default_queue_vrss_enabled.as_ref()
    }

    /// Sets the value of DefaultQueueVrssExcludePrimaryProcessor
    pub fn set_default_queue_vrss_exclude_primary_processor(&mut self, value: bool) {
        self.default_queue_vrss_exclude_primary_processor = Some(value);
    }

    /// Gets the value of DefaultQueueVrssExcludePrimaryProcessor
    pub fn get_default_queue_vrss_exclude_primary_processor(&self) -> Option<&bool> {
        self.default_queue_vrss_exclude_primary_processor.as_ref()
    }

    /// Sets the value of DefaultQueueVrssIndependentHostSpreading
    pub fn set_default_queue_vrss_independent_host_spreading(&mut self, value: bool) {
        self.default_queue_vrss_independent_host_spreading = Some(value);
    }

    /// Gets the value of DefaultQueueVrssIndependentHostSpreading
    pub fn get_default_queue_vrss_independent_host_spreading(&self) -> Option<&bool> {
        self.default_queue_vrss_independent_host_spreading.as_ref()
    }

    /// Sets the value of DefaultQueueVrssMinQueuePairs
    pub fn set_default_queue_vrss_min_queue_pairs(&mut self, value: u32) {
        self.default_queue_vrss_min_queue_pairs = Some(value);
    }

    /// Gets the value of DefaultQueueVrssMinQueuePairs
    pub fn get_default_queue_vrss_min_queue_pairs(&self) -> Option<&u32> {
        self.default_queue_vrss_min_queue_pairs.as_ref()
    }

    /// Sets the value of DefaultQueueVrssQueueSchedulingMode
    pub fn set_default_queue_vrss_queue_scheduling_mode(&mut self, value: u32) {
        self.default_queue_vrss_queue_scheduling_mode = Some(value);
    }

    /// Gets the value of DefaultQueueVrssQueueSchedulingMode
    pub fn get_default_queue_vrss_queue_scheduling_mode(&self) -> Option<&u32> {
        self.default_queue_vrss_queue_scheduling_mode.as_ref()
    }

    /// Sets the value of IovQueuePairCapacity
    pub fn set_iov_queue_pair_capacity(&mut self, value: u32) {
        self.iov_queue_pair_capacity = Some(value);
    }

    /// Gets the value of IovQueuePairCapacity
    pub fn get_iov_queue_pair_capacity(&self) -> Option<&u32> {
        self.iov_queue_pair_capacity.as_ref()
    }

    /// Sets the value of IovQueuePairUsage
    pub fn set_iov_queue_pair_usage(&mut self, value: u32) {
        self.iov_queue_pair_usage = Some(value);
    }

    /// Gets the value of IovQueuePairUsage
    pub fn get_iov_queue_pair_usage(&self) -> Option<&u32> {
        self.iov_queue_pair_usage.as_ref()
    }

    /// Sets the value of IovVfCapacity
    pub fn set_iov_vf_capacity(&mut self, value: u32) {
        self.iov_vf_capacity = Some(value);
    }

    /// Gets the value of IovVfCapacity
    pub fn get_iov_vf_capacity(&self) -> Option<&u32> {
        self.iov_vf_capacity.as_ref()
    }

    /// Sets the value of IovVfUsage
    pub fn set_iov_vf_usage(&mut self, value: u32) {
        self.iov_vf_usage = Some(value);
    }

    /// Gets the value of IovVfUsage
    pub fn get_iov_vf_usage(&self) -> Option<&u32> {
        self.iov_vf_usage.as_ref()
    }

    /// Sets the value of IPsecSACapacity
    pub fn set_ipsec_sacapacity(&mut self, value: u32) {
        self.ipsec_sacapacity = Some(value);
    }

    /// Gets the value of IPsecSACapacity
    pub fn get_ipsec_sacapacity(&self) -> Option<&u32> {
        self.ipsec_sacapacity.as_ref()
    }

    /// Sets the value of IPsecSAUsage
    pub fn set_ipsec_sausage(&mut self, value: u32) {
        self.ipsec_sausage = Some(value);
    }

    /// Gets the value of IPsecSAUsage
    pub fn get_ipsec_sausage(&self) -> Option<&u32> {
        self.ipsec_sausage.as_ref()
    }

    /// Sets the value of PacketDirectInUse
    pub fn set_packet_direct_in_use(&mut self, value: bool) {
        self.packet_direct_in_use = Some(value);
    }

    /// Gets the value of PacketDirectInUse
    pub fn get_packet_direct_in_use(&self) -> Option<&bool> {
        self.packet_direct_in_use.as_ref()
    }

    /// Sets the value of VmqCapacity
    pub fn set_vmq_capacity(&mut self, value: u32) {
        self.vmq_capacity = Some(value);
    }

    /// Gets the value of VmqCapacity
    pub fn get_vmq_capacity(&self) -> Option<&u32> {
        self.vmq_capacity.as_ref()
    }

    /// Sets the value of VmqUsage
    pub fn set_vmq_usage(&mut self, value: u32) {
        self.vmq_usage = Some(value);
    }

    /// Gets the value of VmqUsage
    pub fn get_vmq_usage(&self) -> Option<&u32> {
        self.vmq_usage.as_ref()
    }
}

impl Msvm_EthernetSwitchHardwareOffloadData {
    /// Gets the related Msvm_VirtualEthernetSwitch object(s)
    pub fn get_related__virtual_ethernet_switch(&self) -> Result<Msvm_VirtualEthernetSwitch, WmiError> {
        self.get_related("Msvm_VirtualEthernetSwitch")
    }

}

