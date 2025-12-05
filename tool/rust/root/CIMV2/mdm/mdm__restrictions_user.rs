// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_RestrictionsUser struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_RestrictionsUser {

/// 
    #[serde(rename = "key")]
    pub key: Option<u32>,

/// 
    #[serde(rename = "PCSettingsMeteredNetworkSyncEnabled")]
    pub pcsettings_metered_network_sync_enabled: Option<bool>,

/// 
    #[serde(rename = "PCSettingsPasswordSyncEnabled")]
    pub pcsettings_password_sync_enabled: Option<bool>,

/// 
    #[serde(rename = "PCSettingsSyncEnabled")]
    pub pcsettings_sync_enabled: Option<bool>,
}

impl MDM_RestrictionsUser {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            key: None,
            pcsettings_metered_network_sync_enabled: None,
            pcsettings_password_sync_enabled: None,
            pcsettings_sync_enabled: None,
        }
    }


    /// Sets the value of key
    pub fn set_key(&mut self, value: u32) {
        self.key = Some(value);
    }

    /// Gets the value of key
    pub fn get_key(&self) -> Option<&u32> {
        self.key.as_ref()
    }

    /// Sets the value of PCSettingsMeteredNetworkSyncEnabled
    pub fn set_pcsettings_metered_network_sync_enabled(&mut self, value: bool) {
        self.pcsettings_metered_network_sync_enabled = Some(value);
    }

    /// Gets the value of PCSettingsMeteredNetworkSyncEnabled
    pub fn get_pcsettings_metered_network_sync_enabled(&self) -> Option<&bool> {
        self.pcsettings_metered_network_sync_enabled.as_ref()
    }

    /// Sets the value of PCSettingsPasswordSyncEnabled
    pub fn set_pcsettings_password_sync_enabled(&mut self, value: bool) {
        self.pcsettings_password_sync_enabled = Some(value);
    }

    /// Gets the value of PCSettingsPasswordSyncEnabled
    pub fn get_pcsettings_password_sync_enabled(&self) -> Option<&bool> {
        self.pcsettings_password_sync_enabled.as_ref()
    }

    /// Sets the value of PCSettingsSyncEnabled
    pub fn set_pcsettings_sync_enabled(&mut self, value: bool) {
        self.pcsettings_sync_enabled = Some(value);
    }

    /// Gets the value of PCSettingsSyncEnabled
    pub fn get_pcsettings_sync_enabled(&self) -> Option<&bool> {
        self.pcsettings_sync_enabled.as_ref()
    }
}

