// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_80211_AddWEP struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_80211_AddWEP {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "KeyIndex")]
    pub key_index: Option<u32>,

/// 
    #[serde(rename = "KeyLength")]
    pub key_length: Option<u32>,

/// 
    #[serde(rename = "KeyMaterial")]
    pub key_material: Vec<u8>,

/// 
    #[serde(rename = "Length")]
    pub length: Option<u32>,
}

impl MSNdis_80211_AddWEP {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            active: None,
            instance_name: None,
            key_index: None,
            key_length: None,
            key_material: Vec::new(),
            length: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of KeyIndex
    pub fn set_key_index(&mut self, value: u32) {
        self.key_index = Some(value);
    }

    /// Gets the value of KeyIndex
    pub fn get_key_index(&self) -> Option<&u32> {
        self.key_index.as_ref()
    }

    /// Sets the value of KeyLength
    pub fn set_key_length(&mut self, value: u32) {
        self.key_length = Some(value);
    }

    /// Gets the value of KeyLength
    pub fn get_key_length(&self) -> Option<&u32> {
        self.key_length.as_ref()
    }

    /// Sets the value of KeyMaterial
    pub fn set_key_material(&mut self, value: Vec<u8>) {
        self.key_material = value;
    }

    /// Gets the value of KeyMaterial
    pub fn get_key_material(&self) -> &Vec<u8> {
        &self.key_material
    }

    /// Sets the value of Length
    pub fn set_length(&mut self, value: u32) {
        self.length = Some(value);
    }

    /// Gets the value of Length
    pub fn get_length(&self) -> Option<&u32> {
        self.length.as_ref()
    }
}

