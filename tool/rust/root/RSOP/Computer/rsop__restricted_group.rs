// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_RestrictedGroup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_RestrictedGroup {
    #[serde(flatten)]
    pub base: RSOP_SecuritySettings,

/// 
    #[serde(rename = "GroupName")]
    pub group_name: Option<String>,

/// 
    #[serde(rename = "Members")]
    pub members: Vec<String>,
}

impl RSOP_RestrictedGroup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_SecuritySettings::new(),
            group_name: None,
            members: Vec::new(),
        }
    }


    /// Sets the value of GroupName
    pub fn set_group_name(&mut self, value: String) {
        self.group_name = Some(value);
    }

    /// Gets the value of GroupName
    pub fn get_group_name(&self) -> Option<&String> {
        self.group_name.as_ref()
    }

    /// Sets the value of Members
    pub fn set_members(&mut self, value: Vec<String>) {
        self.members = value;
    }

    /// Gets the value of Members
    pub fn get_members(&self) -> &Vec<String> {
        &self.members
    }
}

