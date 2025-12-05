// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ServerManager
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ServerClusterInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ServerClusterInformation {

/// 
    #[serde(rename = "GroupType")]
    pub group_type: Option<i32>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "ObjectType")]
    pub object_type: Option<u8>,
}

impl MSFT_ServerClusterInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            group_type: None,
            name: None,
            object_type: None,
        }
    }


    /// Sets the value of GroupType
    pub fn set_group_type(&mut self, value: i32) {
        self.group_type = Some(value);
    }

    /// Gets the value of GroupType
    pub fn get_group_type(&self) -> Option<&i32> {
        self.group_type.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of ObjectType
    pub fn set_object_type(&mut self, value: u8) {
        self.object_type = Some(value);
    }

    /// Gets the value of ObjectType
    pub fn get_object_type(&self) -> Option<&u8> {
        self.object_type.as_ref()
    }
}

