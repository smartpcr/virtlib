// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_Browser02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_Browser02 {

/// 
    #[serde(rename = "AllowAddressBarDropdown")]
    pub allow_address_bar_dropdown: Option<i32>,

/// 
    #[serde(rename = "AllowAutofill")]
    pub allow_autofill: Option<i32>,

/// 
    #[serde(rename = "AllowConfigurationUpdateForBooksLibrary")]
    pub allow_configuration_update_for_books_library: Option<i32>,

/// 
    #[serde(rename = "AllowCookies")]
    pub allow_cookies: Option<i32>,

/// 
    #[serde(rename = "AllowDeveloperTools")]
    pub allow_developer_tools: Option<i32>,

/// 
    #[serde(rename = "AllowDoNotTrack")]
    pub allow_do_not_track: Option<i32>,

/// 
    #[serde(rename = "AllowExtensions")]
    pub allow_extensions: Option<i32>,

/// 
    #[serde(rename = "AllowFlash")]
    pub allow_flash: Option<i32>,

/// 
    #[serde(rename = "AllowFlashClickToRun")]
    pub allow_flash_click_to_run: Option<i32>,

/// 
    #[serde(rename = "AllowFullScreenMode")]
    pub allow_full_screen_mode: Option<i32>,

/// 
    #[serde(rename = "AllowInPrivate")]
    pub allow_in_private: Option<i32>,

/// 
    #[serde(rename = "AllowMicrosoftCompatibilityList")]
    pub allow_microsoft_compatibility_list: Option<i32>,

/// 
    #[serde(rename = "AllowPasswordManager")]
    pub allow_password_manager: Option<i32>,

/// 
    #[serde(rename = "AllowPopups")]
    pub allow_popups: Option<i32>,

/// 
    #[serde(rename = "AllowPrelaunch")]
    pub allow_prelaunch: Option<i32>,

/// 
    #[serde(rename = "AllowPrinting")]
    pub allow_printing: Option<i32>,

/// 
    #[serde(rename = "AllowSavingHistory")]
    pub allow_saving_history: Option<i32>,

/// 
    #[serde(rename = "AllowSearchEngineCustomization")]
    pub allow_search_engine_customization: Option<i32>,

/// 
    #[serde(rename = "AllowSearchSuggestionsinAddressBar")]
    pub allow_search_suggestionsin_address_bar: Option<i32>,

/// 
    #[serde(rename = "AllowSideloadingOfExtensions")]
    pub allow_sideloading_of_extensions: Option<i32>,

/// 
    #[serde(rename = "AllowSmartScreen")]
    pub allow_smart_screen: Option<i32>,

/// 
    #[serde(rename = "AllowTabPreloading")]
    pub allow_tab_preloading: Option<i32>,

/// 
    #[serde(rename = "AllowWebContentOnNewTabPage")]
    pub allow_web_content_on_new_tab_page: Option<i32>,

/// 
    #[serde(rename = "AlwaysEnableBooksLibrary")]
    pub always_enable_books_library: Option<i32>,

/// 
    #[serde(rename = "ClearBrowsingDataOnExit")]
    pub clear_browsing_data_on_exit: Option<i32>,

/// 
    #[serde(rename = "ConfigureAdditionalSearchEngines")]
    pub configure_additional_search_engines: Option<String>,

/// 
    #[serde(rename = "ConfigureFavoritesBar")]
    pub configure_favorites_bar: Option<i32>,

/// 
    #[serde(rename = "ConfigureHomeButton")]
    pub configure_home_button: Option<i32>,

/// 
    #[serde(rename = "ConfigureKioskMode")]
    pub configure_kiosk_mode: Option<i32>,

/// 
    #[serde(rename = "ConfigureKioskResetAfterIdleTimeout")]
    pub configure_kiosk_reset_after_idle_timeout: Option<i32>,

/// 
    #[serde(rename = "ConfigureOpenMicrosoftEdgeWith")]
    pub configure_open_microsoft_edge_with: Option<i32>,

/// 
    #[serde(rename = "ConfigureTelemetryForMicrosoft365Analytics")]
    pub configure_telemetry_for_microsoft365_analytics: Option<i32>,

/// 
    #[serde(rename = "DefaultFavoriteBarItem1Name")]
    pub default_favorite_bar_item1_name: Option<String>,

/// 
    #[serde(rename = "DefaultFavoriteBarItem1Url")]
    pub default_favorite_bar_item1_url: Option<String>,

/// 
    #[serde(rename = "DefaultFavoriteBarItem2Name")]
    pub default_favorite_bar_item2_name: Option<String>,

/// 
    #[serde(rename = "DefaultFavoriteBarItem2Url")]
    pub default_favorite_bar_item2_url: Option<String>,

/// 
    #[serde(rename = "DefaultFavoriteBarItem3Name")]
    pub default_favorite_bar_item3_name: Option<String>,

/// 
    #[serde(rename = "DefaultFavoriteBarItem3Url")]
    pub default_favorite_bar_item3_url: Option<String>,

/// 
    #[serde(rename = "DisableLockdownOfStartPages")]
    pub disable_lockdown_of_start_pages: Option<i32>,

/// 
    #[serde(rename = "EnableExtendedBooksTelemetry")]
    pub enable_extended_books_telemetry: Option<i32>,

/// 
    #[serde(rename = "EnterpriseModeSiteList")]
    pub enterprise_mode_site_list: Option<String>,

/// 
    #[serde(rename = "EnterpriseSiteListServiceUrl")]
    pub enterprise_site_list_service_url: Option<String>,

/// 
    #[serde(rename = "HomePages")]
    pub home_pages: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "LockdownFavorites")]
    pub lockdown_favorites: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PreventAccessToAboutFlagsInMicrosoftEdge")]
    pub prevent_access_to_about_flags_in_microsoft_edge: Option<i32>,

/// 
    #[serde(rename = "PreventCertErrorOverrides")]
    pub prevent_cert_error_overrides: Option<i32>,

/// 
    #[serde(rename = "PreventFirstRunPage")]
    pub prevent_first_run_page: Option<i32>,

/// 
    #[serde(rename = "PreventLiveTileDataCollection")]
    pub prevent_live_tile_data_collection: Option<i32>,

/// 
    #[serde(rename = "PreventSmartScreenPromptOverride")]
    pub prevent_smart_screen_prompt_override: Option<i32>,

/// 
    #[serde(rename = "PreventSmartScreenPromptOverrideForFiles")]
    pub prevent_smart_screen_prompt_override_for_files: Option<i32>,

/// 
    #[serde(rename = "PreventTurningOffRequiredExtensions")]
    pub prevent_turning_off_required_extensions: Option<String>,

/// 
    #[serde(rename = "PreventUsingLocalHostIPAddressForWebRTC")]
    pub prevent_using_local_host_ipaddress_for_web_rtc: Option<i32>,

/// 
    #[serde(rename = "ProvisionFavorites")]
    pub provision_favorites: Option<String>,

/// 
    #[serde(rename = "SendIntranetTraffictoInternetExplorer")]
    pub send_intranet_trafficto_internet_explorer: Option<i32>,

/// 
    #[serde(rename = "SetDefaultSearchEngine")]
    pub set_default_search_engine: Option<String>,

/// 
    #[serde(rename = "SetHomeButtonURL")]
    pub set_home_button_url: Option<String>,

/// 
    #[serde(rename = "SetNewTabPageURL")]
    pub set_new_tab_page_url: Option<String>,

/// 
    #[serde(rename = "ShowMessageWhenOpeningSitesInInternetExplorer")]
    pub show_message_when_opening_sites_in_internet_explorer: Option<i32>,

/// 
    #[serde(rename = "SyncFavoritesBetweenIEAndMicrosoftEdge")]
    pub sync_favorites_between_ieand_microsoft_edge: Option<i32>,

/// 
    #[serde(rename = "UnlockHomeButton")]
    pub unlock_home_button: Option<i32>,

/// 
    #[serde(rename = "UseSharedFolderForBooks")]
    pub use_shared_folder_for_books: Option<i32>,
}

impl MDM_Policy_Config01_Browser02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_address_bar_dropdown: None,
            allow_autofill: None,
            allow_configuration_update_for_books_library: None,
            allow_cookies: None,
            allow_developer_tools: None,
            allow_do_not_track: None,
            allow_extensions: None,
            allow_flash: None,
            allow_flash_click_to_run: None,
            allow_full_screen_mode: None,
            allow_in_private: None,
            allow_microsoft_compatibility_list: None,
            allow_password_manager: None,
            allow_popups: None,
            allow_prelaunch: None,
            allow_printing: None,
            allow_saving_history: None,
            allow_search_engine_customization: None,
            allow_search_suggestionsin_address_bar: None,
            allow_sideloading_of_extensions: None,
            allow_smart_screen: None,
            allow_tab_preloading: None,
            allow_web_content_on_new_tab_page: None,
            always_enable_books_library: None,
            clear_browsing_data_on_exit: None,
            configure_additional_search_engines: None,
            configure_favorites_bar: None,
            configure_home_button: None,
            configure_kiosk_mode: None,
            configure_kiosk_reset_after_idle_timeout: None,
            configure_open_microsoft_edge_with: None,
            configure_telemetry_for_microsoft365_analytics: None,
            default_favorite_bar_item1_name: None,
            default_favorite_bar_item1_url: None,
            default_favorite_bar_item2_name: None,
            default_favorite_bar_item2_url: None,
            default_favorite_bar_item3_name: None,
            default_favorite_bar_item3_url: None,
            disable_lockdown_of_start_pages: None,
            enable_extended_books_telemetry: None,
            enterprise_mode_site_list: None,
            enterprise_site_list_service_url: None,
            home_pages: None,
            instance_id: None,
            lockdown_favorites: None,
            parent_id: None,
            prevent_access_to_about_flags_in_microsoft_edge: None,
            prevent_cert_error_overrides: None,
            prevent_first_run_page: None,
            prevent_live_tile_data_collection: None,
            prevent_smart_screen_prompt_override: None,
            prevent_smart_screen_prompt_override_for_files: None,
            prevent_turning_off_required_extensions: None,
            prevent_using_local_host_ipaddress_for_web_rtc: None,
            provision_favorites: None,
            send_intranet_trafficto_internet_explorer: None,
            set_default_search_engine: None,
            set_home_button_url: None,
            set_new_tab_page_url: None,
            show_message_when_opening_sites_in_internet_explorer: None,
            sync_favorites_between_ieand_microsoft_edge: None,
            unlock_home_button: None,
            use_shared_folder_for_books: None,
        }
    }


    /// Sets the value of AllowAddressBarDropdown
    pub fn set_allow_address_bar_dropdown(&mut self, value: i32) {
        self.allow_address_bar_dropdown = Some(value);
    }

    /// Gets the value of AllowAddressBarDropdown
    pub fn get_allow_address_bar_dropdown(&self) -> Option<&i32> {
        self.allow_address_bar_dropdown.as_ref()
    }

    /// Sets the value of AllowAutofill
    pub fn set_allow_autofill(&mut self, value: i32) {
        self.allow_autofill = Some(value);
    }

    /// Gets the value of AllowAutofill
    pub fn get_allow_autofill(&self) -> Option<&i32> {
        self.allow_autofill.as_ref()
    }

    /// Sets the value of AllowConfigurationUpdateForBooksLibrary
    pub fn set_allow_configuration_update_for_books_library(&mut self, value: i32) {
        self.allow_configuration_update_for_books_library = Some(value);
    }

    /// Gets the value of AllowConfigurationUpdateForBooksLibrary
    pub fn get_allow_configuration_update_for_books_library(&self) -> Option<&i32> {
        self.allow_configuration_update_for_books_library.as_ref()
    }

    /// Sets the value of AllowCookies
    pub fn set_allow_cookies(&mut self, value: i32) {
        self.allow_cookies = Some(value);
    }

    /// Gets the value of AllowCookies
    pub fn get_allow_cookies(&self) -> Option<&i32> {
        self.allow_cookies.as_ref()
    }

    /// Sets the value of AllowDeveloperTools
    pub fn set_allow_developer_tools(&mut self, value: i32) {
        self.allow_developer_tools = Some(value);
    }

    /// Gets the value of AllowDeveloperTools
    pub fn get_allow_developer_tools(&self) -> Option<&i32> {
        self.allow_developer_tools.as_ref()
    }

    /// Sets the value of AllowDoNotTrack
    pub fn set_allow_do_not_track(&mut self, value: i32) {
        self.allow_do_not_track = Some(value);
    }

    /// Gets the value of AllowDoNotTrack
    pub fn get_allow_do_not_track(&self) -> Option<&i32> {
        self.allow_do_not_track.as_ref()
    }

    /// Sets the value of AllowExtensions
    pub fn set_allow_extensions(&mut self, value: i32) {
        self.allow_extensions = Some(value);
    }

    /// Gets the value of AllowExtensions
    pub fn get_allow_extensions(&self) -> Option<&i32> {
        self.allow_extensions.as_ref()
    }

    /// Sets the value of AllowFlash
    pub fn set_allow_flash(&mut self, value: i32) {
        self.allow_flash = Some(value);
    }

    /// Gets the value of AllowFlash
    pub fn get_allow_flash(&self) -> Option<&i32> {
        self.allow_flash.as_ref()
    }

    /// Sets the value of AllowFlashClickToRun
    pub fn set_allow_flash_click_to_run(&mut self, value: i32) {
        self.allow_flash_click_to_run = Some(value);
    }

    /// Gets the value of AllowFlashClickToRun
    pub fn get_allow_flash_click_to_run(&self) -> Option<&i32> {
        self.allow_flash_click_to_run.as_ref()
    }

    /// Sets the value of AllowFullScreenMode
    pub fn set_allow_full_screen_mode(&mut self, value: i32) {
        self.allow_full_screen_mode = Some(value);
    }

    /// Gets the value of AllowFullScreenMode
    pub fn get_allow_full_screen_mode(&self) -> Option<&i32> {
        self.allow_full_screen_mode.as_ref()
    }

    /// Sets the value of AllowInPrivate
    pub fn set_allow_in_private(&mut self, value: i32) {
        self.allow_in_private = Some(value);
    }

    /// Gets the value of AllowInPrivate
    pub fn get_allow_in_private(&self) -> Option<&i32> {
        self.allow_in_private.as_ref()
    }

    /// Sets the value of AllowMicrosoftCompatibilityList
    pub fn set_allow_microsoft_compatibility_list(&mut self, value: i32) {
        self.allow_microsoft_compatibility_list = Some(value);
    }

    /// Gets the value of AllowMicrosoftCompatibilityList
    pub fn get_allow_microsoft_compatibility_list(&self) -> Option<&i32> {
        self.allow_microsoft_compatibility_list.as_ref()
    }

    /// Sets the value of AllowPasswordManager
    pub fn set_allow_password_manager(&mut self, value: i32) {
        self.allow_password_manager = Some(value);
    }

    /// Gets the value of AllowPasswordManager
    pub fn get_allow_password_manager(&self) -> Option<&i32> {
        self.allow_password_manager.as_ref()
    }

    /// Sets the value of AllowPopups
    pub fn set_allow_popups(&mut self, value: i32) {
        self.allow_popups = Some(value);
    }

    /// Gets the value of AllowPopups
    pub fn get_allow_popups(&self) -> Option<&i32> {
        self.allow_popups.as_ref()
    }

    /// Sets the value of AllowPrelaunch
    pub fn set_allow_prelaunch(&mut self, value: i32) {
        self.allow_prelaunch = Some(value);
    }

    /// Gets the value of AllowPrelaunch
    pub fn get_allow_prelaunch(&self) -> Option<&i32> {
        self.allow_prelaunch.as_ref()
    }

    /// Sets the value of AllowPrinting
    pub fn set_allow_printing(&mut self, value: i32) {
        self.allow_printing = Some(value);
    }

    /// Gets the value of AllowPrinting
    pub fn get_allow_printing(&self) -> Option<&i32> {
        self.allow_printing.as_ref()
    }

    /// Sets the value of AllowSavingHistory
    pub fn set_allow_saving_history(&mut self, value: i32) {
        self.allow_saving_history = Some(value);
    }

    /// Gets the value of AllowSavingHistory
    pub fn get_allow_saving_history(&self) -> Option<&i32> {
        self.allow_saving_history.as_ref()
    }

    /// Sets the value of AllowSearchEngineCustomization
    pub fn set_allow_search_engine_customization(&mut self, value: i32) {
        self.allow_search_engine_customization = Some(value);
    }

    /// Gets the value of AllowSearchEngineCustomization
    pub fn get_allow_search_engine_customization(&self) -> Option<&i32> {
        self.allow_search_engine_customization.as_ref()
    }

    /// Sets the value of AllowSearchSuggestionsinAddressBar
    pub fn set_allow_search_suggestionsin_address_bar(&mut self, value: i32) {
        self.allow_search_suggestionsin_address_bar = Some(value);
    }

    /// Gets the value of AllowSearchSuggestionsinAddressBar
    pub fn get_allow_search_suggestionsin_address_bar(&self) -> Option<&i32> {
        self.allow_search_suggestionsin_address_bar.as_ref()
    }

    /// Sets the value of AllowSideloadingOfExtensions
    pub fn set_allow_sideloading_of_extensions(&mut self, value: i32) {
        self.allow_sideloading_of_extensions = Some(value);
    }

    /// Gets the value of AllowSideloadingOfExtensions
    pub fn get_allow_sideloading_of_extensions(&self) -> Option<&i32> {
        self.allow_sideloading_of_extensions.as_ref()
    }

    /// Sets the value of AllowSmartScreen
    pub fn set_allow_smart_screen(&mut self, value: i32) {
        self.allow_smart_screen = Some(value);
    }

    /// Gets the value of AllowSmartScreen
    pub fn get_allow_smart_screen(&self) -> Option<&i32> {
        self.allow_smart_screen.as_ref()
    }

    /// Sets the value of AllowTabPreloading
    pub fn set_allow_tab_preloading(&mut self, value: i32) {
        self.allow_tab_preloading = Some(value);
    }

    /// Gets the value of AllowTabPreloading
    pub fn get_allow_tab_preloading(&self) -> Option<&i32> {
        self.allow_tab_preloading.as_ref()
    }

    /// Sets the value of AllowWebContentOnNewTabPage
    pub fn set_allow_web_content_on_new_tab_page(&mut self, value: i32) {
        self.allow_web_content_on_new_tab_page = Some(value);
    }

    /// Gets the value of AllowWebContentOnNewTabPage
    pub fn get_allow_web_content_on_new_tab_page(&self) -> Option<&i32> {
        self.allow_web_content_on_new_tab_page.as_ref()
    }

    /// Sets the value of AlwaysEnableBooksLibrary
    pub fn set_always_enable_books_library(&mut self, value: i32) {
        self.always_enable_books_library = Some(value);
    }

    /// Gets the value of AlwaysEnableBooksLibrary
    pub fn get_always_enable_books_library(&self) -> Option<&i32> {
        self.always_enable_books_library.as_ref()
    }

    /// Sets the value of ClearBrowsingDataOnExit
    pub fn set_clear_browsing_data_on_exit(&mut self, value: i32) {
        self.clear_browsing_data_on_exit = Some(value);
    }

    /// Gets the value of ClearBrowsingDataOnExit
    pub fn get_clear_browsing_data_on_exit(&self) -> Option<&i32> {
        self.clear_browsing_data_on_exit.as_ref()
    }

    /// Sets the value of ConfigureAdditionalSearchEngines
    pub fn set_configure_additional_search_engines(&mut self, value: String) {
        self.configure_additional_search_engines = Some(value);
    }

    /// Gets the value of ConfigureAdditionalSearchEngines
    pub fn get_configure_additional_search_engines(&self) -> Option<&String> {
        self.configure_additional_search_engines.as_ref()
    }

    /// Sets the value of ConfigureFavoritesBar
    pub fn set_configure_favorites_bar(&mut self, value: i32) {
        self.configure_favorites_bar = Some(value);
    }

    /// Gets the value of ConfigureFavoritesBar
    pub fn get_configure_favorites_bar(&self) -> Option<&i32> {
        self.configure_favorites_bar.as_ref()
    }

    /// Sets the value of ConfigureHomeButton
    pub fn set_configure_home_button(&mut self, value: i32) {
        self.configure_home_button = Some(value);
    }

    /// Gets the value of ConfigureHomeButton
    pub fn get_configure_home_button(&self) -> Option<&i32> {
        self.configure_home_button.as_ref()
    }

    /// Sets the value of ConfigureKioskMode
    pub fn set_configure_kiosk_mode(&mut self, value: i32) {
        self.configure_kiosk_mode = Some(value);
    }

    /// Gets the value of ConfigureKioskMode
    pub fn get_configure_kiosk_mode(&self) -> Option<&i32> {
        self.configure_kiosk_mode.as_ref()
    }

    /// Sets the value of ConfigureKioskResetAfterIdleTimeout
    pub fn set_configure_kiosk_reset_after_idle_timeout(&mut self, value: i32) {
        self.configure_kiosk_reset_after_idle_timeout = Some(value);
    }

    /// Gets the value of ConfigureKioskResetAfterIdleTimeout
    pub fn get_configure_kiosk_reset_after_idle_timeout(&self) -> Option<&i32> {
        self.configure_kiosk_reset_after_idle_timeout.as_ref()
    }

    /// Sets the value of ConfigureOpenMicrosoftEdgeWith
    pub fn set_configure_open_microsoft_edge_with(&mut self, value: i32) {
        self.configure_open_microsoft_edge_with = Some(value);
    }

    /// Gets the value of ConfigureOpenMicrosoftEdgeWith
    pub fn get_configure_open_microsoft_edge_with(&self) -> Option<&i32> {
        self.configure_open_microsoft_edge_with.as_ref()
    }

    /// Sets the value of ConfigureTelemetryForMicrosoft365Analytics
    pub fn set_configure_telemetry_for_microsoft365_analytics(&mut self, value: i32) {
        self.configure_telemetry_for_microsoft365_analytics = Some(value);
    }

    /// Gets the value of ConfigureTelemetryForMicrosoft365Analytics
    pub fn get_configure_telemetry_for_microsoft365_analytics(&self) -> Option<&i32> {
        self.configure_telemetry_for_microsoft365_analytics.as_ref()
    }

    /// Sets the value of DefaultFavoriteBarItem1Name
    pub fn set_default_favorite_bar_item1_name(&mut self, value: String) {
        self.default_favorite_bar_item1_name = Some(value);
    }

    /// Gets the value of DefaultFavoriteBarItem1Name
    pub fn get_default_favorite_bar_item1_name(&self) -> Option<&String> {
        self.default_favorite_bar_item1_name.as_ref()
    }

    /// Sets the value of DefaultFavoriteBarItem1Url
    pub fn set_default_favorite_bar_item1_url(&mut self, value: String) {
        self.default_favorite_bar_item1_url = Some(value);
    }

    /// Gets the value of DefaultFavoriteBarItem1Url
    pub fn get_default_favorite_bar_item1_url(&self) -> Option<&String> {
        self.default_favorite_bar_item1_url.as_ref()
    }

    /// Sets the value of DefaultFavoriteBarItem2Name
    pub fn set_default_favorite_bar_item2_name(&mut self, value: String) {
        self.default_favorite_bar_item2_name = Some(value);
    }

    /// Gets the value of DefaultFavoriteBarItem2Name
    pub fn get_default_favorite_bar_item2_name(&self) -> Option<&String> {
        self.default_favorite_bar_item2_name.as_ref()
    }

    /// Sets the value of DefaultFavoriteBarItem2Url
    pub fn set_default_favorite_bar_item2_url(&mut self, value: String) {
        self.default_favorite_bar_item2_url = Some(value);
    }

    /// Gets the value of DefaultFavoriteBarItem2Url
    pub fn get_default_favorite_bar_item2_url(&self) -> Option<&String> {
        self.default_favorite_bar_item2_url.as_ref()
    }

    /// Sets the value of DefaultFavoriteBarItem3Name
    pub fn set_default_favorite_bar_item3_name(&mut self, value: String) {
        self.default_favorite_bar_item3_name = Some(value);
    }

    /// Gets the value of DefaultFavoriteBarItem3Name
    pub fn get_default_favorite_bar_item3_name(&self) -> Option<&String> {
        self.default_favorite_bar_item3_name.as_ref()
    }

    /// Sets the value of DefaultFavoriteBarItem3Url
    pub fn set_default_favorite_bar_item3_url(&mut self, value: String) {
        self.default_favorite_bar_item3_url = Some(value);
    }

    /// Gets the value of DefaultFavoriteBarItem3Url
    pub fn get_default_favorite_bar_item3_url(&self) -> Option<&String> {
        self.default_favorite_bar_item3_url.as_ref()
    }

    /// Sets the value of DisableLockdownOfStartPages
    pub fn set_disable_lockdown_of_start_pages(&mut self, value: i32) {
        self.disable_lockdown_of_start_pages = Some(value);
    }

    /// Gets the value of DisableLockdownOfStartPages
    pub fn get_disable_lockdown_of_start_pages(&self) -> Option<&i32> {
        self.disable_lockdown_of_start_pages.as_ref()
    }

    /// Sets the value of EnableExtendedBooksTelemetry
    pub fn set_enable_extended_books_telemetry(&mut self, value: i32) {
        self.enable_extended_books_telemetry = Some(value);
    }

    /// Gets the value of EnableExtendedBooksTelemetry
    pub fn get_enable_extended_books_telemetry(&self) -> Option<&i32> {
        self.enable_extended_books_telemetry.as_ref()
    }

    /// Sets the value of EnterpriseModeSiteList
    pub fn set_enterprise_mode_site_list(&mut self, value: String) {
        self.enterprise_mode_site_list = Some(value);
    }

    /// Gets the value of EnterpriseModeSiteList
    pub fn get_enterprise_mode_site_list(&self) -> Option<&String> {
        self.enterprise_mode_site_list.as_ref()
    }

    /// Sets the value of EnterpriseSiteListServiceUrl
    pub fn set_enterprise_site_list_service_url(&mut self, value: String) {
        self.enterprise_site_list_service_url = Some(value);
    }

    /// Gets the value of EnterpriseSiteListServiceUrl
    pub fn get_enterprise_site_list_service_url(&self) -> Option<&String> {
        self.enterprise_site_list_service_url.as_ref()
    }

    /// Sets the value of HomePages
    pub fn set_home_pages(&mut self, value: String) {
        self.home_pages = Some(value);
    }

    /// Gets the value of HomePages
    pub fn get_home_pages(&self) -> Option<&String> {
        self.home_pages.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of LockdownFavorites
    pub fn set_lockdown_favorites(&mut self, value: i32) {
        self.lockdown_favorites = Some(value);
    }

    /// Gets the value of LockdownFavorites
    pub fn get_lockdown_favorites(&self) -> Option<&i32> {
        self.lockdown_favorites.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PreventAccessToAboutFlagsInMicrosoftEdge
    pub fn set_prevent_access_to_about_flags_in_microsoft_edge(&mut self, value: i32) {
        self.prevent_access_to_about_flags_in_microsoft_edge = Some(value);
    }

    /// Gets the value of PreventAccessToAboutFlagsInMicrosoftEdge
    pub fn get_prevent_access_to_about_flags_in_microsoft_edge(&self) -> Option<&i32> {
        self.prevent_access_to_about_flags_in_microsoft_edge.as_ref()
    }

    /// Sets the value of PreventCertErrorOverrides
    pub fn set_prevent_cert_error_overrides(&mut self, value: i32) {
        self.prevent_cert_error_overrides = Some(value);
    }

    /// Gets the value of PreventCertErrorOverrides
    pub fn get_prevent_cert_error_overrides(&self) -> Option<&i32> {
        self.prevent_cert_error_overrides.as_ref()
    }

    /// Sets the value of PreventFirstRunPage
    pub fn set_prevent_first_run_page(&mut self, value: i32) {
        self.prevent_first_run_page = Some(value);
    }

    /// Gets the value of PreventFirstRunPage
    pub fn get_prevent_first_run_page(&self) -> Option<&i32> {
        self.prevent_first_run_page.as_ref()
    }

    /// Sets the value of PreventLiveTileDataCollection
    pub fn set_prevent_live_tile_data_collection(&mut self, value: i32) {
        self.prevent_live_tile_data_collection = Some(value);
    }

    /// Gets the value of PreventLiveTileDataCollection
    pub fn get_prevent_live_tile_data_collection(&self) -> Option<&i32> {
        self.prevent_live_tile_data_collection.as_ref()
    }

    /// Sets the value of PreventSmartScreenPromptOverride
    pub fn set_prevent_smart_screen_prompt_override(&mut self, value: i32) {
        self.prevent_smart_screen_prompt_override = Some(value);
    }

    /// Gets the value of PreventSmartScreenPromptOverride
    pub fn get_prevent_smart_screen_prompt_override(&self) -> Option<&i32> {
        self.prevent_smart_screen_prompt_override.as_ref()
    }

    /// Sets the value of PreventSmartScreenPromptOverrideForFiles
    pub fn set_prevent_smart_screen_prompt_override_for_files(&mut self, value: i32) {
        self.prevent_smart_screen_prompt_override_for_files = Some(value);
    }

    /// Gets the value of PreventSmartScreenPromptOverrideForFiles
    pub fn get_prevent_smart_screen_prompt_override_for_files(&self) -> Option<&i32> {
        self.prevent_smart_screen_prompt_override_for_files.as_ref()
    }

    /// Sets the value of PreventTurningOffRequiredExtensions
    pub fn set_prevent_turning_off_required_extensions(&mut self, value: String) {
        self.prevent_turning_off_required_extensions = Some(value);
    }

    /// Gets the value of PreventTurningOffRequiredExtensions
    pub fn get_prevent_turning_off_required_extensions(&self) -> Option<&String> {
        self.prevent_turning_off_required_extensions.as_ref()
    }

    /// Sets the value of PreventUsingLocalHostIPAddressForWebRTC
    pub fn set_prevent_using_local_host_ipaddress_for_web_rtc(&mut self, value: i32) {
        self.prevent_using_local_host_ipaddress_for_web_rtc = Some(value);
    }

    /// Gets the value of PreventUsingLocalHostIPAddressForWebRTC
    pub fn get_prevent_using_local_host_ipaddress_for_web_rtc(&self) -> Option<&i32> {
        self.prevent_using_local_host_ipaddress_for_web_rtc.as_ref()
    }

    /// Sets the value of ProvisionFavorites
    pub fn set_provision_favorites(&mut self, value: String) {
        self.provision_favorites = Some(value);
    }

    /// Gets the value of ProvisionFavorites
    pub fn get_provision_favorites(&self) -> Option<&String> {
        self.provision_favorites.as_ref()
    }

    /// Sets the value of SendIntranetTraffictoInternetExplorer
    pub fn set_send_intranet_trafficto_internet_explorer(&mut self, value: i32) {
        self.send_intranet_trafficto_internet_explorer = Some(value);
    }

    /// Gets the value of SendIntranetTraffictoInternetExplorer
    pub fn get_send_intranet_trafficto_internet_explorer(&self) -> Option<&i32> {
        self.send_intranet_trafficto_internet_explorer.as_ref()
    }

    /// Sets the value of SetDefaultSearchEngine
    pub fn set_set_default_search_engine(&mut self, value: String) {
        self.set_default_search_engine = Some(value);
    }

    /// Gets the value of SetDefaultSearchEngine
    pub fn get_set_default_search_engine(&self) -> Option<&String> {
        self.set_default_search_engine.as_ref()
    }

    /// Sets the value of SetHomeButtonURL
    pub fn set_set_home_button_url(&mut self, value: String) {
        self.set_home_button_url = Some(value);
    }

    /// Gets the value of SetHomeButtonURL
    pub fn get_set_home_button_url(&self) -> Option<&String> {
        self.set_home_button_url.as_ref()
    }

    /// Sets the value of SetNewTabPageURL
    pub fn set_set_new_tab_page_url(&mut self, value: String) {
        self.set_new_tab_page_url = Some(value);
    }

    /// Gets the value of SetNewTabPageURL
    pub fn get_set_new_tab_page_url(&self) -> Option<&String> {
        self.set_new_tab_page_url.as_ref()
    }

    /// Sets the value of ShowMessageWhenOpeningSitesInInternetExplorer
    pub fn set_show_message_when_opening_sites_in_internet_explorer(&mut self, value: i32) {
        self.show_message_when_opening_sites_in_internet_explorer = Some(value);
    }

    /// Gets the value of ShowMessageWhenOpeningSitesInInternetExplorer
    pub fn get_show_message_when_opening_sites_in_internet_explorer(&self) -> Option<&i32> {
        self.show_message_when_opening_sites_in_internet_explorer.as_ref()
    }

    /// Sets the value of SyncFavoritesBetweenIEAndMicrosoftEdge
    pub fn set_sync_favorites_between_ieand_microsoft_edge(&mut self, value: i32) {
        self.sync_favorites_between_ieand_microsoft_edge = Some(value);
    }

    /// Gets the value of SyncFavoritesBetweenIEAndMicrosoftEdge
    pub fn get_sync_favorites_between_ieand_microsoft_edge(&self) -> Option<&i32> {
        self.sync_favorites_between_ieand_microsoft_edge.as_ref()
    }

    /// Sets the value of UnlockHomeButton
    pub fn set_unlock_home_button(&mut self, value: i32) {
        self.unlock_home_button = Some(value);
    }

    /// Gets the value of UnlockHomeButton
    pub fn get_unlock_home_button(&self) -> Option<&i32> {
        self.unlock_home_button.as_ref()
    }

    /// Sets the value of UseSharedFolderForBooks
    pub fn set_use_shared_folder_for_books(&mut self, value: i32) {
        self.use_shared_folder_for_books = Some(value);
    }

    /// Gets the value of UseSharedFolderForBooks
    pub fn get_use_shared_folder_for_books(&self) -> Option<&i32> {
        self.use_shared_folder_for_books.as_ref()
    }
}

