// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.directory.LDAP
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DS_LDAP_Instance_Containment struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DS_LDAP_Instance_Containment {

/// 
    #[serde(rename = "ChildInstance")]
    pub child_instance: Option<DS_LDAP_Root_Class>,

/// 
    #[serde(rename = "ParentInstance")]
    pub parent_instance: Option<DS_LDAP_Root_Class>,
}

impl DS_LDAP_Instance_Containment {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            child_instance: None,
            parent_instance: None,
        }
    }


    /// Sets the value of ChildInstance
    pub fn set_child_instance(&mut self, value: DS_LDAP_Root_Class) {
        self.child_instance = Some(value);
    }

    /// Gets the value of ChildInstance
    pub fn get_child_instance(&self) -> Option<&DS_LDAP_Root_Class> {
        self.child_instance.as_ref()
    }

    /// Sets the value of ParentInstance
    pub fn set_parent_instance(&mut self, value: DS_LDAP_Root_Class) {
        self.parent_instance = Some(value);
    }

    /// Gets the value of ParentInstance
    pub fn get_parent_instance(&self) -> Option<&DS_LDAP_Root_Class> {
        self.parent_instance.as_ref()
    }
}

