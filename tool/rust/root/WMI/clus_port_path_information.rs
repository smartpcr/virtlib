// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ClusPortPathInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusPortPathInformation {

/// InActiveList.
    #[serde(rename = "InActiveList")]
    pub in_active_list: Option<bool>,

/// InReadOnlyList.
    #[serde(rename = "InReadOnlyList")]
    pub in_read_only_list: Option<bool>,

/// InStandbyList.
    #[serde(rename = "InStandbyList")]
    pub in_standby_list: Option<bool>,

/// IsReadOnly.
    #[serde(rename = "IsReadOnly")]
    pub is_read_only: Option<bool>,

/// Node Id.
    #[serde(rename = "NodeId")]
    pub node_id: Option<u32>,

/// NodeName .
    #[serde(rename = "NodeName")]
    pub node_name: Option<String>,

/// Path Id.
    #[serde(rename = "PathId")]
    pub path_id: Option<u32>,

/// Path Key.
    #[serde(rename = "PathKey")]
    pub path_key: Option<u64>,

/// ClusPort Device State.
    #[serde(rename = "PathState")]
    pub path_state: Option<u32>,
}

impl ClusPortPathInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            in_active_list: None,
            in_read_only_list: None,
            in_standby_list: None,
            is_read_only: None,
            node_id: None,
            node_name: None,
            path_id: None,
            path_key: None,
            path_state: None,
        }
    }


    /// Sets the value of InActiveList
    pub fn set_in_active_list(&mut self, value: bool) {
        self.in_active_list = Some(value);
    }

    /// Gets the value of InActiveList
    pub fn get_in_active_list(&self) -> Option<&bool> {
        self.in_active_list.as_ref()
    }

    /// Sets the value of InReadOnlyList
    pub fn set_in_read_only_list(&mut self, value: bool) {
        self.in_read_only_list = Some(value);
    }

    /// Gets the value of InReadOnlyList
    pub fn get_in_read_only_list(&self) -> Option<&bool> {
        self.in_read_only_list.as_ref()
    }

    /// Sets the value of InStandbyList
    pub fn set_in_standby_list(&mut self, value: bool) {
        self.in_standby_list = Some(value);
    }

    /// Gets the value of InStandbyList
    pub fn get_in_standby_list(&self) -> Option<&bool> {
        self.in_standby_list.as_ref()
    }

    /// Sets the value of IsReadOnly
    pub fn set_is_read_only(&mut self, value: bool) {
        self.is_read_only = Some(value);
    }

    /// Gets the value of IsReadOnly
    pub fn get_is_read_only(&self) -> Option<&bool> {
        self.is_read_only.as_ref()
    }

    /// Sets the value of NodeId
    pub fn set_node_id(&mut self, value: u32) {
        self.node_id = Some(value);
    }

    /// Gets the value of NodeId
    pub fn get_node_id(&self) -> Option<&u32> {
        self.node_id.as_ref()
    }

    /// Sets the value of NodeName
    pub fn set_node_name(&mut self, value: String) {
        self.node_name = Some(value);
    }

    /// Gets the value of NodeName
    pub fn get_node_name(&self) -> Option<&String> {
        self.node_name.as_ref()
    }

    /// Sets the value of PathId
    pub fn set_path_id(&mut self, value: u32) {
        self.path_id = Some(value);
    }

    /// Gets the value of PathId
    pub fn get_path_id(&self) -> Option<&u32> {
        self.path_id.as_ref()
    }

    /// Sets the value of PathKey
    pub fn set_path_key(&mut self, value: u64) {
        self.path_key = Some(value);
    }

    /// Gets the value of PathKey
    pub fn get_path_key(&self) -> Option<&u64> {
        self.path_key.as_ref()
    }

    /// Sets the value of PathState
    pub fn set_path_state(&mut self, value: u32) {
        self.path_state = Some(value);
    }

    /// Gets the value of PathState
    pub fn get_path_state(&self) -> Option<&u32> {
        self.path_state.as_ref()
    }
}

