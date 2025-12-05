// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TokenGroups struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TokenGroups {

/// 
    #[serde(rename = "GroupCount")]
    pub group_count: Option<u32>,

/// 
    #[serde(rename = "Groups")]
    pub groups: Vec<Win32_SIDandAttributes>,
}

impl Win32_TokenGroups {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            group_count: None,
            groups: Vec::new(),
        }
    }


    /// Sets the value of GroupCount
    pub fn set_group_count(&mut self, value: u32) {
        self.group_count = Some(value);
    }

    /// Gets the value of GroupCount
    pub fn get_group_count(&self) -> Option<&u32> {
        self.group_count.as_ref()
    }

    /// Sets the value of Groups
    pub fn set_groups(&mut self, value: Vec<Win32_SIDandAttributes>) {
        self.groups = value;
    }

    /// Gets the value of Groups
    pub fn get_groups(&self) -> &Vec<Win32_SIDandAttributes> {
        &self.groups
    }
}

