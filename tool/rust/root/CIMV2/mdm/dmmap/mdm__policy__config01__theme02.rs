// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_Theme02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_Theme02 {

/// 
    #[serde(rename = "DefaultBackgroundImage")]
    pub default_background_image: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "SystemUsesLightTheme")]
    pub system_uses_light_theme: Option<i32>,
}

impl MDM_Policy_Config01_Theme02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            default_background_image: None,
            instance_id: None,
            parent_id: None,
            system_uses_light_theme: None,
        }
    }


    /// Sets the value of DefaultBackgroundImage
    pub fn set_default_background_image(&mut self, value: String) {
        self.default_background_image = Some(value);
    }

    /// Gets the value of DefaultBackgroundImage
    pub fn get_default_background_image(&self) -> Option<&String> {
        self.default_background_image.as_ref()
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

    /// Sets the value of SystemUsesLightTheme
    pub fn set_system_uses_light_theme(&mut self, value: i32) {
        self.system_uses_light_theme = Some(value);
    }

    /// Gets the value of SystemUsesLightTheme
    pub fn get_system_uses_light_theme(&self) -> Option<&i32> {
        self.system_uses_light_theme.as_ref()
    }
}

