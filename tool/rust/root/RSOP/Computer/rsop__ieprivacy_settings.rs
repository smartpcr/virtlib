// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IEPrivacySettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IEPrivacySettings {

/// 
    #[serde(rename = "firstPartyPrivacyType")]
    pub first_party_privacy_type: Option<u32>,

/// 
    #[serde(rename = "firstPartyPrivacyTypeText")]
    pub first_party_privacy_type_text: Option<String>,

/// 
    #[serde(rename = "rsopID")]
    pub rsop_id: Option<String>,

/// 
    #[serde(rename = "rsopPrecedence")]
    pub rsop_precedence: Option<i32>,

/// 
    #[serde(rename = "thirdPartyPrivacyType")]
    pub third_party_privacy_type: Option<u32>,

/// 
    #[serde(rename = "thirdPartyPrivacyTypeText")]
    pub third_party_privacy_type_text: Option<String>,

/// 
    #[serde(rename = "useAdvancedSettings")]
    pub use_advanced_settings: Option<bool>,
}

impl RSOP_IEPrivacySettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            first_party_privacy_type: None,
            first_party_privacy_type_text: None,
            rsop_id: None,
            rsop_precedence: None,
            third_party_privacy_type: None,
            third_party_privacy_type_text: None,
            use_advanced_settings: None,
        }
    }


    /// Sets the value of firstPartyPrivacyType
    pub fn set_first_party_privacy_type(&mut self, value: u32) {
        self.first_party_privacy_type = Some(value);
    }

    /// Gets the value of firstPartyPrivacyType
    pub fn get_first_party_privacy_type(&self) -> Option<&u32> {
        self.first_party_privacy_type.as_ref()
    }

    /// Sets the value of firstPartyPrivacyTypeText
    pub fn set_first_party_privacy_type_text(&mut self, value: String) {
        self.first_party_privacy_type_text = Some(value);
    }

    /// Gets the value of firstPartyPrivacyTypeText
    pub fn get_first_party_privacy_type_text(&self) -> Option<&String> {
        self.first_party_privacy_type_text.as_ref()
    }

    /// Sets the value of rsopID
    pub fn set_rsop_id(&mut self, value: String) {
        self.rsop_id = Some(value);
    }

    /// Gets the value of rsopID
    pub fn get_rsop_id(&self) -> Option<&String> {
        self.rsop_id.as_ref()
    }

    /// Sets the value of rsopPrecedence
    pub fn set_rsop_precedence(&mut self, value: i32) {
        self.rsop_precedence = Some(value);
    }

    /// Gets the value of rsopPrecedence
    pub fn get_rsop_precedence(&self) -> Option<&i32> {
        self.rsop_precedence.as_ref()
    }

    /// Sets the value of thirdPartyPrivacyType
    pub fn set_third_party_privacy_type(&mut self, value: u32) {
        self.third_party_privacy_type = Some(value);
    }

    /// Gets the value of thirdPartyPrivacyType
    pub fn get_third_party_privacy_type(&self) -> Option<&u32> {
        self.third_party_privacy_type.as_ref()
    }

    /// Sets the value of thirdPartyPrivacyTypeText
    pub fn set_third_party_privacy_type_text(&mut self, value: String) {
        self.third_party_privacy_type_text = Some(value);
    }

    /// Gets the value of thirdPartyPrivacyTypeText
    pub fn get_third_party_privacy_type_text(&self) -> Option<&String> {
        self.third_party_privacy_type_text.as_ref()
    }

    /// Sets the value of useAdvancedSettings
    pub fn set_use_advanced_settings(&mut self, value: bool) {
        self.use_advanced_settings = Some(value);
    }

    /// Gets the value of useAdvancedSettings
    pub fn get_use_advanced_settings(&self) -> Option<&bool> {
        self.use_advanced_settings.as_ref()
    }
}

