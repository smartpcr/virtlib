// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_RDAllowListFileAssociation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_RDAllowListFileAssociation {

/// Alias of this file association's RemoteApp
    #[serde(rename = "AppAlias")]
    pub app_alias: Option<String>,

/// Name of the extension (e.g. .txt)
    #[serde(rename = "ExtName")]
    pub ext_name: Option<String>,

/// Hint to help open documents with this file association
    #[serde(rename = "ProgIdHint")]
    pub prog_id_hint: Option<String>,
}

impl Win32_RDAllowListFileAssociation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            app_alias: None,
            ext_name: None,
            prog_id_hint: None,
        }
    }


    /// Sets the value of AppAlias
    pub fn set_app_alias(&mut self, value: String) {
        self.app_alias = Some(value);
    }

    /// Gets the value of AppAlias
    pub fn get_app_alias(&self) -> Option<&String> {
        self.app_alias.as_ref()
    }

    /// Sets the value of ExtName
    pub fn set_ext_name(&mut self, value: String) {
        self.ext_name = Some(value);
    }

    /// Gets the value of ExtName
    pub fn get_ext_name(&self) -> Option<&String> {
        self.ext_name.as_ref()
    }

    /// Sets the value of ProgIdHint
    pub fn set_prog_id_hint(&mut self, value: String) {
        self.prog_id_hint = Some(value);
    }

    /// Gets the value of ProgIdHint
    pub fn get_prog_id_hint(&self) -> Option<&String> {
        self.prog_id_hint.as_ref()
    }
}

