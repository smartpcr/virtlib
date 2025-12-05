// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_System02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_System02 {

/// 
    #[serde(rename = "AllowBuildPreview")]
    pub allow_build_preview: Option<i32>,

/// 
    #[serde(rename = "AllowCommercialDataPipeline")]
    pub allow_commercial_data_pipeline: Option<i32>,

/// 
    #[serde(rename = "AllowDesktopAnalyticsProcessing")]
    pub allow_desktop_analytics_processing: Option<i32>,

/// 
    #[serde(rename = "AllowDeviceNameInDiagnosticData")]
    pub allow_device_name_in_diagnostic_data: Option<i32>,

/// 
    #[serde(rename = "AllowEmbeddedMode")]
    pub allow_embedded_mode: Option<i32>,

/// 
    #[serde(rename = "AllowExperimentation")]
    pub allow_experimentation: Option<i32>,

/// 
    #[serde(rename = "AllowFontProviders")]
    pub allow_font_providers: Option<i32>,

/// 
    #[serde(rename = "AllowLocation")]
    pub allow_location: Option<i32>,

/// 
    #[serde(rename = "AllowMicrosoftManagedDesktopProcessing")]
    pub allow_microsoft_managed_desktop_processing: Option<i32>,

/// 
    #[serde(rename = "AllowStorageCard")]
    pub allow_storage_card: Option<i32>,

/// 
    #[serde(rename = "AllowTelemetry")]
    pub allow_telemetry: Option<i32>,

/// 
    #[serde(rename = "AllowUpdateComplianceProcessing")]
    pub allow_update_compliance_processing: Option<i32>,

/// 
    #[serde(rename = "AllowUserToResetPhone")]
    pub allow_user_to_reset_phone: Option<i32>,

/// 
    #[serde(rename = "AllowWUfBCloudProcessing")]
    pub allow_wuf_bcloud_processing: Option<i32>,

/// 
    #[serde(rename = "BootStartDriverInitialization")]
    pub boot_start_driver_initialization: Option<String>,

/// 
    #[serde(rename = "ConfigureMicrosoft365UploadEndpoint")]
    pub configure_microsoft365_upload_endpoint: Option<String>,

/// 
    #[serde(rename = "ConfigureTelemetryOptInChangeNotification")]
    pub configure_telemetry_opt_in_change_notification: Option<i32>,

/// 
    #[serde(rename = "ConfigureTelemetryOptInSettingsUx")]
    pub configure_telemetry_opt_in_settings_ux: Option<i32>,

/// 
    #[serde(rename = "DisableDeviceDelete")]
    pub disable_device_delete: Option<i32>,

/// 
    #[serde(rename = "DisableDiagnosticDataViewer")]
    pub disable_diagnostic_data_viewer: Option<i32>,

/// 
    #[serde(rename = "DisableDirectXDatabaseUpdate")]
    pub disable_direct_xdatabase_update: Option<i32>,

/// 
    #[serde(rename = "DisableEnterpriseAuthProxy")]
    pub disable_enterprise_auth_proxy: Option<i32>,

/// 
    #[serde(rename = "DisableOneDriveFileSync")]
    pub disable_one_drive_file_sync: Option<i32>,

/// 
    #[serde(rename = "DisableOneSettingsDownloads")]
    pub disable_one_settings_downloads: Option<i32>,

/// 
    #[serde(rename = "DisableSystemRestore")]
    pub disable_system_restore: Option<String>,

/// 
    #[serde(rename = "EnableOneSettingsAuditing")]
    pub enable_one_settings_auditing: Option<i32>,

/// 
    #[serde(rename = "FeedbackHubAlwaysSaveDiagnosticsLocally")]
    pub feedback_hub_always_save_diagnostics_locally: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "LimitDiagnosticLogCollection")]
    pub limit_diagnostic_log_collection: Option<i32>,

/// 
    #[serde(rename = "LimitDumpCollection")]
    pub limit_dump_collection: Option<i32>,

/// 
    #[serde(rename = "LimitEnhancedDiagnosticDataWindowsAnalytics")]
    pub limit_enhanced_diagnostic_data_windows_analytics: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "TelemetryProxy")]
    pub telemetry_proxy: Option<String>,

/// 
    #[serde(rename = "TurnOffFileHistory")]
    pub turn_off_file_history: Option<i32>,
}

impl MDM_Policy_Config01_System02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_build_preview: None,
            allow_commercial_data_pipeline: None,
            allow_desktop_analytics_processing: None,
            allow_device_name_in_diagnostic_data: None,
            allow_embedded_mode: None,
            allow_experimentation: None,
            allow_font_providers: None,
            allow_location: None,
            allow_microsoft_managed_desktop_processing: None,
            allow_storage_card: None,
            allow_telemetry: None,
            allow_update_compliance_processing: None,
            allow_user_to_reset_phone: None,
            allow_wuf_bcloud_processing: None,
            boot_start_driver_initialization: None,
            configure_microsoft365_upload_endpoint: None,
            configure_telemetry_opt_in_change_notification: None,
            configure_telemetry_opt_in_settings_ux: None,
            disable_device_delete: None,
            disable_diagnostic_data_viewer: None,
            disable_direct_xdatabase_update: None,
            disable_enterprise_auth_proxy: None,
            disable_one_drive_file_sync: None,
            disable_one_settings_downloads: None,
            disable_system_restore: None,
            enable_one_settings_auditing: None,
            feedback_hub_always_save_diagnostics_locally: None,
            instance_id: None,
            limit_diagnostic_log_collection: None,
            limit_dump_collection: None,
            limit_enhanced_diagnostic_data_windows_analytics: None,
            parent_id: None,
            telemetry_proxy: None,
            turn_off_file_history: None,
        }
    }


    /// Sets the value of AllowBuildPreview
    pub fn set_allow_build_preview(&mut self, value: i32) {
        self.allow_build_preview = Some(value);
    }

    /// Gets the value of AllowBuildPreview
    pub fn get_allow_build_preview(&self) -> Option<&i32> {
        self.allow_build_preview.as_ref()
    }

    /// Sets the value of AllowCommercialDataPipeline
    pub fn set_allow_commercial_data_pipeline(&mut self, value: i32) {
        self.allow_commercial_data_pipeline = Some(value);
    }

    /// Gets the value of AllowCommercialDataPipeline
    pub fn get_allow_commercial_data_pipeline(&self) -> Option<&i32> {
        self.allow_commercial_data_pipeline.as_ref()
    }

    /// Sets the value of AllowDesktopAnalyticsProcessing
    pub fn set_allow_desktop_analytics_processing(&mut self, value: i32) {
        self.allow_desktop_analytics_processing = Some(value);
    }

    /// Gets the value of AllowDesktopAnalyticsProcessing
    pub fn get_allow_desktop_analytics_processing(&self) -> Option<&i32> {
        self.allow_desktop_analytics_processing.as_ref()
    }

    /// Sets the value of AllowDeviceNameInDiagnosticData
    pub fn set_allow_device_name_in_diagnostic_data(&mut self, value: i32) {
        self.allow_device_name_in_diagnostic_data = Some(value);
    }

    /// Gets the value of AllowDeviceNameInDiagnosticData
    pub fn get_allow_device_name_in_diagnostic_data(&self) -> Option<&i32> {
        self.allow_device_name_in_diagnostic_data.as_ref()
    }

    /// Sets the value of AllowEmbeddedMode
    pub fn set_allow_embedded_mode(&mut self, value: i32) {
        self.allow_embedded_mode = Some(value);
    }

    /// Gets the value of AllowEmbeddedMode
    pub fn get_allow_embedded_mode(&self) -> Option<&i32> {
        self.allow_embedded_mode.as_ref()
    }

    /// Sets the value of AllowExperimentation
    pub fn set_allow_experimentation(&mut self, value: i32) {
        self.allow_experimentation = Some(value);
    }

    /// Gets the value of AllowExperimentation
    pub fn get_allow_experimentation(&self) -> Option<&i32> {
        self.allow_experimentation.as_ref()
    }

    /// Sets the value of AllowFontProviders
    pub fn set_allow_font_providers(&mut self, value: i32) {
        self.allow_font_providers = Some(value);
    }

    /// Gets the value of AllowFontProviders
    pub fn get_allow_font_providers(&self) -> Option<&i32> {
        self.allow_font_providers.as_ref()
    }

    /// Sets the value of AllowLocation
    pub fn set_allow_location(&mut self, value: i32) {
        self.allow_location = Some(value);
    }

    /// Gets the value of AllowLocation
    pub fn get_allow_location(&self) -> Option<&i32> {
        self.allow_location.as_ref()
    }

    /// Sets the value of AllowMicrosoftManagedDesktopProcessing
    pub fn set_allow_microsoft_managed_desktop_processing(&mut self, value: i32) {
        self.allow_microsoft_managed_desktop_processing = Some(value);
    }

    /// Gets the value of AllowMicrosoftManagedDesktopProcessing
    pub fn get_allow_microsoft_managed_desktop_processing(&self) -> Option<&i32> {
        self.allow_microsoft_managed_desktop_processing.as_ref()
    }

    /// Sets the value of AllowStorageCard
    pub fn set_allow_storage_card(&mut self, value: i32) {
        self.allow_storage_card = Some(value);
    }

    /// Gets the value of AllowStorageCard
    pub fn get_allow_storage_card(&self) -> Option<&i32> {
        self.allow_storage_card.as_ref()
    }

    /// Sets the value of AllowTelemetry
    pub fn set_allow_telemetry(&mut self, value: i32) {
        self.allow_telemetry = Some(value);
    }

    /// Gets the value of AllowTelemetry
    pub fn get_allow_telemetry(&self) -> Option<&i32> {
        self.allow_telemetry.as_ref()
    }

    /// Sets the value of AllowUpdateComplianceProcessing
    pub fn set_allow_update_compliance_processing(&mut self, value: i32) {
        self.allow_update_compliance_processing = Some(value);
    }

    /// Gets the value of AllowUpdateComplianceProcessing
    pub fn get_allow_update_compliance_processing(&self) -> Option<&i32> {
        self.allow_update_compliance_processing.as_ref()
    }

    /// Sets the value of AllowUserToResetPhone
    pub fn set_allow_user_to_reset_phone(&mut self, value: i32) {
        self.allow_user_to_reset_phone = Some(value);
    }

    /// Gets the value of AllowUserToResetPhone
    pub fn get_allow_user_to_reset_phone(&self) -> Option<&i32> {
        self.allow_user_to_reset_phone.as_ref()
    }

    /// Sets the value of AllowWUfBCloudProcessing
    pub fn set_allow_wuf_bcloud_processing(&mut self, value: i32) {
        self.allow_wuf_bcloud_processing = Some(value);
    }

    /// Gets the value of AllowWUfBCloudProcessing
    pub fn get_allow_wuf_bcloud_processing(&self) -> Option<&i32> {
        self.allow_wuf_bcloud_processing.as_ref()
    }

    /// Sets the value of BootStartDriverInitialization
    pub fn set_boot_start_driver_initialization(&mut self, value: String) {
        self.boot_start_driver_initialization = Some(value);
    }

    /// Gets the value of BootStartDriverInitialization
    pub fn get_boot_start_driver_initialization(&self) -> Option<&String> {
        self.boot_start_driver_initialization.as_ref()
    }

    /// Sets the value of ConfigureMicrosoft365UploadEndpoint
    pub fn set_configure_microsoft365_upload_endpoint(&mut self, value: String) {
        self.configure_microsoft365_upload_endpoint = Some(value);
    }

    /// Gets the value of ConfigureMicrosoft365UploadEndpoint
    pub fn get_configure_microsoft365_upload_endpoint(&self) -> Option<&String> {
        self.configure_microsoft365_upload_endpoint.as_ref()
    }

    /// Sets the value of ConfigureTelemetryOptInChangeNotification
    pub fn set_configure_telemetry_opt_in_change_notification(&mut self, value: i32) {
        self.configure_telemetry_opt_in_change_notification = Some(value);
    }

    /// Gets the value of ConfigureTelemetryOptInChangeNotification
    pub fn get_configure_telemetry_opt_in_change_notification(&self) -> Option<&i32> {
        self.configure_telemetry_opt_in_change_notification.as_ref()
    }

    /// Sets the value of ConfigureTelemetryOptInSettingsUx
    pub fn set_configure_telemetry_opt_in_settings_ux(&mut self, value: i32) {
        self.configure_telemetry_opt_in_settings_ux = Some(value);
    }

    /// Gets the value of ConfigureTelemetryOptInSettingsUx
    pub fn get_configure_telemetry_opt_in_settings_ux(&self) -> Option<&i32> {
        self.configure_telemetry_opt_in_settings_ux.as_ref()
    }

    /// Sets the value of DisableDeviceDelete
    pub fn set_disable_device_delete(&mut self, value: i32) {
        self.disable_device_delete = Some(value);
    }

    /// Gets the value of DisableDeviceDelete
    pub fn get_disable_device_delete(&self) -> Option<&i32> {
        self.disable_device_delete.as_ref()
    }

    /// Sets the value of DisableDiagnosticDataViewer
    pub fn set_disable_diagnostic_data_viewer(&mut self, value: i32) {
        self.disable_diagnostic_data_viewer = Some(value);
    }

    /// Gets the value of DisableDiagnosticDataViewer
    pub fn get_disable_diagnostic_data_viewer(&self) -> Option<&i32> {
        self.disable_diagnostic_data_viewer.as_ref()
    }

    /// Sets the value of DisableDirectXDatabaseUpdate
    pub fn set_disable_direct_xdatabase_update(&mut self, value: i32) {
        self.disable_direct_xdatabase_update = Some(value);
    }

    /// Gets the value of DisableDirectXDatabaseUpdate
    pub fn get_disable_direct_xdatabase_update(&self) -> Option<&i32> {
        self.disable_direct_xdatabase_update.as_ref()
    }

    /// Sets the value of DisableEnterpriseAuthProxy
    pub fn set_disable_enterprise_auth_proxy(&mut self, value: i32) {
        self.disable_enterprise_auth_proxy = Some(value);
    }

    /// Gets the value of DisableEnterpriseAuthProxy
    pub fn get_disable_enterprise_auth_proxy(&self) -> Option<&i32> {
        self.disable_enterprise_auth_proxy.as_ref()
    }

    /// Sets the value of DisableOneDriveFileSync
    pub fn set_disable_one_drive_file_sync(&mut self, value: i32) {
        self.disable_one_drive_file_sync = Some(value);
    }

    /// Gets the value of DisableOneDriveFileSync
    pub fn get_disable_one_drive_file_sync(&self) -> Option<&i32> {
        self.disable_one_drive_file_sync.as_ref()
    }

    /// Sets the value of DisableOneSettingsDownloads
    pub fn set_disable_one_settings_downloads(&mut self, value: i32) {
        self.disable_one_settings_downloads = Some(value);
    }

    /// Gets the value of DisableOneSettingsDownloads
    pub fn get_disable_one_settings_downloads(&self) -> Option<&i32> {
        self.disable_one_settings_downloads.as_ref()
    }

    /// Sets the value of DisableSystemRestore
    pub fn set_disable_system_restore(&mut self, value: String) {
        self.disable_system_restore = Some(value);
    }

    /// Gets the value of DisableSystemRestore
    pub fn get_disable_system_restore(&self) -> Option<&String> {
        self.disable_system_restore.as_ref()
    }

    /// Sets the value of EnableOneSettingsAuditing
    pub fn set_enable_one_settings_auditing(&mut self, value: i32) {
        self.enable_one_settings_auditing = Some(value);
    }

    /// Gets the value of EnableOneSettingsAuditing
    pub fn get_enable_one_settings_auditing(&self) -> Option<&i32> {
        self.enable_one_settings_auditing.as_ref()
    }

    /// Sets the value of FeedbackHubAlwaysSaveDiagnosticsLocally
    pub fn set_feedback_hub_always_save_diagnostics_locally(&mut self, value: i32) {
        self.feedback_hub_always_save_diagnostics_locally = Some(value);
    }

    /// Gets the value of FeedbackHubAlwaysSaveDiagnosticsLocally
    pub fn get_feedback_hub_always_save_diagnostics_locally(&self) -> Option<&i32> {
        self.feedback_hub_always_save_diagnostics_locally.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of LimitDiagnosticLogCollection
    pub fn set_limit_diagnostic_log_collection(&mut self, value: i32) {
        self.limit_diagnostic_log_collection = Some(value);
    }

    /// Gets the value of LimitDiagnosticLogCollection
    pub fn get_limit_diagnostic_log_collection(&self) -> Option<&i32> {
        self.limit_diagnostic_log_collection.as_ref()
    }

    /// Sets the value of LimitDumpCollection
    pub fn set_limit_dump_collection(&mut self, value: i32) {
        self.limit_dump_collection = Some(value);
    }

    /// Gets the value of LimitDumpCollection
    pub fn get_limit_dump_collection(&self) -> Option<&i32> {
        self.limit_dump_collection.as_ref()
    }

    /// Sets the value of LimitEnhancedDiagnosticDataWindowsAnalytics
    pub fn set_limit_enhanced_diagnostic_data_windows_analytics(&mut self, value: i32) {
        self.limit_enhanced_diagnostic_data_windows_analytics = Some(value);
    }

    /// Gets the value of LimitEnhancedDiagnosticDataWindowsAnalytics
    pub fn get_limit_enhanced_diagnostic_data_windows_analytics(&self) -> Option<&i32> {
        self.limit_enhanced_diagnostic_data_windows_analytics.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of TelemetryProxy
    pub fn set_telemetry_proxy(&mut self, value: String) {
        self.telemetry_proxy = Some(value);
    }

    /// Gets the value of TelemetryProxy
    pub fn get_telemetry_proxy(&self) -> Option<&String> {
        self.telemetry_proxy.as_ref()
    }

    /// Sets the value of TurnOffFileHistory
    pub fn set_turn_off_file_history(&mut self, value: i32) {
        self.turn_off_file_history = Some(value);
    }

    /// Gets the value of TurnOffFileHistory
    pub fn get_turn_off_file_history(&self) -> Option<&i32> {
        self.turn_off_file_history.as_ref()
    }
}

