// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IESecurityZoneSettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IESecurityZoneSettings {

/// 
    #[serde(rename = "actionValues")]
    pub action_values: Vec<String>,

/// 
    #[serde(rename = "currentTemplateLevel")]
    pub current_template_level: Option<u32>,

/// 
    #[serde(rename = "description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "iconPath")]
    pub icon_path: Option<String>,

/// 
    #[serde(rename = "minimumTemplateLevel")]
    pub minimum_template_level: Option<u32>,

/// 
    #[serde(rename = "recommendedTemplateLevel")]
    pub recommended_template_level: Option<u32>,

/// 
    #[serde(rename = "rsopID")]
    pub rsop_id: Option<String>,

/// 
    #[serde(rename = "rsopPrecedence")]
    pub rsop_precedence: Option<i32>,

/// 
    #[serde(rename = "useHKLM")]
    pub use_hklm: Option<bool>,

/// 
    #[serde(rename = "zoneIndex")]
    pub zone_index: Option<u32>,

/// 
    #[serde(rename = "zoneMappings")]
    pub zone_mappings: Vec<String>,
}

impl RSOP_IESecurityZoneSettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            action_values: Vec::new(),
            current_template_level: None,
            description: None,
            display_name: None,
            flags: None,
            icon_path: None,
            minimum_template_level: None,
            recommended_template_level: None,
            rsop_id: None,
            rsop_precedence: None,
            use_hklm: None,
            zone_index: None,
            zone_mappings: Vec::new(),
        }
    }


    /// Sets the value of actionValues
    pub fn set_action_values(&mut self, value: Vec<String>) {
        self.action_values = value;
    }

    /// Gets the value of actionValues
    pub fn get_action_values(&self) -> &Vec<String> {
        &self.action_values
    }

    /// Sets the value of currentTemplateLevel
    pub fn set_current_template_level(&mut self, value: u32) {
        self.current_template_level = Some(value);
    }

    /// Gets the value of currentTemplateLevel
    pub fn get_current_template_level(&self) -> Option<&u32> {
        self.current_template_level.as_ref()
    }

    /// Sets the value of description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of displayName
    pub fn set_display_name(&mut self, value: String) {
        self.display_name = Some(value);
    }

    /// Gets the value of displayName
    pub fn get_display_name(&self) -> Option<&String> {
        self.display_name.as_ref()
    }

    /// Sets the value of flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of iconPath
    pub fn set_icon_path(&mut self, value: String) {
        self.icon_path = Some(value);
    }

    /// Gets the value of iconPath
    pub fn get_icon_path(&self) -> Option<&String> {
        self.icon_path.as_ref()
    }

    /// Sets the value of minimumTemplateLevel
    pub fn set_minimum_template_level(&mut self, value: u32) {
        self.minimum_template_level = Some(value);
    }

    /// Gets the value of minimumTemplateLevel
    pub fn get_minimum_template_level(&self) -> Option<&u32> {
        self.minimum_template_level.as_ref()
    }

    /// Sets the value of recommendedTemplateLevel
    pub fn set_recommended_template_level(&mut self, value: u32) {
        self.recommended_template_level = Some(value);
    }

    /// Gets the value of recommendedTemplateLevel
    pub fn get_recommended_template_level(&self) -> Option<&u32> {
        self.recommended_template_level.as_ref()
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

    /// Sets the value of useHKLM
    pub fn set_use_hklm(&mut self, value: bool) {
        self.use_hklm = Some(value);
    }

    /// Gets the value of useHKLM
    pub fn get_use_hklm(&self) -> Option<&bool> {
        self.use_hklm.as_ref()
    }

    /// Sets the value of zoneIndex
    pub fn set_zone_index(&mut self, value: u32) {
        self.zone_index = Some(value);
    }

    /// Gets the value of zoneIndex
    pub fn get_zone_index(&self) -> Option<&u32> {
        self.zone_index.as_ref()
    }

    /// Sets the value of zoneMappings
    pub fn set_zone_mappings(&mut self, value: Vec<String>) {
        self.zone_mappings = value;
    }

    /// Gets the value of zoneMappings
    pub fn get_zone_mappings(&self) -> &Vec<String> {
        &self.zone_mappings
    }
}

