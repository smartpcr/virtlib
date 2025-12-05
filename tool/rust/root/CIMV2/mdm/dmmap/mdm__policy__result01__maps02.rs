// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_Maps02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_Maps02 {

/// 
    #[serde(rename = "AllowOfflineMapsDownloadOverMeteredConnection")]
    pub allow_offline_maps_download_over_metered_connection: Option<i32>,

/// 
    #[serde(rename = "EnableOfflineMapsAutoUpdate")]
    pub enable_offline_maps_auto_update: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_Result01_Maps02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_offline_maps_download_over_metered_connection: None,
            enable_offline_maps_auto_update: None,
            instance_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of AllowOfflineMapsDownloadOverMeteredConnection
    pub fn set_allow_offline_maps_download_over_metered_connection(&mut self, value: i32) {
        self.allow_offline_maps_download_over_metered_connection = Some(value);
    }

    /// Gets the value of AllowOfflineMapsDownloadOverMeteredConnection
    pub fn get_allow_offline_maps_download_over_metered_connection(&self) -> Option<&i32> {
        self.allow_offline_maps_download_over_metered_connection.as_ref()
    }

    /// Sets the value of EnableOfflineMapsAutoUpdate
    pub fn set_enable_offline_maps_auto_update(&mut self, value: i32) {
        self.enable_offline_maps_auto_update = Some(value);
    }

    /// Gets the value of EnableOfflineMapsAutoUpdate
    pub fn get_enable_offline_maps_auto_update(&self) -> Option<&i32> {
        self.enable_offline_maps_auto_update.as_ref()
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
}

