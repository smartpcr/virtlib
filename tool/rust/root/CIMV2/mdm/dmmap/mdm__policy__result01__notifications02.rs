// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_Notifications02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_Notifications02 {

/// 
    #[serde(rename = "DisallowCloudNotification")]
    pub disallow_cloud_notification: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "WnsEndpoint")]
    pub wns_endpoint: Option<String>,
}

impl MDM_Policy_Result01_Notifications02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            disallow_cloud_notification: None,
            instance_id: None,
            parent_id: None,
            wns_endpoint: None,
        }
    }


    /// Sets the value of DisallowCloudNotification
    pub fn set_disallow_cloud_notification(&mut self, value: i32) {
        self.disallow_cloud_notification = Some(value);
    }

    /// Gets the value of DisallowCloudNotification
    pub fn get_disallow_cloud_notification(&self) -> Option<&i32> {
        self.disallow_cloud_notification.as_ref()
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

    /// Sets the value of WnsEndpoint
    pub fn set_wns_endpoint(&mut self, value: String) {
        self.wns_endpoint = Some(value);
    }

    /// Gets the value of WnsEndpoint
    pub fn get_wns_endpoint(&self) -> Option<&String> {
        self.wns_endpoint.as_ref()
    }
}

