// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Process_V4_TypeGroup1 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Process_V4_TypeGroup1 {
    #[serde(flatten)]
    pub base: Process_V4,

/// 
    #[serde(rename = "ApplicationId")]
    pub application_id: Option<String>,

/// 
    #[serde(rename = "CommandLine")]
    pub command_line: Option<String>,

/// 
    #[serde(rename = "DirectoryTableBase")]
    pub directory_table_base: Option<u32>,

/// 
    #[serde(rename = "ExitStatus")]
    pub exit_status: Option<i32>,

/// 
    #[serde(rename = "ImageFileName")]
    pub image_file_name: Option<String>,

/// 
    #[serde(rename = "PackageFullName")]
    pub package_full_name: Option<String>,

/// 
    #[serde(rename = "ParentId")]
    pub parent_id: Option<u32>,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "SessionId")]
    pub session_id: Option<u32>,

/// 
    #[serde(rename = "UniqueProcessKey")]
    pub unique_process_key: Option<u32>,

/// 
    #[serde(rename = "UserSID")]
    pub user_sid: Option<serde_json::Value>,
}

impl Process_V4_TypeGroup1 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Process_V4::new(),
            application_id: None,
            command_line: None,
            directory_table_base: None,
            exit_status: None,
            image_file_name: None,
            package_full_name: None,
            parent_id: None,
            process_id: None,
            session_id: None,
            unique_process_key: None,
            user_sid: None,
        }
    }


    /// Sets the value of ApplicationId
    pub fn set_application_id(&mut self, value: String) {
        self.application_id = Some(value);
    }

    /// Gets the value of ApplicationId
    pub fn get_application_id(&self) -> Option<&String> {
        self.application_id.as_ref()
    }

    /// Sets the value of CommandLine
    pub fn set_command_line(&mut self, value: String) {
        self.command_line = Some(value);
    }

    /// Gets the value of CommandLine
    pub fn get_command_line(&self) -> Option<&String> {
        self.command_line.as_ref()
    }

    /// Sets the value of DirectoryTableBase
    pub fn set_directory_table_base(&mut self, value: u32) {
        self.directory_table_base = Some(value);
    }

    /// Gets the value of DirectoryTableBase
    pub fn get_directory_table_base(&self) -> Option<&u32> {
        self.directory_table_base.as_ref()
    }

    /// Sets the value of ExitStatus
    pub fn set_exit_status(&mut self, value: i32) {
        self.exit_status = Some(value);
    }

    /// Gets the value of ExitStatus
    pub fn get_exit_status(&self) -> Option<&i32> {
        self.exit_status.as_ref()
    }

    /// Sets the value of ImageFileName
    pub fn set_image_file_name(&mut self, value: String) {
        self.image_file_name = Some(value);
    }

    /// Gets the value of ImageFileName
    pub fn get_image_file_name(&self) -> Option<&String> {
        self.image_file_name.as_ref()
    }

    /// Sets the value of PackageFullName
    pub fn set_package_full_name(&mut self, value: String) {
        self.package_full_name = Some(value);
    }

    /// Gets the value of PackageFullName
    pub fn get_package_full_name(&self) -> Option<&String> {
        self.package_full_name.as_ref()
    }

    /// Sets the value of ParentId
    pub fn set_parent_id(&mut self, value: u32) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentId
    pub fn get_parent_id(&self) -> Option<&u32> {
        self.parent_id.as_ref()
    }

    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }

    /// Sets the value of SessionId
    pub fn set_session_id(&mut self, value: u32) {
        self.session_id = Some(value);
    }

    /// Gets the value of SessionId
    pub fn get_session_id(&self) -> Option<&u32> {
        self.session_id.as_ref()
    }

    /// Sets the value of UniqueProcessKey
    pub fn set_unique_process_key(&mut self, value: u32) {
        self.unique_process_key = Some(value);
    }

    /// Gets the value of UniqueProcessKey
    pub fn get_unique_process_key(&self) -> Option<&u32> {
        self.unique_process_key.as_ref()
    }

    /// Sets the value of UserSID
    pub fn set_user_sid(&mut self, value: serde_json::Value) {
        self.user_sid = Some(value);
    }

    /// Gets the value of UserSID
    pub fn get_user_sid(&self) -> Option<&serde_json::Value> {
        self.user_sid.as_ref()
    }
}

