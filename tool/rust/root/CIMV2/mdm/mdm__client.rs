// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Client struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Client {

/// 
    #[serde(rename = "DeviceClientID")]
    pub device_client_id: Option<String>,

/// 
    #[serde(rename = "DeviceName")]
    pub device_name: Option<String>,

/// 
    #[serde(rename = "DomainSID")]
    pub domain_sid: Option<String>,

/// 
    #[serde(rename = "PlatformID")]
    pub platform_id: Option<String>,

/// 
    #[serde(rename = "ProcessorDescription")]
    pub processor_description: Option<String>,

/// 
    #[serde(rename = "UserSid")]
    pub user_sid: Option<String>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

impl MDM_Client {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            device_client_id: None,
            device_name: None,
            domain_sid: None,
            platform_id: None,
            processor_description: None,
            user_sid: None,
            version: None,
        }
    }


    /// Sets the value of DeviceClientID
    pub fn set_device_client_id(&mut self, value: String) {
        self.device_client_id = Some(value);
    }

    /// Gets the value of DeviceClientID
    pub fn get_device_client_id(&self) -> Option<&String> {
        self.device_client_id.as_ref()
    }

    /// Sets the value of DeviceName
    pub fn set_device_name(&mut self, value: String) {
        self.device_name = Some(value);
    }

    /// Gets the value of DeviceName
    pub fn get_device_name(&self) -> Option<&String> {
        self.device_name.as_ref()
    }

    /// Sets the value of DomainSID
    pub fn set_domain_sid(&mut self, value: String) {
        self.domain_sid = Some(value);
    }

    /// Gets the value of DomainSID
    pub fn get_domain_sid(&self) -> Option<&String> {
        self.domain_sid.as_ref()
    }

    /// Sets the value of PlatformID
    pub fn set_platform_id(&mut self, value: String) {
        self.platform_id = Some(value);
    }

    /// Gets the value of PlatformID
    pub fn get_platform_id(&self) -> Option<&String> {
        self.platform_id.as_ref()
    }

    /// Sets the value of ProcessorDescription
    pub fn set_processor_description(&mut self, value: String) {
        self.processor_description = Some(value);
    }

    /// Gets the value of ProcessorDescription
    pub fn get_processor_description(&self) -> Option<&String> {
        self.processor_description.as_ref()
    }

    /// Sets the value of UserSid
    pub fn set_user_sid(&mut self, value: String) {
        self.user_sid = Some(value);
    }

    /// Gets the value of UserSid
    pub fn get_user_sid(&self) -> Option<&String> {
        self.user_sid.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: String) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&String> {
        self.version.as_ref()
    }

/// 

    /// * `device_client_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn send_unenroll_request(&self, device_client_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DeviceClientID".to_string(), value: device_client_id.into() });
        self.invoke_method("SendUnenrollRequest", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn lock_workstation(&self) -> Result<(), WmiError> {
        self.invoke_method("LockWorkstation", &[])

    }


/// 

    /// * `config_string` -  (String)

    /// * `return_value` -  (u32)
    pub fn reset_user_password(&self, config_string: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ConfigString".to_string(), value: config_string.into() });
        self.invoke_method("ResetUserPassword", &args)

    }

}

