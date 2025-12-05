// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_BrowserSettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_BrowserSettings {

/// 
    #[serde(rename = "AlwaysSendDoNotTrackHeader")]
    pub always_send_do_not_track_header: Option<bool>,

/// 
    #[serde(rename = "AutofillEnabled")]
    pub autofill_enabled: Option<bool>,

/// 
    #[serde(rename = "ForceFraudWarning")]
    pub force_fraud_warning: Option<bool>,

/// 
    #[serde(rename = "GoToIntranetForSingleWord")]
    pub go_to_intranet_for_single_word: Option<bool>,

/// 
    #[serde(rename = "InternetBlockPopups")]
    pub internet_block_popups: Option<bool>,

/// 
    #[serde(rename = "InternetPluginsEnabled")]
    pub internet_plugins_enabled: Option<bool>,

/// 
    #[serde(rename = "InternetProtectedModeEnabled")]
    pub internet_protected_mode_enabled: Option<bool>,

/// 
    #[serde(rename = "InternetScriptingEnabled")]
    pub internet_scripting_enabled: Option<bool>,

/// 
    #[serde(rename = "InternetZoneSecurityLevel")]
    pub internet_zone_security_level: Option<u32>,

/// 
    #[serde(rename = "IntranetSecurityZoneEnabled")]
    pub intranet_security_zone_enabled: Option<bool>,

/// 
    #[serde(rename = "IntranetZoneSecurityLevel")]
    pub intranet_zone_security_level: Option<u32>,

/// 
    #[serde(rename = "key")]
    pub key: Option<u32>,

/// 
    #[serde(rename = "RestrictedSitesZoneSecurityLevel")]
    pub restricted_sites_zone_security_level: Option<u32>,

/// 
    #[serde(rename = "TrustedSitesZoneSecurityLevel")]
    pub trusted_sites_zone_security_level: Option<u32>,
}

impl MDM_BrowserSettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            always_send_do_not_track_header: None,
            autofill_enabled: None,
            force_fraud_warning: None,
            go_to_intranet_for_single_word: None,
            internet_block_popups: None,
            internet_plugins_enabled: None,
            internet_protected_mode_enabled: None,
            internet_scripting_enabled: None,
            internet_zone_security_level: None,
            intranet_security_zone_enabled: None,
            intranet_zone_security_level: None,
            key: None,
            restricted_sites_zone_security_level: None,
            trusted_sites_zone_security_level: None,
        }
    }


    /// Sets the value of AlwaysSendDoNotTrackHeader
    pub fn set_always_send_do_not_track_header(&mut self, value: bool) {
        self.always_send_do_not_track_header = Some(value);
    }

    /// Gets the value of AlwaysSendDoNotTrackHeader
    pub fn get_always_send_do_not_track_header(&self) -> Option<&bool> {
        self.always_send_do_not_track_header.as_ref()
    }

    /// Sets the value of AutofillEnabled
    pub fn set_autofill_enabled(&mut self, value: bool) {
        self.autofill_enabled = Some(value);
    }

    /// Gets the value of AutofillEnabled
    pub fn get_autofill_enabled(&self) -> Option<&bool> {
        self.autofill_enabled.as_ref()
    }

    /// Sets the value of ForceFraudWarning
    pub fn set_force_fraud_warning(&mut self, value: bool) {
        self.force_fraud_warning = Some(value);
    }

    /// Gets the value of ForceFraudWarning
    pub fn get_force_fraud_warning(&self) -> Option<&bool> {
        self.force_fraud_warning.as_ref()
    }

    /// Sets the value of GoToIntranetForSingleWord
    pub fn set_go_to_intranet_for_single_word(&mut self, value: bool) {
        self.go_to_intranet_for_single_word = Some(value);
    }

    /// Gets the value of GoToIntranetForSingleWord
    pub fn get_go_to_intranet_for_single_word(&self) -> Option<&bool> {
        self.go_to_intranet_for_single_word.as_ref()
    }

    /// Sets the value of InternetBlockPopups
    pub fn set_internet_block_popups(&mut self, value: bool) {
        self.internet_block_popups = Some(value);
    }

    /// Gets the value of InternetBlockPopups
    pub fn get_internet_block_popups(&self) -> Option<&bool> {
        self.internet_block_popups.as_ref()
    }

    /// Sets the value of InternetPluginsEnabled
    pub fn set_internet_plugins_enabled(&mut self, value: bool) {
        self.internet_plugins_enabled = Some(value);
    }

    /// Gets the value of InternetPluginsEnabled
    pub fn get_internet_plugins_enabled(&self) -> Option<&bool> {
        self.internet_plugins_enabled.as_ref()
    }

    /// Sets the value of InternetProtectedModeEnabled
    pub fn set_internet_protected_mode_enabled(&mut self, value: bool) {
        self.internet_protected_mode_enabled = Some(value);
    }

    /// Gets the value of InternetProtectedModeEnabled
    pub fn get_internet_protected_mode_enabled(&self) -> Option<&bool> {
        self.internet_protected_mode_enabled.as_ref()
    }

    /// Sets the value of InternetScriptingEnabled
    pub fn set_internet_scripting_enabled(&mut self, value: bool) {
        self.internet_scripting_enabled = Some(value);
    }

    /// Gets the value of InternetScriptingEnabled
    pub fn get_internet_scripting_enabled(&self) -> Option<&bool> {
        self.internet_scripting_enabled.as_ref()
    }

    /// Sets the value of InternetZoneSecurityLevel
    pub fn set_internet_zone_security_level(&mut self, value: u32) {
        self.internet_zone_security_level = Some(value);
    }

    /// Gets the value of InternetZoneSecurityLevel
    pub fn get_internet_zone_security_level(&self) -> Option<&u32> {
        self.internet_zone_security_level.as_ref()
    }

    /// Sets the value of IntranetSecurityZoneEnabled
    pub fn set_intranet_security_zone_enabled(&mut self, value: bool) {
        self.intranet_security_zone_enabled = Some(value);
    }

    /// Gets the value of IntranetSecurityZoneEnabled
    pub fn get_intranet_security_zone_enabled(&self) -> Option<&bool> {
        self.intranet_security_zone_enabled.as_ref()
    }

    /// Sets the value of IntranetZoneSecurityLevel
    pub fn set_intranet_zone_security_level(&mut self, value: u32) {
        self.intranet_zone_security_level = Some(value);
    }

    /// Gets the value of IntranetZoneSecurityLevel
    pub fn get_intranet_zone_security_level(&self) -> Option<&u32> {
        self.intranet_zone_security_level.as_ref()
    }

    /// Sets the value of key
    pub fn set_key(&mut self, value: u32) {
        self.key = Some(value);
    }

    /// Gets the value of key
    pub fn get_key(&self) -> Option<&u32> {
        self.key.as_ref()
    }

    /// Sets the value of RestrictedSitesZoneSecurityLevel
    pub fn set_restricted_sites_zone_security_level(&mut self, value: u32) {
        self.restricted_sites_zone_security_level = Some(value);
    }

    /// Gets the value of RestrictedSitesZoneSecurityLevel
    pub fn get_restricted_sites_zone_security_level(&self) -> Option<&u32> {
        self.restricted_sites_zone_security_level.as_ref()
    }

    /// Sets the value of TrustedSitesZoneSecurityLevel
    pub fn set_trusted_sites_zone_security_level(&mut self, value: u32) {
        self.trusted_sites_zone_security_level = Some(value);
    }

    /// Gets the value of TrustedSitesZoneSecurityLevel
    pub fn get_trusted_sites_zone_security_level(&self) -> Option<&u32> {
        self.trusted_sites_zone_security_level.as_ref()
    }
}

