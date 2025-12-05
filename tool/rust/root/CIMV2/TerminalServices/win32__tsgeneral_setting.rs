// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TSGeneralSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TSGeneralSetting {
    #[serde(flatten)]
    pub base: Win32_TerminalSetting,

/// 
    #[serde(rename = "CertificateName")]
    pub certificate_name: Option<String>,

/// 
    #[serde(rename = "Certificates")]
    pub certificates: Vec<u8>,

/// 
    #[serde(rename = "Comment")]
    pub comment: Option<String>,

/// 
    #[serde(rename = "MinEncryptionLevel")]
    pub min_encryption_level: Option<u32>,

/// 
    #[serde(rename = "PolicySourceMinEncryptionLevel")]
    pub policy_source_min_encryption_level: Option<u32>,

/// 
    #[serde(rename = "PolicySourceSecurityLayer")]
    pub policy_source_security_layer: Option<u32>,

/// 
    #[serde(rename = "PolicySourceUserAuthenticationRequired")]
    pub policy_source_user_authentication_required: Option<u32>,

/// 
    #[serde(rename = "SecurityLayer")]
    pub security_layer: Option<u32>,

/// 
    #[serde(rename = "SSLCertificateSHA1Hash")]
    pub sslcertificate_sha1_hash: Option<String>,

/// 
    #[serde(rename = "SSLCertificateSHA1HashType")]
    pub sslcertificate_sha1_hash_type: Option<u32>,

/// 
    #[serde(rename = "TerminalProtocol")]
    pub terminal_protocol: Option<String>,

/// 
    #[serde(rename = "Transport")]
    pub transport: Option<String>,

/// 
    #[serde(rename = "UserAuthenticationRequired")]
    pub user_authentication_required: Option<u32>,

/// 
    #[serde(rename = "WindowsAuthentication")]
    pub windows_authentication: Option<u32>,
}

impl Win32_TSGeneralSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_TerminalSetting::new(),
            certificate_name: None,
            certificates: Vec::new(),
            comment: None,
            min_encryption_level: None,
            policy_source_min_encryption_level: None,
            policy_source_security_layer: None,
            policy_source_user_authentication_required: None,
            security_layer: None,
            sslcertificate_sha1_hash: None,
            sslcertificate_sha1_hash_type: None,
            terminal_protocol: None,
            transport: None,
            user_authentication_required: None,
            windows_authentication: None,
        }
    }


    /// Sets the value of CertificateName
    pub fn set_certificate_name(&mut self, value: String) {
        self.certificate_name = Some(value);
    }

    /// Gets the value of CertificateName
    pub fn get_certificate_name(&self) -> Option<&String> {
        self.certificate_name.as_ref()
    }

    /// Sets the value of Certificates
    pub fn set_certificates(&mut self, value: Vec<u8>) {
        self.certificates = value;
    }

    /// Gets the value of Certificates
    pub fn get_certificates(&self) -> &Vec<u8> {
        &self.certificates
    }

    /// Sets the value of Comment
    pub fn set_comment(&mut self, value: String) {
        self.comment = Some(value);
    }

    /// Gets the value of Comment
    pub fn get_comment(&self) -> Option<&String> {
        self.comment.as_ref()
    }

    /// Sets the value of MinEncryptionLevel
    pub fn set_min_encryption_level(&mut self, value: u32) {
        self.min_encryption_level = Some(value);
    }

    /// Gets the value of MinEncryptionLevel
    pub fn get_min_encryption_level(&self) -> Option<&u32> {
        self.min_encryption_level.as_ref()
    }

    /// Sets the value of PolicySourceMinEncryptionLevel
    pub fn set_policy_source_min_encryption_level(&mut self, value: u32) {
        self.policy_source_min_encryption_level = Some(value);
    }

    /// Gets the value of PolicySourceMinEncryptionLevel
    pub fn get_policy_source_min_encryption_level(&self) -> Option<&u32> {
        self.policy_source_min_encryption_level.as_ref()
    }

    /// Sets the value of PolicySourceSecurityLayer
    pub fn set_policy_source_security_layer(&mut self, value: u32) {
        self.policy_source_security_layer = Some(value);
    }

    /// Gets the value of PolicySourceSecurityLayer
    pub fn get_policy_source_security_layer(&self) -> Option<&u32> {
        self.policy_source_security_layer.as_ref()
    }

    /// Sets the value of PolicySourceUserAuthenticationRequired
    pub fn set_policy_source_user_authentication_required(&mut self, value: u32) {
        self.policy_source_user_authentication_required = Some(value);
    }

    /// Gets the value of PolicySourceUserAuthenticationRequired
    pub fn get_policy_source_user_authentication_required(&self) -> Option<&u32> {
        self.policy_source_user_authentication_required.as_ref()
    }

    /// Sets the value of SecurityLayer
    pub fn set_security_layer(&mut self, value: u32) {
        self.security_layer = Some(value);
    }

    /// Gets the value of SecurityLayer
    pub fn get_security_layer(&self) -> Option<&u32> {
        self.security_layer.as_ref()
    }

    /// Sets the value of SSLCertificateSHA1Hash
    pub fn set_sslcertificate_sha1_hash(&mut self, value: String) {
        self.sslcertificate_sha1_hash = Some(value);
    }

    /// Gets the value of SSLCertificateSHA1Hash
    pub fn get_sslcertificate_sha1_hash(&self) -> Option<&String> {
        self.sslcertificate_sha1_hash.as_ref()
    }

    /// Sets the value of SSLCertificateSHA1HashType
    pub fn set_sslcertificate_sha1_hash_type(&mut self, value: u32) {
        self.sslcertificate_sha1_hash_type = Some(value);
    }

    /// Gets the value of SSLCertificateSHA1HashType
    pub fn get_sslcertificate_sha1_hash_type(&self) -> Option<&u32> {
        self.sslcertificate_sha1_hash_type.as_ref()
    }

    /// Sets the value of TerminalProtocol
    pub fn set_terminal_protocol(&mut self, value: String) {
        self.terminal_protocol = Some(value);
    }

    /// Gets the value of TerminalProtocol
    pub fn get_terminal_protocol(&self) -> Option<&String> {
        self.terminal_protocol.as_ref()
    }

    /// Sets the value of Transport
    pub fn set_transport(&mut self, value: String) {
        self.transport = Some(value);
    }

    /// Gets the value of Transport
    pub fn get_transport(&self) -> Option<&String> {
        self.transport.as_ref()
    }

    /// Sets the value of UserAuthenticationRequired
    pub fn set_user_authentication_required(&mut self, value: u32) {
        self.user_authentication_required = Some(value);
    }

    /// Gets the value of UserAuthenticationRequired
    pub fn get_user_authentication_required(&self) -> Option<&u32> {
        self.user_authentication_required.as_ref()
    }

    /// Sets the value of WindowsAuthentication
    pub fn set_windows_authentication(&mut self, value: u32) {
        self.windows_authentication = Some(value);
    }

    /// Gets the value of WindowsAuthentication
    pub fn get_windows_authentication(&self) -> Option<&u32> {
        self.windows_authentication.as_ref()
    }

/// 

    /// * `min_encryption_level` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_encryption_level(&self, min_encryption_level: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "MinEncryptionLevel".to_string(), value: min_encryption_level.into() });
        self.invoke_method("SetEncryptionLevel", &args)

    }


/// 

    /// * `security_layer` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_security_layer(&self, security_layer: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SecurityLayer".to_string(), value: security_layer.into() });
        self.invoke_method("SetSecurityLayer", &args)

    }


/// 

    /// * `user_authentication_required` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_user_authentication_required(&self, user_authentication_required: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "UserAuthenticationRequired".to_string(), value: user_authentication_required.into() });
        self.invoke_method("SetUserAuthenticationRequired", &args)

    }

}

