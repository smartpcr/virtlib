// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_DfsNode struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_DfsNode {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "Root")]
    pub root: Option<bool>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,

/// 
    #[serde(rename = "Timeout")]
    pub timeout: Option<u32>,
}

impl Win32_DfsNode {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            root: None,
            state: None,
            timeout: None,
        }
    }


    /// Sets the value of Root
    pub fn set_root(&mut self, value: bool) {
        self.root = Some(value);
    }

    /// Gets the value of Root
    pub fn get_root(&self) -> Option<&bool> {
        self.root.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u32> {
        self.state.as_ref()
    }

    /// Sets the value of Timeout
    pub fn set_timeout(&mut self, value: u32) {
        self.timeout = Some(value);
    }

    /// Gets the value of Timeout
    pub fn get_timeout(&self) -> Option<&u32> {
        self.timeout.as_ref()
    }

/// 

    /// * `description` -  (String)
    /// * `dfs_entry_path` -  (String)
    /// * `server_name` -  (String)
    /// * `share_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn create(&self, dfs_entry_path: &String, server_name: &String, share_name: &String, description: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DfsEntryPath".to_string(), value: dfs_entry_path.into() });
        args.push(MethodParameter { name: "ServerName".to_string(), value: server_name.into() });
        args.push(MethodParameter { name: "ShareName".to_string(), value: share_name.into() });
        if let Some(val) = description {
            args.push(MethodParameter { name: "Description".to_string(), value: val.into() });
        }
        self.invoke_method("Create", &args)

    }

}

