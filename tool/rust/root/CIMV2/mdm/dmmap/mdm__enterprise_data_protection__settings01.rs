// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_EnterpriseDataProtection_Settings01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_EnterpriseDataProtection_Settings01 {

/// 
    #[serde(rename = "AllowAzureRMSForEDP")]
    pub allow_azure_rmsfor_edp: Option<i32>,

/// 
    #[serde(rename = "AllowUserDecryption")]
    pub allow_user_decryption: Option<i32>,

/// 
    #[serde(rename = "DataRecoveryCertificate")]
    pub data_recovery_certificate: Option<String>,

/// 
    #[serde(rename = "EDPEnforcementLevel")]
    pub edpenforcement_level: Option<i32>,

/// 
    #[serde(rename = "EDPShowIcons")]
    pub edpshow_icons: Option<i32>,

/// 
    #[serde(rename = "EnterpriseProtectedDomainNames")]
    pub enterprise_protected_domain_names: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "RevokeOnUnenroll")]
    pub revoke_on_unenroll: Option<i32>,

/// 
    #[serde(rename = "RMSTemplateIDForEDP")]
    pub rmstemplate_idfor_edp: Option<String>,

/// 
    #[serde(rename = "SMBAutoEncryptedFileExtensions")]
    pub smbauto_encrypted_file_extensions: Option<String>,
}

impl MDM_EnterpriseDataProtection_Settings01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_azure_rmsfor_edp: None,
            allow_user_decryption: None,
            data_recovery_certificate: None,
            edpenforcement_level: None,
            edpshow_icons: None,
            enterprise_protected_domain_names: None,
            instance_id: None,
            parent_id: None,
            revoke_on_unenroll: None,
            rmstemplate_idfor_edp: None,
            smbauto_encrypted_file_extensions: None,
        }
    }


    /// Sets the value of AllowAzureRMSForEDP
    pub fn set_allow_azure_rmsfor_edp(&mut self, value: i32) {
        self.allow_azure_rmsfor_edp = Some(value);
    }

    /// Gets the value of AllowAzureRMSForEDP
    pub fn get_allow_azure_rmsfor_edp(&self) -> Option<&i32> {
        self.allow_azure_rmsfor_edp.as_ref()
    }

    /// Sets the value of AllowUserDecryption
    pub fn set_allow_user_decryption(&mut self, value: i32) {
        self.allow_user_decryption = Some(value);
    }

    /// Gets the value of AllowUserDecryption
    pub fn get_allow_user_decryption(&self) -> Option<&i32> {
        self.allow_user_decryption.as_ref()
    }

    /// Sets the value of DataRecoveryCertificate
    pub fn set_data_recovery_certificate(&mut self, value: String) {
        self.data_recovery_certificate = Some(value);
    }

    /// Gets the value of DataRecoveryCertificate
    pub fn get_data_recovery_certificate(&self) -> Option<&String> {
        self.data_recovery_certificate.as_ref()
    }

    /// Sets the value of EDPEnforcementLevel
    pub fn set_edpenforcement_level(&mut self, value: i32) {
        self.edpenforcement_level = Some(value);
    }

    /// Gets the value of EDPEnforcementLevel
    pub fn get_edpenforcement_level(&self) -> Option<&i32> {
        self.edpenforcement_level.as_ref()
    }

    /// Sets the value of EDPShowIcons
    pub fn set_edpshow_icons(&mut self, value: i32) {
        self.edpshow_icons = Some(value);
    }

    /// Gets the value of EDPShowIcons
    pub fn get_edpshow_icons(&self) -> Option<&i32> {
        self.edpshow_icons.as_ref()
    }

    /// Sets the value of EnterpriseProtectedDomainNames
    pub fn set_enterprise_protected_domain_names(&mut self, value: String) {
        self.enterprise_protected_domain_names = Some(value);
    }

    /// Gets the value of EnterpriseProtectedDomainNames
    pub fn get_enterprise_protected_domain_names(&self) -> Option<&String> {
        self.enterprise_protected_domain_names.as_ref()
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

    /// Sets the value of RevokeOnUnenroll
    pub fn set_revoke_on_unenroll(&mut self, value: i32) {
        self.revoke_on_unenroll = Some(value);
    }

    /// Gets the value of RevokeOnUnenroll
    pub fn get_revoke_on_unenroll(&self) -> Option<&i32> {
        self.revoke_on_unenroll.as_ref()
    }

    /// Sets the value of RMSTemplateIDForEDP
    pub fn set_rmstemplate_idfor_edp(&mut self, value: String) {
        self.rmstemplate_idfor_edp = Some(value);
    }

    /// Gets the value of RMSTemplateIDForEDP
    pub fn get_rmstemplate_idfor_edp(&self) -> Option<&String> {
        self.rmstemplate_idfor_edp.as_ref()
    }

    /// Sets the value of SMBAutoEncryptedFileExtensions
    pub fn set_smbauto_encrypted_file_extensions(&mut self, value: String) {
        self.smbauto_encrypted_file_extensions = Some(value);
    }

    /// Gets the value of SMBAutoEncryptedFileExtensions
    pub fn get_smbauto_encrypted_file_extensions(&self) -> Option<&String> {
        self.smbauto_encrypted_file_extensions.as_ref()
    }
}

