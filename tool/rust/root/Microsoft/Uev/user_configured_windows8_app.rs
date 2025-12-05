// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Uev
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// UserConfiguredWindows8App struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserConfiguredWindows8App {

/// 
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "Installed")]
    pub installed: Option<bool>,

/// 
    #[serde(rename = "PackageFamilyName")]
    pub package_family_name: Option<String>,
}

impl UserConfiguredWindows8App {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            display_name: None,
            enabled: None,
            installed: None,
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

    /// Sets the value of Installed
    pub fn set_installed(&mut self, value: bool) {
        self.installed = Some(value);
    }

    /// Gets the value of Installed
    pub fn get_installed(&self) -> Option<&bool> {
        self.installed.as_ref()
    }

    /// Sets the value of PackageFamilyName
    pub fn set_package_family_name(&mut self, value: String) {
        self.package_family_name = Some(value);
    }

    /// Gets the value of PackageFamilyName
    pub fn get_package_family_name(&self) -> Option<&String> {
        self.package_family_name.as_ref()
    }

/// 

    /// * `package_family_name` -  (String)
    pub fn enable_app(&self, package_family_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "packageFamilyName".to_string(), value: package_family_name.into() });
        self.invoke_method("EnableApp", &args)

    }


/// 

    /// * `package_family_name` -  (String)
    pub fn disable_app(&self, package_family_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "packageFamilyName".to_string(), value: package_family_name.into() });
        self.invoke_method("DisableApp", &args)

    }


/// 

    /// * `package_family_name` -  (String)
    pub fn remove_app(&self, package_family_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "packageFamilyName".to_string(), value: package_family_name.into() });
        self.invoke_method("RemoveApp", &args)

    }


/// 

    /// * `package_family_name` -  (String)

    /// * `return_value` -  (bool)
    pub fn check_app(&self, package_family_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "packageFamilyName".to_string(), value: package_family_name.into() });
        self.invoke_method("CheckApp", &args)

    }

}

