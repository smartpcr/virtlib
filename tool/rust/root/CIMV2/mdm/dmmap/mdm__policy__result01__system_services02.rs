// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_SystemServices02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_SystemServices02 {

/// 
    #[serde(rename = "ConfigureComputerBrowserServiceStartupMode")]
    pub configure_computer_browser_service_startup_mode: Option<i32>,

/// 
    #[serde(rename = "ConfigureHomeGroupListenerServiceStartupMode")]
    pub configure_home_group_listener_service_startup_mode: Option<i32>,

/// 
    #[serde(rename = "ConfigureHomeGroupProviderServiceStartupMode")]
    pub configure_home_group_provider_service_startup_mode: Option<i32>,

/// 
    #[serde(rename = "ConfigureIISAdminServiceStartupMode")]
    pub configure_iisadmin_service_startup_mode: Option<i32>,

/// 
    #[serde(rename = "ConfigureInfraredMonitorServiceStartupMode")]
    pub configure_infrared_monitor_service_startup_mode: Option<i32>,

/// 
    #[serde(rename = "ConfigureInternetConnectionSharingServiceStartupMode")]
    pub configure_internet_connection_sharing_service_startup_mode: Option<i32>,

/// 
    #[serde(rename = "ConfigureLxssManagerServiceStartupMode")]
    pub configure_lxss_manager_service_startup_mode: Option<i32>,

/// 
    #[serde(rename = "ConfigureMicrosoftFTPServiceStartupMode")]
    pub configure_microsoft_ftpservice_startup_mode: Option<i32>,

/// 
    #[serde(rename = "ConfigureRemoteProcedureCallLocatorServiceStartupMode")]
    pub configure_remote_procedure_call_locator_service_startup_mode: Option<i32>,

/// 
    #[serde(rename = "ConfigureRoutingAndRemoteAccessServiceStartupMode")]
    pub configure_routing_and_remote_access_service_startup_mode: Option<i32>,

/// 
    #[serde(rename = "ConfigureSimpleTCPIPServicesStartupMode")]
    pub configure_simple_tcpipservices_startup_mode: Option<i32>,

/// 
    #[serde(rename = "ConfigureSpecialAdministrationConsoleHelperServiceStartupMode")]
    pub configure_special_administration_console_helper_service_startup_mode: Option<i32>,

/// 
    #[serde(rename = "ConfigureSSDPDiscoveryServiceStartupMode")]
    pub configure_ssdpdiscovery_service_startup_mode: Option<i32>,

/// 
    #[serde(rename = "ConfigureUPnPDeviceHostServiceStartupMode")]
    pub configure_upn_pdevice_host_service_startup_mode: Option<i32>,

/// 
    #[serde(rename = "ConfigureWebManagementServiceStartupMode")]
    pub configure_web_management_service_startup_mode: Option<i32>,

/// 
    #[serde(rename = "ConfigureWindowsMediaPlayerNetworkSharingServiceStartupMode")]
    pub configure_windows_media_player_network_sharing_service_startup_mode: Option<i32>,

/// 
    #[serde(rename = "ConfigureWindowsMobileHotspotServiceStartupMode")]
    pub configure_windows_mobile_hotspot_service_startup_mode: Option<i32>,

/// 
    #[serde(rename = "ConfigureWorldWideWebPublishingServiceStartupMode")]
    pub configure_world_wide_web_publishing_service_startup_mode: Option<i32>,

/// 
    #[serde(rename = "ConfigureXboxAccessoryManagementServiceStartupMode")]
    pub configure_xbox_accessory_management_service_startup_mode: Option<i32>,

/// 
    #[serde(rename = "ConfigureXboxLiveAuthManagerServiceStartupMode")]
    pub configure_xbox_live_auth_manager_service_startup_mode: Option<i32>,

/// 
    #[serde(rename = "ConfigureXboxLiveGameSaveServiceStartupMode")]
    pub configure_xbox_live_game_save_service_startup_mode: Option<i32>,

/// 
    #[serde(rename = "ConfigureXboxLiveNetworkingServiceStartupMode")]
    pub configure_xbox_live_networking_service_startup_mode: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_Result01_SystemServices02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            configure_computer_browser_service_startup_mode: None,
            configure_home_group_listener_service_startup_mode: None,
            configure_home_group_provider_service_startup_mode: None,
            configure_iisadmin_service_startup_mode: None,
            configure_infrared_monitor_service_startup_mode: None,
            configure_internet_connection_sharing_service_startup_mode: None,
            configure_lxss_manager_service_startup_mode: None,
            configure_microsoft_ftpservice_startup_mode: None,
            configure_remote_procedure_call_locator_service_startup_mode: None,
            configure_routing_and_remote_access_service_startup_mode: None,
            configure_simple_tcpipservices_startup_mode: None,
            configure_special_administration_console_helper_service_startup_mode: None,
            configure_ssdpdiscovery_service_startup_mode: None,
            configure_upn_pdevice_host_service_startup_mode: None,
            configure_web_management_service_startup_mode: None,
            configure_windows_media_player_network_sharing_service_startup_mode: None,
            configure_windows_mobile_hotspot_service_startup_mode: None,
            configure_world_wide_web_publishing_service_startup_mode: None,
            configure_xbox_accessory_management_service_startup_mode: None,
            configure_xbox_live_auth_manager_service_startup_mode: None,
            configure_xbox_live_game_save_service_startup_mode: None,
            configure_xbox_live_networking_service_startup_mode: None,
            instance_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of ConfigureComputerBrowserServiceStartupMode
    pub fn set_configure_computer_browser_service_startup_mode(&mut self, value: i32) {
        self.configure_computer_browser_service_startup_mode = Some(value);
    }

    /// Gets the value of ConfigureComputerBrowserServiceStartupMode
    pub fn get_configure_computer_browser_service_startup_mode(&self) -> Option<&i32> {
        self.configure_computer_browser_service_startup_mode.as_ref()
    }

    /// Sets the value of ConfigureHomeGroupListenerServiceStartupMode
    pub fn set_configure_home_group_listener_service_startup_mode(&mut self, value: i32) {
        self.configure_home_group_listener_service_startup_mode = Some(value);
    }

    /// Gets the value of ConfigureHomeGroupListenerServiceStartupMode
    pub fn get_configure_home_group_listener_service_startup_mode(&self) -> Option<&i32> {
        self.configure_home_group_listener_service_startup_mode.as_ref()
    }

    /// Sets the value of ConfigureHomeGroupProviderServiceStartupMode
    pub fn set_configure_home_group_provider_service_startup_mode(&mut self, value: i32) {
        self.configure_home_group_provider_service_startup_mode = Some(value);
    }

    /// Gets the value of ConfigureHomeGroupProviderServiceStartupMode
    pub fn get_configure_home_group_provider_service_startup_mode(&self) -> Option<&i32> {
        self.configure_home_group_provider_service_startup_mode.as_ref()
    }

    /// Sets the value of ConfigureIISAdminServiceStartupMode
    pub fn set_configure_iisadmin_service_startup_mode(&mut self, value: i32) {
        self.configure_iisadmin_service_startup_mode = Some(value);
    }

    /// Gets the value of ConfigureIISAdminServiceStartupMode
    pub fn get_configure_iisadmin_service_startup_mode(&self) -> Option<&i32> {
        self.configure_iisadmin_service_startup_mode.as_ref()
    }

    /// Sets the value of ConfigureInfraredMonitorServiceStartupMode
    pub fn set_configure_infrared_monitor_service_startup_mode(&mut self, value: i32) {
        self.configure_infrared_monitor_service_startup_mode = Some(value);
    }

    /// Gets the value of ConfigureInfraredMonitorServiceStartupMode
    pub fn get_configure_infrared_monitor_service_startup_mode(&self) -> Option<&i32> {
        self.configure_infrared_monitor_service_startup_mode.as_ref()
    }

    /// Sets the value of ConfigureInternetConnectionSharingServiceStartupMode
    pub fn set_configure_internet_connection_sharing_service_startup_mode(&mut self, value: i32) {
        self.configure_internet_connection_sharing_service_startup_mode = Some(value);
    }

    /// Gets the value of ConfigureInternetConnectionSharingServiceStartupMode
    pub fn get_configure_internet_connection_sharing_service_startup_mode(&self) -> Option<&i32> {
        self.configure_internet_connection_sharing_service_startup_mode.as_ref()
    }

    /// Sets the value of ConfigureLxssManagerServiceStartupMode
    pub fn set_configure_lxss_manager_service_startup_mode(&mut self, value: i32) {
        self.configure_lxss_manager_service_startup_mode = Some(value);
    }

    /// Gets the value of ConfigureLxssManagerServiceStartupMode
    pub fn get_configure_lxss_manager_service_startup_mode(&self) -> Option<&i32> {
        self.configure_lxss_manager_service_startup_mode.as_ref()
    }

    /// Sets the value of ConfigureMicrosoftFTPServiceStartupMode
    pub fn set_configure_microsoft_ftpservice_startup_mode(&mut self, value: i32) {
        self.configure_microsoft_ftpservice_startup_mode = Some(value);
    }

    /// Gets the value of ConfigureMicrosoftFTPServiceStartupMode
    pub fn get_configure_microsoft_ftpservice_startup_mode(&self) -> Option<&i32> {
        self.configure_microsoft_ftpservice_startup_mode.as_ref()
    }

    /// Sets the value of ConfigureRemoteProcedureCallLocatorServiceStartupMode
    pub fn set_configure_remote_procedure_call_locator_service_startup_mode(&mut self, value: i32) {
        self.configure_remote_procedure_call_locator_service_startup_mode = Some(value);
    }

    /// Gets the value of ConfigureRemoteProcedureCallLocatorServiceStartupMode
    pub fn get_configure_remote_procedure_call_locator_service_startup_mode(&self) -> Option<&i32> {
        self.configure_remote_procedure_call_locator_service_startup_mode.as_ref()
    }

    /// Sets the value of ConfigureRoutingAndRemoteAccessServiceStartupMode
    pub fn set_configure_routing_and_remote_access_service_startup_mode(&mut self, value: i32) {
        self.configure_routing_and_remote_access_service_startup_mode = Some(value);
    }

    /// Gets the value of ConfigureRoutingAndRemoteAccessServiceStartupMode
    pub fn get_configure_routing_and_remote_access_service_startup_mode(&self) -> Option<&i32> {
        self.configure_routing_and_remote_access_service_startup_mode.as_ref()
    }

    /// Sets the value of ConfigureSimpleTCPIPServicesStartupMode
    pub fn set_configure_simple_tcpipservices_startup_mode(&mut self, value: i32) {
        self.configure_simple_tcpipservices_startup_mode = Some(value);
    }

    /// Gets the value of ConfigureSimpleTCPIPServicesStartupMode
    pub fn get_configure_simple_tcpipservices_startup_mode(&self) -> Option<&i32> {
        self.configure_simple_tcpipservices_startup_mode.as_ref()
    }

    /// Sets the value of ConfigureSpecialAdministrationConsoleHelperServiceStartupMode
    pub fn set_configure_special_administration_console_helper_service_startup_mode(&mut self, value: i32) {
        self.configure_special_administration_console_helper_service_startup_mode = Some(value);
    }

    /// Gets the value of ConfigureSpecialAdministrationConsoleHelperServiceStartupMode
    pub fn get_configure_special_administration_console_helper_service_startup_mode(&self) -> Option<&i32> {
        self.configure_special_administration_console_helper_service_startup_mode.as_ref()
    }

    /// Sets the value of ConfigureSSDPDiscoveryServiceStartupMode
    pub fn set_configure_ssdpdiscovery_service_startup_mode(&mut self, value: i32) {
        self.configure_ssdpdiscovery_service_startup_mode = Some(value);
    }

    /// Gets the value of ConfigureSSDPDiscoveryServiceStartupMode
    pub fn get_configure_ssdpdiscovery_service_startup_mode(&self) -> Option<&i32> {
        self.configure_ssdpdiscovery_service_startup_mode.as_ref()
    }

    /// Sets the value of ConfigureUPnPDeviceHostServiceStartupMode
    pub fn set_configure_upn_pdevice_host_service_startup_mode(&mut self, value: i32) {
        self.configure_upn_pdevice_host_service_startup_mode = Some(value);
    }

    /// Gets the value of ConfigureUPnPDeviceHostServiceStartupMode
    pub fn get_configure_upn_pdevice_host_service_startup_mode(&self) -> Option<&i32> {
        self.configure_upn_pdevice_host_service_startup_mode.as_ref()
    }

    /// Sets the value of ConfigureWebManagementServiceStartupMode
    pub fn set_configure_web_management_service_startup_mode(&mut self, value: i32) {
        self.configure_web_management_service_startup_mode = Some(value);
    }

    /// Gets the value of ConfigureWebManagementServiceStartupMode
    pub fn get_configure_web_management_service_startup_mode(&self) -> Option<&i32> {
        self.configure_web_management_service_startup_mode.as_ref()
    }

    /// Sets the value of ConfigureWindowsMediaPlayerNetworkSharingServiceStartupMode
    pub fn set_configure_windows_media_player_network_sharing_service_startup_mode(&mut self, value: i32) {
        self.configure_windows_media_player_network_sharing_service_startup_mode = Some(value);
    }

    /// Gets the value of ConfigureWindowsMediaPlayerNetworkSharingServiceStartupMode
    pub fn get_configure_windows_media_player_network_sharing_service_startup_mode(&self) -> Option<&i32> {
        self.configure_windows_media_player_network_sharing_service_startup_mode.as_ref()
    }

    /// Sets the value of ConfigureWindowsMobileHotspotServiceStartupMode
    pub fn set_configure_windows_mobile_hotspot_service_startup_mode(&mut self, value: i32) {
        self.configure_windows_mobile_hotspot_service_startup_mode = Some(value);
    }

    /// Gets the value of ConfigureWindowsMobileHotspotServiceStartupMode
    pub fn get_configure_windows_mobile_hotspot_service_startup_mode(&self) -> Option<&i32> {
        self.configure_windows_mobile_hotspot_service_startup_mode.as_ref()
    }

    /// Sets the value of ConfigureWorldWideWebPublishingServiceStartupMode
    pub fn set_configure_world_wide_web_publishing_service_startup_mode(&mut self, value: i32) {
        self.configure_world_wide_web_publishing_service_startup_mode = Some(value);
    }

    /// Gets the value of ConfigureWorldWideWebPublishingServiceStartupMode
    pub fn get_configure_world_wide_web_publishing_service_startup_mode(&self) -> Option<&i32> {
        self.configure_world_wide_web_publishing_service_startup_mode.as_ref()
    }

    /// Sets the value of ConfigureXboxAccessoryManagementServiceStartupMode
    pub fn set_configure_xbox_accessory_management_service_startup_mode(&mut self, value: i32) {
        self.configure_xbox_accessory_management_service_startup_mode = Some(value);
    }

    /// Gets the value of ConfigureXboxAccessoryManagementServiceStartupMode
    pub fn get_configure_xbox_accessory_management_service_startup_mode(&self) -> Option<&i32> {
        self.configure_xbox_accessory_management_service_startup_mode.as_ref()
    }

    /// Sets the value of ConfigureXboxLiveAuthManagerServiceStartupMode
    pub fn set_configure_xbox_live_auth_manager_service_startup_mode(&mut self, value: i32) {
        self.configure_xbox_live_auth_manager_service_startup_mode = Some(value);
    }

    /// Gets the value of ConfigureXboxLiveAuthManagerServiceStartupMode
    pub fn get_configure_xbox_live_auth_manager_service_startup_mode(&self) -> Option<&i32> {
        self.configure_xbox_live_auth_manager_service_startup_mode.as_ref()
    }

    /// Sets the value of ConfigureXboxLiveGameSaveServiceStartupMode
    pub fn set_configure_xbox_live_game_save_service_startup_mode(&mut self, value: i32) {
        self.configure_xbox_live_game_save_service_startup_mode = Some(value);
    }

    /// Gets the value of ConfigureXboxLiveGameSaveServiceStartupMode
    pub fn get_configure_xbox_live_game_save_service_startup_mode(&self) -> Option<&i32> {
        self.configure_xbox_live_game_save_service_startup_mode.as_ref()
    }

    /// Sets the value of ConfigureXboxLiveNetworkingServiceStartupMode
    pub fn set_configure_xbox_live_networking_service_startup_mode(&mut self, value: i32) {
        self.configure_xbox_live_networking_service_startup_mode = Some(value);
    }

    /// Gets the value of ConfigureXboxLiveNetworkingServiceStartupMode
    pub fn get_configure_xbox_live_networking_service_startup_mode(&self) -> Option<&i32> {
        self.configure_xbox_live_networking_service_startup_mode.as_ref()
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
}

