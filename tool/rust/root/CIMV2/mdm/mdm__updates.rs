// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Updates struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Updates {

/// 
    #[serde(rename = "AutoUpdatePolicy")]
    pub auto_update_policy: Option<u32>,

/// 
    #[serde(rename = "key")]
    pub key: Option<u32>,
}

impl MDM_Updates {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            auto_update_policy: None,
            key: None,
        }
    }


    /// Sets the value of AutoUpdatePolicy
    pub fn set_auto_update_policy(&mut self, value: u32) {
        self.auto_update_policy = Some(value);
    }

    /// Gets the value of AutoUpdatePolicy
    pub fn get_auto_update_policy(&self) -> Option<&u32> {
        self.auto_update_policy.as_ref()
    }

    /// Sets the value of key
    pub fn set_key(&mut self, value: u32) {
        self.key = Some(value);
    }

    /// Gets the value of key
    pub fn get_key(&self) -> Option<&u32> {
        self.key.as_ref()
    }
}

