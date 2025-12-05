// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_LogicalProgramGroup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_LogicalProgramGroup {
    #[serde(flatten)]
    pub base: Win32_ProgramGroupOrItem,

/// 
    #[serde(rename = "GroupName")]
    pub group_name: Option<String>,

/// 
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,
}

impl Win32_LogicalProgramGroup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_ProgramGroupOrItem::new(),
            group_name: None,
            user_name: None,
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

    /// Sets the value of UserName
    pub fn set_user_name(&mut self, value: String) {
        self.user_name = Some(value);
    }

    /// Gets the value of UserName
    pub fn get_user_name(&self) -> Option<&String> {
        self.user_name.as_ref()
    }
}

