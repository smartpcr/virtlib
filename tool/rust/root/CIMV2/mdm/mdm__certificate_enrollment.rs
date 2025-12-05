// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_CertificateEnrollment struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_CertificateEnrollment {

/// 
    #[serde(rename = "ConfigurationParameters")]
    pub configuration_parameters: Option<String>,

/// 
    #[serde(rename = "EnhancedKeyUsages")]
    pub enhanced_key_usages: Option<String>,

/// 
    #[serde(rename = "Error")]
    pub error: Option<u32>,

/// 
    #[serde(rename = "ExpirationThreshold")]
    pub expiration_threshold: Option<u32>,

/// 
    #[serde(rename = "Issuers")]
    pub issuers: Option<String>,

/// 
    #[serde(rename = "RequestID")]
    pub request_id: Option<String>,

/// 
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<u32>,

/// 
    #[serde(rename = "StoreLocation")]
    pub store_location: Option<u8>,

/// 
    #[serde(rename = "SubjectAlternativeNames")]
    pub subject_alternative_names: Option<String>,

/// 
    #[serde(rename = "SubjectName")]
    pub subject_name: Option<String>,

/// 
    #[serde(rename = "Thumbprint")]
    pub thumbprint: Option<String>,

/// 
    #[serde(rename = "ValidFrom")]
    pub valid_from: Option<String>,

/// 
    #[serde(rename = "ValidTo")]
    pub valid_to: Option<String>,
}

impl MDM_CertificateEnrollment {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            configuration_parameters: None,
            enhanced_key_usages: None,
            error: None,
            expiration_threshold: None,
            issuers: None,
            request_id: None,
            serial_number: None,
            status: None,
            store_location: None,
            subject_alternative_names: None,
            subject_name: None,
            thumbprint: None,
            valid_from: None,
            valid_to: None,
        }
    }


    /// Sets the value of ConfigurationParameters
    pub fn set_configuration_parameters(&mut self, value: String) {
        self.configuration_parameters = Some(value);
    }

    /// Gets the value of ConfigurationParameters
    pub fn get_configuration_parameters(&self) -> Option<&String> {
        self.configuration_parameters.as_ref()
    }

    /// Sets the value of EnhancedKeyUsages
    pub fn set_enhanced_key_usages(&mut self, value: String) {
        self.enhanced_key_usages = Some(value);
    }

    /// Gets the value of EnhancedKeyUsages
    pub fn get_enhanced_key_usages(&self) -> Option<&String> {
        self.enhanced_key_usages.as_ref()
    }

    /// Sets the value of Error
    pub fn set_error(&mut self, value: u32) {
        self.error = Some(value);
    }

    /// Gets the value of Error
    pub fn get_error(&self) -> Option<&u32> {
        self.error.as_ref()
    }

    /// Sets the value of ExpirationThreshold
    pub fn set_expiration_threshold(&mut self, value: u32) {
        self.expiration_threshold = Some(value);
    }

    /// Gets the value of ExpirationThreshold
    pub fn get_expiration_threshold(&self) -> Option<&u32> {
        self.expiration_threshold.as_ref()
    }

    /// Sets the value of Issuers
    pub fn set_issuers(&mut self, value: String) {
        self.issuers = Some(value);
    }

    /// Gets the value of Issuers
    pub fn get_issuers(&self) -> Option<&String> {
        self.issuers.as_ref()
    }

    /// Sets the value of RequestID
    pub fn set_request_id(&mut self, value: String) {
        self.request_id = Some(value);
    }

    /// Gets the value of RequestID
    pub fn get_request_id(&self) -> Option<&String> {
        self.request_id.as_ref()
    }

    /// Sets the value of SerialNumber
    pub fn set_serial_number(&mut self, value: String) {
        self.serial_number = Some(value);
    }

    /// Gets the value of SerialNumber
    pub fn get_serial_number(&self) -> Option<&String> {
        self.serial_number.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u32> {
        self.status.as_ref()
    }

    /// Sets the value of StoreLocation
    pub fn set_store_location(&mut self, value: u8) {
        self.store_location = Some(value);
    }

    /// Gets the value of StoreLocation
    pub fn get_store_location(&self) -> Option<&u8> {
        self.store_location.as_ref()
    }

    /// Sets the value of SubjectAlternativeNames
    pub fn set_subject_alternative_names(&mut self, value: String) {
        self.subject_alternative_names = Some(value);
    }

    /// Gets the value of SubjectAlternativeNames
    pub fn get_subject_alternative_names(&self) -> Option<&String> {
        self.subject_alternative_names.as_ref()
    }

    /// Sets the value of SubjectName
    pub fn set_subject_name(&mut self, value: String) {
        self.subject_name = Some(value);
    }

    /// Gets the value of SubjectName
    pub fn get_subject_name(&self) -> Option<&String> {
        self.subject_name.as_ref()
    }

    /// Sets the value of Thumbprint
    pub fn set_thumbprint(&mut self, value: String) {
        self.thumbprint = Some(value);
    }

    /// Gets the value of Thumbprint
    pub fn get_thumbprint(&self) -> Option<&String> {
        self.thumbprint.as_ref()
    }

    /// Sets the value of ValidFrom
    pub fn set_valid_from(&mut self, value: String) {
        self.valid_from = Some(value);
    }

    /// Gets the value of ValidFrom
    pub fn get_valid_from(&self) -> Option<&String> {
        self.valid_from.as_ref()
    }

    /// Sets the value of ValidTo
    pub fn set_valid_to(&mut self, value: String) {
        self.valid_to = Some(value);
    }

    /// Gets the value of ValidTo
    pub fn get_valid_to(&self) -> Option<&String> {
        self.valid_to.as_ref()
    }
}

