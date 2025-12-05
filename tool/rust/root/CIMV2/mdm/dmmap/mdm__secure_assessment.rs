// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_SecureAssessment struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_SecureAssessment {

/// 
    #[serde(rename = "AllowScreenMonitoring")]
    pub allow_screen_monitoring: Option<bool>,

/// 
    #[serde(rename = "AllowTextSuggestions")]
    pub allow_text_suggestions: Option<bool>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "LaunchURI")]
    pub launch_uri: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "RequirePrinting")]
    pub require_printing: Option<bool>,

/// 
    #[serde(rename = "TesterAccount")]
    pub tester_account: Option<String>,
}

impl MDM_SecureAssessment {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_screen_monitoring: None,
            allow_text_suggestions: None,
            instance_id: None,
            launch_uri: None,
            parent_id: None,
            require_printing: None,
            tester_account: None,
        }
    }


    /// Sets the value of AllowScreenMonitoring
    pub fn set_allow_screen_monitoring(&mut self, value: bool) {
        self.allow_screen_monitoring = Some(value);
    }

    /// Gets the value of AllowScreenMonitoring
    pub fn get_allow_screen_monitoring(&self) -> Option<&bool> {
        self.allow_screen_monitoring.as_ref()
    }

    /// Sets the value of AllowTextSuggestions
    pub fn set_allow_text_suggestions(&mut self, value: bool) {
        self.allow_text_suggestions = Some(value);
    }

    /// Gets the value of AllowTextSuggestions
    pub fn get_allow_text_suggestions(&self) -> Option<&bool> {
        self.allow_text_suggestions.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of LaunchURI
    pub fn set_launch_uri(&mut self, value: String) {
        self.launch_uri = Some(value);
    }

    /// Gets the value of LaunchURI
    pub fn get_launch_uri(&self) -> Option<&String> {
        self.launch_uri.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of RequirePrinting
    pub fn set_require_printing(&mut self, value: bool) {
        self.require_printing = Some(value);
    }

    /// Gets the value of RequirePrinting
    pub fn get_require_printing(&self) -> Option<&bool> {
        self.require_printing.as_ref()
    }

    /// Sets the value of TesterAccount
    pub fn set_tester_account(&mut self, value: String) {
        self.tester_account = Some(value);
    }

    /// Gets the value of TesterAccount
    pub fn get_tester_account(&self) -> Option<&String> {
        self.tester_account.as_ref()
    }
}

