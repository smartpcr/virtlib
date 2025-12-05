// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_EnterpriseModernAppManagement_ReleaseManagement01_01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_EnterpriseModernAppManagement_ReleaseManagement01_01 {

/// 
    #[serde(rename = "ChannelId")]
    pub channel_id: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "ReleaseManagementId")]
    pub release_management_id: Option<String>,
}

impl MDM_EnterpriseModernAppManagement_ReleaseManagement01_01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            channel_id: None,
            instance_id: None,
            parent_id: None,
            release_management_id: None,
        }
    }


    /// Sets the value of ChannelId
    pub fn set_channel_id(&mut self, value: String) {
        self.channel_id = Some(value);
    }

    /// Gets the value of ChannelId
    pub fn get_channel_id(&self) -> Option<&String> {
        self.channel_id.as_ref()
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

    /// Sets the value of ReleaseManagementId
    pub fn set_release_management_id(&mut self, value: String) {
        self.release_management_id = Some(value);
    }

    /// Gets the value of ReleaseManagementId
    pub fn get_release_management_id(&self) -> Option<&String> {
        self.release_management_id.as_ref()
    }
}

