// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_RDFileAssociation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_RDFileAssociation {

/// Name of the extension (e.g. .txt)
    #[serde(rename = "ExtName")]
    pub ext_name: Option<String>,

/// Contents of the icon for this file association
    #[serde(rename = "IconContents")]
    pub icon_contents: Vec<u8>,

/// Index of the icon for this file association
    #[serde(rename = "IconIndex")]
    pub icon_index: Option<i32>,

/// The path to the icon for this file association
    #[serde(rename = "IconPath")]
    pub icon_path: Option<String>,

/// Whether this association is for a primary handler
    #[serde(rename = "PrimaryHandler")]
    pub primary_handler: Option<bool>,

/// Hint to help open documents with this file association
    #[serde(rename = "ProgIdHint")]
    pub prog_id_hint: Option<String>,
}

impl Win32_RDFileAssociation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            ext_name: None,
            icon_contents: Vec::new(),
            icon_index: None,
            icon_path: None,
            primary_handler: None,
            prog_id_hint: None,
        }
    }


    /// Sets the value of ExtName
    pub fn set_ext_name(&mut self, value: String) {
        self.ext_name = Some(value);
    }

    /// Gets the value of ExtName
    pub fn get_ext_name(&self) -> Option<&String> {
        self.ext_name.as_ref()
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

    /// Sets the value of PrimaryHandler
    pub fn set_primary_handler(&mut self, value: bool) {
        self.primary_handler = Some(value);
    }

    /// Gets the value of PrimaryHandler
    pub fn get_primary_handler(&self) -> Option<&bool> {
        self.primary_handler.as_ref()
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

