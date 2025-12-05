// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_DfsTarget struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_DfsTarget {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "LinkName")]
    pub link_name: Option<String>,

/// 
    #[serde(rename = "ServerName")]
    pub server_name: Option<String>,

/// 
    #[serde(rename = "ShareName")]
    pub share_name: Option<String>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,
}

impl Win32_DfsTarget {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            link_name: None,
            server_name: None,
            share_name: None,
            state: None,
        }
    }


    /// Sets the value of LinkName
    pub fn set_link_name(&mut self, value: String) {
        self.link_name = Some(value);
    }

    /// Gets the value of LinkName
    pub fn get_link_name(&self) -> Option<&String> {
        self.link_name.as_ref()
    }

    /// Sets the value of ServerName
    pub fn set_server_name(&mut self, value: String) {
        self.server_name = Some(value);
    }

    /// Gets the value of ServerName
    pub fn get_server_name(&self) -> Option<&String> {
        self.server_name.as_ref()
    }

    /// Sets the value of ShareName
    pub fn set_share_name(&mut self, value: String) {
        self.share_name = Some(value);
    }

    /// Gets the value of ShareName
    pub fn get_share_name(&self) -> Option<&String> {
        self.share_name.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u32> {
        self.state.as_ref()
    }
}

