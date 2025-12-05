// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_WMISetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_WMISetting {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "ASPScriptDefaultNamespace")]
    pub aspscript_default_namespace: Option<String>,

/// 
    #[serde(rename = "ASPScriptEnabled")]
    pub aspscript_enabled: Option<bool>,

/// 
    #[serde(rename = "AutorecoverMofs")]
    pub autorecover_mofs: Vec<String>,

/// 
    #[serde(rename = "AutoStartWin9X")]
    pub auto_start_win9_x: Option<u32>,

/// 
    #[serde(rename = "BackupInterval")]
    pub backup_interval: Option<u32>,

/// 
    #[serde(rename = "BackupLastTime")]
    pub backup_last_time: Option<String>,

/// 
    #[serde(rename = "BuildVersion")]
    pub build_version: Option<String>,

/// 
    #[serde(rename = "DatabaseDirectory")]
    pub database_directory: Option<String>,

/// 
    #[serde(rename = "DatabaseMaxSize")]
    pub database_max_size: Option<u32>,

/// 
    #[serde(rename = "EnableAnonWin9xConnections")]
    pub enable_anon_win9x_connections: Option<bool>,

/// 
    #[serde(rename = "EnableEvents")]
    pub enable_events: Option<bool>,

/// 
    #[serde(rename = "EnableStartupHeapPreallocation")]
    pub enable_startup_heap_preallocation: Option<bool>,

/// 
    #[serde(rename = "HighThresholdOnClientObjects")]
    pub high_threshold_on_client_objects: Option<u32>,

/// 
    #[serde(rename = "HighThresholdOnEvents")]
    pub high_threshold_on_events: Option<u32>,

/// 
    #[serde(rename = "InstallationDirectory")]
    pub installation_directory: Option<String>,

/// 
    #[serde(rename = "LastStartupHeapPreallocation")]
    pub last_startup_heap_preallocation: Option<u32>,

/// 
    #[serde(rename = "LoggingDirectory")]
    pub logging_directory: Option<String>,

/// 
    #[serde(rename = "LoggingLevel")]
    pub logging_level: Option<u32>,

/// 
    #[serde(rename = "LowThresholdOnClientObjects")]
    pub low_threshold_on_client_objects: Option<u32>,

/// 
    #[serde(rename = "LowThresholdOnEvents")]
    pub low_threshold_on_events: Option<u32>,

/// 
    #[serde(rename = "MaxLogFileSize")]
    pub max_log_file_size: Option<u32>,

/// 
    #[serde(rename = "MaxWaitOnClientObjects")]
    pub max_wait_on_client_objects: Option<u32>,

/// 
    #[serde(rename = "MaxWaitOnEvents")]
    pub max_wait_on_events: Option<u32>,

/// 
    #[serde(rename = "MofSelfInstallDirectory")]
    pub mof_self_install_directory: Option<String>,
}

impl Win32_WMISetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            aspscript_default_namespace: None,
            aspscript_enabled: None,
            autorecover_mofs: Vec::new(),
            auto_start_win9_x: None,
            backup_interval: None,
            backup_last_time: None,
            build_version: None,
            database_directory: None,
            database_max_size: None,
            enable_anon_win9x_connections: None,
            enable_events: None,
            enable_startup_heap_preallocation: None,
            high_threshold_on_client_objects: None,
            high_threshold_on_events: None,
            installation_directory: None,
            last_startup_heap_preallocation: None,
            logging_directory: None,
            logging_level: None,
            low_threshold_on_client_objects: None,
            low_threshold_on_events: None,
            max_log_file_size: None,
            max_wait_on_client_objects: None,
            max_wait_on_events: None,
            mof_self_install_directory: None,
        }
    }


    /// Sets the value of ASPScriptDefaultNamespace
    pub fn set_aspscript_default_namespace(&mut self, value: String) {
        self.aspscript_default_namespace = Some(value);
    }

    /// Gets the value of ASPScriptDefaultNamespace
    pub fn get_aspscript_default_namespace(&self) -> Option<&String> {
        self.aspscript_default_namespace.as_ref()
    }

    /// Sets the value of ASPScriptEnabled
    pub fn set_aspscript_enabled(&mut self, value: bool) {
        self.aspscript_enabled = Some(value);
    }

    /// Gets the value of ASPScriptEnabled
    pub fn get_aspscript_enabled(&self) -> Option<&bool> {
        self.aspscript_enabled.as_ref()
    }

    /// Sets the value of AutorecoverMofs
    pub fn set_autorecover_mofs(&mut self, value: Vec<String>) {
        self.autorecover_mofs = value;
    }

    /// Gets the value of AutorecoverMofs
    pub fn get_autorecover_mofs(&self) -> &Vec<String> {
        &self.autorecover_mofs
    }

    /// Sets the value of AutoStartWin9X
    pub fn set_auto_start_win9_x(&mut self, value: u32) {
        self.auto_start_win9_x = Some(value);
    }

    /// Gets the value of AutoStartWin9X
    pub fn get_auto_start_win9_x(&self) -> Option<&u32> {
        self.auto_start_win9_x.as_ref()
    }

    /// Sets the value of BackupInterval
    pub fn set_backup_interval(&mut self, value: u32) {
        self.backup_interval = Some(value);
    }

    /// Gets the value of BackupInterval
    pub fn get_backup_interval(&self) -> Option<&u32> {
        self.backup_interval.as_ref()
    }

    /// Sets the value of BackupLastTime
    pub fn set_backup_last_time(&mut self, value: String) {
        self.backup_last_time = Some(value);
    }

    /// Gets the value of BackupLastTime
    pub fn get_backup_last_time(&self) -> Option<&String> {
        self.backup_last_time.as_ref()
    }

    /// Sets the value of BuildVersion
    pub fn set_build_version(&mut self, value: String) {
        self.build_version = Some(value);
    }

    /// Gets the value of BuildVersion
    pub fn get_build_version(&self) -> Option<&String> {
        self.build_version.as_ref()
    }

    /// Sets the value of DatabaseDirectory
    pub fn set_database_directory(&mut self, value: String) {
        self.database_directory = Some(value);
    }

    /// Gets the value of DatabaseDirectory
    pub fn get_database_directory(&self) -> Option<&String> {
        self.database_directory.as_ref()
    }

    /// Sets the value of DatabaseMaxSize
    pub fn set_database_max_size(&mut self, value: u32) {
        self.database_max_size = Some(value);
    }

    /// Gets the value of DatabaseMaxSize
    pub fn get_database_max_size(&self) -> Option<&u32> {
        self.database_max_size.as_ref()
    }

    /// Sets the value of EnableAnonWin9xConnections
    pub fn set_enable_anon_win9x_connections(&mut self, value: bool) {
        self.enable_anon_win9x_connections = Some(value);
    }

    /// Gets the value of EnableAnonWin9xConnections
    pub fn get_enable_anon_win9x_connections(&self) -> Option<&bool> {
        self.enable_anon_win9x_connections.as_ref()
    }

    /// Sets the value of EnableEvents
    pub fn set_enable_events(&mut self, value: bool) {
        self.enable_events = Some(value);
    }

    /// Gets the value of EnableEvents
    pub fn get_enable_events(&self) -> Option<&bool> {
        self.enable_events.as_ref()
    }

    /// Sets the value of EnableStartupHeapPreallocation
    pub fn set_enable_startup_heap_preallocation(&mut self, value: bool) {
        self.enable_startup_heap_preallocation = Some(value);
    }

    /// Gets the value of EnableStartupHeapPreallocation
    pub fn get_enable_startup_heap_preallocation(&self) -> Option<&bool> {
        self.enable_startup_heap_preallocation.as_ref()
    }

    /// Sets the value of HighThresholdOnClientObjects
    pub fn set_high_threshold_on_client_objects(&mut self, value: u32) {
        self.high_threshold_on_client_objects = Some(value);
    }

    /// Gets the value of HighThresholdOnClientObjects
    pub fn get_high_threshold_on_client_objects(&self) -> Option<&u32> {
        self.high_threshold_on_client_objects.as_ref()
    }

    /// Sets the value of HighThresholdOnEvents
    pub fn set_high_threshold_on_events(&mut self, value: u32) {
        self.high_threshold_on_events = Some(value);
    }

    /// Gets the value of HighThresholdOnEvents
    pub fn get_high_threshold_on_events(&self) -> Option<&u32> {
        self.high_threshold_on_events.as_ref()
    }

    /// Sets the value of InstallationDirectory
    pub fn set_installation_directory(&mut self, value: String) {
        self.installation_directory = Some(value);
    }

    /// Gets the value of InstallationDirectory
    pub fn get_installation_directory(&self) -> Option<&String> {
        self.installation_directory.as_ref()
    }

    /// Sets the value of LastStartupHeapPreallocation
    pub fn set_last_startup_heap_preallocation(&mut self, value: u32) {
        self.last_startup_heap_preallocation = Some(value);
    }

    /// Gets the value of LastStartupHeapPreallocation
    pub fn get_last_startup_heap_preallocation(&self) -> Option<&u32> {
        self.last_startup_heap_preallocation.as_ref()
    }

    /// Sets the value of LoggingDirectory
    pub fn set_logging_directory(&mut self, value: String) {
        self.logging_directory = Some(value);
    }

    /// Gets the value of LoggingDirectory
    pub fn get_logging_directory(&self) -> Option<&String> {
        self.logging_directory.as_ref()
    }

    /// Sets the value of LoggingLevel
    pub fn set_logging_level(&mut self, value: u32) {
        self.logging_level = Some(value);
    }

    /// Gets the value of LoggingLevel
    pub fn get_logging_level(&self) -> Option<&u32> {
        self.logging_level.as_ref()
    }

    /// Sets the value of LowThresholdOnClientObjects
    pub fn set_low_threshold_on_client_objects(&mut self, value: u32) {
        self.low_threshold_on_client_objects = Some(value);
    }

    /// Gets the value of LowThresholdOnClientObjects
    pub fn get_low_threshold_on_client_objects(&self) -> Option<&u32> {
        self.low_threshold_on_client_objects.as_ref()
    }

    /// Sets the value of LowThresholdOnEvents
    pub fn set_low_threshold_on_events(&mut self, value: u32) {
        self.low_threshold_on_events = Some(value);
    }

    /// Gets the value of LowThresholdOnEvents
    pub fn get_low_threshold_on_events(&self) -> Option<&u32> {
        self.low_threshold_on_events.as_ref()
    }

    /// Sets the value of MaxLogFileSize
    pub fn set_max_log_file_size(&mut self, value: u32) {
        self.max_log_file_size = Some(value);
    }

    /// Gets the value of MaxLogFileSize
    pub fn get_max_log_file_size(&self) -> Option<&u32> {
        self.max_log_file_size.as_ref()
    }

    /// Sets the value of MaxWaitOnClientObjects
    pub fn set_max_wait_on_client_objects(&mut self, value: u32) {
        self.max_wait_on_client_objects = Some(value);
    }

    /// Gets the value of MaxWaitOnClientObjects
    pub fn get_max_wait_on_client_objects(&self) -> Option<&u32> {
        self.max_wait_on_client_objects.as_ref()
    }

    /// Sets the value of MaxWaitOnEvents
    pub fn set_max_wait_on_events(&mut self, value: u32) {
        self.max_wait_on_events = Some(value);
    }

    /// Gets the value of MaxWaitOnEvents
    pub fn get_max_wait_on_events(&self) -> Option<&u32> {
        self.max_wait_on_events.as_ref()
    }

    /// Sets the value of MofSelfInstallDirectory
    pub fn set_mof_self_install_directory(&mut self, value: String) {
        self.mof_self_install_directory = Some(value);
    }

    /// Gets the value of MofSelfInstallDirectory
    pub fn get_mof_self_install_directory(&self) -> Option<&String> {
        self.mof_self_install_directory.as_ref()
    }
}

