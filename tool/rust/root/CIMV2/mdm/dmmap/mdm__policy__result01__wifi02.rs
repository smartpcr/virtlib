// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_Wifi02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_Wifi02 {

/// 
    #[serde(rename = "AllowAutoConnectToWiFiSenseHotspots")]
    pub allow_auto_connect_to_wi_fi_sense_hotspots: Option<i32>,

/// 
    #[serde(rename = "AllowInternetSharing")]
    pub allow_internet_sharing: Option<i32>,

/// 
    #[serde(rename = "AllowManualWiFiConfiguration")]
    pub allow_manual_wi_fi_configuration: Option<i32>,

/// 
    #[serde(rename = "AllowWiFi")]
    pub allow_wi_fi: Option<i32>,

/// 
    #[serde(rename = "AllowWiFiDirect")]
    pub allow_wi_fi_direct: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "WiFiToWlan")]
    pub wi_fi_to_wlan: Option<i32>,

/// 
    #[serde(rename = "WLANScanMode")]
    pub wlanscan_mode: Option<i32>,
}

impl MDM_Policy_Result01_Wifi02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_auto_connect_to_wi_fi_sense_hotspots: None,
            allow_internet_sharing: None,
            allow_manual_wi_fi_configuration: None,
            allow_wi_fi: None,
            allow_wi_fi_direct: None,
            instance_id: None,
            parent_id: None,
            wi_fi_to_wlan: None,
            wlanscan_mode: None,
        }
    }


    /// Sets the value of AllowAutoConnectToWiFiSenseHotspots
    pub fn set_allow_auto_connect_to_wi_fi_sense_hotspots(&mut self, value: i32) {
        self.allow_auto_connect_to_wi_fi_sense_hotspots = Some(value);
    }

    /// Gets the value of AllowAutoConnectToWiFiSenseHotspots
    pub fn get_allow_auto_connect_to_wi_fi_sense_hotspots(&self) -> Option<&i32> {
        self.allow_auto_connect_to_wi_fi_sense_hotspots.as_ref()
    }

    /// Sets the value of AllowInternetSharing
    pub fn set_allow_internet_sharing(&mut self, value: i32) {
        self.allow_internet_sharing = Some(value);
    }

    /// Gets the value of AllowInternetSharing
    pub fn get_allow_internet_sharing(&self) -> Option<&i32> {
        self.allow_internet_sharing.as_ref()
    }

    /// Sets the value of AllowManualWiFiConfiguration
    pub fn set_allow_manual_wi_fi_configuration(&mut self, value: i32) {
        self.allow_manual_wi_fi_configuration = Some(value);
    }

    /// Gets the value of AllowManualWiFiConfiguration
    pub fn get_allow_manual_wi_fi_configuration(&self) -> Option<&i32> {
        self.allow_manual_wi_fi_configuration.as_ref()
    }

    /// Sets the value of AllowWiFi
    pub fn set_allow_wi_fi(&mut self, value: i32) {
        self.allow_wi_fi = Some(value);
    }

    /// Gets the value of AllowWiFi
    pub fn get_allow_wi_fi(&self) -> Option<&i32> {
        self.allow_wi_fi.as_ref()
    }

    /// Sets the value of AllowWiFiDirect
    pub fn set_allow_wi_fi_direct(&mut self, value: i32) {
        self.allow_wi_fi_direct = Some(value);
    }

    /// Gets the value of AllowWiFiDirect
    pub fn get_allow_wi_fi_direct(&self) -> Option<&i32> {
        self.allow_wi_fi_direct.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of WiFiToWlan
    pub fn set_wi_fi_to_wlan(&mut self, value: i32) {
        self.wi_fi_to_wlan = Some(value);
    }

    /// Gets the value of WiFiToWlan
    pub fn get_wi_fi_to_wlan(&self) -> Option<&i32> {
        self.wi_fi_to_wlan.as_ref()
    }

    /// Sets the value of WLANScanMode
    pub fn set_wlanscan_mode(&mut self, value: i32) {
        self.wlanscan_mode = Some(value);
    }

    /// Gets the value of WLANScanMode
    pub fn get_wlanscan_mode(&self) -> Option<&i32> {
        self.wlanscan_mode.as_ref()
    }
}

