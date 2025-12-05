// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_PolicySetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_PolicySetting {

/// 
    #[serde(rename = "creationTime")]
    pub creation_time: Option<String>,

/// 
    #[serde(rename = "GPOID")]
    pub gpoid: Option<String>,

/// 
    #[serde(rename = "id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "precedence")]
    pub precedence: Option<u32>,

/// 
    #[serde(rename = "SOMID")]
    pub somid: Option<String>,
}

impl RSOP_PolicySetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            creation_time: None,
            gpoid: None,
            id: None,
            name: None,
            precedence: None,
            somid: None,
        }
    }


    /// Sets the value of creationTime
    pub fn set_creation_time(&mut self, value: String) {
        self.creation_time = Some(value);
    }

    /// Gets the value of creationTime
    pub fn get_creation_time(&self) -> Option<&String> {
        self.creation_time.as_ref()
    }

    /// Sets the value of GPOID
    pub fn set_gpoid(&mut self, value: String) {
        self.gpoid = Some(value);
    }

    /// Gets the value of GPOID
    pub fn get_gpoid(&self) -> Option<&String> {
        self.gpoid.as_ref()
    }

    /// Sets the value of id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of precedence
    pub fn set_precedence(&mut self, value: u32) {
        self.precedence = Some(value);
    }

    /// Gets the value of precedence
    pub fn get_precedence(&self) -> Option<&u32> {
        self.precedence.as_ref()
    }

    /// Sets the value of SOMID
    pub fn set_somid(&mut self, value: String) {
        self.somid = Some(value);
    }

    /// Gets the value of SOMID
    pub fn get_somid(&self) -> Option<&String> {
        self.somid.as_ref()
    }
}

