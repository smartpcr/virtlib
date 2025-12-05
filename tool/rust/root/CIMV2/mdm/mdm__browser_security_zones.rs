// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_BrowserSecurityZones struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_BrowserSecurityZones {

/// 
    #[serde(rename = "Exists")]
    pub exists: Option<bool>,

/// 
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,

/// 
    #[serde(rename = "Zone")]
    pub zone: Option<u32>,
}

impl MDM_BrowserSecurityZones {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            exists: None,
            namespace: None,
            zone: None,
        }
    }


    /// Sets the value of Exists
    pub fn set_exists(&mut self, value: bool) {
        self.exists = Some(value);
    }

    /// Gets the value of Exists
    pub fn get_exists(&self) -> Option<&bool> {
        self.exists.as_ref()
    }

    /// Sets the value of Namespace
    pub fn set_namespace(&mut self, value: String) {
        self.namespace = Some(value);
    }

    /// Gets the value of Namespace
    pub fn get_namespace(&self) -> Option<&String> {
        self.namespace.as_ref()
    }

    /// Sets the value of Zone
    pub fn set_zone(&mut self, value: u32) {
        self.zone = Some(value);
    }

    /// Gets the value of Zone
    pub fn get_zone(&self) -> Option<&u32> {
        self.zone.as_ref()
    }
}

