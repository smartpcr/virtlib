// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_80211_WLanBssId struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_80211_WLanBssId {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Ndis80211Configuration")]
    pub ndis80211_configuration: Option<MSNdis_80211_ConfigurationInfo>,

/// 
    #[serde(rename = "Ndis80211InfrastructureMode")]
    pub ndis80211_infrastructure_mode: Option<MSNdis_80211_NetworkInfrastructure>,

/// 
    #[serde(rename = "Ndis80211MacAddress")]
    pub ndis80211_mac_address: Vec<u8>,

/// 
    #[serde(rename = "Ndis80211NetworkTypeInUse")]
    pub ndis80211_network_type_in_use: Option<MSNdis_80211_NetworkType>,

/// 
    #[serde(rename = "Ndis80211Privacy")]
    pub ndis80211_privacy: Option<u32>,

/// 
    #[serde(rename = "Ndis80211Rssi")]
    pub ndis80211_rssi: Option<u32>,

/// 
    #[serde(rename = "Ndis80211SsId")]
    pub ndis80211_ss_id: Vec<u8>,

/// 
    #[serde(rename = "Ndis80211SsIdLength")]
    pub ndis80211_ss_id_length: Option<u32>,

/// 
    #[serde(rename = "Ndis80211SupportedRate")]
    pub ndis80211_supported_rate: Vec<u8>,

/// 
    #[serde(rename = "Ndis80211WLanBssIdLength")]
    pub ndis80211_wlan_bss_id_length: Option<u32>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<u16>,
}

impl MSNdis_80211_WLanBssId {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            ndis80211_configuration: None,
            ndis80211_infrastructure_mode: None,
            ndis80211_mac_address: Vec::new(),
            ndis80211_network_type_in_use: None,
            ndis80211_privacy: None,
            ndis80211_rssi: None,
            ndis80211_ss_id: Vec::new(),
            ndis80211_ss_id_length: None,
            ndis80211_supported_rate: Vec::new(),
            ndis80211_wlan_bss_id_length: None,
            reserved: None,
        }
    }


    /// Sets the value of Ndis80211Configuration
    pub fn set_ndis80211_configuration(&mut self, value: MSNdis_80211_ConfigurationInfo) {
        self.ndis80211_configuration = Some(value);
    }

    /// Gets the value of Ndis80211Configuration
    pub fn get_ndis80211_configuration(&self) -> Option<&MSNdis_80211_ConfigurationInfo> {
        self.ndis80211_configuration.as_ref()
    }

    /// Sets the value of Ndis80211InfrastructureMode
    pub fn set_ndis80211_infrastructure_mode(&mut self, value: MSNdis_80211_NetworkInfrastructure) {
        self.ndis80211_infrastructure_mode = Some(value);
    }

    /// Gets the value of Ndis80211InfrastructureMode
    pub fn get_ndis80211_infrastructure_mode(&self) -> Option<&MSNdis_80211_NetworkInfrastructure> {
        self.ndis80211_infrastructure_mode.as_ref()
    }

    /// Sets the value of Ndis80211MacAddress
    pub fn set_ndis80211_mac_address(&mut self, value: Vec<u8>) {
        self.ndis80211_mac_address = value;
    }

    /// Gets the value of Ndis80211MacAddress
    pub fn get_ndis80211_mac_address(&self) -> &Vec<u8> {
        &self.ndis80211_mac_address
    }

    /// Sets the value of Ndis80211NetworkTypeInUse
    pub fn set_ndis80211_network_type_in_use(&mut self, value: MSNdis_80211_NetworkType) {
        self.ndis80211_network_type_in_use = Some(value);
    }

    /// Gets the value of Ndis80211NetworkTypeInUse
    pub fn get_ndis80211_network_type_in_use(&self) -> Option<&MSNdis_80211_NetworkType> {
        self.ndis80211_network_type_in_use.as_ref()
    }

    /// Sets the value of Ndis80211Privacy
    pub fn set_ndis80211_privacy(&mut self, value: u32) {
        self.ndis80211_privacy = Some(value);
    }

    /// Gets the value of Ndis80211Privacy
    pub fn get_ndis80211_privacy(&self) -> Option<&u32> {
        self.ndis80211_privacy.as_ref()
    }

    /// Sets the value of Ndis80211Rssi
    pub fn set_ndis80211_rssi(&mut self, value: u32) {
        self.ndis80211_rssi = Some(value);
    }

    /// Gets the value of Ndis80211Rssi
    pub fn get_ndis80211_rssi(&self) -> Option<&u32> {
        self.ndis80211_rssi.as_ref()
    }

    /// Sets the value of Ndis80211SsId
    pub fn set_ndis80211_ss_id(&mut self, value: Vec<u8>) {
        self.ndis80211_ss_id = value;
    }

    /// Gets the value of Ndis80211SsId
    pub fn get_ndis80211_ss_id(&self) -> &Vec<u8> {
        &self.ndis80211_ss_id
    }

    /// Sets the value of Ndis80211SsIdLength
    pub fn set_ndis80211_ss_id_length(&mut self, value: u32) {
        self.ndis80211_ss_id_length = Some(value);
    }

    /// Gets the value of Ndis80211SsIdLength
    pub fn get_ndis80211_ss_id_length(&self) -> Option<&u32> {
        self.ndis80211_ss_id_length.as_ref()
    }

    /// Sets the value of Ndis80211SupportedRate
    pub fn set_ndis80211_supported_rate(&mut self, value: Vec<u8>) {
        self.ndis80211_supported_rate = value;
    }

    /// Gets the value of Ndis80211SupportedRate
    pub fn get_ndis80211_supported_rate(&self) -> &Vec<u8> {
        &self.ndis80211_supported_rate
    }

    /// Sets the value of Ndis80211WLanBssIdLength
    pub fn set_ndis80211_wlan_bss_id_length(&mut self, value: u32) {
        self.ndis80211_wlan_bss_id_length = Some(value);
    }

    /// Gets the value of Ndis80211WLanBssIdLength
    pub fn get_ndis80211_wlan_bss_id_length(&self) -> Option<&u32> {
        self.ndis80211_wlan_bss_id_length.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u16) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u16> {
        self.reserved.as_ref()
    }
}

