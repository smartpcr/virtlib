// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ISCSI_TargetPortalGroup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ISCSI_TargetPortalGroup {

/// 
    #[serde(rename = "PortalCount")]
    pub portal_count: Option<u32>,

/// 
    #[serde(rename = "Portals")]
    pub portals: Vec<ISCSI_TargetPortal>,
}

impl ISCSI_TargetPortalGroup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            portal_count: None,
            portals: Vec::new(),
        }
    }


    /// Sets the value of PortalCount
    pub fn set_portal_count(&mut self, value: u32) {
        self.portal_count = Some(value);
    }

    /// Gets the value of PortalCount
    pub fn get_portal_count(&self) -> Option<&u32> {
        self.portal_count.as_ref()
    }

    /// Sets the value of Portals
    pub fn set_portals(&mut self, value: Vec<ISCSI_TargetPortal>) {
        self.portals = value;
    }

    /// Gets the value of Portals
    pub fn get_portals(&self) -> &Vec<ISCSI_TargetPortal> {
        &self.portals
    }
}

