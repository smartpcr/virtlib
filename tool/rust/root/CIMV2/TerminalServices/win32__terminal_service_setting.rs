// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TerminalServiceSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TerminalServiceSetting {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "ActiveDesktop")]
    pub active_desktop: Option<u32>,

/// 
    #[serde(rename = "AllowTSConnections")]
    pub allow_tsconnections: Option<u32>,

/// 
    #[serde(rename = "DeleteTempFolders")]
    pub delete_temp_folders: Option<u32>,

/// 
    #[serde(rename = "DirectConnectLicenseServers")]
    pub direct_connect_license_servers: Option<String>,

/// 
    #[serde(rename = "DisableForcibleLogoff")]
    pub disable_forcible_logoff: Option<u32>,

/// 
    #[serde(rename = "EnableAutomaticReconnection")]
    pub enable_automatic_reconnection: Option<u32>,

/// 
    #[serde(rename = "EnableDFSS")]
    pub enable_dfss: Option<u32>,

/// 
    #[serde(rename = "EnableDiskFSS")]
    pub enable_disk_fss: Option<u32>,

/// 
    #[serde(rename = "EnableNetworkFSS")]
    pub enable_network_fss: Option<u32>,

/// 
    #[serde(rename = "EnableRemoteDesktopMSI")]
    pub enable_remote_desktop_msi: Option<u32>,

/// 
    #[serde(rename = "FallbackPrintDriverType")]
    pub fallback_print_driver_type: Option<u32>,

/// 
    #[serde(rename = "GetCapabilitiesID")]
    pub get_capabilities_id: Option<u32>,

/// 
    #[serde(rename = "HomeDirectory")]
    pub home_directory: Option<String>,

/// 
    #[serde(rename = "LicensingDescription")]
    pub licensing_description: Option<String>,

/// 
    #[serde(rename = "LicensingName")]
    pub licensing_name: Option<String>,

/// 
    #[serde(rename = "LicensingType")]
    pub licensing_type: Option<u32>,

/// 
    #[serde(rename = "LimitedUserSessions")]
    pub limited_user_sessions: Option<u32>,

/// 
    #[serde(rename = "Logons")]
    pub logons: Option<String>,

/// 
    #[serde(rename = "NetworkFSSCatchAllWeight")]
    pub network_fsscatch_all_weight: Option<u32>,

/// 
    #[serde(rename = "NetworkFSSLocalSystemWeight")]
    pub network_fsslocal_system_weight: Option<u32>,

/// 
    #[serde(rename = "NetworkFSSUserSessionWeight")]
    pub network_fssuser_session_weight: Option<u32>,

/// 
    #[serde(rename = "PolicySourceAllowTSConnections")]
    pub policy_source_allow_tsconnections: Option<u32>,

/// 
    #[serde(rename = "PolicySourceConfiguredLicenseServers")]
    pub policy_source_configured_license_servers: Option<u32>,

/// 
    #[serde(rename = "PolicySourceDeleteTempFolders")]
    pub policy_source_delete_temp_folders: Option<u32>,

/// 
    #[serde(rename = "PolicySourceDirectConnectLicenseServers")]
    pub policy_source_direct_connect_license_servers: Option<u32>,

/// 
    #[serde(rename = "PolicySourceEnableAutomaticReconnection")]
    pub policy_source_enable_automatic_reconnection: Option<u32>,

/// 
    #[serde(rename = "PolicySourceEnableDFSS")]
    pub policy_source_enable_dfss: Option<u32>,

/// 
    #[serde(rename = "PolicySourceEnableRemoteDesktopMSI")]
    pub policy_source_enable_remote_desktop_msi: Option<u32>,

/// 
    #[serde(rename = "PolicySourceFallbackPrintDriverType")]
    pub policy_source_fallback_print_driver_type: Option<u32>,

/// 
    #[serde(rename = "PolicySourceHomeDirectory")]
    pub policy_source_home_directory: Option<u32>,

/// 
    #[serde(rename = "PolicySourceLicensingType")]
    pub policy_source_licensing_type: Option<u32>,

/// 
    #[serde(rename = "PolicySourceProfilePath")]
    pub policy_source_profile_path: Option<u32>,

/// 
    #[serde(rename = "PolicySourceRedirectSmartCards")]
    pub policy_source_redirect_smart_cards: Option<u32>,

/// 
    #[serde(rename = "PolicySourceSingleSession")]
    pub policy_source_single_session: Option<u32>,

/// 
    #[serde(rename = "PolicySourceTimeZoneRedirection")]
    pub policy_source_time_zone_redirection: Option<u32>,

/// 
    #[serde(rename = "PolicySourceUseRDEasyPrintDriver")]
    pub policy_source_use_rdeasy_print_driver: Option<u32>,

/// 
    #[serde(rename = "PolicySourceUseTempFolders")]
    pub policy_source_use_temp_folders: Option<u32>,

/// 
    #[serde(rename = "PossibleLicensingTypes")]
    pub possible_licensing_types: Option<u32>,

/// 
    #[serde(rename = "ProfilePath")]
    pub profile_path: Option<String>,

/// 
    #[serde(rename = "RedirectSmartCards")]
    pub redirect_smart_cards: Option<u32>,

/// 
    #[serde(rename = "ServerName")]
    pub server_name: Option<String>,

/// 
    #[serde(rename = "SessionBrokerDrainMode")]
    pub session_broker_drain_mode: Option<u32>,

/// 
    #[serde(rename = "SingleSession")]
    pub single_session: Option<u32>,

/// 
    #[serde(rename = "TerminalServerMode")]
    pub terminal_server_mode: Option<u32>,

/// 
    #[serde(rename = "TimeZoneRedirection")]
    pub time_zone_redirection: Option<u32>,

/// 
    #[serde(rename = "UseRDEasyPrintDriver")]
    pub use_rdeasy_print_driver: Option<u32>,

/// 
    #[serde(rename = "UserPermission")]
    pub user_permission: Option<u32>,

/// 
    #[serde(rename = "UseTempFolders")]
    pub use_temp_folders: Option<u32>,
}

impl Win32_TerminalServiceSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            active_desktop: None,
            allow_tsconnections: None,
            delete_temp_folders: None,
            direct_connect_license_servers: None,
            disable_forcible_logoff: None,
            enable_automatic_reconnection: None,
            enable_dfss: None,
            enable_disk_fss: None,
            enable_network_fss: None,
            enable_remote_desktop_msi: None,
            fallback_print_driver_type: None,
            get_capabilities_id: None,
            home_directory: None,
            licensing_description: None,
            licensing_name: None,
            licensing_type: None,
            limited_user_sessions: None,
            logons: None,
            network_fsscatch_all_weight: None,
            network_fsslocal_system_weight: None,
            network_fssuser_session_weight: None,
            policy_source_allow_tsconnections: None,
            policy_source_configured_license_servers: None,
            policy_source_delete_temp_folders: None,
            policy_source_direct_connect_license_servers: None,
            policy_source_enable_automatic_reconnection: None,
            policy_source_enable_dfss: None,
            policy_source_enable_remote_desktop_msi: None,
            policy_source_fallback_print_driver_type: None,
            policy_source_home_directory: None,
            policy_source_licensing_type: None,
            policy_source_profile_path: None,
            policy_source_redirect_smart_cards: None,
            policy_source_single_session: None,
            policy_source_time_zone_redirection: None,
            policy_source_use_rdeasy_print_driver: None,
            policy_source_use_temp_folders: None,
            possible_licensing_types: None,
            profile_path: None,
            redirect_smart_cards: None,
            server_name: None,
            session_broker_drain_mode: None,
            single_session: None,
            terminal_server_mode: None,
            time_zone_redirection: None,
            use_rdeasy_print_driver: None,
            user_permission: None,
            use_temp_folders: None,
        }
    }


    /// Sets the value of ActiveDesktop
    pub fn set_active_desktop(&mut self, value: u32) {
        self.active_desktop = Some(value);
    }

    /// Gets the value of ActiveDesktop
    pub fn get_active_desktop(&self) -> Option<&u32> {
        self.active_desktop.as_ref()
    }

    /// Sets the value of AllowTSConnections
    pub fn set_allow_tsconnections(&mut self, value: u32) {
        self.allow_tsconnections = Some(value);
    }

    /// Gets the value of AllowTSConnections
    pub fn get_allow_tsconnections(&self) -> Option<&u32> {
        self.allow_tsconnections.as_ref()
    }

    /// Sets the value of DeleteTempFolders
    pub fn set_delete_temp_folders(&mut self, value: u32) {
        self.delete_temp_folders = Some(value);
    }

    /// Gets the value of DeleteTempFolders
    pub fn get_delete_temp_folders(&self) -> Option<&u32> {
        self.delete_temp_folders.as_ref()
    }

    /// Sets the value of DirectConnectLicenseServers
    pub fn set_direct_connect_license_servers(&mut self, value: String) {
        self.direct_connect_license_servers = Some(value);
    }

    /// Gets the value of DirectConnectLicenseServers
    pub fn get_direct_connect_license_servers(&self) -> Option<&String> {
        self.direct_connect_license_servers.as_ref()
    }

    /// Sets the value of DisableForcibleLogoff
    pub fn set_disable_forcible_logoff(&mut self, value: u32) {
        self.disable_forcible_logoff = Some(value);
    }

    /// Gets the value of DisableForcibleLogoff
    pub fn get_disable_forcible_logoff(&self) -> Option<&u32> {
        self.disable_forcible_logoff.as_ref()
    }

    /// Sets the value of EnableAutomaticReconnection
    pub fn set_enable_automatic_reconnection(&mut self, value: u32) {
        self.enable_automatic_reconnection = Some(value);
    }

    /// Gets the value of EnableAutomaticReconnection
    pub fn get_enable_automatic_reconnection(&self) -> Option<&u32> {
        self.enable_automatic_reconnection.as_ref()
    }

    /// Sets the value of EnableDFSS
    pub fn set_enable_dfss(&mut self, value: u32) {
        self.enable_dfss = Some(value);
    }

    /// Gets the value of EnableDFSS
    pub fn get_enable_dfss(&self) -> Option<&u32> {
        self.enable_dfss.as_ref()
    }

    /// Sets the value of EnableDiskFSS
    pub fn set_enable_disk_fss(&mut self, value: u32) {
        self.enable_disk_fss = Some(value);
    }

    /// Gets the value of EnableDiskFSS
    pub fn get_enable_disk_fss(&self) -> Option<&u32> {
        self.enable_disk_fss.as_ref()
    }

    /// Sets the value of EnableNetworkFSS
    pub fn set_enable_network_fss(&mut self, value: u32) {
        self.enable_network_fss = Some(value);
    }

    /// Gets the value of EnableNetworkFSS
    pub fn get_enable_network_fss(&self) -> Option<&u32> {
        self.enable_network_fss.as_ref()
    }

    /// Sets the value of EnableRemoteDesktopMSI
    pub fn set_enable_remote_desktop_msi(&mut self, value: u32) {
        self.enable_remote_desktop_msi = Some(value);
    }

    /// Gets the value of EnableRemoteDesktopMSI
    pub fn get_enable_remote_desktop_msi(&self) -> Option<&u32> {
        self.enable_remote_desktop_msi.as_ref()
    }

    /// Sets the value of FallbackPrintDriverType
    pub fn set_fallback_print_driver_type(&mut self, value: u32) {
        self.fallback_print_driver_type = Some(value);
    }

    /// Gets the value of FallbackPrintDriverType
    pub fn get_fallback_print_driver_type(&self) -> Option<&u32> {
        self.fallback_print_driver_type.as_ref()
    }

    /// Sets the value of GetCapabilitiesID
    pub fn set_get_capabilities_id(&mut self, value: u32) {
        self.get_capabilities_id = Some(value);
    }

    /// Gets the value of GetCapabilitiesID
    pub fn get_get_capabilities_id(&self) -> Option<&u32> {
        self.get_capabilities_id.as_ref()
    }

    /// Sets the value of HomeDirectory
    pub fn set_home_directory(&mut self, value: String) {
        self.home_directory = Some(value);
    }

    /// Gets the value of HomeDirectory
    pub fn get_home_directory(&self) -> Option<&String> {
        self.home_directory.as_ref()
    }

    /// Sets the value of LicensingDescription
    pub fn set_licensing_description(&mut self, value: String) {
        self.licensing_description = Some(value);
    }

    /// Gets the value of LicensingDescription
    pub fn get_licensing_description(&self) -> Option<&String> {
        self.licensing_description.as_ref()
    }

    /// Sets the value of LicensingName
    pub fn set_licensing_name(&mut self, value: String) {
        self.licensing_name = Some(value);
    }

    /// Gets the value of LicensingName
    pub fn get_licensing_name(&self) -> Option<&String> {
        self.licensing_name.as_ref()
    }

    /// Sets the value of LicensingType
    pub fn set_licensing_type(&mut self, value: u32) {
        self.licensing_type = Some(value);
    }

    /// Gets the value of LicensingType
    pub fn get_licensing_type(&self) -> Option<&u32> {
        self.licensing_type.as_ref()
    }

    /// Sets the value of LimitedUserSessions
    pub fn set_limited_user_sessions(&mut self, value: u32) {
        self.limited_user_sessions = Some(value);
    }

    /// Gets the value of LimitedUserSessions
    pub fn get_limited_user_sessions(&self) -> Option<&u32> {
        self.limited_user_sessions.as_ref()
    }

    /// Sets the value of Logons
    pub fn set_logons(&mut self, value: String) {
        self.logons = Some(value);
    }

    /// Gets the value of Logons
    pub fn get_logons(&self) -> Option<&String> {
        self.logons.as_ref()
    }

    /// Sets the value of NetworkFSSCatchAllWeight
    pub fn set_network_fsscatch_all_weight(&mut self, value: u32) {
        self.network_fsscatch_all_weight = Some(value);
    }

    /// Gets the value of NetworkFSSCatchAllWeight
    pub fn get_network_fsscatch_all_weight(&self) -> Option<&u32> {
        self.network_fsscatch_all_weight.as_ref()
    }

    /// Sets the value of NetworkFSSLocalSystemWeight
    pub fn set_network_fsslocal_system_weight(&mut self, value: u32) {
        self.network_fsslocal_system_weight = Some(value);
    }

    /// Gets the value of NetworkFSSLocalSystemWeight
    pub fn get_network_fsslocal_system_weight(&self) -> Option<&u32> {
        self.network_fsslocal_system_weight.as_ref()
    }

    /// Sets the value of NetworkFSSUserSessionWeight
    pub fn set_network_fssuser_session_weight(&mut self, value: u32) {
        self.network_fssuser_session_weight = Some(value);
    }

    /// Gets the value of NetworkFSSUserSessionWeight
    pub fn get_network_fssuser_session_weight(&self) -> Option<&u32> {
        self.network_fssuser_session_weight.as_ref()
    }

    /// Sets the value of PolicySourceAllowTSConnections
    pub fn set_policy_source_allow_tsconnections(&mut self, value: u32) {
        self.policy_source_allow_tsconnections = Some(value);
    }

    /// Gets the value of PolicySourceAllowTSConnections
    pub fn get_policy_source_allow_tsconnections(&self) -> Option<&u32> {
        self.policy_source_allow_tsconnections.as_ref()
    }

    /// Sets the value of PolicySourceConfiguredLicenseServers
    pub fn set_policy_source_configured_license_servers(&mut self, value: u32) {
        self.policy_source_configured_license_servers = Some(value);
    }

    /// Gets the value of PolicySourceConfiguredLicenseServers
    pub fn get_policy_source_configured_license_servers(&self) -> Option<&u32> {
        self.policy_source_configured_license_servers.as_ref()
    }

    /// Sets the value of PolicySourceDeleteTempFolders
    pub fn set_policy_source_delete_temp_folders(&mut self, value: u32) {
        self.policy_source_delete_temp_folders = Some(value);
    }

    /// Gets the value of PolicySourceDeleteTempFolders
    pub fn get_policy_source_delete_temp_folders(&self) -> Option<&u32> {
        self.policy_source_delete_temp_folders.as_ref()
    }

    /// Sets the value of PolicySourceDirectConnectLicenseServers
    pub fn set_policy_source_direct_connect_license_servers(&mut self, value: u32) {
        self.policy_source_direct_connect_license_servers = Some(value);
    }

    /// Gets the value of PolicySourceDirectConnectLicenseServers
    pub fn get_policy_source_direct_connect_license_servers(&self) -> Option<&u32> {
        self.policy_source_direct_connect_license_servers.as_ref()
    }

    /// Sets the value of PolicySourceEnableAutomaticReconnection
    pub fn set_policy_source_enable_automatic_reconnection(&mut self, value: u32) {
        self.policy_source_enable_automatic_reconnection = Some(value);
    }

    /// Gets the value of PolicySourceEnableAutomaticReconnection
    pub fn get_policy_source_enable_automatic_reconnection(&self) -> Option<&u32> {
        self.policy_source_enable_automatic_reconnection.as_ref()
    }

    /// Sets the value of PolicySourceEnableDFSS
    pub fn set_policy_source_enable_dfss(&mut self, value: u32) {
        self.policy_source_enable_dfss = Some(value);
    }

    /// Gets the value of PolicySourceEnableDFSS
    pub fn get_policy_source_enable_dfss(&self) -> Option<&u32> {
        self.policy_source_enable_dfss.as_ref()
    }

    /// Sets the value of PolicySourceEnableRemoteDesktopMSI
    pub fn set_policy_source_enable_remote_desktop_msi(&mut self, value: u32) {
        self.policy_source_enable_remote_desktop_msi = Some(value);
    }

    /// Gets the value of PolicySourceEnableRemoteDesktopMSI
    pub fn get_policy_source_enable_remote_desktop_msi(&self) -> Option<&u32> {
        self.policy_source_enable_remote_desktop_msi.as_ref()
    }

    /// Sets the value of PolicySourceFallbackPrintDriverType
    pub fn set_policy_source_fallback_print_driver_type(&mut self, value: u32) {
        self.policy_source_fallback_print_driver_type = Some(value);
    }

    /// Gets the value of PolicySourceFallbackPrintDriverType
    pub fn get_policy_source_fallback_print_driver_type(&self) -> Option<&u32> {
        self.policy_source_fallback_print_driver_type.as_ref()
    }

    /// Sets the value of PolicySourceHomeDirectory
    pub fn set_policy_source_home_directory(&mut self, value: u32) {
        self.policy_source_home_directory = Some(value);
    }

    /// Gets the value of PolicySourceHomeDirectory
    pub fn get_policy_source_home_directory(&self) -> Option<&u32> {
        self.policy_source_home_directory.as_ref()
    }

    /// Sets the value of PolicySourceLicensingType
    pub fn set_policy_source_licensing_type(&mut self, value: u32) {
        self.policy_source_licensing_type = Some(value);
    }

    /// Gets the value of PolicySourceLicensingType
    pub fn get_policy_source_licensing_type(&self) -> Option<&u32> {
        self.policy_source_licensing_type.as_ref()
    }

    /// Sets the value of PolicySourceProfilePath
    pub fn set_policy_source_profile_path(&mut self, value: u32) {
        self.policy_source_profile_path = Some(value);
    }

    /// Gets the value of PolicySourceProfilePath
    pub fn get_policy_source_profile_path(&self) -> Option<&u32> {
        self.policy_source_profile_path.as_ref()
    }

    /// Sets the value of PolicySourceRedirectSmartCards
    pub fn set_policy_source_redirect_smart_cards(&mut self, value: u32) {
        self.policy_source_redirect_smart_cards = Some(value);
    }

    /// Gets the value of PolicySourceRedirectSmartCards
    pub fn get_policy_source_redirect_smart_cards(&self) -> Option<&u32> {
        self.policy_source_redirect_smart_cards.as_ref()
    }

    /// Sets the value of PolicySourceSingleSession
    pub fn set_policy_source_single_session(&mut self, value: u32) {
        self.policy_source_single_session = Some(value);
    }

    /// Gets the value of PolicySourceSingleSession
    pub fn get_policy_source_single_session(&self) -> Option<&u32> {
        self.policy_source_single_session.as_ref()
    }

    /// Sets the value of PolicySourceTimeZoneRedirection
    pub fn set_policy_source_time_zone_redirection(&mut self, value: u32) {
        self.policy_source_time_zone_redirection = Some(value);
    }

    /// Gets the value of PolicySourceTimeZoneRedirection
    pub fn get_policy_source_time_zone_redirection(&self) -> Option<&u32> {
        self.policy_source_time_zone_redirection.as_ref()
    }

    /// Sets the value of PolicySourceUseRDEasyPrintDriver
    pub fn set_policy_source_use_rdeasy_print_driver(&mut self, value: u32) {
        self.policy_source_use_rdeasy_print_driver = Some(value);
    }

    /// Gets the value of PolicySourceUseRDEasyPrintDriver
    pub fn get_policy_source_use_rdeasy_print_driver(&self) -> Option<&u32> {
        self.policy_source_use_rdeasy_print_driver.as_ref()
    }

    /// Sets the value of PolicySourceUseTempFolders
    pub fn set_policy_source_use_temp_folders(&mut self, value: u32) {
        self.policy_source_use_temp_folders = Some(value);
    }

    /// Gets the value of PolicySourceUseTempFolders
    pub fn get_policy_source_use_temp_folders(&self) -> Option<&u32> {
        self.policy_source_use_temp_folders.as_ref()
    }

    /// Sets the value of PossibleLicensingTypes
    pub fn set_possible_licensing_types(&mut self, value: u32) {
        self.possible_licensing_types = Some(value);
    }

    /// Gets the value of PossibleLicensingTypes
    pub fn get_possible_licensing_types(&self) -> Option<&u32> {
        self.possible_licensing_types.as_ref()
    }

    /// Sets the value of ProfilePath
    pub fn set_profile_path(&mut self, value: String) {
        self.profile_path = Some(value);
    }

    /// Gets the value of ProfilePath
    pub fn get_profile_path(&self) -> Option<&String> {
        self.profile_path.as_ref()
    }

    /// Sets the value of RedirectSmartCards
    pub fn set_redirect_smart_cards(&mut self, value: u32) {
        self.redirect_smart_cards = Some(value);
    }

    /// Gets the value of RedirectSmartCards
    pub fn get_redirect_smart_cards(&self) -> Option<&u32> {
        self.redirect_smart_cards.as_ref()
    }

    /// Sets the value of ServerName
    pub fn set_server_name(&mut self, value: String) {
        self.server_name = Some(value);
    }

    /// Gets the value of ServerName
    pub fn get_server_name(&self) -> Option<&String> {
        self.server_name.as_ref()
    }

    /// Sets the value of SessionBrokerDrainMode
    pub fn set_session_broker_drain_mode(&mut self, value: u32) {
        self.session_broker_drain_mode = Some(value);
    }

    /// Gets the value of SessionBrokerDrainMode
    pub fn get_session_broker_drain_mode(&self) -> Option<&u32> {
        self.session_broker_drain_mode.as_ref()
    }

    /// Sets the value of SingleSession
    pub fn set_single_session(&mut self, value: u32) {
        self.single_session = Some(value);
    }

    /// Gets the value of SingleSession
    pub fn get_single_session(&self) -> Option<&u32> {
        self.single_session.as_ref()
    }

    /// Sets the value of TerminalServerMode
    pub fn set_terminal_server_mode(&mut self, value: u32) {
        self.terminal_server_mode = Some(value);
    }

    /// Gets the value of TerminalServerMode
    pub fn get_terminal_server_mode(&self) -> Option<&u32> {
        self.terminal_server_mode.as_ref()
    }

    /// Sets the value of TimeZoneRedirection
    pub fn set_time_zone_redirection(&mut self, value: u32) {
        self.time_zone_redirection = Some(value);
    }

    /// Gets the value of TimeZoneRedirection
    pub fn get_time_zone_redirection(&self) -> Option<&u32> {
        self.time_zone_redirection.as_ref()
    }

    /// Sets the value of UseRDEasyPrintDriver
    pub fn set_use_rdeasy_print_driver(&mut self, value: u32) {
        self.use_rdeasy_print_driver = Some(value);
    }

    /// Gets the value of UseRDEasyPrintDriver
    pub fn get_use_rdeasy_print_driver(&self) -> Option<&u32> {
        self.use_rdeasy_print_driver.as_ref()
    }

    /// Sets the value of UserPermission
    pub fn set_user_permission(&mut self, value: u32) {
        self.user_permission = Some(value);
    }

    /// Gets the value of UserPermission
    pub fn get_user_permission(&self) -> Option<&u32> {
        self.user_permission.as_ref()
    }

    /// Sets the value of UseTempFolders
    pub fn set_use_temp_folders(&mut self, value: u32) {
        self.use_temp_folders = Some(value);
    }

    /// Gets the value of UseTempFolders
    pub fn get_use_temp_folders(&self) -> Option<&u32> {
        self.use_temp_folders.as_ref()
    }

/// 

    /// * `time_zone_redirection` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_time_zone_redirection(&self, time_zone_redirection: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TimeZoneRedirection".to_string(), value: time_zone_redirection.into() });
        self.invoke_method("SetTimeZoneRedirection", &args)

    }


/// 

    /// * `license_server_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn add_direct_connect_license_server(&self, license_server_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "LicenseServerName".to_string(), value: license_server_name.into() });
        self.invoke_method("AddDirectConnectLicenseServer", &args)

    }


/// 

    /// * `license_server_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn delete_direct_connect_license_server(&self, license_server_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "LicenseServerName".to_string(), value: license_server_name.into() });
        self.invoke_method("DeleteDirectConnectLicenseServer", &args)

    }


/// 

    /// * `license_server_list` -  (String)

    /// * `return_value` -  (u32)
    pub fn update_direct_connect_license_server(&self, license_server_list: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "LicenseServerList".to_string(), value: license_server_list.into() });
        self.invoke_method("UpdateDirectConnectLicenseServer", &args)

    }


/// 

    /// * `registered_lslist` -  (String[])
    /// * `return_value` -  (u32)
    pub fn get_registered_license_server_list(&self, registered_lslist: &mut Vec<String>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetRegisteredLicenseServerList", &[])?;
        let registered_lslist = result.get_value("RegisteredLSList")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    /// * `specified_lslist` -  (String[])
    pub fn get_specified_license_server_list(&self, specified_lslist: &mut Vec<String>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetSpecifiedLicenseServerList", &[])?;
        let specified_lslist = result.get_value("SpecifiedLSList")?;
        Ok(result.return_value)

    }


/// 

    /// * `specified_lslist` -  (String[])

    /// * `return_value` -  (u32)
    pub fn set_specified_license_server_list(&self, specified_lslist: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SpecifiedLSList".to_string(), value: specified_lslist.into() });
        self.invoke_method("SetSpecifiedLicenseServerList", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn empty_specified_license_server_list(&self) -> Result<(), WmiError> {
        self.invoke_method("EmptySpecifiedLicenseServerList", &[])

    }


/// 

    /// * `license_server_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_primary_license_server(&self, license_server_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "LicenseServerName".to_string(), value: license_server_name.into() });
        self.invoke_method("SetPrimaryLicenseServer", &args)

    }


/// 

    /// * `license_server_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn add_lsto_specified_license_server_list(&self, license_server_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "LicenseServerName".to_string(), value: license_server_name.into() });
        self.invoke_method("AddLSToSpecifiedLicenseServerList", &args)

    }


/// 

    /// * `license_server_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn remove_lsfrom_specified_license_server_list(&self, license_server_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "LicenseServerName".to_string(), value: license_server_name.into() });
        self.invoke_method("RemoveLSFromSpecifiedLicenseServerList", &args)

    }


/// 

    /// * `licensing_client_id` -  (String)
    /// * `return_value` -  (u32)
    /// * `tenant_aad_token` -  (String)
    /// * `token_length` -  (u32)
    pub fn get_licensing_aad_info(&self, licensing_client_id: &mut String, token_length: &mut u32, tenant_aad_token: &mut String) -> Result<(), WmiError> {

        let result = self.invoke_method("GetLicensingAadInfo", &[])?;
        let licensing_client_id = result.get_value("LicensingClientId")?;
        let tenant_aad_token = result.get_value("TenantAadToken")?;
        let token_length = result.get_value("TokenLength")?;
        Ok(result.return_value)

    }


/// 

    /// * `licensing_client_id` -  (String)
    /// * `tenant_aad_token` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_licensing_aad_info(&self, licensing_client_id: &String, tenant_aad_token: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "LicensingClientId".to_string(), value: licensing_client_id.into() });
        args.push(MethodParameter { name: "TenantAadToken".to_string(), value: tenant_aad_token.into() });
        self.invoke_method("SetLicensingAadInfo", &args)

    }


/// 

    /// * `return_value` -  (u32)
    /// * `version` -  (u32)
    pub fn get_rdsversion(&self, version: &mut u32) -> Result<(), WmiError> {

        let result = self.invoke_method("GetRDSVersion", &[])?;
        let version = result.get_value("Version")?;
        Ok(result.return_value)

    }


/// 

    /// * `licensing_type` -  (u32)

    /// * `return_value` -  (u32)
    pub fn change_mode(&self, licensing_type: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "LicensingType".to_string(), value: licensing_type.into() });
        self.invoke_method("ChangeMode", &args)

    }


/// 

    /// * `allow_tsconnections` -  (u32)
    /// * `modify_firewall_exception` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_allow_tsconnections(&self, allow_tsconnections: u32, modify_firewall_exception: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AllowTSConnections".to_string(), value: allow_tsconnections.into() });
        args.push(MethodParameter { name: "ModifyFirewallException".to_string(), value: modify_firewall_exception.into() });
        self.invoke_method("SetAllowTSConnections", &args)

    }


/// 

    /// * `single_session` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_single_session(&self, single_session: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SingleSession".to_string(), value: single_session.into() });
        self.invoke_method("SetSingleSession", &args)

    }


/// 

    /// * `disable_forcible_logoff` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_disable_forcible_logoff(&self, disable_forcible_logoff: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DisableForcibleLogoff".to_string(), value: disable_forcible_logoff.into() });
        self.invoke_method("SetDisableForcibleLogoff", &args)

    }


/// 

    /// * `profile_path` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_profile_path(&self, profile_path: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ProfilePath".to_string(), value: profile_path.into() });
        self.invoke_method("SetProfilePath", &args)

    }


/// 

    /// * `home_directory` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_home_directory(&self, home_directory: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HomeDirectory".to_string(), value: home_directory.into() });
        self.invoke_method("SetHomeDirectory", &args)

    }


/// 

    /// * `property_name` -  (String)
    /// * `value` -  (bool)

    /// * `return_value` -  (u32)
    pub fn set_policy_property_name(&self, property_name: &String, value: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PropertyName".to_string(), value: property_name.into() });
        args.push(MethodParameter { name: "Value".to_string(), value: value.into() });
        self.invoke_method("SetPolicyPropertyName", &args)

    }


/// 

    /// * `fallback_print_driver_type` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_fallback_print_driver_type(&self, fallback_print_driver_type: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FallbackPrintDriverType".to_string(), value: fallback_print_driver_type.into() });
        self.invoke_method("SetFallbackPrintDriverType", &args)

    }


/// 

    /// * `lana_id_descriptions` -  (String[])
    /// * `lana_id_list` -  (u32[])
    /// * `return_value` -  (u32)
    pub fn get_tslana_ids(&self, lana_id_list: &mut Vec<u32>, lana_id_descriptions: &mut Vec<String>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetTSLanaIds", &[])?;
        let lana_id_descriptions = result.get_value("LanaIdDescriptions")?;
        let lana_id_list = result.get_value("LanaIdList")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    /// * `winsta_driver_names` -  (String[])
    pub fn get_winstation_driver_names(&self, winsta_driver_names: &mut Vec<String>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetWinstationDriverNames", &[])?;
        let winsta_driver_names = result.get_value("WinstaDriverNames")?;
        Ok(result.return_value)

    }


/// 

    /// * `lana_id` -  (u32)
    /// * `name` -  (String)
    /// * `winsta_driver_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn create_winstation(&self, name: &String, winsta_driver_name: &String, lana_id: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "WinstaDriverName".to_string(), value: winsta_driver_name.into() });
        args.push(MethodParameter { name: "LanaId".to_string(), value: lana_id.into() });
        self.invoke_method("CreateWinstation", &args)

    }


/// 

    /// * `server_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn ping_license_server(&self, server_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ServerName".to_string(), value: server_name.into() });
        self.invoke_method("PingLicenseServer", &args)

    }


/// 

    /// * `server_name` -  (String)

    /// * `return_value` -  (u32)
    /// * `tsto_lsconnectivity_status` -  (u32)
    pub fn get_tsto_lsconnectivity_status(&self, server_name: &String, tsto_lsconnectivity_status: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ServerName".to_string(), value: server_name.into() });

        let result = self.invoke_method("GetTStoLSConnectivityStatus", &args)?;
        let tsto_lsconnectivity_status = result.get_value("TStoLSConnectivityStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `server_name` -  (String)

    /// * `access_allowed` -  (u32)
    /// * `return_value` -  (u32)
    pub fn can_access_license_server(&self, server_name: &String, access_allowed: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ServerName".to_string(), value: server_name.into() });

        let result = self.invoke_method("CanAccessLicenseServer", &args)?;
        let access_allowed = result.get_value("AccessAllowed")?;
        Ok(result.return_value)

    }


/// 

    /// * `count` -  (u32)
    /// * `license_servers_list` -  (Win32_TSDiscoveredLicenseServer[])
    /// * `return_value` -  (u32)
    pub fn find_license_servers(&self, license_servers_list: &mut Vec<Win32_TSDiscoveredLicenseServer>, count: &mut u32) -> Result<(), WmiError> {

        let result = self.invoke_method("FindLicenseServers", &[])?;
        let count = result.get_value("Count")?;
        let license_servers_list = result.get_value("LicenseServersList")?;
        Ok(result.return_value)

    }


/// 

    /// * `days_left` -  (u32)
    /// * `return_value` -  (u32)
    pub fn get_grace_period_days(&self, days_left: &mut u32) -> Result<(), WmiError> {

        let result = self.invoke_method("GetGracePeriodDays", &[])?;
        let days_left = result.get_value("DaysLeft")?;
        Ok(result.return_value)

    }


/// 

    /// * `domain` -  (String)
    /// * `return_value` -  (u32)
    pub fn get_domain(&self, domain: &mut String) -> Result<(), WmiError> {

        let result = self.invoke_method("GetDomain", &[])?;
        let domain = result.get_value("Domain")?;
        Ok(result.return_value)

    }

}

