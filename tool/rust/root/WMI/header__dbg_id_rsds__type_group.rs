// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Header_DbgIdRSDS_TypeGroup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Header_DbgIdRSDS_TypeGroup {
    #[serde(flatten)]
    pub base: EventTraceEvent,

/// 
    #[serde(rename = "Age")]
    pub age: Option<u32>,

/// 
    #[serde(rename = "Guid")]
    pub guid: Option<serde_json::Value>,

/// 
    #[serde(rename = "PdbName")]
    pub pdb_name: Option<String>,
}

impl Header_DbgIdRSDS_TypeGroup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: EventTraceEvent::new(),
            age: None,
            guid: None,
            pdb_name: None,
        }
    }


    /// Sets the value of Age
    pub fn set_age(&mut self, value: u32) {
        self.age = Some(value);
    }

    /// Gets the value of Age
    pub fn get_age(&self) -> Option<&u32> {
        self.age.as_ref()
    }

    /// Sets the value of Guid
    pub fn set_guid(&mut self, value: serde_json::Value) {
        self.guid = Some(value);
    }

    /// Gets the value of Guid
    pub fn get_guid(&self) -> Option<&serde_json::Value> {
        self.guid.as_ref()
    }

    /// Sets the value of PdbName
    pub fn set_pdb_name(&mut self, value: String) {
        self.pdb_name = Some(value);
    }

    /// Gets the value of PdbName
    pub fn get_pdb_name(&self) -> Option<&String> {
        self.pdb_name.as_ref()
    }
}

