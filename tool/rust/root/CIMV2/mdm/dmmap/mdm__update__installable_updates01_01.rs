// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Update_InstallableUpdates01_01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Update_InstallableUpdates01_01 {

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "RevisionNumber")]
    pub revision_number: Option<i32>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<i32>,
}

impl MDM_Update_InstallableUpdates01_01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            parent_id: None,
            revision_number: None,
            type: None,
        }
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

    /// Sets the value of RevisionNumber
    pub fn set_revision_number(&mut self, value: i32) {
        self.revision_number = Some(value);
    }

    /// Gets the value of RevisionNumber
    pub fn get_revision_number(&self) -> Option<&i32> {
        self.revision_number.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: i32) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&i32> {
        self.type.as_ref()
    }
}

