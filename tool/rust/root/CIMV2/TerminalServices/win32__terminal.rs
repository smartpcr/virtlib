// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_Terminal struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_Terminal {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "fEnableTerminal")]
    pub f_enable_terminal: Option<u32>,

/// 
    #[serde(rename = "LoggedOnUsers")]
    pub logged_on_users: Option<u32>,

/// 
    #[serde(rename = "TerminalName")]
    pub terminal_name: Option<String>,
}

impl Win32_Terminal {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            f_enable_terminal: None,
            logged_on_users: None,
            terminal_name: None,
        }
    }


    /// Sets the value of fEnableTerminal
    pub fn set_f_enable_terminal(&mut self, value: u32) {
        self.f_enable_terminal = Some(value);
    }

    /// Gets the value of fEnableTerminal
    pub fn get_f_enable_terminal(&self) -> Option<&u32> {
        self.f_enable_terminal.as_ref()
    }

    /// Sets the value of LoggedOnUsers
    pub fn set_logged_on_users(&mut self, value: u32) {
        self.logged_on_users = Some(value);
    }

    /// Gets the value of LoggedOnUsers
    pub fn get_logged_on_users(&self) -> Option<&u32> {
        self.logged_on_users.as_ref()
    }

    /// Sets the value of TerminalName
    pub fn set_terminal_name(&mut self, value: String) {
        self.terminal_name = Some(value);
    }

    /// Gets the value of TerminalName
    pub fn get_terminal_name(&self) -> Option<&String> {
        self.terminal_name.as_ref()
    }

/// 

    /// * `f_enable_terminal` -  (u32)

    /// * `return_value` -  (u32)
    pub fn enable(&self, f_enable_terminal: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "fEnableTerminal".to_string(), value: f_enable_terminal.into() });
        self.invoke_method("Enable", &args)

    }


/// 

    /// * `new_terminal_name` -  (String)
    /// * `terminal_protocol` -  (String)
    /// * `transport` -  (String)

    /// * `return_value` -  (u32)
    pub fn create(&self, new_terminal_name: &String, transport: &String, terminal_protocol: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewTerminalName".to_string(), value: new_terminal_name.into() });
        args.push(MethodParameter { name: "Transport".to_string(), value: transport.into() });
        args.push(MethodParameter { name: "TerminalProtocol".to_string(), value: terminal_protocol.into() });
        self.invoke_method("Create", &args)

    }


/// 

    /// * `new_terminal_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn rename(&self, new_terminal_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewTerminalName".to_string(), value: new_terminal_name.into() });
        self.invoke_method("Rename", &args)

    }


/// 

    /// * `new_terminal_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn delete(&self, new_terminal_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewTerminalName".to_string(), value: new_terminal_name.into() });
        self.invoke_method("Delete", &args)

    }

}

