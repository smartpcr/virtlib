// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SecurityCenter
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// FirewallProduct struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FirewallProduct {

/// 
    #[serde(rename = "companyName")]
    pub company_name: Option<String>,

/// 
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "instanceGuid")]
    pub instance_guid: Option<String>,

/// 
    #[serde(rename = "pathToSignedProductExe")]
    pub path_to_signed_product_exe: Option<String>,

/// 
    #[serde(rename = "productHasNotifiedUser")]
    pub product_has_notified_user: Option<bool>,

/// 
    #[serde(rename = "productState")]
    pub product_state: Option<u8>,

/// 
    #[serde(rename = "productWantsWscNotifications")]
    pub product_wants_wsc_notifications: Option<bool>,

/// 
    #[serde(rename = "versionNumber")]
    pub version_number: Option<String>,
}

impl FirewallProduct {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            company_name: None,
            display_name: None,
            enabled: None,
            instance_guid: None,
            path_to_signed_product_exe: None,
            product_has_notified_user: None,
            product_state: None,
            product_wants_wsc_notifications: None,
            version_number: None,
        }
    }


    /// Sets the value of companyName
    pub fn set_company_name(&mut self, value: String) {
        self.company_name = Some(value);
    }

    /// Gets the value of companyName
    pub fn get_company_name(&self) -> Option<&String> {
        self.company_name.as_ref()
    }

    /// Sets the value of displayName
    pub fn set_display_name(&mut self, value: String) {
        self.display_name = Some(value);
    }

    /// Gets the value of displayName
    pub fn get_display_name(&self) -> Option<&String> {
        self.display_name.as_ref()
    }

    /// Sets the value of enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of instanceGuid
    pub fn set_instance_guid(&mut self, value: String) {
        self.instance_guid = Some(value);
    }

    /// Gets the value of instanceGuid
    pub fn get_instance_guid(&self) -> Option<&String> {
        self.instance_guid.as_ref()
    }

    /// Sets the value of pathToSignedProductExe
    pub fn set_path_to_signed_product_exe(&mut self, value: String) {
        self.path_to_signed_product_exe = Some(value);
    }

    /// Gets the value of pathToSignedProductExe
    pub fn get_path_to_signed_product_exe(&self) -> Option<&String> {
        self.path_to_signed_product_exe.as_ref()
    }

    /// Sets the value of productHasNotifiedUser
    pub fn set_product_has_notified_user(&mut self, value: bool) {
        self.product_has_notified_user = Some(value);
    }

    /// Gets the value of productHasNotifiedUser
    pub fn get_product_has_notified_user(&self) -> Option<&bool> {
        self.product_has_notified_user.as_ref()
    }

    /// Sets the value of productState
    pub fn set_product_state(&mut self, value: u8) {
        self.product_state = Some(value);
    }

    /// Gets the value of productState
    pub fn get_product_state(&self) -> Option<&u8> {
        self.product_state.as_ref()
    }

    /// Sets the value of productWantsWscNotifications
    pub fn set_product_wants_wsc_notifications(&mut self, value: bool) {
        self.product_wants_wsc_notifications = Some(value);
    }

    /// Gets the value of productWantsWscNotifications
    pub fn get_product_wants_wsc_notifications(&self) -> Option<&bool> {
        self.product_wants_wsc_notifications.as_ref()
    }

    /// Sets the value of versionNumber
    pub fn set_version_number(&mut self, value: String) {
        self.version_number = Some(value);
    }

    /// Gets the value of versionNumber
    pub fn get_version_number(&self) -> Option<&String> {
        self.version_number.as_ref()
    }
}

