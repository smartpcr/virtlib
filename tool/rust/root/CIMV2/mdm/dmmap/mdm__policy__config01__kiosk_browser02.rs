// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_KioskBrowser02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_KioskBrowser02 {

/// 
    #[serde(rename = "BlockedUrlExceptions")]
    pub blocked_url_exceptions: Option<String>,

/// 
    #[serde(rename = "BlockedUrls")]
    pub blocked_urls: Option<String>,

/// 
    #[serde(rename = "DefaultURL")]
    pub default_url: Option<String>,

/// 
    #[serde(rename = "EnableEndSessionButton")]
    pub enable_end_session_button: Option<i32>,

/// 
    #[serde(rename = "EnableHomeButton")]
    pub enable_home_button: Option<i32>,

/// 
    #[serde(rename = "EnableNavigationButtons")]
    pub enable_navigation_buttons: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "RestartOnIdleTime")]
    pub restart_on_idle_time: Option<i32>,
}

impl MDM_Policy_Config01_KioskBrowser02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            blocked_url_exceptions: None,
            blocked_urls: None,
            default_url: None,
            enable_end_session_button: None,
            enable_home_button: None,
            enable_navigation_buttons: None,
            instance_id: None,
            parent_id: None,
            restart_on_idle_time: None,
        }
    }


    /// Sets the value of BlockedUrlExceptions
    pub fn set_blocked_url_exceptions(&mut self, value: String) {
        self.blocked_url_exceptions = Some(value);
    }

    /// Gets the value of BlockedUrlExceptions
    pub fn get_blocked_url_exceptions(&self) -> Option<&String> {
        self.blocked_url_exceptions.as_ref()
    }

    /// Sets the value of BlockedUrls
    pub fn set_blocked_urls(&mut self, value: String) {
        self.blocked_urls = Some(value);
    }

    /// Gets the value of BlockedUrls
    pub fn get_blocked_urls(&self) -> Option<&String> {
        self.blocked_urls.as_ref()
    }

    /// Sets the value of DefaultURL
    pub fn set_default_url(&mut self, value: String) {
        self.default_url = Some(value);
    }

    /// Gets the value of DefaultURL
    pub fn get_default_url(&self) -> Option<&String> {
        self.default_url.as_ref()
    }

    /// Sets the value of EnableEndSessionButton
    pub fn set_enable_end_session_button(&mut self, value: i32) {
        self.enable_end_session_button = Some(value);
    }

    /// Gets the value of EnableEndSessionButton
    pub fn get_enable_end_session_button(&self) -> Option<&i32> {
        self.enable_end_session_button.as_ref()
    }

    /// Sets the value of EnableHomeButton
    pub fn set_enable_home_button(&mut self, value: i32) {
        self.enable_home_button = Some(value);
    }

    /// Gets the value of EnableHomeButton
    pub fn get_enable_home_button(&self) -> Option<&i32> {
        self.enable_home_button.as_ref()
    }

    /// Sets the value of EnableNavigationButtons
    pub fn set_enable_navigation_buttons(&mut self, value: i32) {
        self.enable_navigation_buttons = Some(value);
    }

    /// Gets the value of EnableNavigationButtons
    pub fn get_enable_navigation_buttons(&self) -> Option<&i32> {
        self.enable_navigation_buttons.as_ref()
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

    /// Sets the value of RestartOnIdleTime
    pub fn set_restart_on_idle_time(&mut self, value: i32) {
        self.restart_on_idle_time = Some(value);
    }

    /// Gets the value of RestartOnIdleTime
    pub fn get_restart_on_idle_time(&self) -> Option<&i32> {
        self.restart_on_idle_time.as_ref()
    }
}

