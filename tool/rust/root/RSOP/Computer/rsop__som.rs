// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_SOM struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_SOM {

/// 
    #[serde(rename = "blocked")]
    pub blocked: Option<bool>,

/// 
    #[serde(rename = "blocking")]
    pub blocking: Option<bool>,

/// 
    #[serde(rename = "id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "reason")]
    pub reason: Option<u32>,

/// 
    #[serde(rename = "SOMOrder")]
    pub somorder: Option<u32>,

/// 
    #[serde(rename = "type")]
    pub type: Option<u32>,
}

impl RSOP_SOM {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            blocked: None,
            blocking: None,
            id: None,
            reason: None,
            somorder: None,
            type: None,
        }
    }


    /// Sets the value of blocked
    pub fn set_blocked(&mut self, value: bool) {
        self.blocked = Some(value);
    }

    /// Gets the value of blocked
    pub fn get_blocked(&self) -> Option<&bool> {
        self.blocked.as_ref()
    }

    /// Sets the value of blocking
    pub fn set_blocking(&mut self, value: bool) {
        self.blocking = Some(value);
    }

    /// Gets the value of blocking
    pub fn get_blocking(&self) -> Option<&bool> {
        self.blocking.as_ref()
    }

    /// Sets the value of id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of reason
    pub fn set_reason(&mut self, value: u32) {
        self.reason = Some(value);
    }

    /// Gets the value of reason
    pub fn get_reason(&self) -> Option<&u32> {
        self.reason.as_ref()
    }

    /// Sets the value of SOMOrder
    pub fn set_somorder(&mut self, value: u32) {
        self.somorder = Some(value);
    }

    /// Gets the value of SOMOrder
    pub fn get_somorder(&self) -> Option<&u32> {
        self.somorder.as_ref()
    }

    /// Sets the value of type
    pub fn set_type(&mut self, value: u32) {
        self.type = Some(value);
    }

    /// Gets the value of type
    pub fn get_type(&self) -> Option<&u32> {
        self.type.as_ref()
    }
}

