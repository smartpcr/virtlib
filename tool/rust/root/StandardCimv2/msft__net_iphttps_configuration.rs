// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetIPHttpsConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetIPHttpsConfiguration {
    #[serde(flatten)]
    pub base: MSFT_NetSettingData,

/// 
    #[serde(rename = "AuthMode")]
    pub auth_mode: Option<u32>,

/// 
    #[serde(rename = "ConfigurationType")]
    pub configuration_type: Option<u32>,

/// 
    #[serde(rename = "PolicyStore")]
    pub policy_store: Option<String>,

/// 
    #[serde(rename = "Profile")]
    pub profile: Option<String>,

/// 
    #[serde(rename = "ProfileActivated")]
    pub profile_activated: Option<bool>,

/// 
    #[serde(rename = "ServerURL")]
    pub server_url: Option<String>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,

/// 
    #[serde(rename = "StrongCRLRequired")]
    pub strong_crlrequired: Option<bool>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u32>,
}

impl MSFT_NetIPHttpsConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSettingData::new(),
            auth_mode: None,
            configuration_type: None,
            policy_store: None,
            profile: None,
            profile_activated: None,
            server_url: None,
            state: None,
            strong_crlrequired: None,
            type: None,
        }
    }


    /// Sets the value of AuthMode
    pub fn set_auth_mode(&mut self, value: u32) {
        self.auth_mode = Some(value);
    }

    /// Gets the value of AuthMode
    pub fn get_auth_mode(&self) -> Option<&u32> {
        self.auth_mode.as_ref()
    }

    /// Sets the value of ConfigurationType
    pub fn set_configuration_type(&mut self, value: u32) {
        self.configuration_type = Some(value);
    }

    /// Gets the value of ConfigurationType
    pub fn get_configuration_type(&self) -> Option<&u32> {
        self.configuration_type.as_ref()
    }

    /// Sets the value of PolicyStore
    pub fn set_policy_store(&mut self, value: String) {
        self.policy_store = Some(value);
    }

    /// Gets the value of PolicyStore
    pub fn get_policy_store(&self) -> Option<&String> {
        self.policy_store.as_ref()
    }

    /// Sets the value of Profile
    pub fn set_profile(&mut self, value: String) {
        self.profile = Some(value);
    }

    /// Gets the value of Profile
    pub fn get_profile(&self) -> Option<&String> {
        self.profile.as_ref()
    }

    /// Sets the value of ProfileActivated
    pub fn set_profile_activated(&mut self, value: bool) {
        self.profile_activated = Some(value);
    }

    /// Gets the value of ProfileActivated
    pub fn get_profile_activated(&self) -> Option<&bool> {
        self.profile_activated.as_ref()
    }

    /// Sets the value of ServerURL
    pub fn set_server_url(&mut self, value: String) {
        self.server_url = Some(value);
    }

    /// Gets the value of ServerURL
    pub fn get_server_url(&self) -> Option<&String> {
        self.server_url.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u32> {
        self.state.as_ref()
    }

    /// Sets the value of StrongCRLRequired
    pub fn set_strong_crlrequired(&mut self, value: bool) {
        self.strong_crlrequired = Some(value);
    }

    /// Gets the value of StrongCRLRequired
    pub fn get_strong_crlrequired(&self) -> Option<&bool> {
        self.strong_crlrequired.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u32) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u32> {
        self.type.as_ref()
    }

/// 

    /// * `profile` -  (String)

    /// * `return_value` -  (u32)
    pub fn enable_profile(&self, profile: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Profile".to_string(), value: profile.into() });
        self.invoke_method("EnableProfile", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn disable_profile(&self) -> Result<(), WmiError> {
        self.invoke_method("DisableProfile", &[])

    }


/// 

    /// * `application_id` -  (String)
    /// * `certificate_hash` -  (String)
    /// * `certificate_store_name` -  (String)
    /// * `ip_port` -  (String)
    /// * `null_encryption` -  (bool)

    /// * `return_value` -  (u32)
    pub fn add_cert_binding(&self, certificate_hash: &String, application_id: &String, ip_port: &String, certificate_store_name: &String, null_encryption: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CertificateHash".to_string(), value: certificate_hash.into() });
        args.push(MethodParameter { name: "ApplicationId".to_string(), value: application_id.into() });
        args.push(MethodParameter { name: "IpPort".to_string(), value: ip_port.into() });
        args.push(MethodParameter { name: "CertificateStoreName".to_string(), value: certificate_store_name.into() });
        args.push(MethodParameter { name: "NullEncryption".to_string(), value: null_encryption.into() });
        self.invoke_method("AddCertBinding", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn remove_cert_binding(&self) -> Result<(), WmiError> {
        self.invoke_method("RemoveCertBinding", &[])

    }


/// 

    /// * `new_name` -  (String)
    /// * `pass_thru` -  (bool)

    /// * `output_object` -  (MSFT_NetIPHttpsConfiguration)
    /// * `return_value` -  (u32)
    pub fn rename(&self, new_name: &String, pass_thru: bool, output_object: &mut MSFT_NetIPHttpsConfiguration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewName".to_string(), value: new_name.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("Rename", &args)?;
        let output_object = result.get_value("OutputObject")?;
        Ok(result.return_value)

    }


/// 

    /// * `auth_mode` -  (bool)
    /// * `pass_thru` -  (bool)
    /// * `state` -  (bool)
    /// * `strong_crlrequired` -  (bool)

    /// * `output_object` -  (MSFT_NetIPHttpsConfiguration)
    /// * `return_value` -  (u32)
    pub fn reset(&self, state: bool, auth_mode: bool, strong_crlrequired: bool, pass_thru: bool, output_object: &mut MSFT_NetIPHttpsConfiguration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "State".to_string(), value: state.into() });
        args.push(MethodParameter { name: "AuthMode".to_string(), value: auth_mode.into() });
        args.push(MethodParameter { name: "StrongCRLRequired".to_string(), value: strong_crlrequired.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("Reset", &args)?;
        let output_object = result.get_value("OutputObject")?;
        Ok(result.return_value)

    }

}

