// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_CodecFile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_CodecFile {
    #[serde(flatten)]
    pub base: CIM_DataFile,

/// 
    #[serde(rename = "Group")]
    pub group: Option<String>,
}

impl Win32_CodecFile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_DataFile::new(),
            group: None,
        }
    }


    /// Sets the value of Group
    pub fn set_group(&mut self, value: String) {
        self.group = Some(value);
    }

    /// Gets the value of Group
    pub fn get_group(&self) -> Option<&String> {
        self.group.as_ref()
    }
}

