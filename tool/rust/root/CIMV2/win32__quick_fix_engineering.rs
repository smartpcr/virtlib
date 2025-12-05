// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_QuickFixEngineering struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_QuickFixEngineering {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "CSName")]
    pub csname: Option<String>,

/// 
    #[serde(rename = "FixComments")]
    pub fix_comments: Option<String>,

/// 
    #[serde(rename = "HotFixID")]
    pub hot_fix_id: Option<String>,

/// 
    #[serde(rename = "InstalledBy")]
    pub installed_by: Option<String>,

/// 
    #[serde(rename = "InstalledOn")]
    pub installed_on: Option<String>,

/// 
    #[serde(rename = "ServicePackInEffect")]
    pub service_pack_in_effect: Option<String>,
}

impl Win32_QuickFixEngineering {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            csname: None,
            fix_comments: None,
            hot_fix_id: None,
            installed_by: None,
            installed_on: None,
            service_pack_in_effect: None,
        }
    }


    /// Sets the value of CSName
    pub fn set_csname(&mut self, value: String) {
        self.csname = Some(value);
    }

    /// Gets the value of CSName
    pub fn get_csname(&self) -> Option<&String> {
        self.csname.as_ref()
    }

    /// Sets the value of FixComments
    pub fn set_fix_comments(&mut self, value: String) {
        self.fix_comments = Some(value);
    }

    /// Gets the value of FixComments
    pub fn get_fix_comments(&self) -> Option<&String> {
        self.fix_comments.as_ref()
    }

    /// Sets the value of HotFixID
    pub fn set_hot_fix_id(&mut self, value: String) {
        self.hot_fix_id = Some(value);
    }

    /// Gets the value of HotFixID
    pub fn get_hot_fix_id(&self) -> Option<&String> {
        self.hot_fix_id.as_ref()
    }

    /// Sets the value of InstalledBy
    pub fn set_installed_by(&mut self, value: String) {
        self.installed_by = Some(value);
    }

    /// Gets the value of InstalledBy
    pub fn get_installed_by(&self) -> Option<&String> {
        self.installed_by.as_ref()
    }

    /// Sets the value of InstalledOn
    pub fn set_installed_on(&mut self, value: String) {
        self.installed_on = Some(value);
    }

    /// Gets the value of InstalledOn
    pub fn get_installed_on(&self) -> Option<&String> {
        self.installed_on.as_ref()
    }

    /// Sets the value of ServicePackInEffect
    pub fn set_service_pack_in_effect(&mut self, value: String) {
        self.service_pack_in_effect = Some(value);
    }

    /// Gets the value of ServicePackInEffect
    pub fn get_service_pack_in_effect(&self) -> Option<&String> {
        self.service_pack_in_effect.as_ref()
    }
}

