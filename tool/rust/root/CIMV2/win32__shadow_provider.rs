// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ShadowProvider struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ShadowProvider {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "CLSID")]
    pub clsid: Option<String>,

/// 
    #[serde(rename = "ID")]
    pub id: Option<String>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u32>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,

/// 
    #[serde(rename = "VersionID")]
    pub version_id: Option<String>,
}

impl Win32_ShadowProvider {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            clsid: None,
            id: None,
            type: None,
            version: None,
            version_id: None,
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

    /// Sets the value of ID
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of ID
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u32) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u32> {
        self.type.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: String) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&String> {
        self.version.as_ref()
    }

    /// Sets the value of VersionID
    pub fn set_version_id(&mut self, value: String) {
        self.version_id = Some(value);
    }

    /// Gets the value of VersionID
    pub fn get_version_id(&self) -> Option<&String> {
        self.version_id.as_ref()
    }
}

