// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_PassportForWork_DeviceUnlock01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_PassportForWork_DeviceUnlock01 {

/// 
    #[serde(rename = "GroupA")]
    pub group_a: Option<String>,

/// 
    #[serde(rename = "GroupB")]
    pub group_b: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "Plugins")]
    pub plugins: Option<String>,
}

impl MDM_PassportForWork_DeviceUnlock01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            group_a: None,
            group_b: None,
            instance_id: None,
            parent_id: None,
            plugins: None,
        }
    }


    /// Sets the value of GroupA
    pub fn set_group_a(&mut self, value: String) {
        self.group_a = Some(value);
    }

    /// Gets the value of GroupA
    pub fn get_group_a(&self) -> Option<&String> {
        self.group_a.as_ref()
    }

    /// Sets the value of GroupB
    pub fn set_group_b(&mut self, value: String) {
        self.group_b = Some(value);
    }

    /// Gets the value of GroupB
    pub fn get_group_b(&self) -> Option<&String> {
        self.group_b.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of Plugins
    pub fn set_plugins(&mut self, value: String) {
        self.plugins = Some(value);
    }

    /// Gets the value of Plugins
    pub fn get_plugins(&self) -> Option<&String> {
        self.plugins.as_ref()
    }
}

