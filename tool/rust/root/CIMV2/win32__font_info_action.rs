// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_FontInfoAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_FontInfoAction {
    #[serde(flatten)]
    pub base: CIM_Action,

/// 
    #[serde(rename = "File")]
    pub file: Option<String>,

/// 
    #[serde(rename = "FontTitle")]
    pub font_title: Option<String>,
}

impl Win32_FontInfoAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Action::new(),
            file: None,
            font_title: None,
        }
    }


    /// Sets the value of File
    pub fn set_file(&mut self, value: String) {
        self.file = Some(value);
    }

    /// Gets the value of File
    pub fn get_file(&self) -> Option<&String> {
        self.file.as_ref()
    }

    /// Sets the value of FontTitle
    pub fn set_font_title(&mut self, value: String) {
        self.font_title = Some(value);
    }

    /// Gets the value of FontTitle
    pub fn get_font_title(&self) -> Option<&String> {
        self.font_title.as_ref()
    }
}

