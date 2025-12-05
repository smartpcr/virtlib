// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_WindowsAdvancedThreatProtection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_WindowsAdvancedThreatProtection {

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "Offboarding")]
    pub offboarding: Option<String>,

/// 
    #[serde(rename = "Onboarding")]
    pub onboarding: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_WindowsAdvancedThreatProtection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            offboarding: None,
            onboarding: None,
            parent_id: None,
        }
    }


    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of Offboarding
    pub fn set_offboarding(&mut self, value: String) {
        self.offboarding = Some(value);
    }

    /// Gets the value of Offboarding
    pub fn get_offboarding(&self) -> Option<&String> {
        self.offboarding.as_ref()
    }

    /// Sets the value of Onboarding
    pub fn set_onboarding(&mut self, value: String) {
        self.onboarding = Some(value);
    }

    /// Gets the value of Onboarding
    pub fn get_onboarding(&self) -> Option<&String> {
        self.onboarding.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }
}

