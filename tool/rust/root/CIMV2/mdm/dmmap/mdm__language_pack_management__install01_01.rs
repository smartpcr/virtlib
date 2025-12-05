// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_LanguagePackManagement_Install01_01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_LanguagePackManagement_Install01_01 {

/// 
    #[serde(rename = "CopyToDeviceInternationalSettings")]
    pub copy_to_device_international_settings: Option<bool>,

/// 
    #[serde(rename = "EnableLanguageFeatureInstallations")]
    pub enable_language_feature_installations: Option<bool>,

/// 
    #[serde(rename = "ErrorCode")]
    pub error_code: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<i32>,
}

impl MDM_LanguagePackManagement_Install01_01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            copy_to_device_international_settings: None,
            enable_language_feature_installations: None,
            error_code: None,
            instance_id: None,
            parent_id: None,
            status: None,
        }
    }


    /// Sets the value of CopyToDeviceInternationalSettings
    pub fn set_copy_to_device_international_settings(&mut self, value: bool) {
        self.copy_to_device_international_settings = Some(value);
    }

    /// Gets the value of CopyToDeviceInternationalSettings
    pub fn get_copy_to_device_international_settings(&self) -> Option<&bool> {
        self.copy_to_device_international_settings.as_ref()
    }

    /// Sets the value of EnableLanguageFeatureInstallations
    pub fn set_enable_language_feature_installations(&mut self, value: bool) {
        self.enable_language_feature_installations = Some(value);
    }

    /// Gets the value of EnableLanguageFeatureInstallations
    pub fn get_enable_language_feature_installations(&self) -> Option<&bool> {
        self.enable_language_feature_installations.as_ref()
    }

    /// Sets the value of ErrorCode
    pub fn set_error_code(&mut self, value: i32) {
        self.error_code = Some(value);
    }

    /// Gets the value of ErrorCode
    pub fn get_error_code(&self) -> Option<&i32> {
        self.error_code.as_ref()
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

    /// Sets the value of Status
    pub fn set_status(&mut self, value: i32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&i32> {
        self.status.as_ref()
    }

/// 

    /// * `param` -  (String)

    /// * `return_value` -  (u32)
    pub fn start_installation_method(&self, param: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "param".to_string(), value: param.into() });
        self.invoke_method("StartInstallationMethod", &args)

    }

}

