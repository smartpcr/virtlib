// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Uev
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// EffectiveWindows8App struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EffectiveWindows8App {

/// 
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "EnabledSource")]
    pub enabled_source: Option<String>,

/// 
    #[serde(rename = "PackageFamilyName")]
    pub package_family_name: Option<String>,
}

impl EffectiveWindows8App {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            display_name: None,
            enabled: None,
            enabled_source: None,
            package_family_name: None,
        }
    }


    /// Sets the value of DisplayName
    pub fn set_display_name(&mut self, value: String) {
        self.display_name = Some(value);
    }

    /// Gets the value of DisplayName
    pub fn get_display_name(&self) -> Option<&String> {
        self.display_name.as_ref()
    }

    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of EnabledSource
    pub fn set_enabled_source(&mut self, value: String) {
        self.enabled_source = Some(value);
    }

    /// Gets the value of EnabledSource
    pub fn get_enabled_source(&self) -> Option<&String> {
        self.enabled_source.as_ref()
    }

    /// Sets the value of PackageFamilyName
    pub fn set_package_family_name(&mut self, value: String) {
        self.package_family_name = Some(value);
    }

    /// Gets the value of PackageFamilyName
    pub fn get_package_family_name(&self) -> Option<&String> {
        self.package_family_name.as_ref()
    }
}

