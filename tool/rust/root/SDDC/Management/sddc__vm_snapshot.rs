// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SDDC.Management
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SDDC_VmSnapshot struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SDDC_VmSnapshot {

/// 
    #[serde(rename = "CreationTime")]
    pub creation_time: Option<String>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "IsCurrentlyApplied")]
    pub is_currently_applied: Option<bool>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "ParentId")]
    pub parent_id: Option<String>,
}

impl SDDC_VmSnapshot {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            creation_time: None,
            id: None,
            is_currently_applied: None,
            name: None,
            parent_id: None,
        }
    }


    /// Sets the value of CreationTime
    pub fn set_creation_time(&mut self, value: String) {
        self.creation_time = Some(value);
    }

    /// Gets the value of CreationTime
    pub fn get_creation_time(&self) -> Option<&String> {
        self.creation_time.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of IsCurrentlyApplied
    pub fn set_is_currently_applied(&mut self, value: bool) {
        self.is_currently_applied = Some(value);
    }

    /// Gets the value of IsCurrentlyApplied
    pub fn get_is_currently_applied(&self) -> Option<&bool> {
        self.is_currently_applied.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of ParentId
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentId
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }
}

