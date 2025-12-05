// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_File struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_File {
    #[serde(flatten)]
    pub base: RSOP_SecuritySettings,

/// 
    #[serde(rename = "Mode")]
    pub mode: Option<File_Mode>,

/// 
    #[serde(rename = "OriginalPath")]
    pub original_path: Option<String>,

/// 
    #[serde(rename = "Path")]
    pub path: Option<String>,

/// 
    #[serde(rename = "SDDLString")]
    pub sddlstring: Option<String>,
}

impl RSOP_File {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_SecuritySettings::new(),
            mode: None,
            original_path: None,
            path: None,
            sddlstring: None,
        }
    }


    /// Sets the value of Mode
    pub fn set_mode(&mut self, value: File_Mode) {
        self.mode = Some(value);
    }

    /// Gets the value of Mode
    pub fn get_mode(&self) -> Option<&File_Mode> {
        self.mode.as_ref()
    }

    /// Sets the value of OriginalPath
    pub fn set_original_path(&mut self, value: String) {
        self.original_path = Some(value);
    }

    /// Gets the value of OriginalPath
    pub fn get_original_path(&self) -> Option<&String> {
        self.original_path.as_ref()
    }

    /// Sets the value of Path
    pub fn set_path(&mut self, value: String) {
        self.path = Some(value);
    }

    /// Gets the value of Path
    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// Sets the value of SDDLString
    pub fn set_sddlstring(&mut self, value: String) {
        self.sddlstring = Some(value);
    }

    /// Gets the value of SDDLString
    pub fn get_sddlstring(&self) -> Option<&String> {
        self.sddlstring.as_ref()
    }
}

