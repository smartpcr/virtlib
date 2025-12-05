// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ClusterShare struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ClusterShare {
    #[serde(flatten)]
    pub base: Win32_Share,

/// 
    #[serde(rename = "ServerName")]
    pub server_name: Option<String>,
}

impl Win32_ClusterShare {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_Share::new(),
            server_name: None,
        }
    }


    /// Sets the value of ServerName
    pub fn set_server_name(&mut self, value: String) {
        self.server_name = Some(value);
    }

    /// Gets the value of ServerName
    pub fn get_server_name(&self) -> Option<&String> {
        self.server_name.as_ref()
    }
}

