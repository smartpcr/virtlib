// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_ClientCertificateInstall_Install03 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_ClientCertificateInstall_Install03 {

/// 
    #[serde(rename = "AADKeyIdentifierList")]
    pub aadkey_identifier_list: Option<String>,

/// 
    #[serde(rename = "CAThumbprint")]
    pub cathumbprint: Option<String>,

/// 
    #[serde(rename = "Challenge")]
    pub challenge: Option<String>,

/// 
    #[serde(rename = "ContainerName")]
    pub container_name: Option<String>,

/// 
    #[serde(rename = "CustomTextToShowInPrompt")]
    pub custom_text_to_show_in_prompt: Option<String>,

/// 
    #[serde(rename = "EKUMapping")]
    pub ekumapping: Option<String>,

/// 
    #[serde(rename = "HashAlgorithm")]
    pub hash_algorithm: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "KeyLength")]
    pub key_length: Option<i32>,

/// 
    #[serde(rename = "KeyProtection")]
    pub key_protection: Option<i32>,

/// 
    #[serde(rename = "KeyUsage")]
    pub key_usage: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "RetryCount")]
    pub retry_count: Option<i32>,

/// 
    #[serde(rename = "RetryDelay")]
    pub retry_delay: Option<i32>,

/// 
    #[serde(rename = "ServerURL")]
    pub server_url: Option<String>,

/// 
    #[serde(rename = "SubjectAlternativeNames")]
    pub subject_alternative_names: Option<String>,

/// 
    #[serde(rename = "SubjectName")]
    pub subject_name: Option<String>,

/// 
    #[serde(rename = "TemplateName")]
    pub template_name: Option<String>,

/// 
    #[serde(rename = "ValidPeriod")]
    pub valid_period: Option<String>,

/// 
    #[serde(rename = "ValidPeriodUnits")]
    pub valid_period_units: Option<i32>,
}

impl MDM_ClientCertificateInstall_Install03 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            aadkey_identifier_list: None,
            cathumbprint: None,
            challenge: None,
            container_name: None,
            custom_text_to_show_in_prompt: None,
            ekumapping: None,
            hash_algorithm: None,
            instance_id: None,
            key_length: None,
            key_protection: None,
            key_usage: None,
            parent_id: None,
            retry_count: None,
            retry_delay: None,
            server_url: None,
            subject_alternative_names: None,
            subject_name: None,
            template_name: None,
            valid_period: None,
            valid_period_units: None,
        }
    }


    /// Sets the value of AADKeyIdentifierList
    pub fn set_aadkey_identifier_list(&mut self, value: String) {
        self.aadkey_identifier_list = Some(value);
    }

    /// Gets the value of AADKeyIdentifierList
    pub fn get_aadkey_identifier_list(&self) -> Option<&String> {
        self.aadkey_identifier_list.as_ref()
    }

    /// Sets the value of CAThumbprint
    pub fn set_cathumbprint(&mut self, value: String) {
        self.cathumbprint = Some(value);
    }

    /// Gets the value of CAThumbprint
    pub fn get_cathumbprint(&self) -> Option<&String> {
        self.cathumbprint.as_ref()
    }

    /// Sets the value of Challenge
    pub fn set_challenge(&mut self, value: String) {
        self.challenge = Some(value);
    }

    /// Gets the value of Challenge
    pub fn get_challenge(&self) -> Option<&String> {
        self.challenge.as_ref()
    }

    /// Sets the value of ContainerName
    pub fn set_container_name(&mut self, value: String) {
        self.container_name = Some(value);
    }

    /// Gets the value of ContainerName
    pub fn get_container_name(&self) -> Option<&String> {
        self.container_name.as_ref()
    }

    /// Sets the value of CustomTextToShowInPrompt
    pub fn set_custom_text_to_show_in_prompt(&mut self, value: String) {
        self.custom_text_to_show_in_prompt = Some(value);
    }

    /// Gets the value of CustomTextToShowInPrompt
    pub fn get_custom_text_to_show_in_prompt(&self) -> Option<&String> {
        self.custom_text_to_show_in_prompt.as_ref()
    }

    /// Sets the value of EKUMapping
    pub fn set_ekumapping(&mut self, value: String) {
        self.ekumapping = Some(value);
    }

    /// Gets the value of EKUMapping
    pub fn get_ekumapping(&self) -> Option<&String> {
        self.ekumapping.as_ref()
    }

    /// Sets the value of HashAlgorithm
    pub fn set_hash_algorithm(&mut self, value: String) {
        self.hash_algorithm = Some(value);
    }

    /// Gets the value of HashAlgorithm
    pub fn get_hash_algorithm(&self) -> Option<&String> {
        self.hash_algorithm.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of KeyLength
    pub fn set_key_length(&mut self, value: i32) {
        self.key_length = Some(value);
    }

    /// Gets the value of KeyLength
    pub fn get_key_length(&self) -> Option<&i32> {
        self.key_length.as_ref()
    }

    /// Sets the value of KeyProtection
    pub fn set_key_protection(&mut self, value: i32) {
        self.key_protection = Some(value);
    }

    /// Gets the value of KeyProtection
    pub fn get_key_protection(&self) -> Option<&i32> {
        self.key_protection.as_ref()
    }

    /// Sets the value of KeyUsage
    pub fn set_key_usage(&mut self, value: i32) {
        self.key_usage = Some(value);
    }

    /// Gets the value of KeyUsage
    pub fn get_key_usage(&self) -> Option<&i32> {
        self.key_usage.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of RetryCount
    pub fn set_retry_count(&mut self, value: i32) {
        self.retry_count = Some(value);
    }

    /// Gets the value of RetryCount
    pub fn get_retry_count(&self) -> Option<&i32> {
        self.retry_count.as_ref()
    }

    /// Sets the value of RetryDelay
    pub fn set_retry_delay(&mut self, value: i32) {
        self.retry_delay = Some(value);
    }

    /// Gets the value of RetryDelay
    pub fn get_retry_delay(&self) -> Option<&i32> {
        self.retry_delay.as_ref()
    }

    /// Sets the value of ServerURL
    pub fn set_server_url(&mut self, value: String) {
        self.server_url = Some(value);
    }

    /// Gets the value of ServerURL
    pub fn get_server_url(&self) -> Option<&String> {
        self.server_url.as_ref()
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

    /// Sets the value of TemplateName
    pub fn set_template_name(&mut self, value: String) {
        self.template_name = Some(value);
    }

    /// Gets the value of TemplateName
    pub fn get_template_name(&self) -> Option<&String> {
        self.template_name.as_ref()
    }

    /// Sets the value of ValidPeriod
    pub fn set_valid_period(&mut self, value: String) {
        self.valid_period = Some(value);
    }

    /// Gets the value of ValidPeriod
    pub fn get_valid_period(&self) -> Option<&String> {
        self.valid_period.as_ref()
    }

    /// Sets the value of ValidPeriodUnits
    pub fn set_valid_period_units(&mut self, value: i32) {
        self.valid_period_units = Some(value);
    }

    /// Gets the value of ValidPeriodUnits
    pub fn get_valid_period_units(&self) -> Option<&i32> {
        self.valid_period_units.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn enroll_method(&self) -> Result<(), WmiError> {
        self.invoke_method("EnrollMethod", &[])

    }

}

