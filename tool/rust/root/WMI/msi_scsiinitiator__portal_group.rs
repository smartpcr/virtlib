// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSIInitiator_PortalGroup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSIInitiator_PortalGroup {

/// 
    #[serde(rename = "Index")]
    pub index: Option<u32>,

/// 
    #[serde(rename = "Portals")]
    pub portals: Vec<MSiSCSIInitiator_Portal>,
}

impl MSiSCSIInitiator_PortalGroup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            index: None,
            portals: Vec::new(),
        }
    }


    /// Sets the value of Index
    pub fn set_index(&mut self, value: u32) {
        self.index = Some(value);
    }

    /// Gets the value of Index
    pub fn get_index(&self) -> Option<&u32> {
        self.index.as_ref()
    }

    /// Sets the value of Portals
    pub fn set_portals(&mut self, value: Vec<MSiSCSIInitiator_Portal>) {
        self.portals = value;
    }

    /// Gets the value of Portals
    pub fn get_portals(&self) -> &Vec<MSiSCSIInitiator_Portal> {
        &self.portals
    }
}

