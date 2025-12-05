// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_ActiveSync_User_Accounts01_01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_ActiveSync_User_Accounts01_01 {

/// 
    #[serde(rename = "AccountIcon")]
    pub account_icon: Option<String>,

/// 
    #[serde(rename = "AccountName")]
    pub account_name: Option<String>,

/// 
    #[serde(rename = "AccountType")]
    pub account_type: Option<String>,

/// 
    #[serde(rename = "Domain")]
    pub domain: Option<String>,

/// 
    #[serde(rename = "EmailAddress")]
    pub email_address: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "Password")]
    pub password: Option<String>,

/// 
    #[serde(rename = "ServerName")]
    pub server_name: Option<String>,

/// 
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,
}

impl MDM_ActiveSync_User_Accounts01_01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            account_icon: None,
            account_name: None,
            account_type: None,
            domain: None,
            email_address: None,
            instance_id: None,
            parent_id: None,
            password: None,
            server_name: None,
            user_name: None,
        }
    }


    /// Sets the value of AccountIcon
    pub fn set_account_icon(&mut self, value: String) {
        self.account_icon = Some(value);
    }

    /// Gets the value of AccountIcon
    pub fn get_account_icon(&self) -> Option<&String> {
        self.account_icon.as_ref()
    }

    /// Sets the value of AccountName
    pub fn set_account_name(&mut self, value: String) {
        self.account_name = Some(value);
    }

    /// Gets the value of AccountName
    pub fn get_account_name(&self) -> Option<&String> {
        self.account_name.as_ref()
    }

    /// Sets the value of AccountType
    pub fn set_account_type(&mut self, value: String) {
        self.account_type = Some(value);
    }

    /// Gets the value of AccountType
    pub fn get_account_type(&self) -> Option<&String> {
        self.account_type.as_ref()
    }

    /// Sets the value of Domain
    pub fn set_domain(&mut self, value: String) {
        self.domain = Some(value);
    }

    /// Gets the value of Domain
    pub fn get_domain(&self) -> Option<&String> {
        self.domain.as_ref()
    }

    /// Sets the value of EmailAddress
    pub fn set_email_address(&mut self, value: String) {
        self.email_address = Some(value);
    }

    /// Gets the value of EmailAddress
    pub fn get_email_address(&self) -> Option<&String> {
        self.email_address.as_ref()
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

    /// Sets the value of Password
    pub fn set_password(&mut self, value: String) {
        self.password = Some(value);
    }

    /// Gets the value of Password
    pub fn get_password(&self) -> Option<&String> {
        self.password.as_ref()
    }

    /// Sets the value of ServerName
    pub fn set_server_name(&mut self, value: String) {
        self.server_name = Some(value);
    }

    /// Gets the value of ServerName
    pub fn get_server_name(&self) -> Option<&String> {
        self.server_name.as_ref()
    }

    /// Sets the value of UserName
    pub fn set_user_name(&mut self, value: String) {
        self.user_name = Some(value);
    }

    /// Gets the value of UserName
    pub fn get_user_name(&self) -> Option<&String> {
        self.user_name.as_ref()
    }
}

