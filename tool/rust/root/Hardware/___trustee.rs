// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Hardware
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __Trustee struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __Trustee {
    #[serde(flatten)]
    pub base: __SecurityRelatedClass,

/// 
    #[serde(rename = "Domain")]
    pub domain: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "SID")]
    pub sid: Vec<u8>,

/// 
    #[serde(rename = "SidLength")]
    pub sid_length: Option<u32>,

/// 
    #[serde(rename = "SIDString")]
    pub sidstring: Option<String>,

/// 
    #[serde(rename = "TIME_CREATED")]
    pub time__created: Option<u64>,
}

impl __Trustee {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __SecurityRelatedClass::new(),
            domain: None,
            name: None,
            sid: Vec::new(),
            sid_length: None,
            sidstring: None,
            time__created: None,
        }
    }


    /// Sets the value of Domain
    pub fn set_domain(&mut self, value: String) {
        self.domain = Some(value);
    }

    /// Gets the value of Domain
    pub fn get_domain(&self) -> Option<&String> {
        self.domain.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of SID
    pub fn set_sid(&mut self, value: Vec<u8>) {
        self.sid = value;
    }

    /// Gets the value of SID
    pub fn get_sid(&self) -> &Vec<u8> {
        &self.sid
    }

    /// Sets the value of SidLength
    pub fn set_sid_length(&mut self, value: u32) {
        self.sid_length = Some(value);
    }

    /// Gets the value of SidLength
    pub fn get_sid_length(&self) -> Option<&u32> {
        self.sid_length.as_ref()
    }

    /// Sets the value of SIDString
    pub fn set_sidstring(&mut self, value: String) {
        self.sidstring = Some(value);
    }

    /// Gets the value of SIDString
    pub fn get_sidstring(&self) -> Option<&String> {
        self.sidstring.as_ref()
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

