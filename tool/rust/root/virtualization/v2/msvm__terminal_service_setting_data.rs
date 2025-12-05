// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_TerminalServiceSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_TerminalServiceSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "AllowedHashAlgorithms")]
    pub allowed_hash_algorithms: Vec<String>,

/// 
    #[serde(rename = "AuthCertificateHash")]
    pub auth_certificate_hash: Vec<u8>,

/// 
    #[serde(rename = "DisableSelfSignedCertificateGeneration")]
    pub disable_self_signed_certificate_generation: Option<bool>,

/// 
    #[serde(rename = "ListenerPort")]
    pub listener_port: Option<u32>,

/// 
    #[serde(rename = "TrustedIssuerCertificateHashes")]
    pub trusted_issuer_certificate_hashes: Vec<String>,
}

impl Msvm_TerminalServiceSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            allowed_hash_algorithms: Vec::new(),
            auth_certificate_hash: Vec::new(),
            disable_self_signed_certificate_generation: None,
            listener_port: None,
            trusted_issuer_certificate_hashes: Vec::new(),
        }
    }


    /// Sets the value of AllowedHashAlgorithms
    pub fn set_allowed_hash_algorithms(&mut self, value: Vec<String>) {
        self.allowed_hash_algorithms = value;
    }

    /// Gets the value of AllowedHashAlgorithms
    pub fn get_allowed_hash_algorithms(&self) -> &Vec<String> {
        &self.allowed_hash_algorithms
    }

    /// Sets the value of AuthCertificateHash
    pub fn set_auth_certificate_hash(&mut self, value: Vec<u8>) {
        self.auth_certificate_hash = value;
    }

    /// Gets the value of AuthCertificateHash
    pub fn get_auth_certificate_hash(&self) -> &Vec<u8> {
        &self.auth_certificate_hash
    }

    /// Sets the value of DisableSelfSignedCertificateGeneration
    pub fn set_disable_self_signed_certificate_generation(&mut self, value: bool) {
        self.disable_self_signed_certificate_generation = Some(value);
    }

    /// Gets the value of DisableSelfSignedCertificateGeneration
    pub fn get_disable_self_signed_certificate_generation(&self) -> Option<&bool> {
        self.disable_self_signed_certificate_generation.as_ref()
    }

    /// Sets the value of ListenerPort
    pub fn set_listener_port(&mut self, value: u32) {
        self.listener_port = Some(value);
    }

    /// Gets the value of ListenerPort
    pub fn get_listener_port(&self) -> Option<&u32> {
        self.listener_port.as_ref()
    }

    /// Sets the value of TrustedIssuerCertificateHashes
    pub fn set_trusted_issuer_certificate_hashes(&mut self, value: Vec<String>) {
        self.trusted_issuer_certificate_hashes = value;
    }

    /// Gets the value of TrustedIssuerCertificateHashes
    pub fn get_trusted_issuer_certificate_hashes(&self) -> &Vec<String> {
        &self.trusted_issuer_certificate_hashes
    }
}

impl Msvm_TerminalServiceSettingData {
    /// Gets the related Msvm_TerminalService object(s)
    pub fn get_related__terminal_service(&self) -> Result<Msvm_TerminalService, WmiError> {
        self.get_related("Msvm_TerminalService")
    }

}

