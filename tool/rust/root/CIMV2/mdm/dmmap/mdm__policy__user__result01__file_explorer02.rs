// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_User_Result01_FileExplorer02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_User_Result01_FileExplorer02 {

/// 
    #[serde(rename = "AllowOptionToShowNetwork")]
    pub allow_option_to_show_network: Option<i32>,

/// 
    #[serde(rename = "AllowOptionToShowThisPC")]
    pub allow_option_to_show_this_pc: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "SetAllowedFolderLocations")]
    pub set_allowed_folder_locations: Option<i32>,

/// 
    #[serde(rename = "SetAllowedStorageLocations")]
    pub set_allowed_storage_locations: Option<i32>,
}

impl MDM_Policy_User_Result01_FileExplorer02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_option_to_show_network: None,
            allow_option_to_show_this_pc: None,
            instance_id: None,
            parent_id: None,
            set_allowed_folder_locations: None,
            set_allowed_storage_locations: None,
        }
    }


    /// Sets the value of AllowOptionToShowNetwork
    pub fn set_allow_option_to_show_network(&mut self, value: i32) {
        self.allow_option_to_show_network = Some(value);
    }

    /// Gets the value of AllowOptionToShowNetwork
    pub fn get_allow_option_to_show_network(&self) -> Option<&i32> {
        self.allow_option_to_show_network.as_ref()
    }

    /// Sets the value of AllowOptionToShowThisPC
    pub fn set_allow_option_to_show_this_pc(&mut self, value: i32) {
        self.allow_option_to_show_this_pc = Some(value);
    }

    /// Gets the value of AllowOptionToShowThisPC
    pub fn get_allow_option_to_show_this_pc(&self) -> Option<&i32> {
        self.allow_option_to_show_this_pc.as_ref()
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

    /// Sets the value of SetAllowedFolderLocations
    pub fn set_set_allowed_folder_locations(&mut self, value: i32) {
        self.set_allowed_folder_locations = Some(value);
    }

    /// Gets the value of SetAllowedFolderLocations
    pub fn get_set_allowed_folder_locations(&self) -> Option<&i32> {
        self.set_allowed_folder_locations.as_ref()
    }

    /// Sets the value of SetAllowedStorageLocations
    pub fn set_set_allowed_storage_locations(&mut self, value: i32) {
        self.set_allowed_storage_locations = Some(value);
    }

    /// Gets the value of SetAllowedStorageLocations
    pub fn get_set_allowed_storage_locations(&self) -> Option<&i32> {
        self.set_allowed_storage_locations.as_ref()
    }
}

