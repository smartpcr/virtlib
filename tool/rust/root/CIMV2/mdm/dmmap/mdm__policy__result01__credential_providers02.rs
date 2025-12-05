// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_CredentialProviders02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_CredentialProviders02 {

/// 
    #[serde(rename = "AllowPINLogon")]
    pub allow_pinlogon: Option<String>,

/// 
    #[serde(rename = "BlockPicturePassword")]
    pub block_picture_password: Option<String>,

/// 
    #[serde(rename = "DisableAutomaticReDeploymentCredentials")]
    pub disable_automatic_re_deployment_credentials: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_Result01_CredentialProviders02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_pinlogon: None,
            block_picture_password: None,
            disable_automatic_re_deployment_credentials: None,
            instance_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of AllowPINLogon
    pub fn set_allow_pinlogon(&mut self, value: String) {
        self.allow_pinlogon = Some(value);
    }

    /// Gets the value of AllowPINLogon
    pub fn get_allow_pinlogon(&self) -> Option<&String> {
        self.allow_pinlogon.as_ref()
    }

    /// Sets the value of BlockPicturePassword
    pub fn set_block_picture_password(&mut self, value: String) {
        self.block_picture_password = Some(value);
    }

    /// Gets the value of BlockPicturePassword
    pub fn get_block_picture_password(&self) -> Option<&String> {
        self.block_picture_password.as_ref()
    }

    /// Sets the value of DisableAutomaticReDeploymentCredentials
    pub fn set_disable_automatic_re_deployment_credentials(&mut self, value: i32) {
        self.disable_automatic_re_deployment_credentials = Some(value);
    }

    /// Gets the value of DisableAutomaticReDeploymentCredentials
    pub fn get_disable_automatic_re_deployment_credentials(&self) -> Option<&i32> {
        self.disable_automatic_re_deployment_credentials.as_ref()
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
}

