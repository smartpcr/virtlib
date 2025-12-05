// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ISCSI_TargetMapping struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ISCSI_TargetMapping {

/// 
    #[serde(rename = "FromPersistentLogin")]
    pub from_persistent_login: Option<bool>,

/// 
    #[serde(rename = "LUNCount")]
    pub luncount: Option<u32>,

/// 
    #[serde(rename = "LUNList")]
    pub lunlist: Vec<ISCSI_LUNList>,

/// 
    #[serde(rename = "OSBus")]
    pub osbus: Option<u32>,

/// 
    #[serde(rename = "OSTarget")]
    pub ostarget: Option<u32>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<u64>,

/// 
    #[serde(rename = "TargetName")]
    pub target_name: Option<String>,

/// 
    #[serde(rename = "UniqueSessionId")]
    pub unique_session_id: Option<u64>,
}

impl ISCSI_TargetMapping {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            from_persistent_login: None,
            luncount: None,
            lunlist: Vec::new(),
            osbus: None,
            ostarget: None,
            reserved: None,
            target_name: None,
            unique_session_id: None,
        }
    }


    /// Sets the value of FromPersistentLogin
    pub fn set_from_persistent_login(&mut self, value: bool) {
        self.from_persistent_login = Some(value);
    }

    /// Gets the value of FromPersistentLogin
    pub fn get_from_persistent_login(&self) -> Option<&bool> {
        self.from_persistent_login.as_ref()
    }

    /// Sets the value of LUNCount
    pub fn set_luncount(&mut self, value: u32) {
        self.luncount = Some(value);
    }

    /// Gets the value of LUNCount
    pub fn get_luncount(&self) -> Option<&u32> {
        self.luncount.as_ref()
    }

    /// Sets the value of LUNList
    pub fn set_lunlist(&mut self, value: Vec<ISCSI_LUNList>) {
        self.lunlist = value;
    }

    /// Gets the value of LUNList
    pub fn get_lunlist(&self) -> &Vec<ISCSI_LUNList> {
        &self.lunlist
    }

    /// Sets the value of OSBus
    pub fn set_osbus(&mut self, value: u32) {
        self.osbus = Some(value);
    }

    /// Gets the value of OSBus
    pub fn get_osbus(&self) -> Option<&u32> {
        self.osbus.as_ref()
    }

    /// Sets the value of OSTarget
    pub fn set_ostarget(&mut self, value: u32) {
        self.ostarget = Some(value);
    }

    /// Gets the value of OSTarget
    pub fn get_ostarget(&self) -> Option<&u32> {
        self.ostarget.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u64) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u64> {
        self.reserved.as_ref()
    }

    /// Sets the value of TargetName
    pub fn set_target_name(&mut self, value: String) {
        self.target_name = Some(value);
    }

    /// Gets the value of TargetName
    pub fn get_target_name(&self) -> Option<&String> {
        self.target_name.as_ref()
    }

    /// Sets the value of UniqueSessionId
    pub fn set_unique_session_id(&mut self, value: u64) {
        self.unique_session_id = Some(value);
    }

    /// Gets the value of UniqueSessionId
    pub fn get_unique_session_id(&self) -> Option<&u64> {
        self.unique_session_id.as_ref()
    }
}

