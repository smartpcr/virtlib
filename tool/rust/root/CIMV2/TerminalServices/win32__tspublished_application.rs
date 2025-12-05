// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TSPublishedApplication struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TSPublishedApplication {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// Alias of the application
    #[serde(rename = "Alias")]
    pub alias: Option<String>,

/// Command Line Arguments setting
    #[serde(rename = "CommandLineSetting")]
    pub command_line_setting: Option<TSPublishedApplication_CommandLineSetting>,

/// Contents of the icon corresponding to the application
    #[serde(rename = "IconContents")]
    pub icon_contents: Vec<u8>,

/// Index of the icon
    #[serde(rename = "IconIndex")]
    pub icon_index: Option<i32>,

/// Path to the application icon
    #[serde(rename = "IconPath")]
    pub icon_path: Option<String>,

/// Path to the application
    #[serde(rename = "Path")]
    pub path: Option<String>,

/// Whether application path is valid
    #[serde(rename = "PathExists")]
    pub path_exists: Option<bool>,

/// Contents of the RDP file corresponding to the application
    #[serde(rename = "RDPFileContents")]
    pub rdpfile_contents: Option<String>,

/// Command Line Arguments required for this application
    #[serde(rename = "RequiredCommandLine")]
    pub required_command_line: Option<String>,

/// Security Descriptor controlling access to the application, in SDDL Format. Empty string implies allow all access. Does not support DENY ACEs, or ACEs referring to non-domain users or groups.
    #[serde(rename = "SecurityDescriptor")]
    pub security_descriptor: Option<String>,

/// Whether this application should be shown in the TS Web Access
    #[serde(rename = "ShowInPortal")]
    pub show_in_portal: Option<bool>,

/// Virtual Path to the application
    #[serde(rename = "VPath")]
    pub vpath: Option<String>,
}

impl Win32_TSPublishedApplication {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            alias: None,
            command_line_setting: None,
            icon_contents: Vec::new(),
            icon_index: None,
            icon_path: None,
            path: None,
            path_exists: None,
            rdpfile_contents: None,
            required_command_line: None,
            security_descriptor: None,
            show_in_portal: None,
            vpath: None,
        }
    }


    /// Sets the value of Alias
    pub fn set_alias(&mut self, value: String) {
        self.alias = Some(value);
    }

    /// Gets the value of Alias
    pub fn get_alias(&self) -> Option<&String> {
        self.alias.as_ref()
    }

    /// Sets the value of CommandLineSetting
    pub fn set_command_line_setting(&mut self, value: TSPublishedApplication_CommandLineSetting) {
        self.command_line_setting = Some(value);
    }

    /// Gets the value of CommandLineSetting
    pub fn get_command_line_setting(&self) -> Option<&TSPublishedApplication_CommandLineSetting> {
        self.command_line_setting.as_ref()
    }

    /// Sets the value of IconContents
    pub fn set_icon_contents(&mut self, value: Vec<u8>) {
        self.icon_contents = value;
    }

    /// Gets the value of IconContents
    pub fn get_icon_contents(&self) -> &Vec<u8> {
        &self.icon_contents
    }

    /// Sets the value of IconIndex
    pub fn set_icon_index(&mut self, value: i32) {
        self.icon_index = Some(value);
    }

    /// Gets the value of IconIndex
    pub fn get_icon_index(&self) -> Option<&i32> {
        self.icon_index.as_ref()
    }

    /// Sets the value of IconPath
    pub fn set_icon_path(&mut self, value: String) {
        self.icon_path = Some(value);
    }

    /// Gets the value of IconPath
    pub fn get_icon_path(&self) -> Option<&String> {
        self.icon_path.as_ref()
    }

    /// Sets the value of Path
    pub fn set_path(&mut self, value: String) {
        self.path = Some(value);
    }

    /// Gets the value of Path
    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// Sets the value of PathExists
    pub fn set_path_exists(&mut self, value: bool) {
        self.path_exists = Some(value);
    }

    /// Gets the value of PathExists
    pub fn get_path_exists(&self) -> Option<&bool> {
        self.path_exists.as_ref()
    }

    /// Sets the value of RDPFileContents
    pub fn set_rdpfile_contents(&mut self, value: String) {
        self.rdpfile_contents = Some(value);
    }

    /// Gets the value of RDPFileContents
    pub fn get_rdpfile_contents(&self) -> Option<&String> {
        self.rdpfile_contents.as_ref()
    }

    /// Sets the value of RequiredCommandLine
    pub fn set_required_command_line(&mut self, value: String) {
        self.required_command_line = Some(value);
    }

    /// Gets the value of RequiredCommandLine
    pub fn get_required_command_line(&self) -> Option<&String> {
        self.required_command_line.as_ref()
    }

    /// Sets the value of SecurityDescriptor
    pub fn set_security_descriptor(&mut self, value: String) {
        self.security_descriptor = Some(value);
    }

    /// Gets the value of SecurityDescriptor
    pub fn get_security_descriptor(&self) -> Option<&String> {
        self.security_descriptor.as_ref()
    }

    /// Sets the value of ShowInPortal
    pub fn set_show_in_portal(&mut self, value: bool) {
        self.show_in_portal = Some(value);
    }

    /// Gets the value of ShowInPortal
    pub fn get_show_in_portal(&self) -> Option<&bool> {
        self.show_in_portal.as_ref()
    }

    /// Sets the value of VPath
    pub fn set_vpath(&mut self, value: String) {
        self.vpath = Some(value);
    }

    /// Gets the value of VPath
    pub fn get_vpath(&self) -> Option<&String> {
        self.vpath.as_ref()
    }
}

