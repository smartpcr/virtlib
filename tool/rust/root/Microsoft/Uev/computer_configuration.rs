// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Uev
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ComputerConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComputerConfiguration {

/// Contact IT Description
    #[serde(rename = "ContactITDescription")]
    pub contact_itdescription: Option<String>,

/// Contact IT URL
    #[serde(rename = "ContactITUrl")]
    pub contact_iturl: Option<String>,

/// Flag for enabling / disabling sync settings for Windows apps
    #[serde(rename = "DontSyncWindows8AppSettings")]
    pub dont_sync_windows8_app_settings: Option<bool>,

/// Flag for enabling / disabling first use notification
    #[serde(rename = "FirstUseNotificationEnabled")]
    pub first_use_notification_enabled: Option<bool>,

/// Max package size (in bytes)
    #[serde(rename = "MaxPackageSizeInBytes")]
    pub max_package_size_in_bytes: Option<u32>,

/// Delay in seconds before displaying the notification on settings import
    #[serde(rename = "SettingsImportNotifyDelayInSeconds")]
    pub settings_import_notify_delay_in_seconds: Option<u32>,

/// Flag for displaying the notification on settings import
    #[serde(rename = "SettingsImportNotifyEnabled")]
    pub settings_import_notify_enabled: Option<bool>,

/// Absolute path to the settings storage path
    #[serde(rename = "SettingsStoragePath")]
    pub settings_storage_path: Option<String>,

/// Absolute path to the settings template catalog path
    #[serde(rename = "SettingsTemplateCatalogPath")]
    pub settings_template_catalog_path: Option<String>,

/// Sync enablement flag
    #[serde(rename = "SyncEnabled")]
    pub sync_enabled: Option<bool>,

/// Synchronization method
    #[serde(rename = "SyncMethod")]
    pub sync_method: Option<String>,

/// Sync over metered network flag
    #[serde(rename = "SyncOverMeteredNetwork")]
    pub sync_over_metered_network: Option<bool>,

/// Sync over metered network when roaming flag
    #[serde(rename = "SyncOverMeteredNetworkWhenRoaming")]
    pub sync_over_metered_network_when_roaming: Option<bool>,

/// Enable ping of the sync provider
    #[serde(rename = "SyncProviderPingEnabled")]
    pub sync_provider_ping_enabled: Option<bool>,

/// Timeout for synchronization from the settings repository (in milliseconds)
    #[serde(rename = "SyncTimeoutInMilliseconds")]
    pub sync_timeout_in_milliseconds: Option<u32>,

/// Flag for enabling / disabling default sync settings for Windows apps
    #[serde(rename = "SyncUnlistedWindows8Apps")]
    pub sync_unlisted_windows8_apps: Option<bool>,

/// Tray icon enablement flag
    #[serde(rename = "TrayIconEnabled")]
    pub tray_icon_enabled: Option<bool>,

/// VDI collection name for current computer
    #[serde(rename = "VdiCollectionName")]
    pub vdi_collection_name: Option<String>,

/// Wait for sync when starting an application
    #[serde(rename = "WaitForSyncOnApplicationStart")]
    pub wait_for_sync_on_application_start: Option<bool>,

/// Wait for sync when logging on
    #[serde(rename = "WaitForSyncOnLogon")]
    pub wait_for_sync_on_logon: Option<bool>,

/// Wait timeout for synchronization from the settings repository (in milliseconds)
    #[serde(rename = "WaitForSyncTimeoutInMilliseconds")]
    pub wait_for_sync_timeout_in_milliseconds: Option<u32>,
}

impl ComputerConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            contact_itdescription: None,
            contact_iturl: None,
            dont_sync_windows8_app_settings: None,
            first_use_notification_enabled: None,
            max_package_size_in_bytes: None,
            settings_import_notify_delay_in_seconds: None,
            settings_import_notify_enabled: None,
            settings_storage_path: None,
            settings_template_catalog_path: None,
            sync_enabled: None,
            sync_method: None,
            sync_over_metered_network: None,
            sync_over_metered_network_when_roaming: None,
            sync_provider_ping_enabled: None,
            sync_timeout_in_milliseconds: None,
            sync_unlisted_windows8_apps: None,
            tray_icon_enabled: None,
            vdi_collection_name: None,
            wait_for_sync_on_application_start: None,
            wait_for_sync_on_logon: None,
            wait_for_sync_timeout_in_milliseconds: None,
        }
    }


    /// Sets the value of ContactITDescription
    pub fn set_contact_itdescription(&mut self, value: String) {
        self.contact_itdescription = Some(value);
    }

    /// Gets the value of ContactITDescription
    pub fn get_contact_itdescription(&self) -> Option<&String> {
        self.contact_itdescription.as_ref()
    }

    /// Sets the value of ContactITUrl
    pub fn set_contact_iturl(&mut self, value: String) {
        self.contact_iturl = Some(value);
    }

    /// Gets the value of ContactITUrl
    pub fn get_contact_iturl(&self) -> Option<&String> {
        self.contact_iturl.as_ref()
    }

    /// Sets the value of DontSyncWindows8AppSettings
    pub fn set_dont_sync_windows8_app_settings(&mut self, value: bool) {
        self.dont_sync_windows8_app_settings = Some(value);
    }

    /// Gets the value of DontSyncWindows8AppSettings
    pub fn get_dont_sync_windows8_app_settings(&self) -> Option<&bool> {
        self.dont_sync_windows8_app_settings.as_ref()
    }

    /// Sets the value of FirstUseNotificationEnabled
    pub fn set_first_use_notification_enabled(&mut self, value: bool) {
        self.first_use_notification_enabled = Some(value);
    }

    /// Gets the value of FirstUseNotificationEnabled
    pub fn get_first_use_notification_enabled(&self) -> Option<&bool> {
        self.first_use_notification_enabled.as_ref()
    }

    /// Sets the value of MaxPackageSizeInBytes
    pub fn set_max_package_size_in_bytes(&mut self, value: u32) {
        self.max_package_size_in_bytes = Some(value);
    }

    /// Gets the value of MaxPackageSizeInBytes
    pub fn get_max_package_size_in_bytes(&self) -> Option<&u32> {
        self.max_package_size_in_bytes.as_ref()
    }

    /// Sets the value of SettingsImportNotifyDelayInSeconds
    pub fn set_settings_import_notify_delay_in_seconds(&mut self, value: u32) {
        self.settings_import_notify_delay_in_seconds = Some(value);
    }

    /// Gets the value of SettingsImportNotifyDelayInSeconds
    pub fn get_settings_import_notify_delay_in_seconds(&self) -> Option<&u32> {
        self.settings_import_notify_delay_in_seconds.as_ref()
    }

    /// Sets the value of SettingsImportNotifyEnabled
    pub fn set_settings_import_notify_enabled(&mut self, value: bool) {
        self.settings_import_notify_enabled = Some(value);
    }

    /// Gets the value of SettingsImportNotifyEnabled
    pub fn get_settings_import_notify_enabled(&self) -> Option<&bool> {
        self.settings_import_notify_enabled.as_ref()
    }

    /// Sets the value of SettingsStoragePath
    pub fn set_settings_storage_path(&mut self, value: String) {
        self.settings_storage_path = Some(value);
    }

    /// Gets the value of SettingsStoragePath
    pub fn get_settings_storage_path(&self) -> Option<&String> {
        self.settings_storage_path.as_ref()
    }

    /// Sets the value of SettingsTemplateCatalogPath
    pub fn set_settings_template_catalog_path(&mut self, value: String) {
        self.settings_template_catalog_path = Some(value);
    }

    /// Gets the value of SettingsTemplateCatalogPath
    pub fn get_settings_template_catalog_path(&self) -> Option<&String> {
        self.settings_template_catalog_path.as_ref()
    }

    /// Sets the value of SyncEnabled
    pub fn set_sync_enabled(&mut self, value: bool) {
        self.sync_enabled = Some(value);
    }

    /// Gets the value of SyncEnabled
    pub fn get_sync_enabled(&self) -> Option<&bool> {
        self.sync_enabled.as_ref()
    }

    /// Sets the value of SyncMethod
    pub fn set_sync_method(&mut self, value: String) {
        self.sync_method = Some(value);
    }

    /// Gets the value of SyncMethod
    pub fn get_sync_method(&self) -> Option<&String> {
        self.sync_method.as_ref()
    }

    /// Sets the value of SyncOverMeteredNetwork
    pub fn set_sync_over_metered_network(&mut self, value: bool) {
        self.sync_over_metered_network = Some(value);
    }

    /// Gets the value of SyncOverMeteredNetwork
    pub fn get_sync_over_metered_network(&self) -> Option<&bool> {
        self.sync_over_metered_network.as_ref()
    }

    /// Sets the value of SyncOverMeteredNetworkWhenRoaming
    pub fn set_sync_over_metered_network_when_roaming(&mut self, value: bool) {
        self.sync_over_metered_network_when_roaming = Some(value);
    }

    /// Gets the value of SyncOverMeteredNetworkWhenRoaming
    pub fn get_sync_over_metered_network_when_roaming(&self) -> Option<&bool> {
        self.sync_over_metered_network_when_roaming.as_ref()
    }

    /// Sets the value of SyncProviderPingEnabled
    pub fn set_sync_provider_ping_enabled(&mut self, value: bool) {
        self.sync_provider_ping_enabled = Some(value);
    }

    /// Gets the value of SyncProviderPingEnabled
    pub fn get_sync_provider_ping_enabled(&self) -> Option<&bool> {
        self.sync_provider_ping_enabled.as_ref()
    }

    /// Sets the value of SyncTimeoutInMilliseconds
    pub fn set_sync_timeout_in_milliseconds(&mut self, value: u32) {
        self.sync_timeout_in_milliseconds = Some(value);
    }

    /// Gets the value of SyncTimeoutInMilliseconds
    pub fn get_sync_timeout_in_milliseconds(&self) -> Option<&u32> {
        self.sync_timeout_in_milliseconds.as_ref()
    }

    /// Sets the value of SyncUnlistedWindows8Apps
    pub fn set_sync_unlisted_windows8_apps(&mut self, value: bool) {
        self.sync_unlisted_windows8_apps = Some(value);
    }

    /// Gets the value of SyncUnlistedWindows8Apps
    pub fn get_sync_unlisted_windows8_apps(&self) -> Option<&bool> {
        self.sync_unlisted_windows8_apps.as_ref()
    }

    /// Sets the value of TrayIconEnabled
    pub fn set_tray_icon_enabled(&mut self, value: bool) {
        self.tray_icon_enabled = Some(value);
    }

    /// Gets the value of TrayIconEnabled
    pub fn get_tray_icon_enabled(&self) -> Option<&bool> {
        self.tray_icon_enabled.as_ref()
    }

    /// Sets the value of VdiCollectionName
    pub fn set_vdi_collection_name(&mut self, value: String) {
        self.vdi_collection_name = Some(value);
    }

    /// Gets the value of VdiCollectionName
    pub fn get_vdi_collection_name(&self) -> Option<&String> {
        self.vdi_collection_name.as_ref()
    }

    /// Sets the value of WaitForSyncOnApplicationStart
    pub fn set_wait_for_sync_on_application_start(&mut self, value: bool) {
        self.wait_for_sync_on_application_start = Some(value);
    }

    /// Gets the value of WaitForSyncOnApplicationStart
    pub fn get_wait_for_sync_on_application_start(&self) -> Option<&bool> {
        self.wait_for_sync_on_application_start.as_ref()
    }

    /// Sets the value of WaitForSyncOnLogon
    pub fn set_wait_for_sync_on_logon(&mut self, value: bool) {
        self.wait_for_sync_on_logon = Some(value);
    }

    /// Gets the value of WaitForSyncOnLogon
    pub fn get_wait_for_sync_on_logon(&self) -> Option<&bool> {
        self.wait_for_sync_on_logon.as_ref()
    }

    /// Sets the value of WaitForSyncTimeoutInMilliseconds
    pub fn set_wait_for_sync_timeout_in_milliseconds(&mut self, value: u32) {
        self.wait_for_sync_timeout_in_milliseconds = Some(value);
    }

    /// Gets the value of WaitForSyncTimeoutInMilliseconds
    pub fn get_wait_for_sync_timeout_in_milliseconds(&self) -> Option<&u32> {
        self.wait_for_sync_timeout_in_milliseconds.as_ref()
    }
}

