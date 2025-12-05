// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ExtensionInfoAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ExtensionInfoAction {
    #[serde(flatten)]
    pub base: CIM_Action,

/// 
    #[serde(rename = "Argument")]
    pub argument: Option<String>,

/// 
    #[serde(rename = "Command")]
    pub command: Option<String>,

/// 
    #[serde(rename = "Extension")]
    pub extension: Option<String>,

/// 
    #[serde(rename = "MIME")]
    pub mime: Option<String>,

/// 
    #[serde(rename = "ProgID")]
    pub prog_id: Option<String>,

/// 
    #[serde(rename = "ShellNew")]
    pub shell_new: Option<String>,

/// 
    #[serde(rename = "ShellNewValue")]
    pub shell_new_value: Option<String>,

/// 
    #[serde(rename = "Verb")]
    pub verb: Option<String>,
}

impl Win32_ExtensionInfoAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Action::new(),
            argument: None,
            command: None,
            extension: None,
            mime: None,
            prog_id: None,
            shell_new: None,
            shell_new_value: None,
            verb: None,
        }
    }


    /// Sets the value of Argument
    pub fn set_argument(&mut self, value: String) {
        self.argument = Some(value);
    }

    /// Gets the value of Argument
    pub fn get_argument(&self) -> Option<&String> {
        self.argument.as_ref()
    }

    /// Sets the value of Command
    pub fn set_command(&mut self, value: String) {
        self.command = Some(value);
    }

    /// Gets the value of Command
    pub fn get_command(&self) -> Option<&String> {
        self.command.as_ref()
    }

    /// Sets the value of Extension
    pub fn set_extension(&mut self, value: String) {
        self.extension = Some(value);
    }

    /// Gets the value of Extension
    pub fn get_extension(&self) -> Option<&String> {
        self.extension.as_ref()
    }

    /// Sets the value of MIME
    pub fn set_mime(&mut self, value: String) {
        self.mime = Some(value);
    }

    /// Gets the value of MIME
    pub fn get_mime(&self) -> Option<&String> {
        self.mime.as_ref()
    }

    /// Sets the value of ProgID
    pub fn set_prog_id(&mut self, value: String) {
        self.prog_id = Some(value);
    }

    /// Gets the value of ProgID
    pub fn get_prog_id(&self) -> Option<&String> {
        self.prog_id.as_ref()
    }

    /// Sets the value of ShellNew
    pub fn set_shell_new(&mut self, value: String) {
        self.shell_new = Some(value);
    }

    /// Gets the value of ShellNew
    pub fn get_shell_new(&self) -> Option<&String> {
        self.shell_new.as_ref()
    }

    /// Sets the value of ShellNewValue
    pub fn set_shell_new_value(&mut self, value: String) {
        self.shell_new_value = Some(value);
    }

    /// Gets the value of ShellNewValue
    pub fn get_shell_new_value(&self) -> Option<&String> {
        self.shell_new_value.as_ref()
    }

    /// Sets the value of Verb
    pub fn set_verb(&mut self, value: String) {
        self.verb = Some(value);
    }

    /// Gets the value of Verb
    pub fn get_verb(&self) -> Option<&String> {
        self.verb.as_ref()
    }
}

