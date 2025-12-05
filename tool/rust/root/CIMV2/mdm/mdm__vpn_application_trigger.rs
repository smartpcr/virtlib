// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_VpnApplicationTrigger struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_VpnApplicationTrigger {

/// 
    #[serde(rename = "ApplicationID")]
    pub application_id: Option<String>,

/// 
    #[serde(rename = "TriggerEnabledInAllMDMProfiles")]
    pub trigger_enabled_in_all_mdmprofiles: Option<bool>,
}

impl MDM_VpnApplicationTrigger {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            application_id: None,
            trigger_enabled_in_all_mdmprofiles: None,
        }
    }


    /// Sets the value of ApplicationID
    pub fn set_application_id(&mut self, value: String) {
        self.application_id = Some(value);
    }

    /// Gets the value of ApplicationID
    pub fn get_application_id(&self) -> Option<&String> {
        self.application_id.as_ref()
    }

    /// Sets the value of TriggerEnabledInAllMDMProfiles
    pub fn set_trigger_enabled_in_all_mdmprofiles(&mut self, value: bool) {
        self.trigger_enabled_in_all_mdmprofiles = Some(value);
    }

    /// Gets the value of TriggerEnabledInAllMDMProfiles
    pub fn get_trigger_enabled_in_all_mdmprofiles(&self) -> Option<&bool> {
        self.trigger_enabled_in_all_mdmprofiles.as_ref()
    }
}

