// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_GPLink struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_GPLink {

/// 
    #[serde(rename = "appliedOrder")]
    pub applied_order: Option<u32>,

/// 
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "GPO")]
    pub gpo: Option<RSOP_GPO>,

/// 
    #[serde(rename = "linkOrder")]
    pub link_order: Option<u32>,

/// 
    #[serde(rename = "noOverride")]
    pub no_override: Option<bool>,

/// 
    #[serde(rename = "SOM")]
    pub som: Option<RSOP_SOM>,

/// 
    #[serde(rename = "somOrder")]
    pub som_order: Option<u32>,
}

impl RSOP_GPLink {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            applied_order: None,
            enabled: None,
            gpo: None,
            link_order: None,
            no_override: None,
            som: None,
            som_order: None,
        }
    }


    /// Sets the value of appliedOrder
    pub fn set_applied_order(&mut self, value: u32) {
        self.applied_order = Some(value);
    }

    /// Gets the value of appliedOrder
    pub fn get_applied_order(&self) -> Option<&u32> {
        self.applied_order.as_ref()
    }

    /// Sets the value of enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of GPO
    pub fn set_gpo(&mut self, value: RSOP_GPO) {
        self.gpo = Some(value);
    }

    /// Gets the value of GPO
    pub fn get_gpo(&self) -> Option<&RSOP_GPO> {
        self.gpo.as_ref()
    }

    /// Sets the value of linkOrder
    pub fn set_link_order(&mut self, value: u32) {
        self.link_order = Some(value);
    }

    /// Gets the value of linkOrder
    pub fn get_link_order(&self) -> Option<&u32> {
        self.link_order.as_ref()
    }

    /// Sets the value of noOverride
    pub fn set_no_override(&mut self, value: bool) {
        self.no_override = Some(value);
    }

    /// Gets the value of noOverride
    pub fn get_no_override(&self) -> Option<&bool> {
        self.no_override.as_ref()
    }

    /// Sets the value of SOM
    pub fn set_som(&mut self, value: RSOP_SOM) {
        self.som = Some(value);
    }

    /// Gets the value of SOM
    pub fn get_som(&self) -> Option<&RSOP_SOM> {
        self.som.as_ref()
    }

    /// Sets the value of somOrder
    pub fn set_som_order(&mut self, value: u32) {
        self.som_order = Some(value);
    }

    /// Gets the value of somOrder
    pub fn get_som_order(&self) -> Option<&u32> {
        self.som_order.as_ref()
    }
}

