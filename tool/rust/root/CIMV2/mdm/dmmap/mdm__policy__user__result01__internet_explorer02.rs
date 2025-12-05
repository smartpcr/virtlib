// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_User_Result01_InternetExplorer02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_User_Result01_InternetExplorer02 {

/// 
    #[serde(rename = "AddSearchProvider")]
    pub add_search_provider: Option<String>,

/// 
    #[serde(rename = "AllowActiveXFiltering")]
    pub allow_active_xfiltering: Option<String>,

/// 
    #[serde(rename = "AllowAddOnList")]
    pub allow_add_on_list: Option<String>,

/// 
    #[serde(rename = "AllowAutoComplete")]
    pub allow_auto_complete: Option<String>,

/// 
    #[serde(rename = "AllowCertificateAddressMismatchWarning")]
    pub allow_certificate_address_mismatch_warning: Option<String>,

/// 
    #[serde(rename = "AllowDeletingBrowsingHistoryOnExit")]
    pub allow_deleting_browsing_history_on_exit: Option<String>,

/// 
    #[serde(rename = "AllowEnhancedProtectedMode")]
    pub allow_enhanced_protected_mode: Option<String>,

/// 
    #[serde(rename = "AllowEnhancedSuggestionsInAddressBar")]
    pub allow_enhanced_suggestions_in_address_bar: Option<String>,

/// 
    #[serde(rename = "AllowEnterpriseModeFromToolsMenu")]
    pub allow_enterprise_mode_from_tools_menu: Option<String>,

/// 
    #[serde(rename = "AllowEnterpriseModeSiteList")]
    pub allow_enterprise_mode_site_list: Option<String>,

/// 
    #[serde(rename = "AllowInternetExplorer7PolicyList")]
    pub allow_internet_explorer7_policy_list: Option<String>,

/// 
    #[serde(rename = "AllowInternetExplorerStandardsMode")]
    pub allow_internet_explorer_standards_mode: Option<String>,

/// 
    #[serde(rename = "AllowInternetZoneTemplate")]
    pub allow_internet_zone_template: Option<String>,

/// 
    #[serde(rename = "AllowIntranetZoneTemplate")]
    pub allow_intranet_zone_template: Option<String>,

/// 
    #[serde(rename = "AllowLocalMachineZoneTemplate")]
    pub allow_local_machine_zone_template: Option<String>,

/// 
    #[serde(rename = "AllowLockedDownInternetZoneTemplate")]
    pub allow_locked_down_internet_zone_template: Option<String>,

/// 
    #[serde(rename = "AllowLockedDownIntranetZoneTemplate")]
    pub allow_locked_down_intranet_zone_template: Option<String>,

/// 
    #[serde(rename = "AllowLockedDownLocalMachineZoneTemplate")]
    pub allow_locked_down_local_machine_zone_template: Option<String>,

/// 
    #[serde(rename = "AllowLockedDownRestrictedSitesZoneTemplate")]
    pub allow_locked_down_restricted_sites_zone_template: Option<String>,

/// 
    #[serde(rename = "AllowOneWordEntry")]
    pub allow_one_word_entry: Option<String>,

/// 
    #[serde(rename = "AllowSaveTargetAsInIEMode")]
    pub allow_save_target_as_in_iemode: Option<String>,

/// 
    #[serde(rename = "AllowSiteToZoneAssignmentList")]
    pub allow_site_to_zone_assignment_list: Option<String>,

/// 
    #[serde(rename = "AllowsLockedDownTrustedSitesZoneTemplate")]
    pub allows_locked_down_trusted_sites_zone_template: Option<String>,

/// 
    #[serde(rename = "AllowSoftwareWhenSignatureIsInvalid")]
    pub allow_software_when_signature_is_invalid: Option<String>,

/// 
    #[serde(rename = "AllowsRestrictedSitesZoneTemplate")]
    pub allows_restricted_sites_zone_template: Option<String>,

/// 
    #[serde(rename = "AllowSuggestedSites")]
    pub allow_suggested_sites: Option<String>,

/// 
    #[serde(rename = "AllowTrustedSitesZoneTemplate")]
    pub allow_trusted_sites_zone_template: Option<String>,

/// 
    #[serde(rename = "CheckServerCertificateRevocation")]
    pub check_server_certificate_revocation: Option<String>,

/// 
    #[serde(rename = "CheckSignaturesOnDownloadedPrograms")]
    pub check_signatures_on_downloaded_programs: Option<String>,

/// 
    #[serde(rename = "ConfigureEdgeRedirectChannel")]
    pub configure_edge_redirect_channel: Option<String>,

/// 
    #[serde(rename = "ConsistentMimeHandlingInternetExplorerProcesses")]
    pub consistent_mime_handling_internet_explorer_processes: Option<String>,

/// 
    #[serde(rename = "DisableActiveXVersionListAutoDownload")]
    pub disable_active_xversion_list_auto_download: Option<String>,

/// 
    #[serde(rename = "DisableBypassOfSmartScreenWarnings")]
    pub disable_bypass_of_smart_screen_warnings: Option<String>,

/// 
    #[serde(rename = "DisableBypassOfSmartScreenWarningsAboutUncommonFiles")]
    pub disable_bypass_of_smart_screen_warnings_about_uncommon_files: Option<String>,

/// 
    #[serde(rename = "DisableCompatView")]
    pub disable_compat_view: Option<String>,

/// 
    #[serde(rename = "DisableConfiguringHistory")]
    pub disable_configuring_history: Option<String>,

/// 
    #[serde(rename = "DisableCrashDetection")]
    pub disable_crash_detection: Option<String>,

/// 
    #[serde(rename = "DisableCustomerExperienceImprovementProgramParticipation")]
    pub disable_customer_experience_improvement_program_participation: Option<String>,

/// 
    #[serde(rename = "DisableDeletingUserVisitedWebsites")]
    pub disable_deleting_user_visited_websites: Option<String>,

/// 
    #[serde(rename = "DisableEnclosureDownloading")]
    pub disable_enclosure_downloading: Option<String>,

/// 
    #[serde(rename = "DisableEncryptionSupport")]
    pub disable_encryption_support: Option<String>,

/// 
    #[serde(rename = "DisableFeedsBackgroundSync")]
    pub disable_feeds_background_sync: Option<String>,

/// 
    #[serde(rename = "DisableFirstRunWizard")]
    pub disable_first_run_wizard: Option<String>,

/// 
    #[serde(rename = "DisableFlipAheadFeature")]
    pub disable_flip_ahead_feature: Option<String>,

/// 
    #[serde(rename = "DisableGeolocation")]
    pub disable_geolocation: Option<String>,

/// 
    #[serde(rename = "DisableHomePageChange")]
    pub disable_home_page_change: Option<String>,

/// 
    #[serde(rename = "DisableIgnoringCertificateErrors")]
    pub disable_ignoring_certificate_errors: Option<String>,

/// 
    #[serde(rename = "DisableInPrivateBrowsing")]
    pub disable_in_private_browsing: Option<String>,

/// 
    #[serde(rename = "DisableInternetExplorerApp")]
    pub disable_internet_explorer_app: Option<String>,

/// 
    #[serde(rename = "DisableProcessesInEnhancedProtectedMode")]
    pub disable_processes_in_enhanced_protected_mode: Option<String>,

/// 
    #[serde(rename = "DisableProxyChange")]
    pub disable_proxy_change: Option<String>,

/// 
    #[serde(rename = "DisableSearchProviderChange")]
    pub disable_search_provider_change: Option<String>,

/// 
    #[serde(rename = "DisableSecondaryHomePageChange")]
    pub disable_secondary_home_page_change: Option<String>,

/// 
    #[serde(rename = "DisableSecuritySettingsCheck")]
    pub disable_security_settings_check: Option<String>,

/// 
    #[serde(rename = "DisableWebAddressAutoComplete")]
    pub disable_web_address_auto_complete: Option<String>,

/// 
    #[serde(rename = "DoNotAllowActiveXControlsInProtectedMode")]
    pub do_not_allow_active_xcontrols_in_protected_mode: Option<String>,

/// 
    #[serde(rename = "DoNotBlockOutdatedActiveXControls")]
    pub do_not_block_outdated_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "DoNotBlockOutdatedActiveXControlsOnSpecificDomains")]
    pub do_not_block_outdated_active_xcontrols_on_specific_domains: Option<String>,

/// 
    #[serde(rename = "EnableExtendedIEModeHotkeys")]
    pub enable_extended_iemode_hotkeys: Option<String>,

/// 
    #[serde(rename = "IncludeAllLocalSites")]
    pub include_all_local_sites: Option<String>,

/// 
    #[serde(rename = "IncludeAllNetworkPaths")]
    pub include_all_network_paths: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "InternetZoneAllowAccessToDataSources")]
    pub internet_zone_allow_access_to_data_sources: Option<String>,

/// 
    #[serde(rename = "InternetZoneAllowAutomaticPromptingForActiveXControls")]
    pub internet_zone_allow_automatic_prompting_for_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "InternetZoneAllowAutomaticPromptingForFileDownloads")]
    pub internet_zone_allow_automatic_prompting_for_file_downloads: Option<String>,

/// 
    #[serde(rename = "InternetZoneAllowCopyPasteViaScript")]
    pub internet_zone_allow_copy_paste_via_script: Option<String>,

/// 
    #[serde(rename = "InternetZoneAllowDragAndDropCopyAndPasteFiles")]
    pub internet_zone_allow_drag_and_drop_copy_and_paste_files: Option<String>,

/// 
    #[serde(rename = "InternetZoneAllowFontDownloads")]
    pub internet_zone_allow_font_downloads: Option<String>,

/// 
    #[serde(rename = "InternetZoneAllowLessPrivilegedSites")]
    pub internet_zone_allow_less_privileged_sites: Option<String>,

/// 
    #[serde(rename = "InternetZoneAllowLoadingOfXAMLFiles")]
    pub internet_zone_allow_loading_of_xamlfiles: Option<String>,

/// 
    #[serde(rename = "InternetZoneAllowNETFrameworkReliantComponents")]
    pub internet_zone_allow_netframework_reliant_components: Option<String>,

/// 
    #[serde(rename = "InternetZoneAllowOnlyApprovedDomainsToUseActiveXControls")]
    pub internet_zone_allow_only_approved_domains_to_use_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "InternetZoneAllowOnlyApprovedDomainsToUseTDCActiveXControl")]
    pub internet_zone_allow_only_approved_domains_to_use_tdcactive_xcontrol: Option<String>,

/// 
    #[serde(rename = "InternetZoneAllowScriptingOfInternetExplorerWebBrowserControls")]
    pub internet_zone_allow_scripting_of_internet_explorer_web_browser_controls: Option<String>,

/// 
    #[serde(rename = "InternetZoneAllowScriptInitiatedWindows")]
    pub internet_zone_allow_script_initiated_windows: Option<String>,

/// 
    #[serde(rename = "InternetZoneAllowScriptlets")]
    pub internet_zone_allow_scriptlets: Option<String>,

/// 
    #[serde(rename = "InternetZoneAllowSmartScreenIE")]
    pub internet_zone_allow_smart_screen_ie: Option<String>,

/// 
    #[serde(rename = "InternetZoneAllowUpdatesToStatusBarViaScript")]
    pub internet_zone_allow_updates_to_status_bar_via_script: Option<String>,

/// 
    #[serde(rename = "InternetZoneAllowUserDataPersistence")]
    pub internet_zone_allow_user_data_persistence: Option<String>,

/// 
    #[serde(rename = "InternetZoneAllowVBScriptToRunInInternetExplorer")]
    pub internet_zone_allow_vbscript_to_run_in_internet_explorer: Option<String>,

/// 
    #[serde(rename = "InternetZoneDoNotRunAntimalwareAgainstActiveXControls")]
    pub internet_zone_do_not_run_antimalware_against_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "InternetZoneDownloadSignedActiveXControls")]
    pub internet_zone_download_signed_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "InternetZoneDownloadUnsignedActiveXControls")]
    pub internet_zone_download_unsigned_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "InternetZoneEnableCrossSiteScriptingFilter")]
    pub internet_zone_enable_cross_site_scripting_filter: Option<String>,

/// 
    #[serde(rename = "InternetZoneEnableDraggingOfContentFromDifferentDomainsAcrossWindows")]
    pub internet_zone_enable_dragging_of_content_from_different_domains_across_windows: Option<String>,

/// 
    #[serde(rename = "InternetZoneEnableDraggingOfContentFromDifferentDomainsWithinWindows")]
    pub internet_zone_enable_dragging_of_content_from_different_domains_within_windows: Option<String>,

/// 
    #[serde(rename = "InternetZoneEnableMIMESniffing")]
    pub internet_zone_enable_mimesniffing: Option<String>,

/// 
    #[serde(rename = "InternetZoneEnableProtectedMode")]
    pub internet_zone_enable_protected_mode: Option<String>,

/// 
    #[serde(rename = "InternetZoneIncludeLocalPathWhenUploadingFilesToServer")]
    pub internet_zone_include_local_path_when_uploading_files_to_server: Option<String>,

/// 
    #[serde(rename = "InternetZoneInitializeAndScriptActiveXControls")]
    pub internet_zone_initialize_and_script_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "InternetZoneJavaPermissions")]
    pub internet_zone_java_permissions: Option<String>,

/// 
    #[serde(rename = "InternetZoneLaunchingApplicationsAndFilesInIFRAME")]
    pub internet_zone_launching_applications_and_files_in_iframe: Option<String>,

/// 
    #[serde(rename = "InternetZoneLogonOptions")]
    pub internet_zone_logon_options: Option<String>,

/// 
    #[serde(rename = "InternetZoneNavigateWindowsAndFrames")]
    pub internet_zone_navigate_windows_and_frames: Option<String>,

/// 
    #[serde(rename = "InternetZoneRunNETFrameworkReliantComponentsSignedWithAuthenticode")]
    pub internet_zone_run_netframework_reliant_components_signed_with_authenticode: Option<String>,

/// 
    #[serde(rename = "InternetZoneShowSecurityWarningForPotentiallyUnsafeFiles")]
    pub internet_zone_show_security_warning_for_potentially_unsafe_files: Option<String>,

/// 
    #[serde(rename = "InternetZoneUsePopupBlocker")]
    pub internet_zone_use_popup_blocker: Option<String>,

/// 
    #[serde(rename = "IntranetZoneAllowAccessToDataSources")]
    pub intranet_zone_allow_access_to_data_sources: Option<String>,

/// 
    #[serde(rename = "IntranetZoneAllowAutomaticPromptingForActiveXControls")]
    pub intranet_zone_allow_automatic_prompting_for_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "IntranetZoneAllowAutomaticPromptingForFileDownloads")]
    pub intranet_zone_allow_automatic_prompting_for_file_downloads: Option<String>,

/// 
    #[serde(rename = "IntranetZoneAllowFontDownloads")]
    pub intranet_zone_allow_font_downloads: Option<String>,

/// 
    #[serde(rename = "IntranetZoneAllowLessPrivilegedSites")]
    pub intranet_zone_allow_less_privileged_sites: Option<String>,

/// 
    #[serde(rename = "IntranetZoneAllowNETFrameworkReliantComponents")]
    pub intranet_zone_allow_netframework_reliant_components: Option<String>,

/// 
    #[serde(rename = "IntranetZoneAllowScriptlets")]
    pub intranet_zone_allow_scriptlets: Option<String>,

/// 
    #[serde(rename = "IntranetZoneAllowSmartScreenIE")]
    pub intranet_zone_allow_smart_screen_ie: Option<String>,

/// 
    #[serde(rename = "IntranetZoneAllowUserDataPersistence")]
    pub intranet_zone_allow_user_data_persistence: Option<String>,

/// 
    #[serde(rename = "IntranetZoneDoNotRunAntimalwareAgainstActiveXControls")]
    pub intranet_zone_do_not_run_antimalware_against_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "IntranetZoneInitializeAndScriptActiveXControls")]
    pub intranet_zone_initialize_and_script_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "IntranetZoneJavaPermissions")]
    pub intranet_zone_java_permissions: Option<String>,

/// 
    #[serde(rename = "IntranetZoneNavigateWindowsAndFrames")]
    pub intranet_zone_navigate_windows_and_frames: Option<String>,

/// 
    #[serde(rename = "JScriptReplacement")]
    pub jscript_replacement: Option<String>,

/// 
    #[serde(rename = "KeepIntranetSitesInInternetExplorer")]
    pub keep_intranet_sites_in_internet_explorer: Option<String>,

/// 
    #[serde(rename = "LocalMachineZoneAllowAccessToDataSources")]
    pub local_machine_zone_allow_access_to_data_sources: Option<String>,

/// 
    #[serde(rename = "LocalMachineZoneAllowAutomaticPromptingForActiveXControls")]
    pub local_machine_zone_allow_automatic_prompting_for_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "LocalMachineZoneAllowAutomaticPromptingForFileDownloads")]
    pub local_machine_zone_allow_automatic_prompting_for_file_downloads: Option<String>,

/// 
    #[serde(rename = "LocalMachineZoneAllowFontDownloads")]
    pub local_machine_zone_allow_font_downloads: Option<String>,

/// 
    #[serde(rename = "LocalMachineZoneAllowLessPrivilegedSites")]
    pub local_machine_zone_allow_less_privileged_sites: Option<String>,

/// 
    #[serde(rename = "LocalMachineZoneAllowNETFrameworkReliantComponents")]
    pub local_machine_zone_allow_netframework_reliant_components: Option<String>,

/// 
    #[serde(rename = "LocalMachineZoneAllowScriptlets")]
    pub local_machine_zone_allow_scriptlets: Option<String>,

/// 
    #[serde(rename = "LocalMachineZoneAllowSmartScreenIE")]
    pub local_machine_zone_allow_smart_screen_ie: Option<String>,

/// 
    #[serde(rename = "LocalMachineZoneAllowUserDataPersistence")]
    pub local_machine_zone_allow_user_data_persistence: Option<String>,

/// 
    #[serde(rename = "LocalMachineZoneDoNotRunAntimalwareAgainstActiveXControls")]
    pub local_machine_zone_do_not_run_antimalware_against_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "LocalMachineZoneInitializeAndScriptActiveXControls")]
    pub local_machine_zone_initialize_and_script_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "LocalMachineZoneJavaPermissions")]
    pub local_machine_zone_java_permissions: Option<String>,

/// 
    #[serde(rename = "LocalMachineZoneNavigateWindowsAndFrames")]
    pub local_machine_zone_navigate_windows_and_frames: Option<String>,

/// 
    #[serde(rename = "LockedDownInternetZoneAllowAccessToDataSources")]
    pub locked_down_internet_zone_allow_access_to_data_sources: Option<String>,

/// 
    #[serde(rename = "LockedDownInternetZoneAllowAutomaticPromptingForActiveXControls")]
    pub locked_down_internet_zone_allow_automatic_prompting_for_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "LockedDownInternetZoneAllowAutomaticPromptingForFileDownloads")]
    pub locked_down_internet_zone_allow_automatic_prompting_for_file_downloads: Option<String>,

/// 
    #[serde(rename = "LockedDownInternetZoneAllowFontDownloads")]
    pub locked_down_internet_zone_allow_font_downloads: Option<String>,

/// 
    #[serde(rename = "LockedDownInternetZoneAllowLessPrivilegedSites")]
    pub locked_down_internet_zone_allow_less_privileged_sites: Option<String>,

/// 
    #[serde(rename = "LockedDownInternetZoneAllowNETFrameworkReliantComponents")]
    pub locked_down_internet_zone_allow_netframework_reliant_components: Option<String>,

/// 
    #[serde(rename = "LockedDownInternetZoneAllowScriptlets")]
    pub locked_down_internet_zone_allow_scriptlets: Option<String>,

/// 
    #[serde(rename = "LockedDownInternetZoneAllowSmartScreenIE")]
    pub locked_down_internet_zone_allow_smart_screen_ie: Option<String>,

/// 
    #[serde(rename = "LockedDownInternetZoneAllowUserDataPersistence")]
    pub locked_down_internet_zone_allow_user_data_persistence: Option<String>,

/// 
    #[serde(rename = "LockedDownInternetZoneInitializeAndScriptActiveXControls")]
    pub locked_down_internet_zone_initialize_and_script_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "LockedDownInternetZoneJavaPermissions")]
    pub locked_down_internet_zone_java_permissions: Option<String>,

/// 
    #[serde(rename = "LockedDownInternetZoneNavigateWindowsAndFrames")]
    pub locked_down_internet_zone_navigate_windows_and_frames: Option<String>,

/// 
    #[serde(rename = "LockedDownIntranetJavaPermissions")]
    pub locked_down_intranet_java_permissions: Option<String>,

/// 
    #[serde(rename = "LockedDownIntranetZoneAllowAccessToDataSources")]
    pub locked_down_intranet_zone_allow_access_to_data_sources: Option<String>,

/// 
    #[serde(rename = "LockedDownIntranetZoneAllowAutomaticPromptingForActiveXControls")]
    pub locked_down_intranet_zone_allow_automatic_prompting_for_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "LockedDownIntranetZoneAllowAutomaticPromptingForFileDownloads")]
    pub locked_down_intranet_zone_allow_automatic_prompting_for_file_downloads: Option<String>,

/// 
    #[serde(rename = "LockedDownIntranetZoneAllowFontDownloads")]
    pub locked_down_intranet_zone_allow_font_downloads: Option<String>,

/// 
    #[serde(rename = "LockedDownIntranetZoneAllowLessPrivilegedSites")]
    pub locked_down_intranet_zone_allow_less_privileged_sites: Option<String>,

/// 
    #[serde(rename = "LockedDownIntranetZoneAllowNETFrameworkReliantComponents")]
    pub locked_down_intranet_zone_allow_netframework_reliant_components: Option<String>,

/// 
    #[serde(rename = "LockedDownIntranetZoneAllowScriptlets")]
    pub locked_down_intranet_zone_allow_scriptlets: Option<String>,

/// 
    #[serde(rename = "LockedDownIntranetZoneAllowSmartScreenIE")]
    pub locked_down_intranet_zone_allow_smart_screen_ie: Option<String>,

/// 
    #[serde(rename = "LockedDownIntranetZoneAllowUserDataPersistence")]
    pub locked_down_intranet_zone_allow_user_data_persistence: Option<String>,

/// 
    #[serde(rename = "LockedDownIntranetZoneInitializeAndScriptActiveXControls")]
    pub locked_down_intranet_zone_initialize_and_script_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "LockedDownIntranetZoneNavigateWindowsAndFrames")]
    pub locked_down_intranet_zone_navigate_windows_and_frames: Option<String>,

/// 
    #[serde(rename = "LockedDownLocalMachineZoneAllowAccessToDataSources")]
    pub locked_down_local_machine_zone_allow_access_to_data_sources: Option<String>,

/// 
    #[serde(rename = "LockedDownLocalMachineZoneAllowAutomaticPromptingForActiveXControls")]
    pub locked_down_local_machine_zone_allow_automatic_prompting_for_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "LockedDownLocalMachineZoneAllowAutomaticPromptingForFileDownloads")]
    pub locked_down_local_machine_zone_allow_automatic_prompting_for_file_downloads: Option<String>,

/// 
    #[serde(rename = "LockedDownLocalMachineZoneAllowFontDownloads")]
    pub locked_down_local_machine_zone_allow_font_downloads: Option<String>,

/// 
    #[serde(rename = "LockedDownLocalMachineZoneAllowLessPrivilegedSites")]
    pub locked_down_local_machine_zone_allow_less_privileged_sites: Option<String>,

/// 
    #[serde(rename = "LockedDownLocalMachineZoneAllowNETFrameworkReliantComponents")]
    pub locked_down_local_machine_zone_allow_netframework_reliant_components: Option<String>,

/// 
    #[serde(rename = "LockedDownLocalMachineZoneAllowScriptlets")]
    pub locked_down_local_machine_zone_allow_scriptlets: Option<String>,

/// 
    #[serde(rename = "LockedDownLocalMachineZoneAllowSmartScreenIE")]
    pub locked_down_local_machine_zone_allow_smart_screen_ie: Option<String>,

/// 
    #[serde(rename = "LockedDownLocalMachineZoneAllowUserDataPersistence")]
    pub locked_down_local_machine_zone_allow_user_data_persistence: Option<String>,

/// 
    #[serde(rename = "LockedDownLocalMachineZoneInitializeAndScriptActiveXControls")]
    pub locked_down_local_machine_zone_initialize_and_script_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "LockedDownLocalMachineZoneJavaPermissions")]
    pub locked_down_local_machine_zone_java_permissions: Option<String>,

/// 
    #[serde(rename = "LockedDownLocalMachineZoneNavigateWindowsAndFrames")]
    pub locked_down_local_machine_zone_navigate_windows_and_frames: Option<String>,

/// 
    #[serde(rename = "LockedDownRestrictedSitesZoneAllowAccessToDataSources")]
    pub locked_down_restricted_sites_zone_allow_access_to_data_sources: Option<String>,

/// 
    #[serde(rename = "LockedDownRestrictedSitesZoneAllowAutomaticPromptingForActiveXControls")]
    pub locked_down_restricted_sites_zone_allow_automatic_prompting_for_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "LockedDownRestrictedSitesZoneAllowAutomaticPromptingForFileDownloads")]
    pub locked_down_restricted_sites_zone_allow_automatic_prompting_for_file_downloads: Option<String>,

/// 
    #[serde(rename = "LockedDownRestrictedSitesZoneAllowFontDownloads")]
    pub locked_down_restricted_sites_zone_allow_font_downloads: Option<String>,

/// 
    #[serde(rename = "LockedDownRestrictedSitesZoneAllowLessPrivilegedSites")]
    pub locked_down_restricted_sites_zone_allow_less_privileged_sites: Option<String>,

/// 
    #[serde(rename = "LockedDownRestrictedSitesZoneAllowNETFrameworkReliantComponents")]
    pub locked_down_restricted_sites_zone_allow_netframework_reliant_components: Option<String>,

/// 
    #[serde(rename = "LockedDownRestrictedSitesZoneAllowScriptlets")]
    pub locked_down_restricted_sites_zone_allow_scriptlets: Option<String>,

/// 
    #[serde(rename = "LockedDownRestrictedSitesZoneAllowSmartScreenIE")]
    pub locked_down_restricted_sites_zone_allow_smart_screen_ie: Option<String>,

/// 
    #[serde(rename = "LockedDownRestrictedSitesZoneAllowUserDataPersistence")]
    pub locked_down_restricted_sites_zone_allow_user_data_persistence: Option<String>,

/// 
    #[serde(rename = "LockedDownRestrictedSitesZoneInitializeAndScriptActiveXControls")]
    pub locked_down_restricted_sites_zone_initialize_and_script_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "LockedDownRestrictedSitesZoneJavaPermissions")]
    pub locked_down_restricted_sites_zone_java_permissions: Option<String>,

/// 
    #[serde(rename = "LockedDownRestrictedSitesZoneNavigateWindowsAndFrames")]
    pub locked_down_restricted_sites_zone_navigate_windows_and_frames: Option<String>,

/// 
    #[serde(rename = "LockedDownTrustedSitesZoneAllowAccessToDataSources")]
    pub locked_down_trusted_sites_zone_allow_access_to_data_sources: Option<String>,

/// 
    #[serde(rename = "LockedDownTrustedSitesZoneAllowAutomaticPromptingForActiveXControls")]
    pub locked_down_trusted_sites_zone_allow_automatic_prompting_for_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "LockedDownTrustedSitesZoneAllowAutomaticPromptingForFileDownloads")]
    pub locked_down_trusted_sites_zone_allow_automatic_prompting_for_file_downloads: Option<String>,

/// 
    #[serde(rename = "LockedDownTrustedSitesZoneAllowFontDownloads")]
    pub locked_down_trusted_sites_zone_allow_font_downloads: Option<String>,

/// 
    #[serde(rename = "LockedDownTrustedSitesZoneAllowLessPrivilegedSites")]
    pub locked_down_trusted_sites_zone_allow_less_privileged_sites: Option<String>,

/// 
    #[serde(rename = "LockedDownTrustedSitesZoneAllowNETFrameworkReliantComponents")]
    pub locked_down_trusted_sites_zone_allow_netframework_reliant_components: Option<String>,

/// 
    #[serde(rename = "LockedDownTrustedSitesZoneAllowScriptlets")]
    pub locked_down_trusted_sites_zone_allow_scriptlets: Option<String>,

/// 
    #[serde(rename = "LockedDownTrustedSitesZoneAllowSmartScreenIE")]
    pub locked_down_trusted_sites_zone_allow_smart_screen_ie: Option<String>,

/// 
    #[serde(rename = "LockedDownTrustedSitesZoneAllowUserDataPersistence")]
    pub locked_down_trusted_sites_zone_allow_user_data_persistence: Option<String>,

/// 
    #[serde(rename = "LockedDownTrustedSitesZoneInitializeAndScriptActiveXControls")]
    pub locked_down_trusted_sites_zone_initialize_and_script_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "LockedDownTrustedSitesZoneJavaPermissions")]
    pub locked_down_trusted_sites_zone_java_permissions: Option<String>,

/// 
    #[serde(rename = "LockedDownTrustedSitesZoneNavigateWindowsAndFrames")]
    pub locked_down_trusted_sites_zone_navigate_windows_and_frames: Option<String>,

/// 
    #[serde(rename = "MimeSniffingSafetyFeatureInternetExplorerProcesses")]
    pub mime_sniffing_safety_feature_internet_explorer_processes: Option<String>,

/// 
    #[serde(rename = "MKProtocolSecurityRestrictionInternetExplorerProcesses")]
    pub mkprotocol_security_restriction_internet_explorer_processes: Option<String>,

/// 
    #[serde(rename = "NewTabDefaultPage")]
    pub new_tab_default_page: Option<String>,

/// 
    #[serde(rename = "NotificationBarInternetExplorerProcesses")]
    pub notification_bar_internet_explorer_processes: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PreventManagingSmartScreenFilter")]
    pub prevent_managing_smart_screen_filter: Option<String>,

/// 
    #[serde(rename = "PreventPerUserInstallationOfActiveXControls")]
    pub prevent_per_user_installation_of_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "ProtectionFromZoneElevationInternetExplorerProcesses")]
    pub protection_from_zone_elevation_internet_explorer_processes: Option<String>,

/// 
    #[serde(rename = "RemoveRunThisTimeButtonForOutdatedActiveXControls")]
    pub remove_run_this_time_button_for_outdated_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "RestrictActiveXInstallInternetExplorerProcesses")]
    pub restrict_active_xinstall_internet_explorer_processes: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneAllowAccessToDataSources")]
    pub restricted_sites_zone_allow_access_to_data_sources: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneAllowActiveScripting")]
    pub restricted_sites_zone_allow_active_scripting: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneAllowAutomaticPromptingForActiveXControls")]
    pub restricted_sites_zone_allow_automatic_prompting_for_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneAllowAutomaticPromptingForFileDownloads")]
    pub restricted_sites_zone_allow_automatic_prompting_for_file_downloads: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneAllowBinaryAndScriptBehaviors")]
    pub restricted_sites_zone_allow_binary_and_script_behaviors: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneAllowCopyPasteViaScript")]
    pub restricted_sites_zone_allow_copy_paste_via_script: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneAllowDragAndDropCopyAndPasteFiles")]
    pub restricted_sites_zone_allow_drag_and_drop_copy_and_paste_files: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneAllowFileDownloads")]
    pub restricted_sites_zone_allow_file_downloads: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneAllowFontDownloads")]
    pub restricted_sites_zone_allow_font_downloads: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneAllowLessPrivilegedSites")]
    pub restricted_sites_zone_allow_less_privileged_sites: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneAllowLoadingOfXAMLFiles")]
    pub restricted_sites_zone_allow_loading_of_xamlfiles: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneAllowMETAREFRESH")]
    pub restricted_sites_zone_allow_metarefresh: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneAllowNETFrameworkReliantComponents")]
    pub restricted_sites_zone_allow_netframework_reliant_components: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneAllowOnlyApprovedDomainsToUseActiveXControls")]
    pub restricted_sites_zone_allow_only_approved_domains_to_use_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneAllowOnlyApprovedDomainsToUseTDCActiveXControl")]
    pub restricted_sites_zone_allow_only_approved_domains_to_use_tdcactive_xcontrol: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneAllowScriptingOfInternetExplorerWebBrowserControls")]
    pub restricted_sites_zone_allow_scripting_of_internet_explorer_web_browser_controls: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneAllowScriptInitiatedWindows")]
    pub restricted_sites_zone_allow_script_initiated_windows: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneAllowScriptlets")]
    pub restricted_sites_zone_allow_scriptlets: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneAllowSmartScreenIE")]
    pub restricted_sites_zone_allow_smart_screen_ie: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneAllowUpdatesToStatusBarViaScript")]
    pub restricted_sites_zone_allow_updates_to_status_bar_via_script: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneAllowUserDataPersistence")]
    pub restricted_sites_zone_allow_user_data_persistence: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneAllowVBScriptToRunInInternetExplorer")]
    pub restricted_sites_zone_allow_vbscript_to_run_in_internet_explorer: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneDoNotRunAntimalwareAgainstActiveXControls")]
    pub restricted_sites_zone_do_not_run_antimalware_against_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneDownloadSignedActiveXControls")]
    pub restricted_sites_zone_download_signed_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneDownloadUnsignedActiveXControls")]
    pub restricted_sites_zone_download_unsigned_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneEnableCrossSiteScriptingFilter")]
    pub restricted_sites_zone_enable_cross_site_scripting_filter: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneEnableDraggingOfContentFromDifferentDomainsAcrossWindows")]
    pub restricted_sites_zone_enable_dragging_of_content_from_different_domains_across_windows: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneEnableDraggingOfContentFromDifferentDomainsWithinWindows")]
    pub restricted_sites_zone_enable_dragging_of_content_from_different_domains_within_windows: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneEnableMIMESniffing")]
    pub restricted_sites_zone_enable_mimesniffing: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneIncludeLocalPathWhenUploadingFilesToServer")]
    pub restricted_sites_zone_include_local_path_when_uploading_files_to_server: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneInitializeAndScriptActiveXControls")]
    pub restricted_sites_zone_initialize_and_script_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneJavaPermissions")]
    pub restricted_sites_zone_java_permissions: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneLaunchingApplicationsAndFilesInIFRAME")]
    pub restricted_sites_zone_launching_applications_and_files_in_iframe: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneLogonOptions")]
    pub restricted_sites_zone_logon_options: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneNavigateWindowsAndFrames")]
    pub restricted_sites_zone_navigate_windows_and_frames: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneRunActiveXControlsAndPlugins")]
    pub restricted_sites_zone_run_active_xcontrols_and_plugins: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneRunNETFrameworkReliantComponentsSignedWithAuthenticode")]
    pub restricted_sites_zone_run_netframework_reliant_components_signed_with_authenticode: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneScriptActiveXControlsMarkedSafeForScripting")]
    pub restricted_sites_zone_script_active_xcontrols_marked_safe_for_scripting: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneScriptingOfJavaApplets")]
    pub restricted_sites_zone_scripting_of_java_applets: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneShowSecurityWarningForPotentiallyUnsafeFiles")]
    pub restricted_sites_zone_show_security_warning_for_potentially_unsafe_files: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneTurnOnProtectedMode")]
    pub restricted_sites_zone_turn_on_protected_mode: Option<String>,

/// 
    #[serde(rename = "RestrictedSitesZoneUsePopupBlocker")]
    pub restricted_sites_zone_use_popup_blocker: Option<String>,

/// 
    #[serde(rename = "RestrictFileDownloadInternetExplorerProcesses")]
    pub restrict_file_download_internet_explorer_processes: Option<String>,

/// 
    #[serde(rename = "ScriptedWindowSecurityRestrictionsInternetExplorerProcesses")]
    pub scripted_window_security_restrictions_internet_explorer_processes: Option<String>,

/// 
    #[serde(rename = "SearchProviderList")]
    pub search_provider_list: Option<String>,

/// 
    #[serde(rename = "SendSitesNotInEnterpriseSiteListToEdge")]
    pub send_sites_not_in_enterprise_site_list_to_edge: Option<String>,

/// 
    #[serde(rename = "SpecifyUseOfActiveXInstallerService")]
    pub specify_use_of_active_xinstaller_service: Option<String>,

/// 
    #[serde(rename = "TrustedSitesZoneAllowAccessToDataSources")]
    pub trusted_sites_zone_allow_access_to_data_sources: Option<String>,

/// 
    #[serde(rename = "TrustedSitesZoneAllowAutomaticPromptingForActiveXControls")]
    pub trusted_sites_zone_allow_automatic_prompting_for_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "TrustedSitesZoneAllowAutomaticPromptingForFileDownloads")]
    pub trusted_sites_zone_allow_automatic_prompting_for_file_downloads: Option<String>,

/// 
    #[serde(rename = "TrustedSitesZoneAllowFontDownloads")]
    pub trusted_sites_zone_allow_font_downloads: Option<String>,

/// 
    #[serde(rename = "TrustedSitesZoneAllowLessPrivilegedSites")]
    pub trusted_sites_zone_allow_less_privileged_sites: Option<String>,

/// 
    #[serde(rename = "TrustedSitesZoneAllowNETFrameworkReliantComponents")]
    pub trusted_sites_zone_allow_netframework_reliant_components: Option<String>,

/// 
    #[serde(rename = "TrustedSitesZoneAllowScriptlets")]
    pub trusted_sites_zone_allow_scriptlets: Option<String>,

/// 
    #[serde(rename = "TrustedSitesZoneAllowSmartScreenIE")]
    pub trusted_sites_zone_allow_smart_screen_ie: Option<String>,

/// 
    #[serde(rename = "TrustedSitesZoneAllowUserDataPersistence")]
    pub trusted_sites_zone_allow_user_data_persistence: Option<String>,

/// 
    #[serde(rename = "TrustedSitesZoneDoNotRunAntimalwareAgainstActiveXControls")]
    pub trusted_sites_zone_do_not_run_antimalware_against_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "TrustedSitesZoneInitializeAndScriptActiveXControls")]
    pub trusted_sites_zone_initialize_and_script_active_xcontrols: Option<String>,

/// 
    #[serde(rename = "TrustedSitesZoneJavaPermissions")]
    pub trusted_sites_zone_java_permissions: Option<String>,

/// 
    #[serde(rename = "TrustedSitesZoneNavigateWindowsAndFrames")]
    pub trusted_sites_zone_navigate_windows_and_frames: Option<String>,
}

impl MDM_Policy_User_Result01_InternetExplorer02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            add_search_provider: None,
            allow_active_xfiltering: None,
            allow_add_on_list: None,
            allow_auto_complete: None,
            allow_certificate_address_mismatch_warning: None,
            allow_deleting_browsing_history_on_exit: None,
            allow_enhanced_protected_mode: None,
            allow_enhanced_suggestions_in_address_bar: None,
            allow_enterprise_mode_from_tools_menu: None,
            allow_enterprise_mode_site_list: None,
            allow_internet_explorer7_policy_list: None,
            allow_internet_explorer_standards_mode: None,
            allow_internet_zone_template: None,
            allow_intranet_zone_template: None,
            allow_local_machine_zone_template: None,
            allow_locked_down_internet_zone_template: None,
            allow_locked_down_intranet_zone_template: None,
            allow_locked_down_local_machine_zone_template: None,
            allow_locked_down_restricted_sites_zone_template: None,
            allow_one_word_entry: None,
            allow_save_target_as_in_iemode: None,
            allow_site_to_zone_assignment_list: None,
            allows_locked_down_trusted_sites_zone_template: None,
            allow_software_when_signature_is_invalid: None,
            allows_restricted_sites_zone_template: None,
            allow_suggested_sites: None,
            allow_trusted_sites_zone_template: None,
            check_server_certificate_revocation: None,
            check_signatures_on_downloaded_programs: None,
            configure_edge_redirect_channel: None,
            consistent_mime_handling_internet_explorer_processes: None,
            disable_active_xversion_list_auto_download: None,
            disable_bypass_of_smart_screen_warnings: None,
            disable_bypass_of_smart_screen_warnings_about_uncommon_files: None,
            disable_compat_view: None,
            disable_configuring_history: None,
            disable_crash_detection: None,
            disable_customer_experience_improvement_program_participation: None,
            disable_deleting_user_visited_websites: None,
            disable_enclosure_downloading: None,
            disable_encryption_support: None,
            disable_feeds_background_sync: None,
            disable_first_run_wizard: None,
            disable_flip_ahead_feature: None,
            disable_geolocation: None,
            disable_home_page_change: None,
            disable_ignoring_certificate_errors: None,
            disable_in_private_browsing: None,
            disable_internet_explorer_app: None,
            disable_processes_in_enhanced_protected_mode: None,
            disable_proxy_change: None,
            disable_search_provider_change: None,
            disable_secondary_home_page_change: None,
            disable_security_settings_check: None,
            disable_web_address_auto_complete: None,
            do_not_allow_active_xcontrols_in_protected_mode: None,
            do_not_block_outdated_active_xcontrols: None,
            do_not_block_outdated_active_xcontrols_on_specific_domains: None,
            enable_extended_iemode_hotkeys: None,
            include_all_local_sites: None,
            include_all_network_paths: None,
            instance_id: None,
            internet_zone_allow_access_to_data_sources: None,
            internet_zone_allow_automatic_prompting_for_active_xcontrols: None,
            internet_zone_allow_automatic_prompting_for_file_downloads: None,
            internet_zone_allow_copy_paste_via_script: None,
            internet_zone_allow_drag_and_drop_copy_and_paste_files: None,
            internet_zone_allow_font_downloads: None,
            internet_zone_allow_less_privileged_sites: None,
            internet_zone_allow_loading_of_xamlfiles: None,
            internet_zone_allow_netframework_reliant_components: None,
            internet_zone_allow_only_approved_domains_to_use_active_xcontrols: None,
            internet_zone_allow_only_approved_domains_to_use_tdcactive_xcontrol: None,
            internet_zone_allow_scripting_of_internet_explorer_web_browser_controls: None,
            internet_zone_allow_script_initiated_windows: None,
            internet_zone_allow_scriptlets: None,
            internet_zone_allow_smart_screen_ie: None,
            internet_zone_allow_updates_to_status_bar_via_script: None,
            internet_zone_allow_user_data_persistence: None,
            internet_zone_allow_vbscript_to_run_in_internet_explorer: None,
            internet_zone_do_not_run_antimalware_against_active_xcontrols: None,
            internet_zone_download_signed_active_xcontrols: None,
            internet_zone_download_unsigned_active_xcontrols: None,
            internet_zone_enable_cross_site_scripting_filter: None,
            internet_zone_enable_dragging_of_content_from_different_domains_across_windows: None,
            internet_zone_enable_dragging_of_content_from_different_domains_within_windows: None,
            internet_zone_enable_mimesniffing: None,
            internet_zone_enable_protected_mode: None,
            internet_zone_include_local_path_when_uploading_files_to_server: None,
            internet_zone_initialize_and_script_active_xcontrols: None,
            internet_zone_java_permissions: None,
            internet_zone_launching_applications_and_files_in_iframe: None,
            internet_zone_logon_options: None,
            internet_zone_navigate_windows_and_frames: None,
            internet_zone_run_netframework_reliant_components_signed_with_authenticode: None,
            internet_zone_show_security_warning_for_potentially_unsafe_files: None,
            internet_zone_use_popup_blocker: None,
            intranet_zone_allow_access_to_data_sources: None,
            intranet_zone_allow_automatic_prompting_for_active_xcontrols: None,
            intranet_zone_allow_automatic_prompting_for_file_downloads: None,
            intranet_zone_allow_font_downloads: None,
            intranet_zone_allow_less_privileged_sites: None,
            intranet_zone_allow_netframework_reliant_components: None,
            intranet_zone_allow_scriptlets: None,
            intranet_zone_allow_smart_screen_ie: None,
            intranet_zone_allow_user_data_persistence: None,
            intranet_zone_do_not_run_antimalware_against_active_xcontrols: None,
            intranet_zone_initialize_and_script_active_xcontrols: None,
            intranet_zone_java_permissions: None,
            intranet_zone_navigate_windows_and_frames: None,
            jscript_replacement: None,
            keep_intranet_sites_in_internet_explorer: None,
            local_machine_zone_allow_access_to_data_sources: None,
            local_machine_zone_allow_automatic_prompting_for_active_xcontrols: None,
            local_machine_zone_allow_automatic_prompting_for_file_downloads: None,
            local_machine_zone_allow_font_downloads: None,
            local_machine_zone_allow_less_privileged_sites: None,
            local_machine_zone_allow_netframework_reliant_components: None,
            local_machine_zone_allow_scriptlets: None,
            local_machine_zone_allow_smart_screen_ie: None,
            local_machine_zone_allow_user_data_persistence: None,
            local_machine_zone_do_not_run_antimalware_against_active_xcontrols: None,
            local_machine_zone_initialize_and_script_active_xcontrols: None,
            local_machine_zone_java_permissions: None,
            local_machine_zone_navigate_windows_and_frames: None,
            locked_down_internet_zone_allow_access_to_data_sources: None,
            locked_down_internet_zone_allow_automatic_prompting_for_active_xcontrols: None,
            locked_down_internet_zone_allow_automatic_prompting_for_file_downloads: None,
            locked_down_internet_zone_allow_font_downloads: None,
            locked_down_internet_zone_allow_less_privileged_sites: None,
            locked_down_internet_zone_allow_netframework_reliant_components: None,
            locked_down_internet_zone_allow_scriptlets: None,
            locked_down_internet_zone_allow_smart_screen_ie: None,
            locked_down_internet_zone_allow_user_data_persistence: None,
            locked_down_internet_zone_initialize_and_script_active_xcontrols: None,
            locked_down_internet_zone_java_permissions: None,
            locked_down_internet_zone_navigate_windows_and_frames: None,
            locked_down_intranet_java_permissions: None,
            locked_down_intranet_zone_allow_access_to_data_sources: None,
            locked_down_intranet_zone_allow_automatic_prompting_for_active_xcontrols: None,
            locked_down_intranet_zone_allow_automatic_prompting_for_file_downloads: None,
            locked_down_intranet_zone_allow_font_downloads: None,
            locked_down_intranet_zone_allow_less_privileged_sites: None,
            locked_down_intranet_zone_allow_netframework_reliant_components: None,
            locked_down_intranet_zone_allow_scriptlets: None,
            locked_down_intranet_zone_allow_smart_screen_ie: None,
            locked_down_intranet_zone_allow_user_data_persistence: None,
            locked_down_intranet_zone_initialize_and_script_active_xcontrols: None,
            locked_down_intranet_zone_navigate_windows_and_frames: None,
            locked_down_local_machine_zone_allow_access_to_data_sources: None,
            locked_down_local_machine_zone_allow_automatic_prompting_for_active_xcontrols: None,
            locked_down_local_machine_zone_allow_automatic_prompting_for_file_downloads: None,
            locked_down_local_machine_zone_allow_font_downloads: None,
            locked_down_local_machine_zone_allow_less_privileged_sites: None,
            locked_down_local_machine_zone_allow_netframework_reliant_components: None,
            locked_down_local_machine_zone_allow_scriptlets: None,
            locked_down_local_machine_zone_allow_smart_screen_ie: None,
            locked_down_local_machine_zone_allow_user_data_persistence: None,
            locked_down_local_machine_zone_initialize_and_script_active_xcontrols: None,
            locked_down_local_machine_zone_java_permissions: None,
            locked_down_local_machine_zone_navigate_windows_and_frames: None,
            locked_down_restricted_sites_zone_allow_access_to_data_sources: None,
            locked_down_restricted_sites_zone_allow_automatic_prompting_for_active_xcontrols: None,
            locked_down_restricted_sites_zone_allow_automatic_prompting_for_file_downloads: None,
            locked_down_restricted_sites_zone_allow_font_downloads: None,
            locked_down_restricted_sites_zone_allow_less_privileged_sites: None,
            locked_down_restricted_sites_zone_allow_netframework_reliant_components: None,
            locked_down_restricted_sites_zone_allow_scriptlets: None,
            locked_down_restricted_sites_zone_allow_smart_screen_ie: None,
            locked_down_restricted_sites_zone_allow_user_data_persistence: None,
            locked_down_restricted_sites_zone_initialize_and_script_active_xcontrols: None,
            locked_down_restricted_sites_zone_java_permissions: None,
            locked_down_restricted_sites_zone_navigate_windows_and_frames: None,
            locked_down_trusted_sites_zone_allow_access_to_data_sources: None,
            locked_down_trusted_sites_zone_allow_automatic_prompting_for_active_xcontrols: None,
            locked_down_trusted_sites_zone_allow_automatic_prompting_for_file_downloads: None,
            locked_down_trusted_sites_zone_allow_font_downloads: None,
            locked_down_trusted_sites_zone_allow_less_privileged_sites: None,
            locked_down_trusted_sites_zone_allow_netframework_reliant_components: None,
            locked_down_trusted_sites_zone_allow_scriptlets: None,
            locked_down_trusted_sites_zone_allow_smart_screen_ie: None,
            locked_down_trusted_sites_zone_allow_user_data_persistence: None,
            locked_down_trusted_sites_zone_initialize_and_script_active_xcontrols: None,
            locked_down_trusted_sites_zone_java_permissions: None,
            locked_down_trusted_sites_zone_navigate_windows_and_frames: None,
            mime_sniffing_safety_feature_internet_explorer_processes: None,
            mkprotocol_security_restriction_internet_explorer_processes: None,
            new_tab_default_page: None,
            notification_bar_internet_explorer_processes: None,
            parent_id: None,
            prevent_managing_smart_screen_filter: None,
            prevent_per_user_installation_of_active_xcontrols: None,
            protection_from_zone_elevation_internet_explorer_processes: None,
            remove_run_this_time_button_for_outdated_active_xcontrols: None,
            restrict_active_xinstall_internet_explorer_processes: None,
            restricted_sites_zone_allow_access_to_data_sources: None,
            restricted_sites_zone_allow_active_scripting: None,
            restricted_sites_zone_allow_automatic_prompting_for_active_xcontrols: None,
            restricted_sites_zone_allow_automatic_prompting_for_file_downloads: None,
            restricted_sites_zone_allow_binary_and_script_behaviors: None,
            restricted_sites_zone_allow_copy_paste_via_script: None,
            restricted_sites_zone_allow_drag_and_drop_copy_and_paste_files: None,
            restricted_sites_zone_allow_file_downloads: None,
            restricted_sites_zone_allow_font_downloads: None,
            restricted_sites_zone_allow_less_privileged_sites: None,
            restricted_sites_zone_allow_loading_of_xamlfiles: None,
            restricted_sites_zone_allow_metarefresh: None,
            restricted_sites_zone_allow_netframework_reliant_components: None,
            restricted_sites_zone_allow_only_approved_domains_to_use_active_xcontrols: None,
            restricted_sites_zone_allow_only_approved_domains_to_use_tdcactive_xcontrol: None,
            restricted_sites_zone_allow_scripting_of_internet_explorer_web_browser_controls: None,
            restricted_sites_zone_allow_script_initiated_windows: None,
            restricted_sites_zone_allow_scriptlets: None,
            restricted_sites_zone_allow_smart_screen_ie: None,
            restricted_sites_zone_allow_updates_to_status_bar_via_script: None,
            restricted_sites_zone_allow_user_data_persistence: None,
            restricted_sites_zone_allow_vbscript_to_run_in_internet_explorer: None,
            restricted_sites_zone_do_not_run_antimalware_against_active_xcontrols: None,
            restricted_sites_zone_download_signed_active_xcontrols: None,
            restricted_sites_zone_download_unsigned_active_xcontrols: None,
            restricted_sites_zone_enable_cross_site_scripting_filter: None,
            restricted_sites_zone_enable_dragging_of_content_from_different_domains_across_windows: None,
            restricted_sites_zone_enable_dragging_of_content_from_different_domains_within_windows: None,
            restricted_sites_zone_enable_mimesniffing: None,
            restricted_sites_zone_include_local_path_when_uploading_files_to_server: None,
            restricted_sites_zone_initialize_and_script_active_xcontrols: None,
            restricted_sites_zone_java_permissions: None,
            restricted_sites_zone_launching_applications_and_files_in_iframe: None,
            restricted_sites_zone_logon_options: None,
            restricted_sites_zone_navigate_windows_and_frames: None,
            restricted_sites_zone_run_active_xcontrols_and_plugins: None,
            restricted_sites_zone_run_netframework_reliant_components_signed_with_authenticode: None,
            restricted_sites_zone_script_active_xcontrols_marked_safe_for_scripting: None,
            restricted_sites_zone_scripting_of_java_applets: None,
            restricted_sites_zone_show_security_warning_for_potentially_unsafe_files: None,
            restricted_sites_zone_turn_on_protected_mode: None,
            restricted_sites_zone_use_popup_blocker: None,
            restrict_file_download_internet_explorer_processes: None,
            scripted_window_security_restrictions_internet_explorer_processes: None,
            search_provider_list: None,
            send_sites_not_in_enterprise_site_list_to_edge: None,
            specify_use_of_active_xinstaller_service: None,
            trusted_sites_zone_allow_access_to_data_sources: None,
            trusted_sites_zone_allow_automatic_prompting_for_active_xcontrols: None,
            trusted_sites_zone_allow_automatic_prompting_for_file_downloads: None,
            trusted_sites_zone_allow_font_downloads: None,
            trusted_sites_zone_allow_less_privileged_sites: None,
            trusted_sites_zone_allow_netframework_reliant_components: None,
            trusted_sites_zone_allow_scriptlets: None,
            trusted_sites_zone_allow_smart_screen_ie: None,
            trusted_sites_zone_allow_user_data_persistence: None,
            trusted_sites_zone_do_not_run_antimalware_against_active_xcontrols: None,
            trusted_sites_zone_initialize_and_script_active_xcontrols: None,
            trusted_sites_zone_java_permissions: None,
            trusted_sites_zone_navigate_windows_and_frames: None,
        }
    }


    /// Sets the value of AddSearchProvider
    pub fn set_add_search_provider(&mut self, value: String) {
        self.add_search_provider = Some(value);
    }

    /// Gets the value of AddSearchProvider
    pub fn get_add_search_provider(&self) -> Option<&String> {
        self.add_search_provider.as_ref()
    }

    /// Sets the value of AllowActiveXFiltering
    pub fn set_allow_active_xfiltering(&mut self, value: String) {
        self.allow_active_xfiltering = Some(value);
    }

    /// Gets the value of AllowActiveXFiltering
    pub fn get_allow_active_xfiltering(&self) -> Option<&String> {
        self.allow_active_xfiltering.as_ref()
    }

    /// Sets the value of AllowAddOnList
    pub fn set_allow_add_on_list(&mut self, value: String) {
        self.allow_add_on_list = Some(value);
    }

    /// Gets the value of AllowAddOnList
    pub fn get_allow_add_on_list(&self) -> Option<&String> {
        self.allow_add_on_list.as_ref()
    }

    /// Sets the value of AllowAutoComplete
    pub fn set_allow_auto_complete(&mut self, value: String) {
        self.allow_auto_complete = Some(value);
    }

    /// Gets the value of AllowAutoComplete
    pub fn get_allow_auto_complete(&self) -> Option<&String> {
        self.allow_auto_complete.as_ref()
    }

    /// Sets the value of AllowCertificateAddressMismatchWarning
    pub fn set_allow_certificate_address_mismatch_warning(&mut self, value: String) {
        self.allow_certificate_address_mismatch_warning = Some(value);
    }

    /// Gets the value of AllowCertificateAddressMismatchWarning
    pub fn get_allow_certificate_address_mismatch_warning(&self) -> Option<&String> {
        self.allow_certificate_address_mismatch_warning.as_ref()
    }

    /// Sets the value of AllowDeletingBrowsingHistoryOnExit
    pub fn set_allow_deleting_browsing_history_on_exit(&mut self, value: String) {
        self.allow_deleting_browsing_history_on_exit = Some(value);
    }

    /// Gets the value of AllowDeletingBrowsingHistoryOnExit
    pub fn get_allow_deleting_browsing_history_on_exit(&self) -> Option<&String> {
        self.allow_deleting_browsing_history_on_exit.as_ref()
    }

    /// Sets the value of AllowEnhancedProtectedMode
    pub fn set_allow_enhanced_protected_mode(&mut self, value: String) {
        self.allow_enhanced_protected_mode = Some(value);
    }

    /// Gets the value of AllowEnhancedProtectedMode
    pub fn get_allow_enhanced_protected_mode(&self) -> Option<&String> {
        self.allow_enhanced_protected_mode.as_ref()
    }

    /// Sets the value of AllowEnhancedSuggestionsInAddressBar
    pub fn set_allow_enhanced_suggestions_in_address_bar(&mut self, value: String) {
        self.allow_enhanced_suggestions_in_address_bar = Some(value);
    }

    /// Gets the value of AllowEnhancedSuggestionsInAddressBar
    pub fn get_allow_enhanced_suggestions_in_address_bar(&self) -> Option<&String> {
        self.allow_enhanced_suggestions_in_address_bar.as_ref()
    }

    /// Sets the value of AllowEnterpriseModeFromToolsMenu
    pub fn set_allow_enterprise_mode_from_tools_menu(&mut self, value: String) {
        self.allow_enterprise_mode_from_tools_menu = Some(value);
    }

    /// Gets the value of AllowEnterpriseModeFromToolsMenu
    pub fn get_allow_enterprise_mode_from_tools_menu(&self) -> Option<&String> {
        self.allow_enterprise_mode_from_tools_menu.as_ref()
    }

    /// Sets the value of AllowEnterpriseModeSiteList
    pub fn set_allow_enterprise_mode_site_list(&mut self, value: String) {
        self.allow_enterprise_mode_site_list = Some(value);
    }

    /// Gets the value of AllowEnterpriseModeSiteList
    pub fn get_allow_enterprise_mode_site_list(&self) -> Option<&String> {
        self.allow_enterprise_mode_site_list.as_ref()
    }

    /// Sets the value of AllowInternetExplorer7PolicyList
    pub fn set_allow_internet_explorer7_policy_list(&mut self, value: String) {
        self.allow_internet_explorer7_policy_list = Some(value);
    }

    /// Gets the value of AllowInternetExplorer7PolicyList
    pub fn get_allow_internet_explorer7_policy_list(&self) -> Option<&String> {
        self.allow_internet_explorer7_policy_list.as_ref()
    }

    /// Sets the value of AllowInternetExplorerStandardsMode
    pub fn set_allow_internet_explorer_standards_mode(&mut self, value: String) {
        self.allow_internet_explorer_standards_mode = Some(value);
    }

    /// Gets the value of AllowInternetExplorerStandardsMode
    pub fn get_allow_internet_explorer_standards_mode(&self) -> Option<&String> {
        self.allow_internet_explorer_standards_mode.as_ref()
    }

    /// Sets the value of AllowInternetZoneTemplate
    pub fn set_allow_internet_zone_template(&mut self, value: String) {
        self.allow_internet_zone_template = Some(value);
    }

    /// Gets the value of AllowInternetZoneTemplate
    pub fn get_allow_internet_zone_template(&self) -> Option<&String> {
        self.allow_internet_zone_template.as_ref()
    }

    /// Sets the value of AllowIntranetZoneTemplate
    pub fn set_allow_intranet_zone_template(&mut self, value: String) {
        self.allow_intranet_zone_template = Some(value);
    }

    /// Gets the value of AllowIntranetZoneTemplate
    pub fn get_allow_intranet_zone_template(&self) -> Option<&String> {
        self.allow_intranet_zone_template.as_ref()
    }

    /// Sets the value of AllowLocalMachineZoneTemplate
    pub fn set_allow_local_machine_zone_template(&mut self, value: String) {
        self.allow_local_machine_zone_template = Some(value);
    }

    /// Gets the value of AllowLocalMachineZoneTemplate
    pub fn get_allow_local_machine_zone_template(&self) -> Option<&String> {
        self.allow_local_machine_zone_template.as_ref()
    }

    /// Sets the value of AllowLockedDownInternetZoneTemplate
    pub fn set_allow_locked_down_internet_zone_template(&mut self, value: String) {
        self.allow_locked_down_internet_zone_template = Some(value);
    }

    /// Gets the value of AllowLockedDownInternetZoneTemplate
    pub fn get_allow_locked_down_internet_zone_template(&self) -> Option<&String> {
        self.allow_locked_down_internet_zone_template.as_ref()
    }

    /// Sets the value of AllowLockedDownIntranetZoneTemplate
    pub fn set_allow_locked_down_intranet_zone_template(&mut self, value: String) {
        self.allow_locked_down_intranet_zone_template = Some(value);
    }

    /// Gets the value of AllowLockedDownIntranetZoneTemplate
    pub fn get_allow_locked_down_intranet_zone_template(&self) -> Option<&String> {
        self.allow_locked_down_intranet_zone_template.as_ref()
    }

    /// Sets the value of AllowLockedDownLocalMachineZoneTemplate
    pub fn set_allow_locked_down_local_machine_zone_template(&mut self, value: String) {
        self.allow_locked_down_local_machine_zone_template = Some(value);
    }

    /// Gets the value of AllowLockedDownLocalMachineZoneTemplate
    pub fn get_allow_locked_down_local_machine_zone_template(&self) -> Option<&String> {
        self.allow_locked_down_local_machine_zone_template.as_ref()
    }

    /// Sets the value of AllowLockedDownRestrictedSitesZoneTemplate
    pub fn set_allow_locked_down_restricted_sites_zone_template(&mut self, value: String) {
        self.allow_locked_down_restricted_sites_zone_template = Some(value);
    }

    /// Gets the value of AllowLockedDownRestrictedSitesZoneTemplate
    pub fn get_allow_locked_down_restricted_sites_zone_template(&self) -> Option<&String> {
        self.allow_locked_down_restricted_sites_zone_template.as_ref()
    }

    /// Sets the value of AllowOneWordEntry
    pub fn set_allow_one_word_entry(&mut self, value: String) {
        self.allow_one_word_entry = Some(value);
    }

    /// Gets the value of AllowOneWordEntry
    pub fn get_allow_one_word_entry(&self) -> Option<&String> {
        self.allow_one_word_entry.as_ref()
    }

    /// Sets the value of AllowSaveTargetAsInIEMode
    pub fn set_allow_save_target_as_in_iemode(&mut self, value: String) {
        self.allow_save_target_as_in_iemode = Some(value);
    }

    /// Gets the value of AllowSaveTargetAsInIEMode
    pub fn get_allow_save_target_as_in_iemode(&self) -> Option<&String> {
        self.allow_save_target_as_in_iemode.as_ref()
    }

    /// Sets the value of AllowSiteToZoneAssignmentList
    pub fn set_allow_site_to_zone_assignment_list(&mut self, value: String) {
        self.allow_site_to_zone_assignment_list = Some(value);
    }

    /// Gets the value of AllowSiteToZoneAssignmentList
    pub fn get_allow_site_to_zone_assignment_list(&self) -> Option<&String> {
        self.allow_site_to_zone_assignment_list.as_ref()
    }

    /// Sets the value of AllowsLockedDownTrustedSitesZoneTemplate
    pub fn set_allows_locked_down_trusted_sites_zone_template(&mut self, value: String) {
        self.allows_locked_down_trusted_sites_zone_template = Some(value);
    }

    /// Gets the value of AllowsLockedDownTrustedSitesZoneTemplate
    pub fn get_allows_locked_down_trusted_sites_zone_template(&self) -> Option<&String> {
        self.allows_locked_down_trusted_sites_zone_template.as_ref()
    }

    /// Sets the value of AllowSoftwareWhenSignatureIsInvalid
    pub fn set_allow_software_when_signature_is_invalid(&mut self, value: String) {
        self.allow_software_when_signature_is_invalid = Some(value);
    }

    /// Gets the value of AllowSoftwareWhenSignatureIsInvalid
    pub fn get_allow_software_when_signature_is_invalid(&self) -> Option<&String> {
        self.allow_software_when_signature_is_invalid.as_ref()
    }

    /// Sets the value of AllowsRestrictedSitesZoneTemplate
    pub fn set_allows_restricted_sites_zone_template(&mut self, value: String) {
        self.allows_restricted_sites_zone_template = Some(value);
    }

    /// Gets the value of AllowsRestrictedSitesZoneTemplate
    pub fn get_allows_restricted_sites_zone_template(&self) -> Option<&String> {
        self.allows_restricted_sites_zone_template.as_ref()
    }

    /// Sets the value of AllowSuggestedSites
    pub fn set_allow_suggested_sites(&mut self, value: String) {
        self.allow_suggested_sites = Some(value);
    }

    /// Gets the value of AllowSuggestedSites
    pub fn get_allow_suggested_sites(&self) -> Option<&String> {
        self.allow_suggested_sites.as_ref()
    }

    /// Sets the value of AllowTrustedSitesZoneTemplate
    pub fn set_allow_trusted_sites_zone_template(&mut self, value: String) {
        self.allow_trusted_sites_zone_template = Some(value);
    }

    /// Gets the value of AllowTrustedSitesZoneTemplate
    pub fn get_allow_trusted_sites_zone_template(&self) -> Option<&String> {
        self.allow_trusted_sites_zone_template.as_ref()
    }

    /// Sets the value of CheckServerCertificateRevocation
    pub fn set_check_server_certificate_revocation(&mut self, value: String) {
        self.check_server_certificate_revocation = Some(value);
    }

    /// Gets the value of CheckServerCertificateRevocation
    pub fn get_check_server_certificate_revocation(&self) -> Option<&String> {
        self.check_server_certificate_revocation.as_ref()
    }

    /// Sets the value of CheckSignaturesOnDownloadedPrograms
    pub fn set_check_signatures_on_downloaded_programs(&mut self, value: String) {
        self.check_signatures_on_downloaded_programs = Some(value);
    }

    /// Gets the value of CheckSignaturesOnDownloadedPrograms
    pub fn get_check_signatures_on_downloaded_programs(&self) -> Option<&String> {
        self.check_signatures_on_downloaded_programs.as_ref()
    }

    /// Sets the value of ConfigureEdgeRedirectChannel
    pub fn set_configure_edge_redirect_channel(&mut self, value: String) {
        self.configure_edge_redirect_channel = Some(value);
    }

    /// Gets the value of ConfigureEdgeRedirectChannel
    pub fn get_configure_edge_redirect_channel(&self) -> Option<&String> {
        self.configure_edge_redirect_channel.as_ref()
    }

    /// Sets the value of ConsistentMimeHandlingInternetExplorerProcesses
    pub fn set_consistent_mime_handling_internet_explorer_processes(&mut self, value: String) {
        self.consistent_mime_handling_internet_explorer_processes = Some(value);
    }

    /// Gets the value of ConsistentMimeHandlingInternetExplorerProcesses
    pub fn get_consistent_mime_handling_internet_explorer_processes(&self) -> Option<&String> {
        self.consistent_mime_handling_internet_explorer_processes.as_ref()
    }

    /// Sets the value of DisableActiveXVersionListAutoDownload
    pub fn set_disable_active_xversion_list_auto_download(&mut self, value: String) {
        self.disable_active_xversion_list_auto_download = Some(value);
    }

    /// Gets the value of DisableActiveXVersionListAutoDownload
    pub fn get_disable_active_xversion_list_auto_download(&self) -> Option<&String> {
        self.disable_active_xversion_list_auto_download.as_ref()
    }

    /// Sets the value of DisableBypassOfSmartScreenWarnings
    pub fn set_disable_bypass_of_smart_screen_warnings(&mut self, value: String) {
        self.disable_bypass_of_smart_screen_warnings = Some(value);
    }

    /// Gets the value of DisableBypassOfSmartScreenWarnings
    pub fn get_disable_bypass_of_smart_screen_warnings(&self) -> Option<&String> {
        self.disable_bypass_of_smart_screen_warnings.as_ref()
    }

    /// Sets the value of DisableBypassOfSmartScreenWarningsAboutUncommonFiles
    pub fn set_disable_bypass_of_smart_screen_warnings_about_uncommon_files(&mut self, value: String) {
        self.disable_bypass_of_smart_screen_warnings_about_uncommon_files = Some(value);
    }

    /// Gets the value of DisableBypassOfSmartScreenWarningsAboutUncommonFiles
    pub fn get_disable_bypass_of_smart_screen_warnings_about_uncommon_files(&self) -> Option<&String> {
        self.disable_bypass_of_smart_screen_warnings_about_uncommon_files.as_ref()
    }

    /// Sets the value of DisableCompatView
    pub fn set_disable_compat_view(&mut self, value: String) {
        self.disable_compat_view = Some(value);
    }

    /// Gets the value of DisableCompatView
    pub fn get_disable_compat_view(&self) -> Option<&String> {
        self.disable_compat_view.as_ref()
    }

    /// Sets the value of DisableConfiguringHistory
    pub fn set_disable_configuring_history(&mut self, value: String) {
        self.disable_configuring_history = Some(value);
    }

    /// Gets the value of DisableConfiguringHistory
    pub fn get_disable_configuring_history(&self) -> Option<&String> {
        self.disable_configuring_history.as_ref()
    }

    /// Sets the value of DisableCrashDetection
    pub fn set_disable_crash_detection(&mut self, value: String) {
        self.disable_crash_detection = Some(value);
    }

    /// Gets the value of DisableCrashDetection
    pub fn get_disable_crash_detection(&self) -> Option<&String> {
        self.disable_crash_detection.as_ref()
    }

    /// Sets the value of DisableCustomerExperienceImprovementProgramParticipation
    pub fn set_disable_customer_experience_improvement_program_participation(&mut self, value: String) {
        self.disable_customer_experience_improvement_program_participation = Some(value);
    }

    /// Gets the value of DisableCustomerExperienceImprovementProgramParticipation
    pub fn get_disable_customer_experience_improvement_program_participation(&self) -> Option<&String> {
        self.disable_customer_experience_improvement_program_participation.as_ref()
    }

    /// Sets the value of DisableDeletingUserVisitedWebsites
    pub fn set_disable_deleting_user_visited_websites(&mut self, value: String) {
        self.disable_deleting_user_visited_websites = Some(value);
    }

    /// Gets the value of DisableDeletingUserVisitedWebsites
    pub fn get_disable_deleting_user_visited_websites(&self) -> Option<&String> {
        self.disable_deleting_user_visited_websites.as_ref()
    }

    /// Sets the value of DisableEnclosureDownloading
    pub fn set_disable_enclosure_downloading(&mut self, value: String) {
        self.disable_enclosure_downloading = Some(value);
    }

    /// Gets the value of DisableEnclosureDownloading
    pub fn get_disable_enclosure_downloading(&self) -> Option<&String> {
        self.disable_enclosure_downloading.as_ref()
    }

    /// Sets the value of DisableEncryptionSupport
    pub fn set_disable_encryption_support(&mut self, value: String) {
        self.disable_encryption_support = Some(value);
    }

    /// Gets the value of DisableEncryptionSupport
    pub fn get_disable_encryption_support(&self) -> Option<&String> {
        self.disable_encryption_support.as_ref()
    }

    /// Sets the value of DisableFeedsBackgroundSync
    pub fn set_disable_feeds_background_sync(&mut self, value: String) {
        self.disable_feeds_background_sync = Some(value);
    }

    /// Gets the value of DisableFeedsBackgroundSync
    pub fn get_disable_feeds_background_sync(&self) -> Option<&String> {
        self.disable_feeds_background_sync.as_ref()
    }

    /// Sets the value of DisableFirstRunWizard
    pub fn set_disable_first_run_wizard(&mut self, value: String) {
        self.disable_first_run_wizard = Some(value);
    }

    /// Gets the value of DisableFirstRunWizard
    pub fn get_disable_first_run_wizard(&self) -> Option<&String> {
        self.disable_first_run_wizard.as_ref()
    }

    /// Sets the value of DisableFlipAheadFeature
    pub fn set_disable_flip_ahead_feature(&mut self, value: String) {
        self.disable_flip_ahead_feature = Some(value);
    }

    /// Gets the value of DisableFlipAheadFeature
    pub fn get_disable_flip_ahead_feature(&self) -> Option<&String> {
        self.disable_flip_ahead_feature.as_ref()
    }

    /// Sets the value of DisableGeolocation
    pub fn set_disable_geolocation(&mut self, value: String) {
        self.disable_geolocation = Some(value);
    }

    /// Gets the value of DisableGeolocation
    pub fn get_disable_geolocation(&self) -> Option<&String> {
        self.disable_geolocation.as_ref()
    }

    /// Sets the value of DisableHomePageChange
    pub fn set_disable_home_page_change(&mut self, value: String) {
        self.disable_home_page_change = Some(value);
    }

    /// Gets the value of DisableHomePageChange
    pub fn get_disable_home_page_change(&self) -> Option<&String> {
        self.disable_home_page_change.as_ref()
    }

    /// Sets the value of DisableIgnoringCertificateErrors
    pub fn set_disable_ignoring_certificate_errors(&mut self, value: String) {
        self.disable_ignoring_certificate_errors = Some(value);
    }

    /// Gets the value of DisableIgnoringCertificateErrors
    pub fn get_disable_ignoring_certificate_errors(&self) -> Option<&String> {
        self.disable_ignoring_certificate_errors.as_ref()
    }

    /// Sets the value of DisableInPrivateBrowsing
    pub fn set_disable_in_private_browsing(&mut self, value: String) {
        self.disable_in_private_browsing = Some(value);
    }

    /// Gets the value of DisableInPrivateBrowsing
    pub fn get_disable_in_private_browsing(&self) -> Option<&String> {
        self.disable_in_private_browsing.as_ref()
    }

    /// Sets the value of DisableInternetExplorerApp
    pub fn set_disable_internet_explorer_app(&mut self, value: String) {
        self.disable_internet_explorer_app = Some(value);
    }

    /// Gets the value of DisableInternetExplorerApp
    pub fn get_disable_internet_explorer_app(&self) -> Option<&String> {
        self.disable_internet_explorer_app.as_ref()
    }

    /// Sets the value of DisableProcessesInEnhancedProtectedMode
    pub fn set_disable_processes_in_enhanced_protected_mode(&mut self, value: String) {
        self.disable_processes_in_enhanced_protected_mode = Some(value);
    }

    /// Gets the value of DisableProcessesInEnhancedProtectedMode
    pub fn get_disable_processes_in_enhanced_protected_mode(&self) -> Option<&String> {
        self.disable_processes_in_enhanced_protected_mode.as_ref()
    }

    /// Sets the value of DisableProxyChange
    pub fn set_disable_proxy_change(&mut self, value: String) {
        self.disable_proxy_change = Some(value);
    }

    /// Gets the value of DisableProxyChange
    pub fn get_disable_proxy_change(&self) -> Option<&String> {
        self.disable_proxy_change.as_ref()
    }

    /// Sets the value of DisableSearchProviderChange
    pub fn set_disable_search_provider_change(&mut self, value: String) {
        self.disable_search_provider_change = Some(value);
    }

    /// Gets the value of DisableSearchProviderChange
    pub fn get_disable_search_provider_change(&self) -> Option<&String> {
        self.disable_search_provider_change.as_ref()
    }

    /// Sets the value of DisableSecondaryHomePageChange
    pub fn set_disable_secondary_home_page_change(&mut self, value: String) {
        self.disable_secondary_home_page_change = Some(value);
    }

    /// Gets the value of DisableSecondaryHomePageChange
    pub fn get_disable_secondary_home_page_change(&self) -> Option<&String> {
        self.disable_secondary_home_page_change.as_ref()
    }

    /// Sets the value of DisableSecuritySettingsCheck
    pub fn set_disable_security_settings_check(&mut self, value: String) {
        self.disable_security_settings_check = Some(value);
    }

    /// Gets the value of DisableSecuritySettingsCheck
    pub fn get_disable_security_settings_check(&self) -> Option<&String> {
        self.disable_security_settings_check.as_ref()
    }

    /// Sets the value of DisableWebAddressAutoComplete
    pub fn set_disable_web_address_auto_complete(&mut self, value: String) {
        self.disable_web_address_auto_complete = Some(value);
    }

    /// Gets the value of DisableWebAddressAutoComplete
    pub fn get_disable_web_address_auto_complete(&self) -> Option<&String> {
        self.disable_web_address_auto_complete.as_ref()
    }

    /// Sets the value of DoNotAllowActiveXControlsInProtectedMode
    pub fn set_do_not_allow_active_xcontrols_in_protected_mode(&mut self, value: String) {
        self.do_not_allow_active_xcontrols_in_protected_mode = Some(value);
    }

    /// Gets the value of DoNotAllowActiveXControlsInProtectedMode
    pub fn get_do_not_allow_active_xcontrols_in_protected_mode(&self) -> Option<&String> {
        self.do_not_allow_active_xcontrols_in_protected_mode.as_ref()
    }

    /// Sets the value of DoNotBlockOutdatedActiveXControls
    pub fn set_do_not_block_outdated_active_xcontrols(&mut self, value: String) {
        self.do_not_block_outdated_active_xcontrols = Some(value);
    }

    /// Gets the value of DoNotBlockOutdatedActiveXControls
    pub fn get_do_not_block_outdated_active_xcontrols(&self) -> Option<&String> {
        self.do_not_block_outdated_active_xcontrols.as_ref()
    }

    /// Sets the value of DoNotBlockOutdatedActiveXControlsOnSpecificDomains
    pub fn set_do_not_block_outdated_active_xcontrols_on_specific_domains(&mut self, value: String) {
        self.do_not_block_outdated_active_xcontrols_on_specific_domains = Some(value);
    }

    /// Gets the value of DoNotBlockOutdatedActiveXControlsOnSpecificDomains
    pub fn get_do_not_block_outdated_active_xcontrols_on_specific_domains(&self) -> Option<&String> {
        self.do_not_block_outdated_active_xcontrols_on_specific_domains.as_ref()
    }

    /// Sets the value of EnableExtendedIEModeHotkeys
    pub fn set_enable_extended_iemode_hotkeys(&mut self, value: String) {
        self.enable_extended_iemode_hotkeys = Some(value);
    }

    /// Gets the value of EnableExtendedIEModeHotkeys
    pub fn get_enable_extended_iemode_hotkeys(&self) -> Option<&String> {
        self.enable_extended_iemode_hotkeys.as_ref()
    }

    /// Sets the value of IncludeAllLocalSites
    pub fn set_include_all_local_sites(&mut self, value: String) {
        self.include_all_local_sites = Some(value);
    }

    /// Gets the value of IncludeAllLocalSites
    pub fn get_include_all_local_sites(&self) -> Option<&String> {
        self.include_all_local_sites.as_ref()
    }

    /// Sets the value of IncludeAllNetworkPaths
    pub fn set_include_all_network_paths(&mut self, value: String) {
        self.include_all_network_paths = Some(value);
    }

    /// Gets the value of IncludeAllNetworkPaths
    pub fn get_include_all_network_paths(&self) -> Option<&String> {
        self.include_all_network_paths.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of InternetZoneAllowAccessToDataSources
    pub fn set_internet_zone_allow_access_to_data_sources(&mut self, value: String) {
        self.internet_zone_allow_access_to_data_sources = Some(value);
    }

    /// Gets the value of InternetZoneAllowAccessToDataSources
    pub fn get_internet_zone_allow_access_to_data_sources(&self) -> Option<&String> {
        self.internet_zone_allow_access_to_data_sources.as_ref()
    }

    /// Sets the value of InternetZoneAllowAutomaticPromptingForActiveXControls
    pub fn set_internet_zone_allow_automatic_prompting_for_active_xcontrols(&mut self, value: String) {
        self.internet_zone_allow_automatic_prompting_for_active_xcontrols = Some(value);
    }

    /// Gets the value of InternetZoneAllowAutomaticPromptingForActiveXControls
    pub fn get_internet_zone_allow_automatic_prompting_for_active_xcontrols(&self) -> Option<&String> {
        self.internet_zone_allow_automatic_prompting_for_active_xcontrols.as_ref()
    }

    /// Sets the value of InternetZoneAllowAutomaticPromptingForFileDownloads
    pub fn set_internet_zone_allow_automatic_prompting_for_file_downloads(&mut self, value: String) {
        self.internet_zone_allow_automatic_prompting_for_file_downloads = Some(value);
    }

    /// Gets the value of InternetZoneAllowAutomaticPromptingForFileDownloads
    pub fn get_internet_zone_allow_automatic_prompting_for_file_downloads(&self) -> Option<&String> {
        self.internet_zone_allow_automatic_prompting_for_file_downloads.as_ref()
    }

    /// Sets the value of InternetZoneAllowCopyPasteViaScript
    pub fn set_internet_zone_allow_copy_paste_via_script(&mut self, value: String) {
        self.internet_zone_allow_copy_paste_via_script = Some(value);
    }

    /// Gets the value of InternetZoneAllowCopyPasteViaScript
    pub fn get_internet_zone_allow_copy_paste_via_script(&self) -> Option<&String> {
        self.internet_zone_allow_copy_paste_via_script.as_ref()
    }

    /// Sets the value of InternetZoneAllowDragAndDropCopyAndPasteFiles
    pub fn set_internet_zone_allow_drag_and_drop_copy_and_paste_files(&mut self, value: String) {
        self.internet_zone_allow_drag_and_drop_copy_and_paste_files = Some(value);
    }

    /// Gets the value of InternetZoneAllowDragAndDropCopyAndPasteFiles
    pub fn get_internet_zone_allow_drag_and_drop_copy_and_paste_files(&self) -> Option<&String> {
        self.internet_zone_allow_drag_and_drop_copy_and_paste_files.as_ref()
    }

    /// Sets the value of InternetZoneAllowFontDownloads
    pub fn set_internet_zone_allow_font_downloads(&mut self, value: String) {
        self.internet_zone_allow_font_downloads = Some(value);
    }

    /// Gets the value of InternetZoneAllowFontDownloads
    pub fn get_internet_zone_allow_font_downloads(&self) -> Option<&String> {
        self.internet_zone_allow_font_downloads.as_ref()
    }

    /// Sets the value of InternetZoneAllowLessPrivilegedSites
    pub fn set_internet_zone_allow_less_privileged_sites(&mut self, value: String) {
        self.internet_zone_allow_less_privileged_sites = Some(value);
    }

    /// Gets the value of InternetZoneAllowLessPrivilegedSites
    pub fn get_internet_zone_allow_less_privileged_sites(&self) -> Option<&String> {
        self.internet_zone_allow_less_privileged_sites.as_ref()
    }

    /// Sets the value of InternetZoneAllowLoadingOfXAMLFiles
    pub fn set_internet_zone_allow_loading_of_xamlfiles(&mut self, value: String) {
        self.internet_zone_allow_loading_of_xamlfiles = Some(value);
    }

    /// Gets the value of InternetZoneAllowLoadingOfXAMLFiles
    pub fn get_internet_zone_allow_loading_of_xamlfiles(&self) -> Option<&String> {
        self.internet_zone_allow_loading_of_xamlfiles.as_ref()
    }

    /// Sets the value of InternetZoneAllowNETFrameworkReliantComponents
    pub fn set_internet_zone_allow_netframework_reliant_components(&mut self, value: String) {
        self.internet_zone_allow_netframework_reliant_components = Some(value);
    }

    /// Gets the value of InternetZoneAllowNETFrameworkReliantComponents
    pub fn get_internet_zone_allow_netframework_reliant_components(&self) -> Option<&String> {
        self.internet_zone_allow_netframework_reliant_components.as_ref()
    }

    /// Sets the value of InternetZoneAllowOnlyApprovedDomainsToUseActiveXControls
    pub fn set_internet_zone_allow_only_approved_domains_to_use_active_xcontrols(&mut self, value: String) {
        self.internet_zone_allow_only_approved_domains_to_use_active_xcontrols = Some(value);
    }

    /// Gets the value of InternetZoneAllowOnlyApprovedDomainsToUseActiveXControls
    pub fn get_internet_zone_allow_only_approved_domains_to_use_active_xcontrols(&self) -> Option<&String> {
        self.internet_zone_allow_only_approved_domains_to_use_active_xcontrols.as_ref()
    }

    /// Sets the value of InternetZoneAllowOnlyApprovedDomainsToUseTDCActiveXControl
    pub fn set_internet_zone_allow_only_approved_domains_to_use_tdcactive_xcontrol(&mut self, value: String) {
        self.internet_zone_allow_only_approved_domains_to_use_tdcactive_xcontrol = Some(value);
    }

    /// Gets the value of InternetZoneAllowOnlyApprovedDomainsToUseTDCActiveXControl
    pub fn get_internet_zone_allow_only_approved_domains_to_use_tdcactive_xcontrol(&self) -> Option<&String> {
        self.internet_zone_allow_only_approved_domains_to_use_tdcactive_xcontrol.as_ref()
    }

    /// Sets the value of InternetZoneAllowScriptingOfInternetExplorerWebBrowserControls
    pub fn set_internet_zone_allow_scripting_of_internet_explorer_web_browser_controls(&mut self, value: String) {
        self.internet_zone_allow_scripting_of_internet_explorer_web_browser_controls = Some(value);
    }

    /// Gets the value of InternetZoneAllowScriptingOfInternetExplorerWebBrowserControls
    pub fn get_internet_zone_allow_scripting_of_internet_explorer_web_browser_controls(&self) -> Option<&String> {
        self.internet_zone_allow_scripting_of_internet_explorer_web_browser_controls.as_ref()
    }

    /// Sets the value of InternetZoneAllowScriptInitiatedWindows
    pub fn set_internet_zone_allow_script_initiated_windows(&mut self, value: String) {
        self.internet_zone_allow_script_initiated_windows = Some(value);
    }

    /// Gets the value of InternetZoneAllowScriptInitiatedWindows
    pub fn get_internet_zone_allow_script_initiated_windows(&self) -> Option<&String> {
        self.internet_zone_allow_script_initiated_windows.as_ref()
    }

    /// Sets the value of InternetZoneAllowScriptlets
    pub fn set_internet_zone_allow_scriptlets(&mut self, value: String) {
        self.internet_zone_allow_scriptlets = Some(value);
    }

    /// Gets the value of InternetZoneAllowScriptlets
    pub fn get_internet_zone_allow_scriptlets(&self) -> Option<&String> {
        self.internet_zone_allow_scriptlets.as_ref()
    }

    /// Sets the value of InternetZoneAllowSmartScreenIE
    pub fn set_internet_zone_allow_smart_screen_ie(&mut self, value: String) {
        self.internet_zone_allow_smart_screen_ie = Some(value);
    }

    /// Gets the value of InternetZoneAllowSmartScreenIE
    pub fn get_internet_zone_allow_smart_screen_ie(&self) -> Option<&String> {
        self.internet_zone_allow_smart_screen_ie.as_ref()
    }

    /// Sets the value of InternetZoneAllowUpdatesToStatusBarViaScript
    pub fn set_internet_zone_allow_updates_to_status_bar_via_script(&mut self, value: String) {
        self.internet_zone_allow_updates_to_status_bar_via_script = Some(value);
    }

    /// Gets the value of InternetZoneAllowUpdatesToStatusBarViaScript
    pub fn get_internet_zone_allow_updates_to_status_bar_via_script(&self) -> Option<&String> {
        self.internet_zone_allow_updates_to_status_bar_via_script.as_ref()
    }

    /// Sets the value of InternetZoneAllowUserDataPersistence
    pub fn set_internet_zone_allow_user_data_persistence(&mut self, value: String) {
        self.internet_zone_allow_user_data_persistence = Some(value);
    }

    /// Gets the value of InternetZoneAllowUserDataPersistence
    pub fn get_internet_zone_allow_user_data_persistence(&self) -> Option<&String> {
        self.internet_zone_allow_user_data_persistence.as_ref()
    }

    /// Sets the value of InternetZoneAllowVBScriptToRunInInternetExplorer
    pub fn set_internet_zone_allow_vbscript_to_run_in_internet_explorer(&mut self, value: String) {
        self.internet_zone_allow_vbscript_to_run_in_internet_explorer = Some(value);
    }

    /// Gets the value of InternetZoneAllowVBScriptToRunInInternetExplorer
    pub fn get_internet_zone_allow_vbscript_to_run_in_internet_explorer(&self) -> Option<&String> {
        self.internet_zone_allow_vbscript_to_run_in_internet_explorer.as_ref()
    }

    /// Sets the value of InternetZoneDoNotRunAntimalwareAgainstActiveXControls
    pub fn set_internet_zone_do_not_run_antimalware_against_active_xcontrols(&mut self, value: String) {
        self.internet_zone_do_not_run_antimalware_against_active_xcontrols = Some(value);
    }

    /// Gets the value of InternetZoneDoNotRunAntimalwareAgainstActiveXControls
    pub fn get_internet_zone_do_not_run_antimalware_against_active_xcontrols(&self) -> Option<&String> {
        self.internet_zone_do_not_run_antimalware_against_active_xcontrols.as_ref()
    }

    /// Sets the value of InternetZoneDownloadSignedActiveXControls
    pub fn set_internet_zone_download_signed_active_xcontrols(&mut self, value: String) {
        self.internet_zone_download_signed_active_xcontrols = Some(value);
    }

    /// Gets the value of InternetZoneDownloadSignedActiveXControls
    pub fn get_internet_zone_download_signed_active_xcontrols(&self) -> Option<&String> {
        self.internet_zone_download_signed_active_xcontrols.as_ref()
    }

    /// Sets the value of InternetZoneDownloadUnsignedActiveXControls
    pub fn set_internet_zone_download_unsigned_active_xcontrols(&mut self, value: String) {
        self.internet_zone_download_unsigned_active_xcontrols = Some(value);
    }

    /// Gets the value of InternetZoneDownloadUnsignedActiveXControls
    pub fn get_internet_zone_download_unsigned_active_xcontrols(&self) -> Option<&String> {
        self.internet_zone_download_unsigned_active_xcontrols.as_ref()
    }

    /// Sets the value of InternetZoneEnableCrossSiteScriptingFilter
    pub fn set_internet_zone_enable_cross_site_scripting_filter(&mut self, value: String) {
        self.internet_zone_enable_cross_site_scripting_filter = Some(value);
    }

    /// Gets the value of InternetZoneEnableCrossSiteScriptingFilter
    pub fn get_internet_zone_enable_cross_site_scripting_filter(&self) -> Option<&String> {
        self.internet_zone_enable_cross_site_scripting_filter.as_ref()
    }

    /// Sets the value of InternetZoneEnableDraggingOfContentFromDifferentDomainsAcrossWindows
    pub fn set_internet_zone_enable_dragging_of_content_from_different_domains_across_windows(&mut self, value: String) {
        self.internet_zone_enable_dragging_of_content_from_different_domains_across_windows = Some(value);
    }

    /// Gets the value of InternetZoneEnableDraggingOfContentFromDifferentDomainsAcrossWindows
    pub fn get_internet_zone_enable_dragging_of_content_from_different_domains_across_windows(&self) -> Option<&String> {
        self.internet_zone_enable_dragging_of_content_from_different_domains_across_windows.as_ref()
    }

    /// Sets the value of InternetZoneEnableDraggingOfContentFromDifferentDomainsWithinWindows
    pub fn set_internet_zone_enable_dragging_of_content_from_different_domains_within_windows(&mut self, value: String) {
        self.internet_zone_enable_dragging_of_content_from_different_domains_within_windows = Some(value);
    }

    /// Gets the value of InternetZoneEnableDraggingOfContentFromDifferentDomainsWithinWindows
    pub fn get_internet_zone_enable_dragging_of_content_from_different_domains_within_windows(&self) -> Option<&String> {
        self.internet_zone_enable_dragging_of_content_from_different_domains_within_windows.as_ref()
    }

    /// Sets the value of InternetZoneEnableMIMESniffing
    pub fn set_internet_zone_enable_mimesniffing(&mut self, value: String) {
        self.internet_zone_enable_mimesniffing = Some(value);
    }

    /// Gets the value of InternetZoneEnableMIMESniffing
    pub fn get_internet_zone_enable_mimesniffing(&self) -> Option<&String> {
        self.internet_zone_enable_mimesniffing.as_ref()
    }

    /// Sets the value of InternetZoneEnableProtectedMode
    pub fn set_internet_zone_enable_protected_mode(&mut self, value: String) {
        self.internet_zone_enable_protected_mode = Some(value);
    }

    /// Gets the value of InternetZoneEnableProtectedMode
    pub fn get_internet_zone_enable_protected_mode(&self) -> Option<&String> {
        self.internet_zone_enable_protected_mode.as_ref()
    }

    /// Sets the value of InternetZoneIncludeLocalPathWhenUploadingFilesToServer
    pub fn set_internet_zone_include_local_path_when_uploading_files_to_server(&mut self, value: String) {
        self.internet_zone_include_local_path_when_uploading_files_to_server = Some(value);
    }

    /// Gets the value of InternetZoneIncludeLocalPathWhenUploadingFilesToServer
    pub fn get_internet_zone_include_local_path_when_uploading_files_to_server(&self) -> Option<&String> {
        self.internet_zone_include_local_path_when_uploading_files_to_server.as_ref()
    }

    /// Sets the value of InternetZoneInitializeAndScriptActiveXControls
    pub fn set_internet_zone_initialize_and_script_active_xcontrols(&mut self, value: String) {
        self.internet_zone_initialize_and_script_active_xcontrols = Some(value);
    }

    /// Gets the value of InternetZoneInitializeAndScriptActiveXControls
    pub fn get_internet_zone_initialize_and_script_active_xcontrols(&self) -> Option<&String> {
        self.internet_zone_initialize_and_script_active_xcontrols.as_ref()
    }

    /// Sets the value of InternetZoneJavaPermissions
    pub fn set_internet_zone_java_permissions(&mut self, value: String) {
        self.internet_zone_java_permissions = Some(value);
    }

    /// Gets the value of InternetZoneJavaPermissions
    pub fn get_internet_zone_java_permissions(&self) -> Option<&String> {
        self.internet_zone_java_permissions.as_ref()
    }

    /// Sets the value of InternetZoneLaunchingApplicationsAndFilesInIFRAME
    pub fn set_internet_zone_launching_applications_and_files_in_iframe(&mut self, value: String) {
        self.internet_zone_launching_applications_and_files_in_iframe = Some(value);
    }

    /// Gets the value of InternetZoneLaunchingApplicationsAndFilesInIFRAME
    pub fn get_internet_zone_launching_applications_and_files_in_iframe(&self) -> Option<&String> {
        self.internet_zone_launching_applications_and_files_in_iframe.as_ref()
    }

    /// Sets the value of InternetZoneLogonOptions
    pub fn set_internet_zone_logon_options(&mut self, value: String) {
        self.internet_zone_logon_options = Some(value);
    }

    /// Gets the value of InternetZoneLogonOptions
    pub fn get_internet_zone_logon_options(&self) -> Option<&String> {
        self.internet_zone_logon_options.as_ref()
    }

    /// Sets the value of InternetZoneNavigateWindowsAndFrames
    pub fn set_internet_zone_navigate_windows_and_frames(&mut self, value: String) {
        self.internet_zone_navigate_windows_and_frames = Some(value);
    }

    /// Gets the value of InternetZoneNavigateWindowsAndFrames
    pub fn get_internet_zone_navigate_windows_and_frames(&self) -> Option<&String> {
        self.internet_zone_navigate_windows_and_frames.as_ref()
    }

    /// Sets the value of InternetZoneRunNETFrameworkReliantComponentsSignedWithAuthenticode
    pub fn set_internet_zone_run_netframework_reliant_components_signed_with_authenticode(&mut self, value: String) {
        self.internet_zone_run_netframework_reliant_components_signed_with_authenticode = Some(value);
    }

    /// Gets the value of InternetZoneRunNETFrameworkReliantComponentsSignedWithAuthenticode
    pub fn get_internet_zone_run_netframework_reliant_components_signed_with_authenticode(&self) -> Option<&String> {
        self.internet_zone_run_netframework_reliant_components_signed_with_authenticode.as_ref()
    }

    /// Sets the value of InternetZoneShowSecurityWarningForPotentiallyUnsafeFiles
    pub fn set_internet_zone_show_security_warning_for_potentially_unsafe_files(&mut self, value: String) {
        self.internet_zone_show_security_warning_for_potentially_unsafe_files = Some(value);
    }

    /// Gets the value of InternetZoneShowSecurityWarningForPotentiallyUnsafeFiles
    pub fn get_internet_zone_show_security_warning_for_potentially_unsafe_files(&self) -> Option<&String> {
        self.internet_zone_show_security_warning_for_potentially_unsafe_files.as_ref()
    }

    /// Sets the value of InternetZoneUsePopupBlocker
    pub fn set_internet_zone_use_popup_blocker(&mut self, value: String) {
        self.internet_zone_use_popup_blocker = Some(value);
    }

    /// Gets the value of InternetZoneUsePopupBlocker
    pub fn get_internet_zone_use_popup_blocker(&self) -> Option<&String> {
        self.internet_zone_use_popup_blocker.as_ref()
    }

    /// Sets the value of IntranetZoneAllowAccessToDataSources
    pub fn set_intranet_zone_allow_access_to_data_sources(&mut self, value: String) {
        self.intranet_zone_allow_access_to_data_sources = Some(value);
    }

    /// Gets the value of IntranetZoneAllowAccessToDataSources
    pub fn get_intranet_zone_allow_access_to_data_sources(&self) -> Option<&String> {
        self.intranet_zone_allow_access_to_data_sources.as_ref()
    }

    /// Sets the value of IntranetZoneAllowAutomaticPromptingForActiveXControls
    pub fn set_intranet_zone_allow_automatic_prompting_for_active_xcontrols(&mut self, value: String) {
        self.intranet_zone_allow_automatic_prompting_for_active_xcontrols = Some(value);
    }

    /// Gets the value of IntranetZoneAllowAutomaticPromptingForActiveXControls
    pub fn get_intranet_zone_allow_automatic_prompting_for_active_xcontrols(&self) -> Option<&String> {
        self.intranet_zone_allow_automatic_prompting_for_active_xcontrols.as_ref()
    }

    /// Sets the value of IntranetZoneAllowAutomaticPromptingForFileDownloads
    pub fn set_intranet_zone_allow_automatic_prompting_for_file_downloads(&mut self, value: String) {
        self.intranet_zone_allow_automatic_prompting_for_file_downloads = Some(value);
    }

    /// Gets the value of IntranetZoneAllowAutomaticPromptingForFileDownloads
    pub fn get_intranet_zone_allow_automatic_prompting_for_file_downloads(&self) -> Option<&String> {
        self.intranet_zone_allow_automatic_prompting_for_file_downloads.as_ref()
    }

    /// Sets the value of IntranetZoneAllowFontDownloads
    pub fn set_intranet_zone_allow_font_downloads(&mut self, value: String) {
        self.intranet_zone_allow_font_downloads = Some(value);
    }

    /// Gets the value of IntranetZoneAllowFontDownloads
    pub fn get_intranet_zone_allow_font_downloads(&self) -> Option<&String> {
        self.intranet_zone_allow_font_downloads.as_ref()
    }

    /// Sets the value of IntranetZoneAllowLessPrivilegedSites
    pub fn set_intranet_zone_allow_less_privileged_sites(&mut self, value: String) {
        self.intranet_zone_allow_less_privileged_sites = Some(value);
    }

    /// Gets the value of IntranetZoneAllowLessPrivilegedSites
    pub fn get_intranet_zone_allow_less_privileged_sites(&self) -> Option<&String> {
        self.intranet_zone_allow_less_privileged_sites.as_ref()
    }

    /// Sets the value of IntranetZoneAllowNETFrameworkReliantComponents
    pub fn set_intranet_zone_allow_netframework_reliant_components(&mut self, value: String) {
        self.intranet_zone_allow_netframework_reliant_components = Some(value);
    }

    /// Gets the value of IntranetZoneAllowNETFrameworkReliantComponents
    pub fn get_intranet_zone_allow_netframework_reliant_components(&self) -> Option<&String> {
        self.intranet_zone_allow_netframework_reliant_components.as_ref()
    }

    /// Sets the value of IntranetZoneAllowScriptlets
    pub fn set_intranet_zone_allow_scriptlets(&mut self, value: String) {
        self.intranet_zone_allow_scriptlets = Some(value);
    }

    /// Gets the value of IntranetZoneAllowScriptlets
    pub fn get_intranet_zone_allow_scriptlets(&self) -> Option<&String> {
        self.intranet_zone_allow_scriptlets.as_ref()
    }

    /// Sets the value of IntranetZoneAllowSmartScreenIE
    pub fn set_intranet_zone_allow_smart_screen_ie(&mut self, value: String) {
        self.intranet_zone_allow_smart_screen_ie = Some(value);
    }

    /// Gets the value of IntranetZoneAllowSmartScreenIE
    pub fn get_intranet_zone_allow_smart_screen_ie(&self) -> Option<&String> {
        self.intranet_zone_allow_smart_screen_ie.as_ref()
    }

    /// Sets the value of IntranetZoneAllowUserDataPersistence
    pub fn set_intranet_zone_allow_user_data_persistence(&mut self, value: String) {
        self.intranet_zone_allow_user_data_persistence = Some(value);
    }

    /// Gets the value of IntranetZoneAllowUserDataPersistence
    pub fn get_intranet_zone_allow_user_data_persistence(&self) -> Option<&String> {
        self.intranet_zone_allow_user_data_persistence.as_ref()
    }

    /// Sets the value of IntranetZoneDoNotRunAntimalwareAgainstActiveXControls
    pub fn set_intranet_zone_do_not_run_antimalware_against_active_xcontrols(&mut self, value: String) {
        self.intranet_zone_do_not_run_antimalware_against_active_xcontrols = Some(value);
    }

    /// Gets the value of IntranetZoneDoNotRunAntimalwareAgainstActiveXControls
    pub fn get_intranet_zone_do_not_run_antimalware_against_active_xcontrols(&self) -> Option<&String> {
        self.intranet_zone_do_not_run_antimalware_against_active_xcontrols.as_ref()
    }

    /// Sets the value of IntranetZoneInitializeAndScriptActiveXControls
    pub fn set_intranet_zone_initialize_and_script_active_xcontrols(&mut self, value: String) {
        self.intranet_zone_initialize_and_script_active_xcontrols = Some(value);
    }

    /// Gets the value of IntranetZoneInitializeAndScriptActiveXControls
    pub fn get_intranet_zone_initialize_and_script_active_xcontrols(&self) -> Option<&String> {
        self.intranet_zone_initialize_and_script_active_xcontrols.as_ref()
    }

    /// Sets the value of IntranetZoneJavaPermissions
    pub fn set_intranet_zone_java_permissions(&mut self, value: String) {
        self.intranet_zone_java_permissions = Some(value);
    }

    /// Gets the value of IntranetZoneJavaPermissions
    pub fn get_intranet_zone_java_permissions(&self) -> Option<&String> {
        self.intranet_zone_java_permissions.as_ref()
    }

    /// Sets the value of IntranetZoneNavigateWindowsAndFrames
    pub fn set_intranet_zone_navigate_windows_and_frames(&mut self, value: String) {
        self.intranet_zone_navigate_windows_and_frames = Some(value);
    }

    /// Gets the value of IntranetZoneNavigateWindowsAndFrames
    pub fn get_intranet_zone_navigate_windows_and_frames(&self) -> Option<&String> {
        self.intranet_zone_navigate_windows_and_frames.as_ref()
    }

    /// Sets the value of JScriptReplacement
    pub fn set_jscript_replacement(&mut self, value: String) {
        self.jscript_replacement = Some(value);
    }

    /// Gets the value of JScriptReplacement
    pub fn get_jscript_replacement(&self) -> Option<&String> {
        self.jscript_replacement.as_ref()
    }

    /// Sets the value of KeepIntranetSitesInInternetExplorer
    pub fn set_keep_intranet_sites_in_internet_explorer(&mut self, value: String) {
        self.keep_intranet_sites_in_internet_explorer = Some(value);
    }

    /// Gets the value of KeepIntranetSitesInInternetExplorer
    pub fn get_keep_intranet_sites_in_internet_explorer(&self) -> Option<&String> {
        self.keep_intranet_sites_in_internet_explorer.as_ref()
    }

    /// Sets the value of LocalMachineZoneAllowAccessToDataSources
    pub fn set_local_machine_zone_allow_access_to_data_sources(&mut self, value: String) {
        self.local_machine_zone_allow_access_to_data_sources = Some(value);
    }

    /// Gets the value of LocalMachineZoneAllowAccessToDataSources
    pub fn get_local_machine_zone_allow_access_to_data_sources(&self) -> Option<&String> {
        self.local_machine_zone_allow_access_to_data_sources.as_ref()
    }

    /// Sets the value of LocalMachineZoneAllowAutomaticPromptingForActiveXControls
    pub fn set_local_machine_zone_allow_automatic_prompting_for_active_xcontrols(&mut self, value: String) {
        self.local_machine_zone_allow_automatic_prompting_for_active_xcontrols = Some(value);
    }

    /// Gets the value of LocalMachineZoneAllowAutomaticPromptingForActiveXControls
    pub fn get_local_machine_zone_allow_automatic_prompting_for_active_xcontrols(&self) -> Option<&String> {
        self.local_machine_zone_allow_automatic_prompting_for_active_xcontrols.as_ref()
    }

    /// Sets the value of LocalMachineZoneAllowAutomaticPromptingForFileDownloads
    pub fn set_local_machine_zone_allow_automatic_prompting_for_file_downloads(&mut self, value: String) {
        self.local_machine_zone_allow_automatic_prompting_for_file_downloads = Some(value);
    }

    /// Gets the value of LocalMachineZoneAllowAutomaticPromptingForFileDownloads
    pub fn get_local_machine_zone_allow_automatic_prompting_for_file_downloads(&self) -> Option<&String> {
        self.local_machine_zone_allow_automatic_prompting_for_file_downloads.as_ref()
    }

    /// Sets the value of LocalMachineZoneAllowFontDownloads
    pub fn set_local_machine_zone_allow_font_downloads(&mut self, value: String) {
        self.local_machine_zone_allow_font_downloads = Some(value);
    }

    /// Gets the value of LocalMachineZoneAllowFontDownloads
    pub fn get_local_machine_zone_allow_font_downloads(&self) -> Option<&String> {
        self.local_machine_zone_allow_font_downloads.as_ref()
    }

    /// Sets the value of LocalMachineZoneAllowLessPrivilegedSites
    pub fn set_local_machine_zone_allow_less_privileged_sites(&mut self, value: String) {
        self.local_machine_zone_allow_less_privileged_sites = Some(value);
    }

    /// Gets the value of LocalMachineZoneAllowLessPrivilegedSites
    pub fn get_local_machine_zone_allow_less_privileged_sites(&self) -> Option<&String> {
        self.local_machine_zone_allow_less_privileged_sites.as_ref()
    }

    /// Sets the value of LocalMachineZoneAllowNETFrameworkReliantComponents
    pub fn set_local_machine_zone_allow_netframework_reliant_components(&mut self, value: String) {
        self.local_machine_zone_allow_netframework_reliant_components = Some(value);
    }

    /// Gets the value of LocalMachineZoneAllowNETFrameworkReliantComponents
    pub fn get_local_machine_zone_allow_netframework_reliant_components(&self) -> Option<&String> {
        self.local_machine_zone_allow_netframework_reliant_components.as_ref()
    }

    /// Sets the value of LocalMachineZoneAllowScriptlets
    pub fn set_local_machine_zone_allow_scriptlets(&mut self, value: String) {
        self.local_machine_zone_allow_scriptlets = Some(value);
    }

    /// Gets the value of LocalMachineZoneAllowScriptlets
    pub fn get_local_machine_zone_allow_scriptlets(&self) -> Option<&String> {
        self.local_machine_zone_allow_scriptlets.as_ref()
    }

    /// Sets the value of LocalMachineZoneAllowSmartScreenIE
    pub fn set_local_machine_zone_allow_smart_screen_ie(&mut self, value: String) {
        self.local_machine_zone_allow_smart_screen_ie = Some(value);
    }

    /// Gets the value of LocalMachineZoneAllowSmartScreenIE
    pub fn get_local_machine_zone_allow_smart_screen_ie(&self) -> Option<&String> {
        self.local_machine_zone_allow_smart_screen_ie.as_ref()
    }

    /// Sets the value of LocalMachineZoneAllowUserDataPersistence
    pub fn set_local_machine_zone_allow_user_data_persistence(&mut self, value: String) {
        self.local_machine_zone_allow_user_data_persistence = Some(value);
    }

    /// Gets the value of LocalMachineZoneAllowUserDataPersistence
    pub fn get_local_machine_zone_allow_user_data_persistence(&self) -> Option<&String> {
        self.local_machine_zone_allow_user_data_persistence.as_ref()
    }

    /// Sets the value of LocalMachineZoneDoNotRunAntimalwareAgainstActiveXControls
    pub fn set_local_machine_zone_do_not_run_antimalware_against_active_xcontrols(&mut self, value: String) {
        self.local_machine_zone_do_not_run_antimalware_against_active_xcontrols = Some(value);
    }

    /// Gets the value of LocalMachineZoneDoNotRunAntimalwareAgainstActiveXControls
    pub fn get_local_machine_zone_do_not_run_antimalware_against_active_xcontrols(&self) -> Option<&String> {
        self.local_machine_zone_do_not_run_antimalware_against_active_xcontrols.as_ref()
    }

    /// Sets the value of LocalMachineZoneInitializeAndScriptActiveXControls
    pub fn set_local_machine_zone_initialize_and_script_active_xcontrols(&mut self, value: String) {
        self.local_machine_zone_initialize_and_script_active_xcontrols = Some(value);
    }

    /// Gets the value of LocalMachineZoneInitializeAndScriptActiveXControls
    pub fn get_local_machine_zone_initialize_and_script_active_xcontrols(&self) -> Option<&String> {
        self.local_machine_zone_initialize_and_script_active_xcontrols.as_ref()
    }

    /// Sets the value of LocalMachineZoneJavaPermissions
    pub fn set_local_machine_zone_java_permissions(&mut self, value: String) {
        self.local_machine_zone_java_permissions = Some(value);
    }

    /// Gets the value of LocalMachineZoneJavaPermissions
    pub fn get_local_machine_zone_java_permissions(&self) -> Option<&String> {
        self.local_machine_zone_java_permissions.as_ref()
    }

    /// Sets the value of LocalMachineZoneNavigateWindowsAndFrames
    pub fn set_local_machine_zone_navigate_windows_and_frames(&mut self, value: String) {
        self.local_machine_zone_navigate_windows_and_frames = Some(value);
    }

    /// Gets the value of LocalMachineZoneNavigateWindowsAndFrames
    pub fn get_local_machine_zone_navigate_windows_and_frames(&self) -> Option<&String> {
        self.local_machine_zone_navigate_windows_and_frames.as_ref()
    }

    /// Sets the value of LockedDownInternetZoneAllowAccessToDataSources
    pub fn set_locked_down_internet_zone_allow_access_to_data_sources(&mut self, value: String) {
        self.locked_down_internet_zone_allow_access_to_data_sources = Some(value);
    }

    /// Gets the value of LockedDownInternetZoneAllowAccessToDataSources
    pub fn get_locked_down_internet_zone_allow_access_to_data_sources(&self) -> Option<&String> {
        self.locked_down_internet_zone_allow_access_to_data_sources.as_ref()
    }

    /// Sets the value of LockedDownInternetZoneAllowAutomaticPromptingForActiveXControls
    pub fn set_locked_down_internet_zone_allow_automatic_prompting_for_active_xcontrols(&mut self, value: String) {
        self.locked_down_internet_zone_allow_automatic_prompting_for_active_xcontrols = Some(value);
    }

    /// Gets the value of LockedDownInternetZoneAllowAutomaticPromptingForActiveXControls
    pub fn get_locked_down_internet_zone_allow_automatic_prompting_for_active_xcontrols(&self) -> Option<&String> {
        self.locked_down_internet_zone_allow_automatic_prompting_for_active_xcontrols.as_ref()
    }

    /// Sets the value of LockedDownInternetZoneAllowAutomaticPromptingForFileDownloads
    pub fn set_locked_down_internet_zone_allow_automatic_prompting_for_file_downloads(&mut self, value: String) {
        self.locked_down_internet_zone_allow_automatic_prompting_for_file_downloads = Some(value);
    }

    /// Gets the value of LockedDownInternetZoneAllowAutomaticPromptingForFileDownloads
    pub fn get_locked_down_internet_zone_allow_automatic_prompting_for_file_downloads(&self) -> Option<&String> {
        self.locked_down_internet_zone_allow_automatic_prompting_for_file_downloads.as_ref()
    }

    /// Sets the value of LockedDownInternetZoneAllowFontDownloads
    pub fn set_locked_down_internet_zone_allow_font_downloads(&mut self, value: String) {
        self.locked_down_internet_zone_allow_font_downloads = Some(value);
    }

    /// Gets the value of LockedDownInternetZoneAllowFontDownloads
    pub fn get_locked_down_internet_zone_allow_font_downloads(&self) -> Option<&String> {
        self.locked_down_internet_zone_allow_font_downloads.as_ref()
    }

    /// Sets the value of LockedDownInternetZoneAllowLessPrivilegedSites
    pub fn set_locked_down_internet_zone_allow_less_privileged_sites(&mut self, value: String) {
        self.locked_down_internet_zone_allow_less_privileged_sites = Some(value);
    }

    /// Gets the value of LockedDownInternetZoneAllowLessPrivilegedSites
    pub fn get_locked_down_internet_zone_allow_less_privileged_sites(&self) -> Option<&String> {
        self.locked_down_internet_zone_allow_less_privileged_sites.as_ref()
    }

    /// Sets the value of LockedDownInternetZoneAllowNETFrameworkReliantComponents
    pub fn set_locked_down_internet_zone_allow_netframework_reliant_components(&mut self, value: String) {
        self.locked_down_internet_zone_allow_netframework_reliant_components = Some(value);
    }

    /// Gets the value of LockedDownInternetZoneAllowNETFrameworkReliantComponents
    pub fn get_locked_down_internet_zone_allow_netframework_reliant_components(&self) -> Option<&String> {
        self.locked_down_internet_zone_allow_netframework_reliant_components.as_ref()
    }

    /// Sets the value of LockedDownInternetZoneAllowScriptlets
    pub fn set_locked_down_internet_zone_allow_scriptlets(&mut self, value: String) {
        self.locked_down_internet_zone_allow_scriptlets = Some(value);
    }

    /// Gets the value of LockedDownInternetZoneAllowScriptlets
    pub fn get_locked_down_internet_zone_allow_scriptlets(&self) -> Option<&String> {
        self.locked_down_internet_zone_allow_scriptlets.as_ref()
    }

    /// Sets the value of LockedDownInternetZoneAllowSmartScreenIE
    pub fn set_locked_down_internet_zone_allow_smart_screen_ie(&mut self, value: String) {
        self.locked_down_internet_zone_allow_smart_screen_ie = Some(value);
    }

    /// Gets the value of LockedDownInternetZoneAllowSmartScreenIE
    pub fn get_locked_down_internet_zone_allow_smart_screen_ie(&self) -> Option<&String> {
        self.locked_down_internet_zone_allow_smart_screen_ie.as_ref()
    }

    /// Sets the value of LockedDownInternetZoneAllowUserDataPersistence
    pub fn set_locked_down_internet_zone_allow_user_data_persistence(&mut self, value: String) {
        self.locked_down_internet_zone_allow_user_data_persistence = Some(value);
    }

    /// Gets the value of LockedDownInternetZoneAllowUserDataPersistence
    pub fn get_locked_down_internet_zone_allow_user_data_persistence(&self) -> Option<&String> {
        self.locked_down_internet_zone_allow_user_data_persistence.as_ref()
    }

    /// Sets the value of LockedDownInternetZoneInitializeAndScriptActiveXControls
    pub fn set_locked_down_internet_zone_initialize_and_script_active_xcontrols(&mut self, value: String) {
        self.locked_down_internet_zone_initialize_and_script_active_xcontrols = Some(value);
    }

    /// Gets the value of LockedDownInternetZoneInitializeAndScriptActiveXControls
    pub fn get_locked_down_internet_zone_initialize_and_script_active_xcontrols(&self) -> Option<&String> {
        self.locked_down_internet_zone_initialize_and_script_active_xcontrols.as_ref()
    }

    /// Sets the value of LockedDownInternetZoneJavaPermissions
    pub fn set_locked_down_internet_zone_java_permissions(&mut self, value: String) {
        self.locked_down_internet_zone_java_permissions = Some(value);
    }

    /// Gets the value of LockedDownInternetZoneJavaPermissions
    pub fn get_locked_down_internet_zone_java_permissions(&self) -> Option<&String> {
        self.locked_down_internet_zone_java_permissions.as_ref()
    }

    /// Sets the value of LockedDownInternetZoneNavigateWindowsAndFrames
    pub fn set_locked_down_internet_zone_navigate_windows_and_frames(&mut self, value: String) {
        self.locked_down_internet_zone_navigate_windows_and_frames = Some(value);
    }

    /// Gets the value of LockedDownInternetZoneNavigateWindowsAndFrames
    pub fn get_locked_down_internet_zone_navigate_windows_and_frames(&self) -> Option<&String> {
        self.locked_down_internet_zone_navigate_windows_and_frames.as_ref()
    }

    /// Sets the value of LockedDownIntranetJavaPermissions
    pub fn set_locked_down_intranet_java_permissions(&mut self, value: String) {
        self.locked_down_intranet_java_permissions = Some(value);
    }

    /// Gets the value of LockedDownIntranetJavaPermissions
    pub fn get_locked_down_intranet_java_permissions(&self) -> Option<&String> {
        self.locked_down_intranet_java_permissions.as_ref()
    }

    /// Sets the value of LockedDownIntranetZoneAllowAccessToDataSources
    pub fn set_locked_down_intranet_zone_allow_access_to_data_sources(&mut self, value: String) {
        self.locked_down_intranet_zone_allow_access_to_data_sources = Some(value);
    }

    /// Gets the value of LockedDownIntranetZoneAllowAccessToDataSources
    pub fn get_locked_down_intranet_zone_allow_access_to_data_sources(&self) -> Option<&String> {
        self.locked_down_intranet_zone_allow_access_to_data_sources.as_ref()
    }

    /// Sets the value of LockedDownIntranetZoneAllowAutomaticPromptingForActiveXControls
    pub fn set_locked_down_intranet_zone_allow_automatic_prompting_for_active_xcontrols(&mut self, value: String) {
        self.locked_down_intranet_zone_allow_automatic_prompting_for_active_xcontrols = Some(value);
    }

    /// Gets the value of LockedDownIntranetZoneAllowAutomaticPromptingForActiveXControls
    pub fn get_locked_down_intranet_zone_allow_automatic_prompting_for_active_xcontrols(&self) -> Option<&String> {
        self.locked_down_intranet_zone_allow_automatic_prompting_for_active_xcontrols.as_ref()
    }

    /// Sets the value of LockedDownIntranetZoneAllowAutomaticPromptingForFileDownloads
    pub fn set_locked_down_intranet_zone_allow_automatic_prompting_for_file_downloads(&mut self, value: String) {
        self.locked_down_intranet_zone_allow_automatic_prompting_for_file_downloads = Some(value);
    }

    /// Gets the value of LockedDownIntranetZoneAllowAutomaticPromptingForFileDownloads
    pub fn get_locked_down_intranet_zone_allow_automatic_prompting_for_file_downloads(&self) -> Option<&String> {
        self.locked_down_intranet_zone_allow_automatic_prompting_for_file_downloads.as_ref()
    }

    /// Sets the value of LockedDownIntranetZoneAllowFontDownloads
    pub fn set_locked_down_intranet_zone_allow_font_downloads(&mut self, value: String) {
        self.locked_down_intranet_zone_allow_font_downloads = Some(value);
    }

    /// Gets the value of LockedDownIntranetZoneAllowFontDownloads
    pub fn get_locked_down_intranet_zone_allow_font_downloads(&self) -> Option<&String> {
        self.locked_down_intranet_zone_allow_font_downloads.as_ref()
    }

    /// Sets the value of LockedDownIntranetZoneAllowLessPrivilegedSites
    pub fn set_locked_down_intranet_zone_allow_less_privileged_sites(&mut self, value: String) {
        self.locked_down_intranet_zone_allow_less_privileged_sites = Some(value);
    }

    /// Gets the value of LockedDownIntranetZoneAllowLessPrivilegedSites
    pub fn get_locked_down_intranet_zone_allow_less_privileged_sites(&self) -> Option<&String> {
        self.locked_down_intranet_zone_allow_less_privileged_sites.as_ref()
    }

    /// Sets the value of LockedDownIntranetZoneAllowNETFrameworkReliantComponents
    pub fn set_locked_down_intranet_zone_allow_netframework_reliant_components(&mut self, value: String) {
        self.locked_down_intranet_zone_allow_netframework_reliant_components = Some(value);
    }

    /// Gets the value of LockedDownIntranetZoneAllowNETFrameworkReliantComponents
    pub fn get_locked_down_intranet_zone_allow_netframework_reliant_components(&self) -> Option<&String> {
        self.locked_down_intranet_zone_allow_netframework_reliant_components.as_ref()
    }

    /// Sets the value of LockedDownIntranetZoneAllowScriptlets
    pub fn set_locked_down_intranet_zone_allow_scriptlets(&mut self, value: String) {
        self.locked_down_intranet_zone_allow_scriptlets = Some(value);
    }

    /// Gets the value of LockedDownIntranetZoneAllowScriptlets
    pub fn get_locked_down_intranet_zone_allow_scriptlets(&self) -> Option<&String> {
        self.locked_down_intranet_zone_allow_scriptlets.as_ref()
    }

    /// Sets the value of LockedDownIntranetZoneAllowSmartScreenIE
    pub fn set_locked_down_intranet_zone_allow_smart_screen_ie(&mut self, value: String) {
        self.locked_down_intranet_zone_allow_smart_screen_ie = Some(value);
    }

    /// Gets the value of LockedDownIntranetZoneAllowSmartScreenIE
    pub fn get_locked_down_intranet_zone_allow_smart_screen_ie(&self) -> Option<&String> {
        self.locked_down_intranet_zone_allow_smart_screen_ie.as_ref()
    }

    /// Sets the value of LockedDownIntranetZoneAllowUserDataPersistence
    pub fn set_locked_down_intranet_zone_allow_user_data_persistence(&mut self, value: String) {
        self.locked_down_intranet_zone_allow_user_data_persistence = Some(value);
    }

    /// Gets the value of LockedDownIntranetZoneAllowUserDataPersistence
    pub fn get_locked_down_intranet_zone_allow_user_data_persistence(&self) -> Option<&String> {
        self.locked_down_intranet_zone_allow_user_data_persistence.as_ref()
    }

    /// Sets the value of LockedDownIntranetZoneInitializeAndScriptActiveXControls
    pub fn set_locked_down_intranet_zone_initialize_and_script_active_xcontrols(&mut self, value: String) {
        self.locked_down_intranet_zone_initialize_and_script_active_xcontrols = Some(value);
    }

    /// Gets the value of LockedDownIntranetZoneInitializeAndScriptActiveXControls
    pub fn get_locked_down_intranet_zone_initialize_and_script_active_xcontrols(&self) -> Option<&String> {
        self.locked_down_intranet_zone_initialize_and_script_active_xcontrols.as_ref()
    }

    /// Sets the value of LockedDownIntranetZoneNavigateWindowsAndFrames
    pub fn set_locked_down_intranet_zone_navigate_windows_and_frames(&mut self, value: String) {
        self.locked_down_intranet_zone_navigate_windows_and_frames = Some(value);
    }

    /// Gets the value of LockedDownIntranetZoneNavigateWindowsAndFrames
    pub fn get_locked_down_intranet_zone_navigate_windows_and_frames(&self) -> Option<&String> {
        self.locked_down_intranet_zone_navigate_windows_and_frames.as_ref()
    }

    /// Sets the value of LockedDownLocalMachineZoneAllowAccessToDataSources
    pub fn set_locked_down_local_machine_zone_allow_access_to_data_sources(&mut self, value: String) {
        self.locked_down_local_machine_zone_allow_access_to_data_sources = Some(value);
    }

    /// Gets the value of LockedDownLocalMachineZoneAllowAccessToDataSources
    pub fn get_locked_down_local_machine_zone_allow_access_to_data_sources(&self) -> Option<&String> {
        self.locked_down_local_machine_zone_allow_access_to_data_sources.as_ref()
    }

    /// Sets the value of LockedDownLocalMachineZoneAllowAutomaticPromptingForActiveXControls
    pub fn set_locked_down_local_machine_zone_allow_automatic_prompting_for_active_xcontrols(&mut self, value: String) {
        self.locked_down_local_machine_zone_allow_automatic_prompting_for_active_xcontrols = Some(value);
    }

    /// Gets the value of LockedDownLocalMachineZoneAllowAutomaticPromptingForActiveXControls
    pub fn get_locked_down_local_machine_zone_allow_automatic_prompting_for_active_xcontrols(&self) -> Option<&String> {
        self.locked_down_local_machine_zone_allow_automatic_prompting_for_active_xcontrols.as_ref()
    }

    /// Sets the value of LockedDownLocalMachineZoneAllowAutomaticPromptingForFileDownloads
    pub fn set_locked_down_local_machine_zone_allow_automatic_prompting_for_file_downloads(&mut self, value: String) {
        self.locked_down_local_machine_zone_allow_automatic_prompting_for_file_downloads = Some(value);
    }

    /// Gets the value of LockedDownLocalMachineZoneAllowAutomaticPromptingForFileDownloads
    pub fn get_locked_down_local_machine_zone_allow_automatic_prompting_for_file_downloads(&self) -> Option<&String> {
        self.locked_down_local_machine_zone_allow_automatic_prompting_for_file_downloads.as_ref()
    }

    /// Sets the value of LockedDownLocalMachineZoneAllowFontDownloads
    pub fn set_locked_down_local_machine_zone_allow_font_downloads(&mut self, value: String) {
        self.locked_down_local_machine_zone_allow_font_downloads = Some(value);
    }

    /// Gets the value of LockedDownLocalMachineZoneAllowFontDownloads
    pub fn get_locked_down_local_machine_zone_allow_font_downloads(&self) -> Option<&String> {
        self.locked_down_local_machine_zone_allow_font_downloads.as_ref()
    }

    /// Sets the value of LockedDownLocalMachineZoneAllowLessPrivilegedSites
    pub fn set_locked_down_local_machine_zone_allow_less_privileged_sites(&mut self, value: String) {
        self.locked_down_local_machine_zone_allow_less_privileged_sites = Some(value);
    }

    /// Gets the value of LockedDownLocalMachineZoneAllowLessPrivilegedSites
    pub fn get_locked_down_local_machine_zone_allow_less_privileged_sites(&self) -> Option<&String> {
        self.locked_down_local_machine_zone_allow_less_privileged_sites.as_ref()
    }

    /// Sets the value of LockedDownLocalMachineZoneAllowNETFrameworkReliantComponents
    pub fn set_locked_down_local_machine_zone_allow_netframework_reliant_components(&mut self, value: String) {
        self.locked_down_local_machine_zone_allow_netframework_reliant_components = Some(value);
    }

    /// Gets the value of LockedDownLocalMachineZoneAllowNETFrameworkReliantComponents
    pub fn get_locked_down_local_machine_zone_allow_netframework_reliant_components(&self) -> Option<&String> {
        self.locked_down_local_machine_zone_allow_netframework_reliant_components.as_ref()
    }

    /// Sets the value of LockedDownLocalMachineZoneAllowScriptlets
    pub fn set_locked_down_local_machine_zone_allow_scriptlets(&mut self, value: String) {
        self.locked_down_local_machine_zone_allow_scriptlets = Some(value);
    }

    /// Gets the value of LockedDownLocalMachineZoneAllowScriptlets
    pub fn get_locked_down_local_machine_zone_allow_scriptlets(&self) -> Option<&String> {
        self.locked_down_local_machine_zone_allow_scriptlets.as_ref()
    }

    /// Sets the value of LockedDownLocalMachineZoneAllowSmartScreenIE
    pub fn set_locked_down_local_machine_zone_allow_smart_screen_ie(&mut self, value: String) {
        self.locked_down_local_machine_zone_allow_smart_screen_ie = Some(value);
    }

    /// Gets the value of LockedDownLocalMachineZoneAllowSmartScreenIE
    pub fn get_locked_down_local_machine_zone_allow_smart_screen_ie(&self) -> Option<&String> {
        self.locked_down_local_machine_zone_allow_smart_screen_ie.as_ref()
    }

    /// Sets the value of LockedDownLocalMachineZoneAllowUserDataPersistence
    pub fn set_locked_down_local_machine_zone_allow_user_data_persistence(&mut self, value: String) {
        self.locked_down_local_machine_zone_allow_user_data_persistence = Some(value);
    }

    /// Gets the value of LockedDownLocalMachineZoneAllowUserDataPersistence
    pub fn get_locked_down_local_machine_zone_allow_user_data_persistence(&self) -> Option<&String> {
        self.locked_down_local_machine_zone_allow_user_data_persistence.as_ref()
    }

    /// Sets the value of LockedDownLocalMachineZoneInitializeAndScriptActiveXControls
    pub fn set_locked_down_local_machine_zone_initialize_and_script_active_xcontrols(&mut self, value: String) {
        self.locked_down_local_machine_zone_initialize_and_script_active_xcontrols = Some(value);
    }

    /// Gets the value of LockedDownLocalMachineZoneInitializeAndScriptActiveXControls
    pub fn get_locked_down_local_machine_zone_initialize_and_script_active_xcontrols(&self) -> Option<&String> {
        self.locked_down_local_machine_zone_initialize_and_script_active_xcontrols.as_ref()
    }

    /// Sets the value of LockedDownLocalMachineZoneJavaPermissions
    pub fn set_locked_down_local_machine_zone_java_permissions(&mut self, value: String) {
        self.locked_down_local_machine_zone_java_permissions = Some(value);
    }

    /// Gets the value of LockedDownLocalMachineZoneJavaPermissions
    pub fn get_locked_down_local_machine_zone_java_permissions(&self) -> Option<&String> {
        self.locked_down_local_machine_zone_java_permissions.as_ref()
    }

    /// Sets the value of LockedDownLocalMachineZoneNavigateWindowsAndFrames
    pub fn set_locked_down_local_machine_zone_navigate_windows_and_frames(&mut self, value: String) {
        self.locked_down_local_machine_zone_navigate_windows_and_frames = Some(value);
    }

    /// Gets the value of LockedDownLocalMachineZoneNavigateWindowsAndFrames
    pub fn get_locked_down_local_machine_zone_navigate_windows_and_frames(&self) -> Option<&String> {
        self.locked_down_local_machine_zone_navigate_windows_and_frames.as_ref()
    }

    /// Sets the value of LockedDownRestrictedSitesZoneAllowAccessToDataSources
    pub fn set_locked_down_restricted_sites_zone_allow_access_to_data_sources(&mut self, value: String) {
        self.locked_down_restricted_sites_zone_allow_access_to_data_sources = Some(value);
    }

    /// Gets the value of LockedDownRestrictedSitesZoneAllowAccessToDataSources
    pub fn get_locked_down_restricted_sites_zone_allow_access_to_data_sources(&self) -> Option<&String> {
        self.locked_down_restricted_sites_zone_allow_access_to_data_sources.as_ref()
    }

    /// Sets the value of LockedDownRestrictedSitesZoneAllowAutomaticPromptingForActiveXControls
    pub fn set_locked_down_restricted_sites_zone_allow_automatic_prompting_for_active_xcontrols(&mut self, value: String) {
        self.locked_down_restricted_sites_zone_allow_automatic_prompting_for_active_xcontrols = Some(value);
    }

    /// Gets the value of LockedDownRestrictedSitesZoneAllowAutomaticPromptingForActiveXControls
    pub fn get_locked_down_restricted_sites_zone_allow_automatic_prompting_for_active_xcontrols(&self) -> Option<&String> {
        self.locked_down_restricted_sites_zone_allow_automatic_prompting_for_active_xcontrols.as_ref()
    }

    /// Sets the value of LockedDownRestrictedSitesZoneAllowAutomaticPromptingForFileDownloads
    pub fn set_locked_down_restricted_sites_zone_allow_automatic_prompting_for_file_downloads(&mut self, value: String) {
        self.locked_down_restricted_sites_zone_allow_automatic_prompting_for_file_downloads = Some(value);
    }

    /// Gets the value of LockedDownRestrictedSitesZoneAllowAutomaticPromptingForFileDownloads
    pub fn get_locked_down_restricted_sites_zone_allow_automatic_prompting_for_file_downloads(&self) -> Option<&String> {
        self.locked_down_restricted_sites_zone_allow_automatic_prompting_for_file_downloads.as_ref()
    }

    /// Sets the value of LockedDownRestrictedSitesZoneAllowFontDownloads
    pub fn set_locked_down_restricted_sites_zone_allow_font_downloads(&mut self, value: String) {
        self.locked_down_restricted_sites_zone_allow_font_downloads = Some(value);
    }

    /// Gets the value of LockedDownRestrictedSitesZoneAllowFontDownloads
    pub fn get_locked_down_restricted_sites_zone_allow_font_downloads(&self) -> Option<&String> {
        self.locked_down_restricted_sites_zone_allow_font_downloads.as_ref()
    }

    /// Sets the value of LockedDownRestrictedSitesZoneAllowLessPrivilegedSites
    pub fn set_locked_down_restricted_sites_zone_allow_less_privileged_sites(&mut self, value: String) {
        self.locked_down_restricted_sites_zone_allow_less_privileged_sites = Some(value);
    }

    /// Gets the value of LockedDownRestrictedSitesZoneAllowLessPrivilegedSites
    pub fn get_locked_down_restricted_sites_zone_allow_less_privileged_sites(&self) -> Option<&String> {
        self.locked_down_restricted_sites_zone_allow_less_privileged_sites.as_ref()
    }

    /// Sets the value of LockedDownRestrictedSitesZoneAllowNETFrameworkReliantComponents
    pub fn set_locked_down_restricted_sites_zone_allow_netframework_reliant_components(&mut self, value: String) {
        self.locked_down_restricted_sites_zone_allow_netframework_reliant_components = Some(value);
    }

    /// Gets the value of LockedDownRestrictedSitesZoneAllowNETFrameworkReliantComponents
    pub fn get_locked_down_restricted_sites_zone_allow_netframework_reliant_components(&self) -> Option<&String> {
        self.locked_down_restricted_sites_zone_allow_netframework_reliant_components.as_ref()
    }

    /// Sets the value of LockedDownRestrictedSitesZoneAllowScriptlets
    pub fn set_locked_down_restricted_sites_zone_allow_scriptlets(&mut self, value: String) {
        self.locked_down_restricted_sites_zone_allow_scriptlets = Some(value);
    }

    /// Gets the value of LockedDownRestrictedSitesZoneAllowScriptlets
    pub fn get_locked_down_restricted_sites_zone_allow_scriptlets(&self) -> Option<&String> {
        self.locked_down_restricted_sites_zone_allow_scriptlets.as_ref()
    }

    /// Sets the value of LockedDownRestrictedSitesZoneAllowSmartScreenIE
    pub fn set_locked_down_restricted_sites_zone_allow_smart_screen_ie(&mut self, value: String) {
        self.locked_down_restricted_sites_zone_allow_smart_screen_ie = Some(value);
    }

    /// Gets the value of LockedDownRestrictedSitesZoneAllowSmartScreenIE
    pub fn get_locked_down_restricted_sites_zone_allow_smart_screen_ie(&self) -> Option<&String> {
        self.locked_down_restricted_sites_zone_allow_smart_screen_ie.as_ref()
    }

    /// Sets the value of LockedDownRestrictedSitesZoneAllowUserDataPersistence
    pub fn set_locked_down_restricted_sites_zone_allow_user_data_persistence(&mut self, value: String) {
        self.locked_down_restricted_sites_zone_allow_user_data_persistence = Some(value);
    }

    /// Gets the value of LockedDownRestrictedSitesZoneAllowUserDataPersistence
    pub fn get_locked_down_restricted_sites_zone_allow_user_data_persistence(&self) -> Option<&String> {
        self.locked_down_restricted_sites_zone_allow_user_data_persistence.as_ref()
    }

    /// Sets the value of LockedDownRestrictedSitesZoneInitializeAndScriptActiveXControls
    pub fn set_locked_down_restricted_sites_zone_initialize_and_script_active_xcontrols(&mut self, value: String) {
        self.locked_down_restricted_sites_zone_initialize_and_script_active_xcontrols = Some(value);
    }

    /// Gets the value of LockedDownRestrictedSitesZoneInitializeAndScriptActiveXControls
    pub fn get_locked_down_restricted_sites_zone_initialize_and_script_active_xcontrols(&self) -> Option<&String> {
        self.locked_down_restricted_sites_zone_initialize_and_script_active_xcontrols.as_ref()
    }

    /// Sets the value of LockedDownRestrictedSitesZoneJavaPermissions
    pub fn set_locked_down_restricted_sites_zone_java_permissions(&mut self, value: String) {
        self.locked_down_restricted_sites_zone_java_permissions = Some(value);
    }

    /// Gets the value of LockedDownRestrictedSitesZoneJavaPermissions
    pub fn get_locked_down_restricted_sites_zone_java_permissions(&self) -> Option<&String> {
        self.locked_down_restricted_sites_zone_java_permissions.as_ref()
    }

    /// Sets the value of LockedDownRestrictedSitesZoneNavigateWindowsAndFrames
    pub fn set_locked_down_restricted_sites_zone_navigate_windows_and_frames(&mut self, value: String) {
        self.locked_down_restricted_sites_zone_navigate_windows_and_frames = Some(value);
    }

    /// Gets the value of LockedDownRestrictedSitesZoneNavigateWindowsAndFrames
    pub fn get_locked_down_restricted_sites_zone_navigate_windows_and_frames(&self) -> Option<&String> {
        self.locked_down_restricted_sites_zone_navigate_windows_and_frames.as_ref()
    }

    /// Sets the value of LockedDownTrustedSitesZoneAllowAccessToDataSources
    pub fn set_locked_down_trusted_sites_zone_allow_access_to_data_sources(&mut self, value: String) {
        self.locked_down_trusted_sites_zone_allow_access_to_data_sources = Some(value);
    }

    /// Gets the value of LockedDownTrustedSitesZoneAllowAccessToDataSources
    pub fn get_locked_down_trusted_sites_zone_allow_access_to_data_sources(&self) -> Option<&String> {
        self.locked_down_trusted_sites_zone_allow_access_to_data_sources.as_ref()
    }

    /// Sets the value of LockedDownTrustedSitesZoneAllowAutomaticPromptingForActiveXControls
    pub fn set_locked_down_trusted_sites_zone_allow_automatic_prompting_for_active_xcontrols(&mut self, value: String) {
        self.locked_down_trusted_sites_zone_allow_automatic_prompting_for_active_xcontrols = Some(value);
    }

    /// Gets the value of LockedDownTrustedSitesZoneAllowAutomaticPromptingForActiveXControls
    pub fn get_locked_down_trusted_sites_zone_allow_automatic_prompting_for_active_xcontrols(&self) -> Option<&String> {
        self.locked_down_trusted_sites_zone_allow_automatic_prompting_for_active_xcontrols.as_ref()
    }

    /// Sets the value of LockedDownTrustedSitesZoneAllowAutomaticPromptingForFileDownloads
    pub fn set_locked_down_trusted_sites_zone_allow_automatic_prompting_for_file_downloads(&mut self, value: String) {
        self.locked_down_trusted_sites_zone_allow_automatic_prompting_for_file_downloads = Some(value);
    }

    /// Gets the value of LockedDownTrustedSitesZoneAllowAutomaticPromptingForFileDownloads
    pub fn get_locked_down_trusted_sites_zone_allow_automatic_prompting_for_file_downloads(&self) -> Option<&String> {
        self.locked_down_trusted_sites_zone_allow_automatic_prompting_for_file_downloads.as_ref()
    }

    /// Sets the value of LockedDownTrustedSitesZoneAllowFontDownloads
    pub fn set_locked_down_trusted_sites_zone_allow_font_downloads(&mut self, value: String) {
        self.locked_down_trusted_sites_zone_allow_font_downloads = Some(value);
    }

    /// Gets the value of LockedDownTrustedSitesZoneAllowFontDownloads
    pub fn get_locked_down_trusted_sites_zone_allow_font_downloads(&self) -> Option<&String> {
        self.locked_down_trusted_sites_zone_allow_font_downloads.as_ref()
    }

    /// Sets the value of LockedDownTrustedSitesZoneAllowLessPrivilegedSites
    pub fn set_locked_down_trusted_sites_zone_allow_less_privileged_sites(&mut self, value: String) {
        self.locked_down_trusted_sites_zone_allow_less_privileged_sites = Some(value);
    }

    /// Gets the value of LockedDownTrustedSitesZoneAllowLessPrivilegedSites
    pub fn get_locked_down_trusted_sites_zone_allow_less_privileged_sites(&self) -> Option<&String> {
        self.locked_down_trusted_sites_zone_allow_less_privileged_sites.as_ref()
    }

    /// Sets the value of LockedDownTrustedSitesZoneAllowNETFrameworkReliantComponents
    pub fn set_locked_down_trusted_sites_zone_allow_netframework_reliant_components(&mut self, value: String) {
        self.locked_down_trusted_sites_zone_allow_netframework_reliant_components = Some(value);
    }

    /// Gets the value of LockedDownTrustedSitesZoneAllowNETFrameworkReliantComponents
    pub fn get_locked_down_trusted_sites_zone_allow_netframework_reliant_components(&self) -> Option<&String> {
        self.locked_down_trusted_sites_zone_allow_netframework_reliant_components.as_ref()
    }

    /// Sets the value of LockedDownTrustedSitesZoneAllowScriptlets
    pub fn set_locked_down_trusted_sites_zone_allow_scriptlets(&mut self, value: String) {
        self.locked_down_trusted_sites_zone_allow_scriptlets = Some(value);
    }

    /// Gets the value of LockedDownTrustedSitesZoneAllowScriptlets
    pub fn get_locked_down_trusted_sites_zone_allow_scriptlets(&self) -> Option<&String> {
        self.locked_down_trusted_sites_zone_allow_scriptlets.as_ref()
    }

    /// Sets the value of LockedDownTrustedSitesZoneAllowSmartScreenIE
    pub fn set_locked_down_trusted_sites_zone_allow_smart_screen_ie(&mut self, value: String) {
        self.locked_down_trusted_sites_zone_allow_smart_screen_ie = Some(value);
    }

    /// Gets the value of LockedDownTrustedSitesZoneAllowSmartScreenIE
    pub fn get_locked_down_trusted_sites_zone_allow_smart_screen_ie(&self) -> Option<&String> {
        self.locked_down_trusted_sites_zone_allow_smart_screen_ie.as_ref()
    }

    /// Sets the value of LockedDownTrustedSitesZoneAllowUserDataPersistence
    pub fn set_locked_down_trusted_sites_zone_allow_user_data_persistence(&mut self, value: String) {
        self.locked_down_trusted_sites_zone_allow_user_data_persistence = Some(value);
    }

    /// Gets the value of LockedDownTrustedSitesZoneAllowUserDataPersistence
    pub fn get_locked_down_trusted_sites_zone_allow_user_data_persistence(&self) -> Option<&String> {
        self.locked_down_trusted_sites_zone_allow_user_data_persistence.as_ref()
    }

    /// Sets the value of LockedDownTrustedSitesZoneInitializeAndScriptActiveXControls
    pub fn set_locked_down_trusted_sites_zone_initialize_and_script_active_xcontrols(&mut self, value: String) {
        self.locked_down_trusted_sites_zone_initialize_and_script_active_xcontrols = Some(value);
    }

    /// Gets the value of LockedDownTrustedSitesZoneInitializeAndScriptActiveXControls
    pub fn get_locked_down_trusted_sites_zone_initialize_and_script_active_xcontrols(&self) -> Option<&String> {
        self.locked_down_trusted_sites_zone_initialize_and_script_active_xcontrols.as_ref()
    }

    /// Sets the value of LockedDownTrustedSitesZoneJavaPermissions
    pub fn set_locked_down_trusted_sites_zone_java_permissions(&mut self, value: String) {
        self.locked_down_trusted_sites_zone_java_permissions = Some(value);
    }

    /// Gets the value of LockedDownTrustedSitesZoneJavaPermissions
    pub fn get_locked_down_trusted_sites_zone_java_permissions(&self) -> Option<&String> {
        self.locked_down_trusted_sites_zone_java_permissions.as_ref()
    }

    /// Sets the value of LockedDownTrustedSitesZoneNavigateWindowsAndFrames
    pub fn set_locked_down_trusted_sites_zone_navigate_windows_and_frames(&mut self, value: String) {
        self.locked_down_trusted_sites_zone_navigate_windows_and_frames = Some(value);
    }

    /// Gets the value of LockedDownTrustedSitesZoneNavigateWindowsAndFrames
    pub fn get_locked_down_trusted_sites_zone_navigate_windows_and_frames(&self) -> Option<&String> {
        self.locked_down_trusted_sites_zone_navigate_windows_and_frames.as_ref()
    }

    /// Sets the value of MimeSniffingSafetyFeatureInternetExplorerProcesses
    pub fn set_mime_sniffing_safety_feature_internet_explorer_processes(&mut self, value: String) {
        self.mime_sniffing_safety_feature_internet_explorer_processes = Some(value);
    }

    /// Gets the value of MimeSniffingSafetyFeatureInternetExplorerProcesses
    pub fn get_mime_sniffing_safety_feature_internet_explorer_processes(&self) -> Option<&String> {
        self.mime_sniffing_safety_feature_internet_explorer_processes.as_ref()
    }

    /// Sets the value of MKProtocolSecurityRestrictionInternetExplorerProcesses
    pub fn set_mkprotocol_security_restriction_internet_explorer_processes(&mut self, value: String) {
        self.mkprotocol_security_restriction_internet_explorer_processes = Some(value);
    }

    /// Gets the value of MKProtocolSecurityRestrictionInternetExplorerProcesses
    pub fn get_mkprotocol_security_restriction_internet_explorer_processes(&self) -> Option<&String> {
        self.mkprotocol_security_restriction_internet_explorer_processes.as_ref()
    }

    /// Sets the value of NewTabDefaultPage
    pub fn set_new_tab_default_page(&mut self, value: String) {
        self.new_tab_default_page = Some(value);
    }

    /// Gets the value of NewTabDefaultPage
    pub fn get_new_tab_default_page(&self) -> Option<&String> {
        self.new_tab_default_page.as_ref()
    }

    /// Sets the value of NotificationBarInternetExplorerProcesses
    pub fn set_notification_bar_internet_explorer_processes(&mut self, value: String) {
        self.notification_bar_internet_explorer_processes = Some(value);
    }

    /// Gets the value of NotificationBarInternetExplorerProcesses
    pub fn get_notification_bar_internet_explorer_processes(&self) -> Option<&String> {
        self.notification_bar_internet_explorer_processes.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PreventManagingSmartScreenFilter
    pub fn set_prevent_managing_smart_screen_filter(&mut self, value: String) {
        self.prevent_managing_smart_screen_filter = Some(value);
    }

    /// Gets the value of PreventManagingSmartScreenFilter
    pub fn get_prevent_managing_smart_screen_filter(&self) -> Option<&String> {
        self.prevent_managing_smart_screen_filter.as_ref()
    }

    /// Sets the value of PreventPerUserInstallationOfActiveXControls
    pub fn set_prevent_per_user_installation_of_active_xcontrols(&mut self, value: String) {
        self.prevent_per_user_installation_of_active_xcontrols = Some(value);
    }

    /// Gets the value of PreventPerUserInstallationOfActiveXControls
    pub fn get_prevent_per_user_installation_of_active_xcontrols(&self) -> Option<&String> {
        self.prevent_per_user_installation_of_active_xcontrols.as_ref()
    }

    /// Sets the value of ProtectionFromZoneElevationInternetExplorerProcesses
    pub fn set_protection_from_zone_elevation_internet_explorer_processes(&mut self, value: String) {
        self.protection_from_zone_elevation_internet_explorer_processes = Some(value);
    }

    /// Gets the value of ProtectionFromZoneElevationInternetExplorerProcesses
    pub fn get_protection_from_zone_elevation_internet_explorer_processes(&self) -> Option<&String> {
        self.protection_from_zone_elevation_internet_explorer_processes.as_ref()
    }

    /// Sets the value of RemoveRunThisTimeButtonForOutdatedActiveXControls
    pub fn set_remove_run_this_time_button_for_outdated_active_xcontrols(&mut self, value: String) {
        self.remove_run_this_time_button_for_outdated_active_xcontrols = Some(value);
    }

    /// Gets the value of RemoveRunThisTimeButtonForOutdatedActiveXControls
    pub fn get_remove_run_this_time_button_for_outdated_active_xcontrols(&self) -> Option<&String> {
        self.remove_run_this_time_button_for_outdated_active_xcontrols.as_ref()
    }

    /// Sets the value of RestrictActiveXInstallInternetExplorerProcesses
    pub fn set_restrict_active_xinstall_internet_explorer_processes(&mut self, value: String) {
        self.restrict_active_xinstall_internet_explorer_processes = Some(value);
    }

    /// Gets the value of RestrictActiveXInstallInternetExplorerProcesses
    pub fn get_restrict_active_xinstall_internet_explorer_processes(&self) -> Option<&String> {
        self.restrict_active_xinstall_internet_explorer_processes.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneAllowAccessToDataSources
    pub fn set_restricted_sites_zone_allow_access_to_data_sources(&mut self, value: String) {
        self.restricted_sites_zone_allow_access_to_data_sources = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneAllowAccessToDataSources
    pub fn get_restricted_sites_zone_allow_access_to_data_sources(&self) -> Option<&String> {
        self.restricted_sites_zone_allow_access_to_data_sources.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneAllowActiveScripting
    pub fn set_restricted_sites_zone_allow_active_scripting(&mut self, value: String) {
        self.restricted_sites_zone_allow_active_scripting = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneAllowActiveScripting
    pub fn get_restricted_sites_zone_allow_active_scripting(&self) -> Option<&String> {
        self.restricted_sites_zone_allow_active_scripting.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneAllowAutomaticPromptingForActiveXControls
    pub fn set_restricted_sites_zone_allow_automatic_prompting_for_active_xcontrols(&mut self, value: String) {
        self.restricted_sites_zone_allow_automatic_prompting_for_active_xcontrols = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneAllowAutomaticPromptingForActiveXControls
    pub fn get_restricted_sites_zone_allow_automatic_prompting_for_active_xcontrols(&self) -> Option<&String> {
        self.restricted_sites_zone_allow_automatic_prompting_for_active_xcontrols.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneAllowAutomaticPromptingForFileDownloads
    pub fn set_restricted_sites_zone_allow_automatic_prompting_for_file_downloads(&mut self, value: String) {
        self.restricted_sites_zone_allow_automatic_prompting_for_file_downloads = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneAllowAutomaticPromptingForFileDownloads
    pub fn get_restricted_sites_zone_allow_automatic_prompting_for_file_downloads(&self) -> Option<&String> {
        self.restricted_sites_zone_allow_automatic_prompting_for_file_downloads.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneAllowBinaryAndScriptBehaviors
    pub fn set_restricted_sites_zone_allow_binary_and_script_behaviors(&mut self, value: String) {
        self.restricted_sites_zone_allow_binary_and_script_behaviors = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneAllowBinaryAndScriptBehaviors
    pub fn get_restricted_sites_zone_allow_binary_and_script_behaviors(&self) -> Option<&String> {
        self.restricted_sites_zone_allow_binary_and_script_behaviors.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneAllowCopyPasteViaScript
    pub fn set_restricted_sites_zone_allow_copy_paste_via_script(&mut self, value: String) {
        self.restricted_sites_zone_allow_copy_paste_via_script = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneAllowCopyPasteViaScript
    pub fn get_restricted_sites_zone_allow_copy_paste_via_script(&self) -> Option<&String> {
        self.restricted_sites_zone_allow_copy_paste_via_script.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneAllowDragAndDropCopyAndPasteFiles
    pub fn set_restricted_sites_zone_allow_drag_and_drop_copy_and_paste_files(&mut self, value: String) {
        self.restricted_sites_zone_allow_drag_and_drop_copy_and_paste_files = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneAllowDragAndDropCopyAndPasteFiles
    pub fn get_restricted_sites_zone_allow_drag_and_drop_copy_and_paste_files(&self) -> Option<&String> {
        self.restricted_sites_zone_allow_drag_and_drop_copy_and_paste_files.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneAllowFileDownloads
    pub fn set_restricted_sites_zone_allow_file_downloads(&mut self, value: String) {
        self.restricted_sites_zone_allow_file_downloads = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneAllowFileDownloads
    pub fn get_restricted_sites_zone_allow_file_downloads(&self) -> Option<&String> {
        self.restricted_sites_zone_allow_file_downloads.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneAllowFontDownloads
    pub fn set_restricted_sites_zone_allow_font_downloads(&mut self, value: String) {
        self.restricted_sites_zone_allow_font_downloads = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneAllowFontDownloads
    pub fn get_restricted_sites_zone_allow_font_downloads(&self) -> Option<&String> {
        self.restricted_sites_zone_allow_font_downloads.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneAllowLessPrivilegedSites
    pub fn set_restricted_sites_zone_allow_less_privileged_sites(&mut self, value: String) {
        self.restricted_sites_zone_allow_less_privileged_sites = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneAllowLessPrivilegedSites
    pub fn get_restricted_sites_zone_allow_less_privileged_sites(&self) -> Option<&String> {
        self.restricted_sites_zone_allow_less_privileged_sites.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneAllowLoadingOfXAMLFiles
    pub fn set_restricted_sites_zone_allow_loading_of_xamlfiles(&mut self, value: String) {
        self.restricted_sites_zone_allow_loading_of_xamlfiles = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneAllowLoadingOfXAMLFiles
    pub fn get_restricted_sites_zone_allow_loading_of_xamlfiles(&self) -> Option<&String> {
        self.restricted_sites_zone_allow_loading_of_xamlfiles.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneAllowMETAREFRESH
    pub fn set_restricted_sites_zone_allow_metarefresh(&mut self, value: String) {
        self.restricted_sites_zone_allow_metarefresh = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneAllowMETAREFRESH
    pub fn get_restricted_sites_zone_allow_metarefresh(&self) -> Option<&String> {
        self.restricted_sites_zone_allow_metarefresh.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneAllowNETFrameworkReliantComponents
    pub fn set_restricted_sites_zone_allow_netframework_reliant_components(&mut self, value: String) {
        self.restricted_sites_zone_allow_netframework_reliant_components = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneAllowNETFrameworkReliantComponents
    pub fn get_restricted_sites_zone_allow_netframework_reliant_components(&self) -> Option<&String> {
        self.restricted_sites_zone_allow_netframework_reliant_components.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneAllowOnlyApprovedDomainsToUseActiveXControls
    pub fn set_restricted_sites_zone_allow_only_approved_domains_to_use_active_xcontrols(&mut self, value: String) {
        self.restricted_sites_zone_allow_only_approved_domains_to_use_active_xcontrols = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneAllowOnlyApprovedDomainsToUseActiveXControls
    pub fn get_restricted_sites_zone_allow_only_approved_domains_to_use_active_xcontrols(&self) -> Option<&String> {
        self.restricted_sites_zone_allow_only_approved_domains_to_use_active_xcontrols.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneAllowOnlyApprovedDomainsToUseTDCActiveXControl
    pub fn set_restricted_sites_zone_allow_only_approved_domains_to_use_tdcactive_xcontrol(&mut self, value: String) {
        self.restricted_sites_zone_allow_only_approved_domains_to_use_tdcactive_xcontrol = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneAllowOnlyApprovedDomainsToUseTDCActiveXControl
    pub fn get_restricted_sites_zone_allow_only_approved_domains_to_use_tdcactive_xcontrol(&self) -> Option<&String> {
        self.restricted_sites_zone_allow_only_approved_domains_to_use_tdcactive_xcontrol.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneAllowScriptingOfInternetExplorerWebBrowserControls
    pub fn set_restricted_sites_zone_allow_scripting_of_internet_explorer_web_browser_controls(&mut self, value: String) {
        self.restricted_sites_zone_allow_scripting_of_internet_explorer_web_browser_controls = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneAllowScriptingOfInternetExplorerWebBrowserControls
    pub fn get_restricted_sites_zone_allow_scripting_of_internet_explorer_web_browser_controls(&self) -> Option<&String> {
        self.restricted_sites_zone_allow_scripting_of_internet_explorer_web_browser_controls.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneAllowScriptInitiatedWindows
    pub fn set_restricted_sites_zone_allow_script_initiated_windows(&mut self, value: String) {
        self.restricted_sites_zone_allow_script_initiated_windows = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneAllowScriptInitiatedWindows
    pub fn get_restricted_sites_zone_allow_script_initiated_windows(&self) -> Option<&String> {
        self.restricted_sites_zone_allow_script_initiated_windows.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneAllowScriptlets
    pub fn set_restricted_sites_zone_allow_scriptlets(&mut self, value: String) {
        self.restricted_sites_zone_allow_scriptlets = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneAllowScriptlets
    pub fn get_restricted_sites_zone_allow_scriptlets(&self) -> Option<&String> {
        self.restricted_sites_zone_allow_scriptlets.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneAllowSmartScreenIE
    pub fn set_restricted_sites_zone_allow_smart_screen_ie(&mut self, value: String) {
        self.restricted_sites_zone_allow_smart_screen_ie = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneAllowSmartScreenIE
    pub fn get_restricted_sites_zone_allow_smart_screen_ie(&self) -> Option<&String> {
        self.restricted_sites_zone_allow_smart_screen_ie.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneAllowUpdatesToStatusBarViaScript
    pub fn set_restricted_sites_zone_allow_updates_to_status_bar_via_script(&mut self, value: String) {
        self.restricted_sites_zone_allow_updates_to_status_bar_via_script = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneAllowUpdatesToStatusBarViaScript
    pub fn get_restricted_sites_zone_allow_updates_to_status_bar_via_script(&self) -> Option<&String> {
        self.restricted_sites_zone_allow_updates_to_status_bar_via_script.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneAllowUserDataPersistence
    pub fn set_restricted_sites_zone_allow_user_data_persistence(&mut self, value: String) {
        self.restricted_sites_zone_allow_user_data_persistence = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneAllowUserDataPersistence
    pub fn get_restricted_sites_zone_allow_user_data_persistence(&self) -> Option<&String> {
        self.restricted_sites_zone_allow_user_data_persistence.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneAllowVBScriptToRunInInternetExplorer
    pub fn set_restricted_sites_zone_allow_vbscript_to_run_in_internet_explorer(&mut self, value: String) {
        self.restricted_sites_zone_allow_vbscript_to_run_in_internet_explorer = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneAllowVBScriptToRunInInternetExplorer
    pub fn get_restricted_sites_zone_allow_vbscript_to_run_in_internet_explorer(&self) -> Option<&String> {
        self.restricted_sites_zone_allow_vbscript_to_run_in_internet_explorer.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneDoNotRunAntimalwareAgainstActiveXControls
    pub fn set_restricted_sites_zone_do_not_run_antimalware_against_active_xcontrols(&mut self, value: String) {
        self.restricted_sites_zone_do_not_run_antimalware_against_active_xcontrols = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneDoNotRunAntimalwareAgainstActiveXControls
    pub fn get_restricted_sites_zone_do_not_run_antimalware_against_active_xcontrols(&self) -> Option<&String> {
        self.restricted_sites_zone_do_not_run_antimalware_against_active_xcontrols.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneDownloadSignedActiveXControls
    pub fn set_restricted_sites_zone_download_signed_active_xcontrols(&mut self, value: String) {
        self.restricted_sites_zone_download_signed_active_xcontrols = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneDownloadSignedActiveXControls
    pub fn get_restricted_sites_zone_download_signed_active_xcontrols(&self) -> Option<&String> {
        self.restricted_sites_zone_download_signed_active_xcontrols.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneDownloadUnsignedActiveXControls
    pub fn set_restricted_sites_zone_download_unsigned_active_xcontrols(&mut self, value: String) {
        self.restricted_sites_zone_download_unsigned_active_xcontrols = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneDownloadUnsignedActiveXControls
    pub fn get_restricted_sites_zone_download_unsigned_active_xcontrols(&self) -> Option<&String> {
        self.restricted_sites_zone_download_unsigned_active_xcontrols.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneEnableCrossSiteScriptingFilter
    pub fn set_restricted_sites_zone_enable_cross_site_scripting_filter(&mut self, value: String) {
        self.restricted_sites_zone_enable_cross_site_scripting_filter = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneEnableCrossSiteScriptingFilter
    pub fn get_restricted_sites_zone_enable_cross_site_scripting_filter(&self) -> Option<&String> {
        self.restricted_sites_zone_enable_cross_site_scripting_filter.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneEnableDraggingOfContentFromDifferentDomainsAcrossWindows
    pub fn set_restricted_sites_zone_enable_dragging_of_content_from_different_domains_across_windows(&mut self, value: String) {
        self.restricted_sites_zone_enable_dragging_of_content_from_different_domains_across_windows = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneEnableDraggingOfContentFromDifferentDomainsAcrossWindows
    pub fn get_restricted_sites_zone_enable_dragging_of_content_from_different_domains_across_windows(&self) -> Option<&String> {
        self.restricted_sites_zone_enable_dragging_of_content_from_different_domains_across_windows.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneEnableDraggingOfContentFromDifferentDomainsWithinWindows
    pub fn set_restricted_sites_zone_enable_dragging_of_content_from_different_domains_within_windows(&mut self, value: String) {
        self.restricted_sites_zone_enable_dragging_of_content_from_different_domains_within_windows = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneEnableDraggingOfContentFromDifferentDomainsWithinWindows
    pub fn get_restricted_sites_zone_enable_dragging_of_content_from_different_domains_within_windows(&self) -> Option<&String> {
        self.restricted_sites_zone_enable_dragging_of_content_from_different_domains_within_windows.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneEnableMIMESniffing
    pub fn set_restricted_sites_zone_enable_mimesniffing(&mut self, value: String) {
        self.restricted_sites_zone_enable_mimesniffing = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneEnableMIMESniffing
    pub fn get_restricted_sites_zone_enable_mimesniffing(&self) -> Option<&String> {
        self.restricted_sites_zone_enable_mimesniffing.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneIncludeLocalPathWhenUploadingFilesToServer
    pub fn set_restricted_sites_zone_include_local_path_when_uploading_files_to_server(&mut self, value: String) {
        self.restricted_sites_zone_include_local_path_when_uploading_files_to_server = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneIncludeLocalPathWhenUploadingFilesToServer
    pub fn get_restricted_sites_zone_include_local_path_when_uploading_files_to_server(&self) -> Option<&String> {
        self.restricted_sites_zone_include_local_path_when_uploading_files_to_server.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneInitializeAndScriptActiveXControls
    pub fn set_restricted_sites_zone_initialize_and_script_active_xcontrols(&mut self, value: String) {
        self.restricted_sites_zone_initialize_and_script_active_xcontrols = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneInitializeAndScriptActiveXControls
    pub fn get_restricted_sites_zone_initialize_and_script_active_xcontrols(&self) -> Option<&String> {
        self.restricted_sites_zone_initialize_and_script_active_xcontrols.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneJavaPermissions
    pub fn set_restricted_sites_zone_java_permissions(&mut self, value: String) {
        self.restricted_sites_zone_java_permissions = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneJavaPermissions
    pub fn get_restricted_sites_zone_java_permissions(&self) -> Option<&String> {
        self.restricted_sites_zone_java_permissions.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneLaunchingApplicationsAndFilesInIFRAME
    pub fn set_restricted_sites_zone_launching_applications_and_files_in_iframe(&mut self, value: String) {
        self.restricted_sites_zone_launching_applications_and_files_in_iframe = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneLaunchingApplicationsAndFilesInIFRAME
    pub fn get_restricted_sites_zone_launching_applications_and_files_in_iframe(&self) -> Option<&String> {
        self.restricted_sites_zone_launching_applications_and_files_in_iframe.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneLogonOptions
    pub fn set_restricted_sites_zone_logon_options(&mut self, value: String) {
        self.restricted_sites_zone_logon_options = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneLogonOptions
    pub fn get_restricted_sites_zone_logon_options(&self) -> Option<&String> {
        self.restricted_sites_zone_logon_options.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneNavigateWindowsAndFrames
    pub fn set_restricted_sites_zone_navigate_windows_and_frames(&mut self, value: String) {
        self.restricted_sites_zone_navigate_windows_and_frames = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneNavigateWindowsAndFrames
    pub fn get_restricted_sites_zone_navigate_windows_and_frames(&self) -> Option<&String> {
        self.restricted_sites_zone_navigate_windows_and_frames.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneRunActiveXControlsAndPlugins
    pub fn set_restricted_sites_zone_run_active_xcontrols_and_plugins(&mut self, value: String) {
        self.restricted_sites_zone_run_active_xcontrols_and_plugins = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneRunActiveXControlsAndPlugins
    pub fn get_restricted_sites_zone_run_active_xcontrols_and_plugins(&self) -> Option<&String> {
        self.restricted_sites_zone_run_active_xcontrols_and_plugins.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneRunNETFrameworkReliantComponentsSignedWithAuthenticode
    pub fn set_restricted_sites_zone_run_netframework_reliant_components_signed_with_authenticode(&mut self, value: String) {
        self.restricted_sites_zone_run_netframework_reliant_components_signed_with_authenticode = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneRunNETFrameworkReliantComponentsSignedWithAuthenticode
    pub fn get_restricted_sites_zone_run_netframework_reliant_components_signed_with_authenticode(&self) -> Option<&String> {
        self.restricted_sites_zone_run_netframework_reliant_components_signed_with_authenticode.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneScriptActiveXControlsMarkedSafeForScripting
    pub fn set_restricted_sites_zone_script_active_xcontrols_marked_safe_for_scripting(&mut self, value: String) {
        self.restricted_sites_zone_script_active_xcontrols_marked_safe_for_scripting = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneScriptActiveXControlsMarkedSafeForScripting
    pub fn get_restricted_sites_zone_script_active_xcontrols_marked_safe_for_scripting(&self) -> Option<&String> {
        self.restricted_sites_zone_script_active_xcontrols_marked_safe_for_scripting.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneScriptingOfJavaApplets
    pub fn set_restricted_sites_zone_scripting_of_java_applets(&mut self, value: String) {
        self.restricted_sites_zone_scripting_of_java_applets = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneScriptingOfJavaApplets
    pub fn get_restricted_sites_zone_scripting_of_java_applets(&self) -> Option<&String> {
        self.restricted_sites_zone_scripting_of_java_applets.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneShowSecurityWarningForPotentiallyUnsafeFiles
    pub fn set_restricted_sites_zone_show_security_warning_for_potentially_unsafe_files(&mut self, value: String) {
        self.restricted_sites_zone_show_security_warning_for_potentially_unsafe_files = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneShowSecurityWarningForPotentiallyUnsafeFiles
    pub fn get_restricted_sites_zone_show_security_warning_for_potentially_unsafe_files(&self) -> Option<&String> {
        self.restricted_sites_zone_show_security_warning_for_potentially_unsafe_files.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneTurnOnProtectedMode
    pub fn set_restricted_sites_zone_turn_on_protected_mode(&mut self, value: String) {
        self.restricted_sites_zone_turn_on_protected_mode = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneTurnOnProtectedMode
    pub fn get_restricted_sites_zone_turn_on_protected_mode(&self) -> Option<&String> {
        self.restricted_sites_zone_turn_on_protected_mode.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneUsePopupBlocker
    pub fn set_restricted_sites_zone_use_popup_blocker(&mut self, value: String) {
        self.restricted_sites_zone_use_popup_blocker = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneUsePopupBlocker
    pub fn get_restricted_sites_zone_use_popup_blocker(&self) -> Option<&String> {
        self.restricted_sites_zone_use_popup_blocker.as_ref()
    }

    /// Sets the value of RestrictFileDownloadInternetExplorerProcesses
    pub fn set_restrict_file_download_internet_explorer_processes(&mut self, value: String) {
        self.restrict_file_download_internet_explorer_processes = Some(value);
    }

    /// Gets the value of RestrictFileDownloadInternetExplorerProcesses
    pub fn get_restrict_file_download_internet_explorer_processes(&self) -> Option<&String> {
        self.restrict_file_download_internet_explorer_processes.as_ref()
    }

    /// Sets the value of ScriptedWindowSecurityRestrictionsInternetExplorerProcesses
    pub fn set_scripted_window_security_restrictions_internet_explorer_processes(&mut self, value: String) {
        self.scripted_window_security_restrictions_internet_explorer_processes = Some(value);
    }

    /// Gets the value of ScriptedWindowSecurityRestrictionsInternetExplorerProcesses
    pub fn get_scripted_window_security_restrictions_internet_explorer_processes(&self) -> Option<&String> {
        self.scripted_window_security_restrictions_internet_explorer_processes.as_ref()
    }

    /// Sets the value of SearchProviderList
    pub fn set_search_provider_list(&mut self, value: String) {
        self.search_provider_list = Some(value);
    }

    /// Gets the value of SearchProviderList
    pub fn get_search_provider_list(&self) -> Option<&String> {
        self.search_provider_list.as_ref()
    }

    /// Sets the value of SendSitesNotInEnterpriseSiteListToEdge
    pub fn set_send_sites_not_in_enterprise_site_list_to_edge(&mut self, value: String) {
        self.send_sites_not_in_enterprise_site_list_to_edge = Some(value);
    }

    /// Gets the value of SendSitesNotInEnterpriseSiteListToEdge
    pub fn get_send_sites_not_in_enterprise_site_list_to_edge(&self) -> Option<&String> {
        self.send_sites_not_in_enterprise_site_list_to_edge.as_ref()
    }

    /// Sets the value of SpecifyUseOfActiveXInstallerService
    pub fn set_specify_use_of_active_xinstaller_service(&mut self, value: String) {
        self.specify_use_of_active_xinstaller_service = Some(value);
    }

    /// Gets the value of SpecifyUseOfActiveXInstallerService
    pub fn get_specify_use_of_active_xinstaller_service(&self) -> Option<&String> {
        self.specify_use_of_active_xinstaller_service.as_ref()
    }

    /// Sets the value of TrustedSitesZoneAllowAccessToDataSources
    pub fn set_trusted_sites_zone_allow_access_to_data_sources(&mut self, value: String) {
        self.trusted_sites_zone_allow_access_to_data_sources = Some(value);
    }

    /// Gets the value of TrustedSitesZoneAllowAccessToDataSources
    pub fn get_trusted_sites_zone_allow_access_to_data_sources(&self) -> Option<&String> {
        self.trusted_sites_zone_allow_access_to_data_sources.as_ref()
    }

    /// Sets the value of TrustedSitesZoneAllowAutomaticPromptingForActiveXControls
    pub fn set_trusted_sites_zone_allow_automatic_prompting_for_active_xcontrols(&mut self, value: String) {
        self.trusted_sites_zone_allow_automatic_prompting_for_active_xcontrols = Some(value);
    }

    /// Gets the value of TrustedSitesZoneAllowAutomaticPromptingForActiveXControls
    pub fn get_trusted_sites_zone_allow_automatic_prompting_for_active_xcontrols(&self) -> Option<&String> {
        self.trusted_sites_zone_allow_automatic_prompting_for_active_xcontrols.as_ref()
    }

    /// Sets the value of TrustedSitesZoneAllowAutomaticPromptingForFileDownloads
    pub fn set_trusted_sites_zone_allow_automatic_prompting_for_file_downloads(&mut self, value: String) {
        self.trusted_sites_zone_allow_automatic_prompting_for_file_downloads = Some(value);
    }

    /// Gets the value of TrustedSitesZoneAllowAutomaticPromptingForFileDownloads
    pub fn get_trusted_sites_zone_allow_automatic_prompting_for_file_downloads(&self) -> Option<&String> {
        self.trusted_sites_zone_allow_automatic_prompting_for_file_downloads.as_ref()
    }

    /// Sets the value of TrustedSitesZoneAllowFontDownloads
    pub fn set_trusted_sites_zone_allow_font_downloads(&mut self, value: String) {
        self.trusted_sites_zone_allow_font_downloads = Some(value);
    }

    /// Gets the value of TrustedSitesZoneAllowFontDownloads
    pub fn get_trusted_sites_zone_allow_font_downloads(&self) -> Option<&String> {
        self.trusted_sites_zone_allow_font_downloads.as_ref()
    }

    /// Sets the value of TrustedSitesZoneAllowLessPrivilegedSites
    pub fn set_trusted_sites_zone_allow_less_privileged_sites(&mut self, value: String) {
        self.trusted_sites_zone_allow_less_privileged_sites = Some(value);
    }

    /// Gets the value of TrustedSitesZoneAllowLessPrivilegedSites
    pub fn get_trusted_sites_zone_allow_less_privileged_sites(&self) -> Option<&String> {
        self.trusted_sites_zone_allow_less_privileged_sites.as_ref()
    }

    /// Sets the value of TrustedSitesZoneAllowNETFrameworkReliantComponents
    pub fn set_trusted_sites_zone_allow_netframework_reliant_components(&mut self, value: String) {
        self.trusted_sites_zone_allow_netframework_reliant_components = Some(value);
    }

    /// Gets the value of TrustedSitesZoneAllowNETFrameworkReliantComponents
    pub fn get_trusted_sites_zone_allow_netframework_reliant_components(&self) -> Option<&String> {
        self.trusted_sites_zone_allow_netframework_reliant_components.as_ref()
    }

    /// Sets the value of TrustedSitesZoneAllowScriptlets
    pub fn set_trusted_sites_zone_allow_scriptlets(&mut self, value: String) {
        self.trusted_sites_zone_allow_scriptlets = Some(value);
    }

    /// Gets the value of TrustedSitesZoneAllowScriptlets
    pub fn get_trusted_sites_zone_allow_scriptlets(&self) -> Option<&String> {
        self.trusted_sites_zone_allow_scriptlets.as_ref()
    }

    /// Sets the value of TrustedSitesZoneAllowSmartScreenIE
    pub fn set_trusted_sites_zone_allow_smart_screen_ie(&mut self, value: String) {
        self.trusted_sites_zone_allow_smart_screen_ie = Some(value);
    }

    /// Gets the value of TrustedSitesZoneAllowSmartScreenIE
    pub fn get_trusted_sites_zone_allow_smart_screen_ie(&self) -> Option<&String> {
        self.trusted_sites_zone_allow_smart_screen_ie.as_ref()
    }

    /// Sets the value of TrustedSitesZoneAllowUserDataPersistence
    pub fn set_trusted_sites_zone_allow_user_data_persistence(&mut self, value: String) {
        self.trusted_sites_zone_allow_user_data_persistence = Some(value);
    }

    /// Gets the value of TrustedSitesZoneAllowUserDataPersistence
    pub fn get_trusted_sites_zone_allow_user_data_persistence(&self) -> Option<&String> {
        self.trusted_sites_zone_allow_user_data_persistence.as_ref()
    }

    /// Sets the value of TrustedSitesZoneDoNotRunAntimalwareAgainstActiveXControls
    pub fn set_trusted_sites_zone_do_not_run_antimalware_against_active_xcontrols(&mut self, value: String) {
        self.trusted_sites_zone_do_not_run_antimalware_against_active_xcontrols = Some(value);
    }

    /// Gets the value of TrustedSitesZoneDoNotRunAntimalwareAgainstActiveXControls
    pub fn get_trusted_sites_zone_do_not_run_antimalware_against_active_xcontrols(&self) -> Option<&String> {
        self.trusted_sites_zone_do_not_run_antimalware_against_active_xcontrols.as_ref()
    }

    /// Sets the value of TrustedSitesZoneInitializeAndScriptActiveXControls
    pub fn set_trusted_sites_zone_initialize_and_script_active_xcontrols(&mut self, value: String) {
        self.trusted_sites_zone_initialize_and_script_active_xcontrols = Some(value);
    }

    /// Gets the value of TrustedSitesZoneInitializeAndScriptActiveXControls
    pub fn get_trusted_sites_zone_initialize_and_script_active_xcontrols(&self) -> Option<&String> {
        self.trusted_sites_zone_initialize_and_script_active_xcontrols.as_ref()
    }

    /// Sets the value of TrustedSitesZoneJavaPermissions
    pub fn set_trusted_sites_zone_java_permissions(&mut self, value: String) {
        self.trusted_sites_zone_java_permissions = Some(value);
    }

    /// Gets the value of TrustedSitesZoneJavaPermissions
    pub fn get_trusted_sites_zone_java_permissions(&self) -> Option<&String> {
        self.trusted_sites_zone_java_permissions.as_ref()
    }

    /// Sets the value of TrustedSitesZoneNavigateWindowsAndFrames
    pub fn set_trusted_sites_zone_navigate_windows_and_frames(&mut self, value: String) {
        self.trusted_sites_zone_navigate_windows_and_frames = Some(value);
    }

    /// Gets the value of TrustedSitesZoneNavigateWindowsAndFrames
    pub fn get_trusted_sites_zone_navigate_windows_and_frames(&self) -> Option<&String> {
        self.trusted_sites_zone_navigate_windows_and_frames.as_ref()
    }
}

