// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Restrictions struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Restrictions {

/// 
    #[serde(rename = "BluetoothEnabled")]
    pub bluetooth_enabled: Option<bool>,

/// 
    #[serde(rename = "DataRoamingEnabled")]
    pub data_roaming_enabled: Option<bool>,

/// 
    #[serde(rename = "DiagnosticsSubmissionEnabled")]
    pub diagnostics_submission_enabled: Option<bool>,

/// 
    #[serde(rename = "EcsAutoProvisionEnabled")]
    pub ecs_auto_provision_enabled: Option<bool>,

/// 
    #[serde(rename = "EcsSyncUrl")]
    pub ecs_sync_url: Option<String>,

/// 
    #[serde(rename = "IEEnterpriseModeEnabled")]
    pub ieenterprise_mode_enabled: Option<bool>,

/// 
    #[serde(rename = "IEEnterpriseModeEnabledURL")]
    pub ieenterprise_mode_enabled_url: Option<String>,

/// 
    #[serde(rename = "IEEnterpriseModeSitelist")]
    pub ieenterprise_mode_sitelist: Option<String>,

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

/// 
    #[serde(rename = "SmartScreenEnabled")]
    pub smart_screen_enabled: Option<bool>,

/// 
    #[serde(rename = "UserAccountControlStatus")]
    pub user_account_control_status: Option<u32>,

/// 
    #[serde(rename = "WifiEnabled")]
    pub wifi_enabled: Option<bool>,
}

impl MDM_Restrictions {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            bluetooth_enabled: None,
            data_roaming_enabled: None,
            diagnostics_submission_enabled: None,
            ecs_auto_provision_enabled: None,
            ecs_sync_url: None,
            ieenterprise_mode_enabled: None,
            ieenterprise_mode_enabled_url: None,
            ieenterprise_mode_sitelist: None,
            key: None,
            pcsettings_metered_network_sync_enabled: None,
            pcsettings_password_sync_enabled: None,
            pcsettings_sync_enabled: None,
            smart_screen_enabled: None,
            user_account_control_status: None,
            wifi_enabled: None,
        }
    }


    /// Sets the value of BluetoothEnabled
    pub fn set_bluetooth_enabled(&mut self, value: bool) {
        self.bluetooth_enabled = Some(value);
    }

    /// Gets the value of BluetoothEnabled
    pub fn get_bluetooth_enabled(&self) -> Option<&bool> {
        self.bluetooth_enabled.as_ref()
    }

    /// Sets the value of DataRoamingEnabled
    pub fn set_data_roaming_enabled(&mut self, value: bool) {
        self.data_roaming_enabled = Some(value);
    }

    /// Gets the value of DataRoamingEnabled
    pub fn get_data_roaming_enabled(&self) -> Option<&bool> {
        self.data_roaming_enabled.as_ref()
    }

    /// Sets the value of DiagnosticsSubmissionEnabled
    pub fn set_diagnostics_submission_enabled(&mut self, value: bool) {
        self.diagnostics_submission_enabled = Some(value);
    }

    /// Gets the value of DiagnosticsSubmissionEnabled
    pub fn get_diagnostics_submission_enabled(&self) -> Option<&bool> {
        self.diagnostics_submission_enabled.as_ref()
    }

    /// Sets the value of EcsAutoProvisionEnabled
    pub fn set_ecs_auto_provision_enabled(&mut self, value: bool) {
        self.ecs_auto_provision_enabled = Some(value);
    }

    /// Gets the value of EcsAutoProvisionEnabled
    pub fn get_ecs_auto_provision_enabled(&self) -> Option<&bool> {
        self.ecs_auto_provision_enabled.as_ref()
    }

    /// Sets the value of EcsSyncUrl
    pub fn set_ecs_sync_url(&mut self, value: String) {
        self.ecs_sync_url = Some(value);
    }

    /// Gets the value of EcsSyncUrl
    pub fn get_ecs_sync_url(&self) -> Option<&String> {
        self.ecs_sync_url.as_ref()
    }

    /// Sets the value of IEEnterpriseModeEnabled
    pub fn set_ieenterprise_mode_enabled(&mut self, value: bool) {
        self.ieenterprise_mode_enabled = Some(value);
    }

    /// Gets the value of IEEnterpriseModeEnabled
    pub fn get_ieenterprise_mode_enabled(&self) -> Option<&bool> {
        self.ieenterprise_mode_enabled.as_ref()
    }

    /// Sets the value of IEEnterpriseModeEnabledURL
    pub fn set_ieenterprise_mode_enabled_url(&mut self, value: String) {
        self.ieenterprise_mode_enabled_url = Some(value);
    }

    /// Gets the value of IEEnterpriseModeEnabledURL
    pub fn get_ieenterprise_mode_enabled_url(&self) -> Option<&String> {
        self.ieenterprise_mode_enabled_url.as_ref()
    }

    /// Sets the value of IEEnterpriseModeSitelist
    pub fn set_ieenterprise_mode_sitelist(&mut self, value: String) {
        self.ieenterprise_mode_sitelist = Some(value);
    }

    /// Gets the value of IEEnterpriseModeSitelist
    pub fn get_ieenterprise_mode_sitelist(&self) -> Option<&String> {
        self.ieenterprise_mode_sitelist.as_ref()
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

    /// Sets the value of SmartScreenEnabled
    pub fn set_smart_screen_enabled(&mut self, value: bool) {
        self.smart_screen_enabled = Some(value);
    }

    /// Gets the value of SmartScreenEnabled
    pub fn get_smart_screen_enabled(&self) -> Option<&bool> {
        self.smart_screen_enabled.as_ref()
    }

    /// Sets the value of UserAccountControlStatus
    pub fn set_user_account_control_status(&mut self, value: u32) {
        self.user_account_control_status = Some(value);
    }

    /// Gets the value of UserAccountControlStatus
    pub fn get_user_account_control_status(&self) -> Option<&u32> {
        self.user_account_control_status.as_ref()
    }

    /// Sets the value of WifiEnabled
    pub fn set_wifi_enabled(&mut self, value: bool) {
        self.wifi_enabled = Some(value);
    }

    /// Gets the value of WifiEnabled
    pub fn get_wifi_enabled(&self) -> Option<&bool> {
        self.wifi_enabled.as_ref()
    }
}

