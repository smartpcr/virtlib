// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_OSRecoveryConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_OSRecoveryConfiguration {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "AutoReboot")]
    pub auto_reboot: Option<bool>,

/// 
    #[serde(rename = "DebugFilePath")]
    pub debug_file_path: Option<String>,

/// 
    #[serde(rename = "DebugInfoType")]
    pub debug_info_type: Option<u32>,

/// 
    #[serde(rename = "ExpandedDebugFilePath")]
    pub expanded_debug_file_path: Option<String>,

/// 
    #[serde(rename = "ExpandedMiniDumpDirectory")]
    pub expanded_mini_dump_directory: Option<String>,

/// 
    #[serde(rename = "KernelDumpOnly")]
    pub kernel_dump_only: Option<bool>,

/// 
    #[serde(rename = "MiniDumpDirectory")]
    pub mini_dump_directory: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "OverwriteExistingDebugFile")]
    pub overwrite_existing_debug_file: Option<bool>,

/// 
    #[serde(rename = "SendAdminAlert")]
    pub send_admin_alert: Option<bool>,

/// 
    #[serde(rename = "WriteDebugInfo")]
    pub write_debug_info: Option<bool>,

/// 
    #[serde(rename = "WriteToSystemLog")]
    pub write_to_system_log: Option<bool>,
}

impl Win32_OSRecoveryConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            auto_reboot: None,
            debug_file_path: None,
            debug_info_type: None,
            expanded_debug_file_path: None,
            expanded_mini_dump_directory: None,
            kernel_dump_only: None,
            mini_dump_directory: None,
            name: None,
            overwrite_existing_debug_file: None,
            send_admin_alert: None,
            write_debug_info: None,
            write_to_system_log: None,
        }
    }


    /// Sets the value of AutoReboot
    pub fn set_auto_reboot(&mut self, value: bool) {
        self.auto_reboot = Some(value);
    }

    /// Gets the value of AutoReboot
    pub fn get_auto_reboot(&self) -> Option<&bool> {
        self.auto_reboot.as_ref()
    }

    /// Sets the value of DebugFilePath
    pub fn set_debug_file_path(&mut self, value: String) {
        self.debug_file_path = Some(value);
    }

    /// Gets the value of DebugFilePath
    pub fn get_debug_file_path(&self) -> Option<&String> {
        self.debug_file_path.as_ref()
    }

    /// Sets the value of DebugInfoType
    pub fn set_debug_info_type(&mut self, value: u32) {
        self.debug_info_type = Some(value);
    }

    /// Gets the value of DebugInfoType
    pub fn get_debug_info_type(&self) -> Option<&u32> {
        self.debug_info_type.as_ref()
    }

    /// Sets the value of ExpandedDebugFilePath
    pub fn set_expanded_debug_file_path(&mut self, value: String) {
        self.expanded_debug_file_path = Some(value);
    }

    /// Gets the value of ExpandedDebugFilePath
    pub fn get_expanded_debug_file_path(&self) -> Option<&String> {
        self.expanded_debug_file_path.as_ref()
    }

    /// Sets the value of ExpandedMiniDumpDirectory
    pub fn set_expanded_mini_dump_directory(&mut self, value: String) {
        self.expanded_mini_dump_directory = Some(value);
    }

    /// Gets the value of ExpandedMiniDumpDirectory
    pub fn get_expanded_mini_dump_directory(&self) -> Option<&String> {
        self.expanded_mini_dump_directory.as_ref()
    }

    /// Sets the value of KernelDumpOnly
    pub fn set_kernel_dump_only(&mut self, value: bool) {
        self.kernel_dump_only = Some(value);
    }

    /// Gets the value of KernelDumpOnly
    pub fn get_kernel_dump_only(&self) -> Option<&bool> {
        self.kernel_dump_only.as_ref()
    }

    /// Sets the value of MiniDumpDirectory
    pub fn set_mini_dump_directory(&mut self, value: String) {
        self.mini_dump_directory = Some(value);
    }

    /// Gets the value of MiniDumpDirectory
    pub fn get_mini_dump_directory(&self) -> Option<&String> {
        self.mini_dump_directory.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of OverwriteExistingDebugFile
    pub fn set_overwrite_existing_debug_file(&mut self, value: bool) {
        self.overwrite_existing_debug_file = Some(value);
    }

    /// Gets the value of OverwriteExistingDebugFile
    pub fn get_overwrite_existing_debug_file(&self) -> Option<&bool> {
        self.overwrite_existing_debug_file.as_ref()
    }

    /// Sets the value of SendAdminAlert
    pub fn set_send_admin_alert(&mut self, value: bool) {
        self.send_admin_alert = Some(value);
    }

    /// Gets the value of SendAdminAlert
    pub fn get_send_admin_alert(&self) -> Option<&bool> {
        self.send_admin_alert.as_ref()
    }

    /// Sets the value of WriteDebugInfo
    pub fn set_write_debug_info(&mut self, value: bool) {
        self.write_debug_info = Some(value);
    }

    /// Gets the value of WriteDebugInfo
    pub fn get_write_debug_info(&self) -> Option<&bool> {
        self.write_debug_info.as_ref()
    }

    /// Sets the value of WriteToSystemLog
    pub fn set_write_to_system_log(&mut self, value: bool) {
        self.write_to_system_log = Some(value);
    }

    /// Gets the value of WriteToSystemLog
    pub fn get_write_to_system_log(&self) -> Option<&bool> {
        self.write_to_system_log.as_ref()
    }
}

