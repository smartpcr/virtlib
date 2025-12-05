// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_VPNv2_Eap04 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_VPNv2_Eap04 {

/// 
    #[serde(rename = "Configuration")]
    pub configuration: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<i32>,
}

impl MDM_VPNv2_Eap04 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            configuration: None,
            instance_id: None,
            parent_id: None,
            type: None,
        }
    }


    /// Sets the value of Configuration
    pub fn set_configuration(&mut self, value: String) {
        self.configuration = Some(value);
    }

    /// Gets the value of Configuration
    pub fn get_configuration(&self) -> Option<&String> {
        self.configuration.as_ref()
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

    /// Sets the value of Type
    pub fn set_type(&mut self, value: i32) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&i32> {
        self.type.as_ref()
    }
}

