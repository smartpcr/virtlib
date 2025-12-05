// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualEthernetSwitchSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualEthernetSwitchSettingData {
    #[serde(flatten)]
    pub base: CIM_VirtualEthernetSwitchSettingData,

/// 
    #[serde(rename = "AllowNetLbfoTeams")]
    pub allow_net_lbfo_teams: Option<bool>,

/// 
    #[serde(rename = "BandwidthReservationMode")]
    pub bandwidth_reservation_mode: Option<u32>,

/// 
    #[serde(rename = "BypassExtensionStack")]
    pub bypass_extension_stack: Option<bool>,

/// 
    #[serde(rename = "ExtensionOrder")]
    pub extension_order: Vec<String>,

/// 
    #[serde(rename = "IOVPreferred")]
    pub iovpreferred: Option<bool>,

/// 
    #[serde(rename = "PacketDirectEnabled")]
    pub packet_direct_enabled: Option<bool>,

/// 
    #[serde(rename = "RequiredExtensionIds")]
    pub required_extension_ids: Vec<String>,

/// 
    #[serde(rename = "TeamingEnabled")]
    pub teaming_enabled: Option<bool>,
}

impl Msvm_VirtualEthernetSwitchSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_VirtualEthernetSwitchSettingData::new(),
            allow_net_lbfo_teams: None,
            bandwidth_reservation_mode: None,
            bypass_extension_stack: None,
            extension_order: Vec::new(),
            iovpreferred: None,
            packet_direct_enabled: None,
            required_extension_ids: Vec::new(),
            teaming_enabled: None,
        }
    }


    /// Sets the value of AllowNetLbfoTeams
    pub fn set_allow_net_lbfo_teams(&mut self, value: bool) {
        self.allow_net_lbfo_teams = Some(value);
    }

    /// Gets the value of AllowNetLbfoTeams
    pub fn get_allow_net_lbfo_teams(&self) -> Option<&bool> {
        self.allow_net_lbfo_teams.as_ref()
    }

    /// Sets the value of BandwidthReservationMode
    pub fn set_bandwidth_reservation_mode(&mut self, value: u32) {
        self.bandwidth_reservation_mode = Some(value);
    }

    /// Gets the value of BandwidthReservationMode
    pub fn get_bandwidth_reservation_mode(&self) -> Option<&u32> {
        self.bandwidth_reservation_mode.as_ref()
    }

    /// Sets the value of BypassExtensionStack
    pub fn set_bypass_extension_stack(&mut self, value: bool) {
        self.bypass_extension_stack = Some(value);
    }

    /// Gets the value of BypassExtensionStack
    pub fn get_bypass_extension_stack(&self) -> Option<&bool> {
        self.bypass_extension_stack.as_ref()
    }

    /// Sets the value of ExtensionOrder
    pub fn set_extension_order(&mut self, value: Vec<String>) {
        self.extension_order = value;
    }

    /// Gets the value of ExtensionOrder
    pub fn get_extension_order(&self) -> &Vec<String> {
        &self.extension_order
    }

    /// Sets the value of IOVPreferred
    pub fn set_iovpreferred(&mut self, value: bool) {
        self.iovpreferred = Some(value);
    }

    /// Gets the value of IOVPreferred
    pub fn get_iovpreferred(&self) -> Option<&bool> {
        self.iovpreferred.as_ref()
    }

    /// Sets the value of PacketDirectEnabled
    pub fn set_packet_direct_enabled(&mut self, value: bool) {
        self.packet_direct_enabled = Some(value);
    }

    /// Gets the value of PacketDirectEnabled
    pub fn get_packet_direct_enabled(&self) -> Option<&bool> {
        self.packet_direct_enabled.as_ref()
    }

    /// Sets the value of RequiredExtensionIds
    pub fn set_required_extension_ids(&mut self, value: Vec<String>) {
        self.required_extension_ids = value;
    }

    /// Gets the value of RequiredExtensionIds
    pub fn get_required_extension_ids(&self) -> &Vec<String> {
        &self.required_extension_ids
    }

    /// Sets the value of TeamingEnabled
    pub fn set_teaming_enabled(&mut self, value: bool) {
        self.teaming_enabled = Some(value);
    }

    /// Gets the value of TeamingEnabled
    pub fn get_teaming_enabled(&self) -> Option<&bool> {
        self.teaming_enabled.as_ref()
    }
}

impl Msvm_VirtualEthernetSwitchSettingData {
    /// Gets the related Msvm_EthernetPortAllocationSettingData object(s)
    pub fn get_related__ethernet_port_allocation_setting_data(&self) -> Result<Msvm_EthernetPortAllocationSettingData, WmiError> {
        self.get_related("Msvm_EthernetPortAllocationSettingData")
    }

    /// Gets the related Msvm_VirtualEthernetSwitch object(s)
    pub fn get_related__virtual_ethernet_switch(&self) -> Result<Msvm_VirtualEthernetSwitch, WmiError> {
        self.get_related("Msvm_VirtualEthernetSwitch")
    }

}

