// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ServerManager
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ServerComponent_UpdateServices_Services struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServerComponent_UpdateServices_Services {
    #[serde(flatten)]
    pub base: MSFT_ServerManagerServerComponentDescriptor,

/// 
    #[serde(rename = "ContentDirectory")]
    pub content_directory: Option<String>,

/// 
    #[serde(rename = "ContentLocal")]
    pub content_local: Option<bool>,
}

impl ServerComponent_UpdateServices_Services {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_ServerManagerServerComponentDescriptor::new(),
            content_directory: None,
            content_local: None,
        }
    }


    /// Sets the value of ContentDirectory
    pub fn set_content_directory(&mut self, value: String) {
        self.content_directory = Some(value);
    }

    /// Gets the value of ContentDirectory
    pub fn get_content_directory(&self) -> Option<&String> {
        self.content_directory.as_ref()
    }

    /// Sets the value of ContentLocal
    pub fn set_content_local(&mut self, value: bool) {
        self.content_local = Some(value);
    }

    /// Gets the value of ContentLocal
    pub fn get_content_local(&self) -> Option<&bool> {
        self.content_local.as_ref()
    }
}

