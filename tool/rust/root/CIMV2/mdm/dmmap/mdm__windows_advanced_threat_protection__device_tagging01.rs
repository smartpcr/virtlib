// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_WindowsAdvancedThreatProtection_DeviceTagging01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_WindowsAdvancedThreatProtection_DeviceTagging01 {

/// 
    #[serde(rename = "Criticality")]
    pub criticality: Option<i32>,

/// 
    #[serde(rename = "Group")]
    pub group: Option<String>,

/// 
    #[serde(rename = "IdMethod")]
    pub id_method: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_WindowsAdvancedThreatProtection_DeviceTagging01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            criticality: None,
            group: None,
            id_method: None,
            instance_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of Criticality
    pub fn set_criticality(&mut self, value: i32) {
        self.criticality = Some(value);
    }

    /// Gets the value of Criticality
    pub fn get_criticality(&self) -> Option<&i32> {
        self.criticality.as_ref()
    }

    /// Sets the value of Group
    pub fn set_group(&mut self, value: String) {
        self.group = Some(value);
    }

    /// Gets the value of Group
    pub fn get_group(&self) -> Option<&String> {
        self.group.as_ref()
    }

    /// Sets the value of IdMethod
    pub fn set_id_method(&mut self, value: i32) {
        self.id_method = Some(value);
    }

    /// Gets the value of IdMethod
    pub fn get_id_method(&self) -> Option<&i32> {
        self.id_method.as_ref()
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

