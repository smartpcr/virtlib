// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Hgs
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_HgsGuardian struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_HgsGuardian {

/// 
    #[serde(rename = "EncryptionCertificate")]
    pub encryption_certificate: Vec<u8>,

/// 
    #[serde(rename = "EncryptionCertificateSignature")]
    pub encryption_certificate_signature: Option<String>,

/// 
    #[serde(rename = "EncryptionCertificateSignatureAlgorithm")]
    pub encryption_certificate_signature_algorithm: Option<String>,

/// 
    #[serde(rename = "GuardianMetadataVersion")]
    pub guardian_metadata_version: Option<u32>,

/// 
    #[serde(rename = "HasPrivateSigningKey")]
    pub has_private_signing_key: Option<bool>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "SigningCertificate")]
    pub signing_certificate: Vec<u8>,
}

impl MSFT_HgsGuardian {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            encryption_certificate: Vec::new(),
            encryption_certificate_signature: None,
            encryption_certificate_signature_algorithm: None,
            guardian_metadata_version: None,
            has_private_signing_key: None,
            name: None,
            signing_certificate: Vec::new(),
        }
    }


    /// Sets the value of EncryptionCertificate
    pub fn set_encryption_certificate(&mut self, value: Vec<u8>) {
        self.encryption_certificate = value;
    }

    /// Gets the value of EncryptionCertificate
    pub fn get_encryption_certificate(&self) -> &Vec<u8> {
        &self.encryption_certificate
    }

    /// Sets the value of EncryptionCertificateSignature
    pub fn set_encryption_certificate_signature(&mut self, value: String) {
        self.encryption_certificate_signature = Some(value);
    }

    /// Gets the value of EncryptionCertificateSignature
    pub fn get_encryption_certificate_signature(&self) -> Option<&String> {
        self.encryption_certificate_signature.as_ref()
    }

    /// Sets the value of EncryptionCertificateSignatureAlgorithm
    pub fn set_encryption_certificate_signature_algorithm(&mut self, value: String) {
        self.encryption_certificate_signature_algorithm = Some(value);
    }

    /// Gets the value of EncryptionCertificateSignatureAlgorithm
    pub fn get_encryption_certificate_signature_algorithm(&self) -> Option<&String> {
        self.encryption_certificate_signature_algorithm.as_ref()
    }

    /// Sets the value of GuardianMetadataVersion
    pub fn set_guardian_metadata_version(&mut self, value: u32) {
        self.guardian_metadata_version = Some(value);
    }

    /// Gets the value of GuardianMetadataVersion
    pub fn get_guardian_metadata_version(&self) -> Option<&u32> {
        self.guardian_metadata_version.as_ref()
    }

    /// Sets the value of HasPrivateSigningKey
    pub fn set_has_private_signing_key(&mut self, value: bool) {
        self.has_private_signing_key = Some(value);
    }

    /// Gets the value of HasPrivateSigningKey
    pub fn get_has_private_signing_key(&self) -> Option<&bool> {
        self.has_private_signing_key.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of SigningCertificate
    pub fn set_signing_certificate(&mut self, value: Vec<u8>) {
        self.signing_certificate = value;
    }

    /// Gets the value of SigningCertificate
    pub fn get_signing_certificate(&self) -> &Vec<u8> {
        &self.signing_certificate
    }

/// 

    /// * `allow_expired` -  (bool)
    /// * `allow_untrusted_root` -  (bool)
    /// * `encryption_certificate` -  (String)
    /// * `encryption_certificate_password` -  (String)
    /// * `name` -  (String)
    /// * `signing_certificate` -  (String)
    /// * `signing_certificate_password` -  (String)

    /// * `cmdlet_output` -  (MSFT_HgsGuardian)
    /// * `return_value` -  (u32)
    pub fn new_by_accept_certificates(&self, name: &String, encryption_certificate: &String, signing_certificate: &String, signing_certificate_password: &String, encryption_certificate_password: &String, allow_expired: bool, allow_untrusted_root: bool, cmdlet_output: &mut MSFT_HgsGuardian) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "EncryptionCertificate".to_string(), value: encryption_certificate.into() });
        args.push(MethodParameter { name: "SigningCertificate".to_string(), value: signing_certificate.into() });
        args.push(MethodParameter { name: "SigningCertificatePassword".to_string(), value: signing_certificate_password.into() });
        args.push(MethodParameter { name: "EncryptionCertificatePassword".to_string(), value: encryption_certificate_password.into() });
        args.push(MethodParameter { name: "AllowExpired".to_string(), value: allow_expired.into() });
        args.push(MethodParameter { name: "AllowUntrustedRoot".to_string(), value: allow_untrusted_root.into() });

        let result = self.invoke_method("NewByAcceptCertificates", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `allow_expired` -  (bool)
    /// * `allow_untrusted_root` -  (bool)
    /// * `encryption_certificate_thumbprint` -  (String)
    /// * `name` -  (String)
    /// * `signing_certificate_thumbprint` -  (String)

    /// * `cmdlet_output` -  (MSFT_HgsGuardian)
    /// * `return_value` -  (u32)
    pub fn new_by_certificate_thumbprints(&self, name: &String, signing_certificate_thumbprint: &String, encryption_certificate_thumbprint: &String, allow_expired: bool, allow_untrusted_root: bool, cmdlet_output: &mut MSFT_HgsGuardian) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "SigningCertificateThumbprint".to_string(), value: signing_certificate_thumbprint.into() });
        args.push(MethodParameter { name: "EncryptionCertificateThumbprint".to_string(), value: encryption_certificate_thumbprint.into() });
        args.push(MethodParameter { name: "AllowExpired".to_string(), value: allow_expired.into() });
        args.push(MethodParameter { name: "AllowUntrustedRoot".to_string(), value: allow_untrusted_root.into() });

        let result = self.invoke_method("NewByCertificateThumbprints", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `generate_certificates` -  (bool)
    /// * `name` -  (String)

    /// * `cmdlet_output` -  (MSFT_HgsGuardian)
    /// * `return_value` -  (u32)
    pub fn new_by_generate_certificates(&self, name: &String, generate_certificates: bool, cmdlet_output: &mut MSFT_HgsGuardian) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "GenerateCertificates".to_string(), value: generate_certificates.into() });

        let result = self.invoke_method("NewByGenerateCertificates", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `allow_expired` -  (bool)
    /// * `allow_untrusted_root` -  (bool)
    /// * `name` -  (String)
    /// * `path` -  (String)

    /// * `cmdlet_output` -  (MSFT_HgsGuardian)
    /// * `return_value` -  (u32)
    pub fn import(&self, path: &String, name: &String, allow_expired: bool, allow_untrusted_root: bool, cmdlet_output: &mut MSFT_HgsGuardian) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "AllowExpired".to_string(), value: allow_expired.into() });
        args.push(MethodParameter { name: "AllowUntrustedRoot".to_string(), value: allow_untrusted_root.into() });

        let result = self.invoke_method("Import", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `input_object` -  (MSFT_HgsGuardian)
    /// * `path` -  (String)

    /// * `return_value` -  (u32)
    pub fn export(&self, path: &String, input_object: MSFT_HgsGuardian) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });
        self.invoke_method("Export", &args)

    }


/// 

    /// * `name` -  (String)

    /// * `return_value` -  (u32)
    pub fn remove(&self, name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        self.invoke_method("Remove", &args)

    }

}

