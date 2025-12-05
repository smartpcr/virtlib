// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Hgs
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_HgsClientConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_HgsClientConfiguration {

/// 
    #[serde(rename = "AttestationOperationMode")]
    pub attestation_operation_mode: Option<u16>,

/// 
    #[serde(rename = "AttestationServerUrl")]
    pub attestation_server_url: Option<String>,

/// 
    #[serde(rename = "AttestationStatus")]
    pub attestation_status: Option<u16>,

/// 
    #[serde(rename = "AttestationSubstatus")]
    pub attestation_substatus: Option<u64>,

/// 
    #[serde(rename = "FallbackAttestationServerUrl")]
    pub fallback_attestation_server_url: Vec<String>,

/// 
    #[serde(rename = "FallbackKeyProtectionServerUrl")]
    pub fallback_key_protection_server_url: Vec<String>,

/// 
    #[serde(rename = "IsHostGuarded")]
    pub is_host_guarded: Option<bool>,

/// 
    #[serde(rename = "KeyProtectionServerUrl")]
    pub key_protection_server_url: Option<String>,

/// 
    #[serde(rename = "LastAttestationServerUrl")]
    pub last_attestation_server_url: Option<String>,

/// 
    #[serde(rename = "LastKeyProtectionServerUrl")]
    pub last_key_protection_server_url: Option<String>,

/// 
    #[serde(rename = "Mode")]
    pub mode: Option<u16>,
}

impl MSFT_HgsClientConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            attestation_operation_mode: None,
            attestation_server_url: None,
            attestation_status: None,
            attestation_substatus: None,
            fallback_attestation_server_url: Vec::new(),
            fallback_key_protection_server_url: Vec::new(),
            is_host_guarded: None,
            key_protection_server_url: None,
            last_attestation_server_url: None,
            last_key_protection_server_url: None,
            mode: None,
        }
    }


    /// Sets the value of AttestationOperationMode
    pub fn set_attestation_operation_mode(&mut self, value: u16) {
        self.attestation_operation_mode = Some(value);
    }

    /// Gets the value of AttestationOperationMode
    pub fn get_attestation_operation_mode(&self) -> Option<&u16> {
        self.attestation_operation_mode.as_ref()
    }

    /// Sets the value of AttestationServerUrl
    pub fn set_attestation_server_url(&mut self, value: String) {
        self.attestation_server_url = Some(value);
    }

    /// Gets the value of AttestationServerUrl
    pub fn get_attestation_server_url(&self) -> Option<&String> {
        self.attestation_server_url.as_ref()
    }

    /// Sets the value of AttestationStatus
    pub fn set_attestation_status(&mut self, value: u16) {
        self.attestation_status = Some(value);
    }

    /// Gets the value of AttestationStatus
    pub fn get_attestation_status(&self) -> Option<&u16> {
        self.attestation_status.as_ref()
    }

    /// Sets the value of AttestationSubstatus
    pub fn set_attestation_substatus(&mut self, value: u64) {
        self.attestation_substatus = Some(value);
    }

    /// Gets the value of AttestationSubstatus
    pub fn get_attestation_substatus(&self) -> Option<&u64> {
        self.attestation_substatus.as_ref()
    }

    /// Sets the value of FallbackAttestationServerUrl
    pub fn set_fallback_attestation_server_url(&mut self, value: Vec<String>) {
        self.fallback_attestation_server_url = value;
    }

    /// Gets the value of FallbackAttestationServerUrl
    pub fn get_fallback_attestation_server_url(&self) -> &Vec<String> {
        &self.fallback_attestation_server_url
    }

    /// Sets the value of FallbackKeyProtectionServerUrl
    pub fn set_fallback_key_protection_server_url(&mut self, value: Vec<String>) {
        self.fallback_key_protection_server_url = value;
    }

    /// Gets the value of FallbackKeyProtectionServerUrl
    pub fn get_fallback_key_protection_server_url(&self) -> &Vec<String> {
        &self.fallback_key_protection_server_url
    }

    /// Sets the value of IsHostGuarded
    pub fn set_is_host_guarded(&mut self, value: bool) {
        self.is_host_guarded = Some(value);
    }

    /// Gets the value of IsHostGuarded
    pub fn get_is_host_guarded(&self) -> Option<&bool> {
        self.is_host_guarded.as_ref()
    }

    /// Sets the value of KeyProtectionServerUrl
    pub fn set_key_protection_server_url(&mut self, value: String) {
        self.key_protection_server_url = Some(value);
    }

    /// Gets the value of KeyProtectionServerUrl
    pub fn get_key_protection_server_url(&self) -> Option<&String> {
        self.key_protection_server_url.as_ref()
    }

    /// Sets the value of LastAttestationServerUrl
    pub fn set_last_attestation_server_url(&mut self, value: String) {
        self.last_attestation_server_url = Some(value);
    }

    /// Gets the value of LastAttestationServerUrl
    pub fn get_last_attestation_server_url(&self) -> Option<&String> {
        self.last_attestation_server_url.as_ref()
    }

    /// Sets the value of LastKeyProtectionServerUrl
    pub fn set_last_key_protection_server_url(&mut self, value: String) {
        self.last_key_protection_server_url = Some(value);
    }

    /// Gets the value of LastKeyProtectionServerUrl
    pub fn get_last_key_protection_server_url(&self) -> Option<&String> {
        self.last_key_protection_server_url.as_ref()
    }

    /// Sets the value of Mode
    pub fn set_mode(&mut self, value: u16) {
        self.mode = Some(value);
    }

    /// Gets the value of Mode
    pub fn get_mode(&self) -> Option<&u16> {
        self.mode.as_ref()
    }

/// 

    /// * `cmdlet_output` -  (MSFT_HgsClientConfiguration)
    /// * `return_value` -  (u32)
    pub fn get(&self, cmdlet_output: &mut MSFT_HgsClientConfiguration) -> Result<(), WmiError> {

        let result = self.invoke_method("Get", &[])?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `attestation_server_url` -  (String)

    /// * `attestation_operation_mode` -  (u16)
    /// * `attestation_status` -  (u16)
    /// * `attestation_substatus` -  (u64)
    /// * `is_host_guarded` -  (bool)
    /// * `return_value` -  (u32)
    pub fn is_host_trusted(&self, attestation_server_url: &String, is_host_guarded: &mut bool, attestation_operation_mode: &mut u16, attestation_status: &mut u16, attestation_substatus: &mut u64) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AttestationServerUrl".to_string(), value: attestation_server_url.into() });

        let result = self.invoke_method("IsHostTrusted", &args)?;
        let attestation_operation_mode = result.get_value("AttestationOperationMode")?;
        let attestation_status = result.get_value("AttestationStatus")?;
        let attestation_substatus = result.get_value("AttestationSubstatus")?;
        let is_host_guarded = result.get_value("IsHostGuarded")?;
        Ok(result.return_value)

    }


/// 

    /// * `enable_local_mode` -  (bool)

    /// * `cmdlet_output` -  (MSFT_HgsClientConfiguration)
    /// * `return_value` -  (u32)
    pub fn set_by_change_to_local_mode(&self, enable_local_mode: bool, cmdlet_output: &mut MSFT_HgsClientConfiguration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "EnableLocalMode".to_string(), value: enable_local_mode.into() });

        let result = self.invoke_method("SetByChangeToLocalMode", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `attestation_server_url` -  (String)
    /// * `fallback_attestation_server_url` -  (String[])
    /// * `fallback_key_protection_server_url` -  (String[])
    /// * `key_protection_server_url` -  (String)

    /// * `cmdlet_output` -  (MSFT_HgsClientConfiguration)
    /// * `return_value` -  (u32)
    pub fn set_by_secure_hosting_service_mode(&self, key_protection_server_url: &String, attestation_server_url: &String, cmdlet_output: &mut MSFT_HgsClientConfiguration, fallback_key_protection_server_url: &Option<Vec<String>>, fallback_attestation_server_url: &Option<Vec<String>>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "KeyProtectionServerUrl".to_string(), value: key_protection_server_url.into() });
        args.push(MethodParameter { name: "AttestationServerUrl".to_string(), value: attestation_server_url.into() });
        if let Some(val) = fallback_key_protection_server_url {
            args.push(MethodParameter { name: "FallbackKeyProtectionServerUrl".to_string(), value: val.into() });
        }
        if let Some(val) = fallback_attestation_server_url {
            args.push(MethodParameter { name: "FallbackAttestationServerUrl".to_string(), value: val.into() });
        }

        let result = self.invoke_method("SetBySecureHostingServiceMode", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

