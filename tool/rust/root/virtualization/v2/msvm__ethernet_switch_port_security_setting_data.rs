// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_EthernetSwitchPortSecuritySettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_EthernetSwitchPortSecuritySettingData {
    #[serde(flatten)]
    pub base: Msvm_EthernetSwitchPortFeatureSettingData,

/// 
    #[serde(rename = "AllowIeeePriorityTag")]
    pub allow_ieee_priority_tag: Option<bool>,

/// 
    #[serde(rename = "AllowMacSpoofing")]
    pub allow_mac_spoofing: Option<bool>,

/// 
    #[serde(rename = "AllowTeaming")]
    pub allow_teaming: Option<bool>,

/// 
    #[serde(rename = "DynamicIPAddressLimit")]
    pub dynamic_ipaddress_limit: Option<u32>,

/// 
    #[serde(rename = "EnableDhcpGuard")]
    pub enable_dhcp_guard: Option<bool>,

/// 
    #[serde(rename = "EnableFixSpeed10G")]
    pub enable_fix_speed10_g: Option<bool>,

/// 
    #[serde(rename = "EnableRouterGuard")]
    pub enable_router_guard: Option<bool>,

/// 
    #[serde(rename = "MonitorMode")]
    pub monitor_mode: Option<u8>,

/// 
    #[serde(rename = "MonitorSession")]
    pub monitor_session: Option<u8>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<bool>,

/// 
    #[serde(rename = "StormLimit")]
    pub storm_limit: Option<u32>,

/// 
    #[serde(rename = "TeamName")]
    pub team_name: Option<String>,

/// 
    #[serde(rename = "TeamNumber")]
    pub team_number: Option<u32>,

/// 
    #[serde(rename = "VirtualSubnetId")]
    pub virtual_subnet_id: Option<u32>,
}

impl Msvm_EthernetSwitchPortSecuritySettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_EthernetSwitchPortFeatureSettingData::new(),
            allow_ieee_priority_tag: None,
            allow_mac_spoofing: None,
            allow_teaming: None,
            dynamic_ipaddress_limit: None,
            enable_dhcp_guard: None,
            enable_fix_speed10_g: None,
            enable_router_guard: None,
            monitor_mode: None,
            monitor_session: None,
            reserved: None,
            storm_limit: None,
            team_name: None,
            team_number: None,
            virtual_subnet_id: None,
        }
    }


    /// Sets the value of AllowIeeePriorityTag
    pub fn set_allow_ieee_priority_tag(&mut self, value: bool) {
        self.allow_ieee_priority_tag = Some(value);
    }

    /// Gets the value of AllowIeeePriorityTag
    pub fn get_allow_ieee_priority_tag(&self) -> Option<&bool> {
        self.allow_ieee_priority_tag.as_ref()
    }

    /// Sets the value of AllowMacSpoofing
    pub fn set_allow_mac_spoofing(&mut self, value: bool) {
        self.allow_mac_spoofing = Some(value);
    }

    /// Gets the value of AllowMacSpoofing
    pub fn get_allow_mac_spoofing(&self) -> Option<&bool> {
        self.allow_mac_spoofing.as_ref()
    }

    /// Sets the value of AllowTeaming
    pub fn set_allow_teaming(&mut self, value: bool) {
        self.allow_teaming = Some(value);
    }

    /// Gets the value of AllowTeaming
    pub fn get_allow_teaming(&self) -> Option<&bool> {
        self.allow_teaming.as_ref()
    }

    /// Sets the value of DynamicIPAddressLimit
    pub fn set_dynamic_ipaddress_limit(&mut self, value: u32) {
        self.dynamic_ipaddress_limit = Some(value);
    }

    /// Gets the value of DynamicIPAddressLimit
    pub fn get_dynamic_ipaddress_limit(&self) -> Option<&u32> {
        self.dynamic_ipaddress_limit.as_ref()
    }

    /// Sets the value of EnableDhcpGuard
    pub fn set_enable_dhcp_guard(&mut self, value: bool) {
        self.enable_dhcp_guard = Some(value);
    }

    /// Gets the value of EnableDhcpGuard
    pub fn get_enable_dhcp_guard(&self) -> Option<&bool> {
        self.enable_dhcp_guard.as_ref()
    }

    /// Sets the value of EnableFixSpeed10G
    pub fn set_enable_fix_speed10_g(&mut self, value: bool) {
        self.enable_fix_speed10_g = Some(value);
    }

    /// Gets the value of EnableFixSpeed10G
    pub fn get_enable_fix_speed10_g(&self) -> Option<&bool> {
        self.enable_fix_speed10_g.as_ref()
    }

    /// Sets the value of EnableRouterGuard
    pub fn set_enable_router_guard(&mut self, value: bool) {
        self.enable_router_guard = Some(value);
    }

    /// Gets the value of EnableRouterGuard
    pub fn get_enable_router_guard(&self) -> Option<&bool> {
        self.enable_router_guard.as_ref()
    }

    /// Sets the value of MonitorMode
    pub fn set_monitor_mode(&mut self, value: u8) {
        self.monitor_mode = Some(value);
    }

    /// Gets the value of MonitorMode
    pub fn get_monitor_mode(&self) -> Option<&u8> {
        self.monitor_mode.as_ref()
    }

    /// Sets the value of MonitorSession
    pub fn set_monitor_session(&mut self, value: u8) {
        self.monitor_session = Some(value);
    }

    /// Gets the value of MonitorSession
    pub fn get_monitor_session(&self) -> Option<&u8> {
        self.monitor_session.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: bool) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&bool> {
        self.reserved.as_ref()
    }

    /// Sets the value of StormLimit
    pub fn set_storm_limit(&mut self, value: u32) {
        self.storm_limit = Some(value);
    }

    /// Gets the value of StormLimit
    pub fn get_storm_limit(&self) -> Option<&u32> {
        self.storm_limit.as_ref()
    }

    /// Sets the value of TeamName
    pub fn set_team_name(&mut self, value: String) {
        self.team_name = Some(value);
    }

    /// Gets the value of TeamName
    pub fn get_team_name(&self) -> Option<&String> {
        self.team_name.as_ref()
    }

    /// Sets the value of TeamNumber
    pub fn set_team_number(&mut self, value: u32) {
        self.team_number = Some(value);
    }

    /// Gets the value of TeamNumber
    pub fn get_team_number(&self) -> Option<&u32> {
        self.team_number.as_ref()
    }

    /// Sets the value of VirtualSubnetId
    pub fn set_virtual_subnet_id(&mut self, value: u32) {
        self.virtual_subnet_id = Some(value);
    }

    /// Gets the value of VirtualSubnetId
    pub fn get_virtual_subnet_id(&self) -> Option<&u32> {
        self.virtual_subnet_id.as_ref()
    }
}

impl Msvm_EthernetSwitchPortSecuritySettingData {
    /// Gets the related Msvm_EthernetSwitchFeatureCapabilities object(s)
    pub fn get_related__ethernet_switch_feature_capabilities(&self) -> Result<Msvm_EthernetSwitchFeatureCapabilities, WmiError> {
        self.get_related("Msvm_EthernetSwitchFeatureCapabilities")
    }

}

