// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_WindowsAdvancedThreatProtection_HealthState01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_WindowsAdvancedThreatProtection_HealthState01 {

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "LastConnected")]
    pub last_connected: Option<String>,

/// 
    #[serde(rename = "OnboardingState")]
    pub onboarding_state: Option<i32>,

/// 
    #[serde(rename = "OrgId")]
    pub org_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "SenseIsRunning")]
    pub sense_is_running: Option<bool>,
}

impl MDM_WindowsAdvancedThreatProtection_HealthState01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            last_connected: None,
            onboarding_state: None,
            org_id: None,
            parent_id: None,
            sense_is_running: None,
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

    /// Sets the value of LastConnected
    pub fn set_last_connected(&mut self, value: String) {
        self.last_connected = Some(value);
    }

    /// Gets the value of LastConnected
    pub fn get_last_connected(&self) -> Option<&String> {
        self.last_connected.as_ref()
    }

    /// Sets the value of OnboardingState
    pub fn set_onboarding_state(&mut self, value: i32) {
        self.onboarding_state = Some(value);
    }

    /// Gets the value of OnboardingState
    pub fn get_onboarding_state(&self) -> Option<&i32> {
        self.onboarding_state.as_ref()
    }

    /// Sets the value of OrgId
    pub fn set_org_id(&mut self, value: String) {
        self.org_id = Some(value);
    }

    /// Gets the value of OrgId
    pub fn get_org_id(&self) -> Option<&String> {
        self.org_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of SenseIsRunning
    pub fn set_sense_is_running(&mut self, value: bool) {
        self.sense_is_running = Some(value);
    }

    /// Gets the value of SenseIsRunning
    pub fn get_sense_is_running(&self) -> Option<&bool> {
        self.sense_is_running.as_ref()
    }
}

