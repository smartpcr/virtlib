// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_User_Config01_Experience02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_User_Config01_Experience02 {

/// 
    #[serde(rename = "AllowSpotlightCollection")]
    pub allow_spotlight_collection: Option<i32>,

/// 
    #[serde(rename = "AllowTailoredExperiencesWithDiagnosticData")]
    pub allow_tailored_experiences_with_diagnostic_data: Option<i32>,

/// 
    #[serde(rename = "AllowThirdPartySuggestionsInWindowsSpotlight")]
    pub allow_third_party_suggestions_in_windows_spotlight: Option<i32>,

/// 
    #[serde(rename = "AllowWindowsSpotlight")]
    pub allow_windows_spotlight: Option<i32>,

/// 
    #[serde(rename = "AllowWindowsSpotlightOnActionCenter")]
    pub allow_windows_spotlight_on_action_center: Option<i32>,

/// 
    #[serde(rename = "AllowWindowsSpotlightOnSettings")]
    pub allow_windows_spotlight_on_settings: Option<i32>,

/// 
    #[serde(rename = "AllowWindowsSpotlightWindowsWelcomeExperience")]
    pub allow_windows_spotlight_windows_welcome_experience: Option<i32>,

/// 
    #[serde(rename = "ConfigureWindowsSpotlightOnLockScreen")]
    pub configure_windows_spotlight_on_lock_screen: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_User_Config01_Experience02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_spotlight_collection: None,
            allow_tailored_experiences_with_diagnostic_data: None,
            allow_third_party_suggestions_in_windows_spotlight: None,
            allow_windows_spotlight: None,
            allow_windows_spotlight_on_action_center: None,
            allow_windows_spotlight_on_settings: None,
            allow_windows_spotlight_windows_welcome_experience: None,
            configure_windows_spotlight_on_lock_screen: None,
            instance_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of AllowSpotlightCollection
    pub fn set_allow_spotlight_collection(&mut self, value: i32) {
        self.allow_spotlight_collection = Some(value);
    }

    /// Gets the value of AllowSpotlightCollection
    pub fn get_allow_spotlight_collection(&self) -> Option<&i32> {
        self.allow_spotlight_collection.as_ref()
    }

    /// Sets the value of AllowTailoredExperiencesWithDiagnosticData
    pub fn set_allow_tailored_experiences_with_diagnostic_data(&mut self, value: i32) {
        self.allow_tailored_experiences_with_diagnostic_data = Some(value);
    }

    /// Gets the value of AllowTailoredExperiencesWithDiagnosticData
    pub fn get_allow_tailored_experiences_with_diagnostic_data(&self) -> Option<&i32> {
        self.allow_tailored_experiences_with_diagnostic_data.as_ref()
    }

    /// Sets the value of AllowThirdPartySuggestionsInWindowsSpotlight
    pub fn set_allow_third_party_suggestions_in_windows_spotlight(&mut self, value: i32) {
        self.allow_third_party_suggestions_in_windows_spotlight = Some(value);
    }

    /// Gets the value of AllowThirdPartySuggestionsInWindowsSpotlight
    pub fn get_allow_third_party_suggestions_in_windows_spotlight(&self) -> Option<&i32> {
        self.allow_third_party_suggestions_in_windows_spotlight.as_ref()
    }

    /// Sets the value of AllowWindowsSpotlight
    pub fn set_allow_windows_spotlight(&mut self, value: i32) {
        self.allow_windows_spotlight = Some(value);
    }

    /// Gets the value of AllowWindowsSpotlight
    pub fn get_allow_windows_spotlight(&self) -> Option<&i32> {
        self.allow_windows_spotlight.as_ref()
    }

    /// Sets the value of AllowWindowsSpotlightOnActionCenter
    pub fn set_allow_windows_spotlight_on_action_center(&mut self, value: i32) {
        self.allow_windows_spotlight_on_action_center = Some(value);
    }

    /// Gets the value of AllowWindowsSpotlightOnActionCenter
    pub fn get_allow_windows_spotlight_on_action_center(&self) -> Option<&i32> {
        self.allow_windows_spotlight_on_action_center.as_ref()
    }

    /// Sets the value of AllowWindowsSpotlightOnSettings
    pub fn set_allow_windows_spotlight_on_settings(&mut self, value: i32) {
        self.allow_windows_spotlight_on_settings = Some(value);
    }

    /// Gets the value of AllowWindowsSpotlightOnSettings
    pub fn get_allow_windows_spotlight_on_settings(&self) -> Option<&i32> {
        self.allow_windows_spotlight_on_settings.as_ref()
    }

    /// Sets the value of AllowWindowsSpotlightWindowsWelcomeExperience
    pub fn set_allow_windows_spotlight_windows_welcome_experience(&mut self, value: i32) {
        self.allow_windows_spotlight_windows_welcome_experience = Some(value);
    }

    /// Gets the value of AllowWindowsSpotlightWindowsWelcomeExperience
    pub fn get_allow_windows_spotlight_windows_welcome_experience(&self) -> Option<&i32> {
        self.allow_windows_spotlight_windows_welcome_experience.as_ref()
    }

    /// Sets the value of ConfigureWindowsSpotlightOnLockScreen
    pub fn set_configure_windows_spotlight_on_lock_screen(&mut self, value: i32) {
        self.configure_windows_spotlight_on_lock_screen = Some(value);
    }

    /// Gets the value of ConfigureWindowsSpotlightOnLockScreen
    pub fn get_configure_windows_spotlight_on_lock_screen(&self) -> Option<&i32> {
        self.configure_windows_spotlight_on_lock_screen.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
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

