// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SECURITY
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __SecurityDescriptor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __SecurityDescriptor {
    #[serde(flatten)]
    pub base: __SecurityRelatedClass,

/// 
    #[serde(rename = "ControlFlags")]
    pub control_flags: Option<u32>,

/// 
    #[serde(rename = "DACL")]
    pub dacl: Vec<__ACE>,

/// 
    #[serde(rename = "Group")]
    pub group: Option<__ACE>,

/// 
    #[serde(rename = "Owner")]
    pub owner: Option<__ACE>,

/// 
    #[serde(rename = "SACL")]
    pub sacl: Vec<__ACE>,

/// 
    #[serde(rename = "TIME_CREATED")]
    pub time__created: Option<u64>,
}

impl __SecurityDescriptor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __SecurityRelatedClass::new(),
            control_flags: None,
            dacl: Vec::new(),
            group: None,
            owner: None,
            sacl: Vec::new(),
            time__created: None,
        }
    }


    /// Sets the value of ControlFlags
    pub fn set_control_flags(&mut self, value: u32) {
        self.control_flags = Some(value);
    }

    /// Gets the value of ControlFlags
    pub fn get_control_flags(&self) -> Option<&u32> {
        self.control_flags.as_ref()
    }

    /// Sets the value of DACL
    pub fn set_dacl(&mut self, value: Vec<__ACE>) {
        self.dacl = value;
    }

    /// Gets the value of DACL
    pub fn get_dacl(&self) -> &Vec<__ACE> {
        &self.dacl
    }

    /// Sets the value of Group
    pub fn set_group(&mut self, value: __ACE) {
        self.group = Some(value);
    }

    /// Gets the value of Group
    pub fn get_group(&self) -> Option<&__ACE> {
        self.group.as_ref()
    }

    /// Sets the value of Owner
    pub fn set_owner(&mut self, value: __ACE) {
        self.owner = Some(value);
    }

    /// Gets the value of Owner
    pub fn get_owner(&self) -> Option<&__ACE> {
        self.owner.as_ref()
    }

    /// Sets the value of SACL
    pub fn set_sacl(&mut self, value: Vec<__ACE>) {
        self.sacl = value;
    }

    /// Gets the value of SACL
    pub fn get_sacl(&self) -> &Vec<__ACE> {
        &self.sacl
    }

    /// Sets the value of TIME_CREATED
    pub fn set_time__created(&mut self, value: u64) {
        self.time__created = Some(value);
    }

    /// Gets the value of TIME_CREATED
    pub fn get_time__created(&self) -> Option<&u64> {
        self.time__created.as_ref()
    }
}

