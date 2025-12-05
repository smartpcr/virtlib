// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_MIMEInfoAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_MIMEInfoAction {
    #[serde(flatten)]
    pub base: CIM_Action,

/// 
    #[serde(rename = "CLSID")]
    pub clsid: Option<String>,

/// 
    #[serde(rename = "ContentType")]
    pub content_type: Option<String>,

/// 
    #[serde(rename = "Extension")]
    pub extension: Option<String>,
}

impl Win32_MIMEInfoAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Action::new(),
            clsid: None,
            content_type: None,
            extension: None,
        }
    }


    /// Sets the value of CLSID
    pub fn set_clsid(&mut self, value: String) {
        self.clsid = Some(value);
    }

    /// Gets the value of CLSID
    pub fn get_clsid(&self) -> Option<&String> {
        self.clsid.as_ref()
    }

    /// Sets the value of ContentType
    pub fn set_content_type(&mut self, value: String) {
        self.content_type = Some(value);
    }

    /// Gets the value of ContentType
    pub fn get_content_type(&self) -> Option<&String> {
        self.content_type.as_ref()
    }

    /// Sets the value of Extension
    pub fn set_extension(&mut self, value: String) {
        self.extension = Some(value);
    }

    /// Gets the value of Extension
    pub fn get_extension(&self) -> Option<&String> {
        self.extension.as_ref()
    }
}

