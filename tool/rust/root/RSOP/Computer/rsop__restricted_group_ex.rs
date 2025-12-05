// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_RestrictedGroupEx struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_RestrictedGroupEx {
    #[serde(flatten)]
    pub base: RSOP_SecuritySettings,

/// 
    #[serde(rename = "GroupName")]
    pub group_name: Option<String>,

/// 
    #[serde(rename = "Members")]
    pub members: Vec<String>,

/// 
    #[serde(rename = "MembersOf")]
    pub members_of: Vec<String>,
}

impl RSOP_RestrictedGroupEx {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_SecuritySettings::new(),
            group_name: None,
            members: Vec::new(),
            members_of: Vec::new(),
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

    /// Sets the value of MembersOf
    pub fn set_members_of(&mut self, value: Vec<String>) {
        self.members_of = value;
    }

    /// Gets the value of MembersOf
    pub fn get_members_of(&self) -> &Vec<String> {
        &self.members_of
    }
}

