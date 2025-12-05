// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TSLogonSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TSLogonSetting {
    #[serde(flatten)]
    pub base: Win32_TerminalSetting,

/// 
    #[serde(rename = "ClientLogonInfoPolicy")]
    pub client_logon_info_policy: Option<u32>,

/// 
    #[serde(rename = "Domain")]
    pub domain: Option<String>,

/// 
    #[serde(rename = "PolicySourceDomain")]
    pub policy_source_domain: Option<u32>,

/// 
    #[serde(rename = "PolicySourcePromptForPassword")]
    pub policy_source_prompt_for_password: Option<u32>,

/// 
    #[serde(rename = "PolicySourceUserName")]
    pub policy_source_user_name: Option<u32>,

/// 
    #[serde(rename = "PromptForPassword")]
    pub prompt_for_password: Option<u32>,

/// 
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,
}

impl Win32_TSLogonSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_TerminalSetting::new(),
            client_logon_info_policy: None,
            domain: None,
            policy_source_domain: None,
            policy_source_prompt_for_password: None,
            policy_source_user_name: None,
            prompt_for_password: None,
            user_name: None,
        }
    }


    /// Sets the value of ClientLogonInfoPolicy
    pub fn set_client_logon_info_policy(&mut self, value: u32) {
        self.client_logon_info_policy = Some(value);
    }

    /// Gets the value of ClientLogonInfoPolicy
    pub fn get_client_logon_info_policy(&self) -> Option<&u32> {
        self.client_logon_info_policy.as_ref()
    }

    /// Sets the value of Domain
    pub fn set_domain(&mut self, value: String) {
        self.domain = Some(value);
    }

    /// Gets the value of Domain
    pub fn get_domain(&self) -> Option<&String> {
        self.domain.as_ref()
    }

    /// Sets the value of PolicySourceDomain
    pub fn set_policy_source_domain(&mut self, value: u32) {
        self.policy_source_domain = Some(value);
    }

    /// Gets the value of PolicySourceDomain
    pub fn get_policy_source_domain(&self) -> Option<&u32> {
        self.policy_source_domain.as_ref()
    }

    /// Sets the value of PolicySourcePromptForPassword
    pub fn set_policy_source_prompt_for_password(&mut self, value: u32) {
        self.policy_source_prompt_for_password = Some(value);
    }

    /// Gets the value of PolicySourcePromptForPassword
    pub fn get_policy_source_prompt_for_password(&self) -> Option<&u32> {
        self.policy_source_prompt_for_password.as_ref()
    }

    /// Sets the value of PolicySourceUserName
    pub fn set_policy_source_user_name(&mut self, value: u32) {
        self.policy_source_user_name = Some(value);
    }

    /// Gets the value of PolicySourceUserName
    pub fn get_policy_source_user_name(&self) -> Option<&u32> {
        self.policy_source_user_name.as_ref()
    }

    /// Sets the value of PromptForPassword
    pub fn set_prompt_for_password(&mut self, value: u32) {
        self.prompt_for_password = Some(value);
    }

    /// Gets the value of PromptForPassword
    pub fn get_prompt_for_password(&self) -> Option<&u32> {
        self.prompt_for_password.as_ref()
    }

    /// Sets the value of UserName
    pub fn set_user_name(&mut self, value: String) {
        self.user_name = Some(value);
    }

    /// Gets the value of UserName
    pub fn get_user_name(&self) -> Option<&String> {
        self.user_name.as_ref()
    }

/// 

    /// * `domain` -  (String)
    /// * `password` -  (String)
    /// * `user_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn explicit_logon(&self, user_name: &String, domain: &String, password: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "UserName".to_string(), value: user_name.into() });
        args.push(MethodParameter { name: "Domain".to_string(), value: domain.into() });
        args.push(MethodParameter { name: "Password".to_string(), value: password.into() });
        self.invoke_method("ExplicitLogon", &args)

    }


/// 

    /// * `prompt_for_password` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_prompt_for_password(&self, prompt_for_password: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PromptForPassword".to_string(), value: prompt_for_password.into() });
        self.invoke_method("SetPromptForPassword", &args)

    }

}

