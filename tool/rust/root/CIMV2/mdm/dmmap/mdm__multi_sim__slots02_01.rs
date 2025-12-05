// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_MultiSIM_Slots02_01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_MultiSIM_Slots02_01 {

/// 
    #[serde(rename = "Identifier")]
    pub identifier: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "IsEmbedded")]
    pub is_embedded: Option<bool>,

/// 
    #[serde(rename = "IsSelected")]
    pub is_selected: Option<bool>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "State")]
    pub state: Option<i32>,
}

impl MDM_MultiSIM_Slots02_01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            identifier: None,
            instance_id: None,
            is_embedded: None,
            is_selected: None,
            parent_id: None,
            state: None,
        }
    }


    /// Sets the value of Identifier
    pub fn set_identifier(&mut self, value: i32) {
        self.identifier = Some(value);
    }

    /// Gets the value of Identifier
    pub fn get_identifier(&self) -> Option<&i32> {
        self.identifier.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of IsEmbedded
    pub fn set_is_embedded(&mut self, value: bool) {
        self.is_embedded = Some(value);
    }

    /// Gets the value of IsEmbedded
    pub fn get_is_embedded(&self) -> Option<&bool> {
        self.is_embedded.as_ref()
    }

    /// Sets the value of IsSelected
    pub fn set_is_selected(&mut self, value: bool) {
        self.is_selected = Some(value);
    }

    /// Gets the value of IsSelected
    pub fn get_is_selected(&self) -> Option<&bool> {
        self.is_selected.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: i32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&i32> {
        self.state.as_ref()
    }
}

