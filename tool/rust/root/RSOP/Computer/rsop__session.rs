// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_Session struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_Session {

/// 
    #[serde(rename = "creationTime")]
    pub creation_time: Option<String>,

/// 
    #[serde(rename = "flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Vec<String>,

/// 
    #[serde(rename = "Site")]
    pub site: Option<String>,

/// 
    #[serde(rename = "slowLink")]
    pub slow_link: Option<bool>,

/// 
    #[serde(rename = "SOM")]
    pub som: Option<String>,

/// 
    #[serde(rename = "targetName")]
    pub target_name: Option<String>,

/// 
    #[serde(rename = "ttlMinutes")]
    pub ttl_minutes: Option<u32>,

/// 
    #[serde(rename = "version")]
    pub version: Option<u32>,
}

impl RSOP_Session {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            creation_time: None,
            flags: None,
            id: None,
            security_groups: Vec::new(),
            site: None,
            slow_link: None,
            som: None,
            target_name: None,
            ttl_minutes: None,
            version: None,
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

    /// Sets the value of flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of SecurityGroups
    pub fn set_security_groups(&mut self, value: Vec<String>) {
        self.security_groups = value;
    }

    /// Gets the value of SecurityGroups
    pub fn get_security_groups(&self) -> &Vec<String> {
        &self.security_groups
    }

    /// Sets the value of Site
    pub fn set_site(&mut self, value: String) {
        self.site = Some(value);
    }

    /// Gets the value of Site
    pub fn get_site(&self) -> Option<&String> {
        self.site.as_ref()
    }

    /// Sets the value of slowLink
    pub fn set_slow_link(&mut self, value: bool) {
        self.slow_link = Some(value);
    }

    /// Gets the value of slowLink
    pub fn get_slow_link(&self) -> Option<&bool> {
        self.slow_link.as_ref()
    }

    /// Sets the value of SOM
    pub fn set_som(&mut self, value: String) {
        self.som = Some(value);
    }

    /// Gets the value of SOM
    pub fn get_som(&self) -> Option<&String> {
        self.som.as_ref()
    }

    /// Sets the value of targetName
    pub fn set_target_name(&mut self, value: String) {
        self.target_name = Some(value);
    }

    /// Gets the value of targetName
    pub fn get_target_name(&self) -> Option<&String> {
        self.target_name.as_ref()
    }

    /// Sets the value of ttlMinutes
    pub fn set_ttl_minutes(&mut self, value: u32) {
        self.ttl_minutes = Some(value);
    }

    /// Gets the value of ttlMinutes
    pub fn get_ttl_minutes(&self) -> Option<&u32> {
        self.ttl_minutes.as_ref()
    }

    /// Sets the value of version
    pub fn set_version(&mut self, value: u32) {
        self.version = Some(value);
    }

    /// Gets the value of version
    pub fn get_version(&self) -> Option<&u32> {
        self.version.as_ref()
    }
}

