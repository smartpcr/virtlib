// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_DataUsage02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_DataUsage02 {

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "SetCost3G")]
    pub set_cost3_g: Option<String>,

/// 
    #[serde(rename = "SetCost4G")]
    pub set_cost4_g: Option<String>,
}

impl MDM_Policy_Config01_DataUsage02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            parent_id: None,
            set_cost3_g: None,
            set_cost4_g: None,
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

    /// Sets the value of SetCost3G
    pub fn set_set_cost3_g(&mut self, value: String) {
        self.set_cost3_g = Some(value);
    }

    /// Gets the value of SetCost3G
    pub fn get_set_cost3_g(&self) -> Option<&String> {
        self.set_cost3_g.as_ref()
    }

    /// Sets the value of SetCost4G
    pub fn set_set_cost4_g(&mut self, value: String) {
        self.set_cost4_g = Some(value);
    }

    /// Gets the value of SetCost4G
    pub fn get_set_cost4_g(&self) -> Option<&String> {
        self.set_cost4_g.as_ref()
    }
}

