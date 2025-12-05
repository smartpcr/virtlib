// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.directory.LDAP
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DS_LDAP_Class_Containment struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DS_LDAP_Class_Containment {

/// 
    #[serde(rename = "ChildClass")]
    pub child_class: Option<serde_json::Value>,

/// 
    #[serde(rename = "ParentClass")]
    pub parent_class: Option<serde_json::Value>,
}

impl DS_LDAP_Class_Containment {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            child_class: None,
            parent_class: None,
        }
    }


    /// Sets the value of ChildClass
    pub fn set_child_class(&mut self, value: serde_json::Value) {
        self.child_class = Some(value);
    }

    /// Gets the value of ChildClass
    pub fn get_child_class(&self) -> Option<&serde_json::Value> {
        self.child_class.as_ref()
    }

    /// Sets the value of ParentClass
    pub fn set_parent_class(&mut self, value: serde_json::Value) {
        self.parent_class = Some(value);
    }

    /// Gets the value of ParentClass
    pub fn get_parent_class(&self) -> Option<&serde_json::Value> {
        self.parent_class.as_ref()
    }
}

