// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterSriovCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterSriovCapabilities {

/// 
    #[serde(rename = "AsymmetricQueuePairsForNonDefaultVPortsSupported")]
    pub asymmetric_queue_pairs_for_non_default_vports_supported: Option<bool>,

/// 
    #[serde(rename = "MaxNumMacAddresses")]
    pub max_num_mac_addresses: Option<u32>,

/// 
    #[serde(rename = "MaxNumQueuePairs")]
    pub max_num_queue_pairs: Option<u32>,

/// 
    #[serde(rename = "MaxNumQueuePairsPerNonDefaultVPort")]
    pub max_num_queue_pairs_per_non_default_vport: Option<u32>,

/// 
    #[serde(rename = "MaxNumSwitches")]
    pub max_num_switches: Option<u32>,

/// 
    #[serde(rename = "MaxNumVFs")]
    pub max_num_vfs: Option<u32>,

/// 
    #[serde(rename = "MaxNumVPorts")]
    pub max_num_vports: Option<u32>,

/// 
    #[serde(rename = "PerVportInterruptModerationSupported")]
    pub per_vport_interrupt_moderation_supported: Option<bool>,

/// 
    #[serde(rename = "SingleVportPoolSupported")]
    pub single_vport_pool_supported: Option<bool>,

/// 
    #[serde(rename = "VfRssSupported")]
    pub vf_rss_supported: Option<bool>,

/// 
    #[serde(rename = "VlanSupported")]
    pub vlan_supported: Option<bool>,
}

impl MSFT_NetAdapterSriovCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            asymmetric_queue_pairs_for_non_default_vports_supported: None,
            max_num_mac_addresses: None,
            max_num_queue_pairs: None,
            max_num_queue_pairs_per_non_default_vport: None,
            max_num_switches: None,
            max_num_vfs: None,
            max_num_vports: None,
            per_vport_interrupt_moderation_supported: None,
            single_vport_pool_supported: None,
            vf_rss_supported: None,
            vlan_supported: None,
        }
    }


    /// Sets the value of AsymmetricQueuePairsForNonDefaultVPortsSupported
    pub fn set_asymmetric_queue_pairs_for_non_default_vports_supported(&mut self, value: bool) {
        self.asymmetric_queue_pairs_for_non_default_vports_supported = Some(value);
    }

    /// Gets the value of AsymmetricQueuePairsForNonDefaultVPortsSupported
    pub fn get_asymmetric_queue_pairs_for_non_default_vports_supported(&self) -> Option<&bool> {
        self.asymmetric_queue_pairs_for_non_default_vports_supported.as_ref()
    }

    /// Sets the value of MaxNumMacAddresses
    pub fn set_max_num_mac_addresses(&mut self, value: u32) {
        self.max_num_mac_addresses = Some(value);
    }

    /// Gets the value of MaxNumMacAddresses
    pub fn get_max_num_mac_addresses(&self) -> Option<&u32> {
        self.max_num_mac_addresses.as_ref()
    }

    /// Sets the value of MaxNumQueuePairs
    pub fn set_max_num_queue_pairs(&mut self, value: u32) {
        self.max_num_queue_pairs = Some(value);
    }

    /// Gets the value of MaxNumQueuePairs
    pub fn get_max_num_queue_pairs(&self) -> Option<&u32> {
        self.max_num_queue_pairs.as_ref()
    }

    /// Sets the value of MaxNumQueuePairsPerNonDefaultVPort
    pub fn set_max_num_queue_pairs_per_non_default_vport(&mut self, value: u32) {
        self.max_num_queue_pairs_per_non_default_vport = Some(value);
    }

    /// Gets the value of MaxNumQueuePairsPerNonDefaultVPort
    pub fn get_max_num_queue_pairs_per_non_default_vport(&self) -> Option<&u32> {
        self.max_num_queue_pairs_per_non_default_vport.as_ref()
    }

    /// Sets the value of MaxNumSwitches
    pub fn set_max_num_switches(&mut self, value: u32) {
        self.max_num_switches = Some(value);
    }

    /// Gets the value of MaxNumSwitches
    pub fn get_max_num_switches(&self) -> Option<&u32> {
        self.max_num_switches.as_ref()
    }

    /// Sets the value of MaxNumVFs
    pub fn set_max_num_vfs(&mut self, value: u32) {
        self.max_num_vfs = Some(value);
    }

    /// Gets the value of MaxNumVFs
    pub fn get_max_num_vfs(&self) -> Option<&u32> {
        self.max_num_vfs.as_ref()
    }

    /// Sets the value of MaxNumVPorts
    pub fn set_max_num_vports(&mut self, value: u32) {
        self.max_num_vports = Some(value);
    }

    /// Gets the value of MaxNumVPorts
    pub fn get_max_num_vports(&self) -> Option<&u32> {
        self.max_num_vports.as_ref()
    }

    /// Sets the value of PerVportInterruptModerationSupported
    pub fn set_per_vport_interrupt_moderation_supported(&mut self, value: bool) {
        self.per_vport_interrupt_moderation_supported = Some(value);
    }

    /// Gets the value of PerVportInterruptModerationSupported
    pub fn get_per_vport_interrupt_moderation_supported(&self) -> Option<&bool> {
        self.per_vport_interrupt_moderation_supported.as_ref()
    }

    /// Sets the value of SingleVportPoolSupported
    pub fn set_single_vport_pool_supported(&mut self, value: bool) {
        self.single_vport_pool_supported = Some(value);
    }

    /// Gets the value of SingleVportPoolSupported
    pub fn get_single_vport_pool_supported(&self) -> Option<&bool> {
        self.single_vport_pool_supported.as_ref()
    }

    /// Sets the value of VfRssSupported
    pub fn set_vf_rss_supported(&mut self, value: bool) {
        self.vf_rss_supported = Some(value);
    }

    /// Gets the value of VfRssSupported
    pub fn get_vf_rss_supported(&self) -> Option<&bool> {
        self.vf_rss_supported.as_ref()
    }

    /// Sets the value of VlanSupported
    pub fn set_vlan_supported(&mut self, value: bool) {
        self.vlan_supported = Some(value);
    }

    /// Gets the value of VlanSupported
    pub fn get_vlan_supported(&self) -> Option<&bool> {
        self.vlan_supported.as_ref()
    }
}

