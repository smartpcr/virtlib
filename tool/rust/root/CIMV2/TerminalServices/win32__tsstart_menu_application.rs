// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TSStartMenuApplication struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TSStartMenuApplication {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// Command line arguments
    #[serde(rename = "CommandLineArguments")]
    pub command_line_arguments: Option<String>,

/// Index of the icon
    #[serde(rename = "IconIndex")]
    pub icon_index: Option<i32>,

/// Path to the application icon
    #[serde(rename = "IconPath")]
    pub icon_path: Option<String>,

/// Path to the application
    #[serde(rename = "Path")]
    pub path: Option<String>,

/// Virtual Path to the application (includes Environment Variables)
    #[serde(rename = "VPath")]
    pub vpath: Option<String>,
}

impl Win32_TSStartMenuApplication {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            command_line_arguments: None,
            icon_index: None,
            icon_path: None,
            path: None,
            vpath: None,
        }
    }


    /// Sets the value of CommandLineArguments
    pub fn set_command_line_arguments(&mut self, value: String) {
        self.command_line_arguments = Some(value);
    }

    /// Gets the value of CommandLineArguments
    pub fn get_command_line_arguments(&self) -> Option<&String> {
        self.command_line_arguments.as_ref()
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

    /// Sets the value of VPath
    pub fn set_vpath(&mut self, value: String) {
        self.vpath = Some(value);
    }

    /// Gets the value of VPath
    pub fn get_vpath(&self) -> Option<&String> {
        self.vpath.as_ref()
    }
}

