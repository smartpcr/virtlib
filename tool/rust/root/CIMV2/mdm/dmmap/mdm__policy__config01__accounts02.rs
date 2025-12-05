// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_Accounts02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_Accounts02 {

/// 
    #[serde(rename = "AllowAddingNonMicrosoftAccountsManually")]
    pub allow_adding_non_microsoft_accounts_manually: Option<i32>,

/// 
    #[serde(rename = "AllowMicrosoftAccountConnection")]
    pub allow_microsoft_account_connection: Option<i32>,

/// 
    #[serde(rename = "AllowMicrosoftAccountSignInAssistant")]
    pub allow_microsoft_account_sign_in_assistant: Option<i32>,

/// 
    #[serde(rename = "DomainNamesForEmailSync")]
    pub domain_names_for_email_sync: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_Config01_Accounts02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_adding_non_microsoft_accounts_manually: None,
            allow_microsoft_account_connection: None,
            allow_microsoft_account_sign_in_assistant: None,
            domain_names_for_email_sync: None,
            instance_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of AllowAddingNonMicrosoftAccountsManually
    pub fn set_allow_adding_non_microsoft_accounts_manually(&mut self, value: i32) {
        self.allow_adding_non_microsoft_accounts_manually = Some(value);
    }

    /// Gets the value of AllowAddingNonMicrosoftAccountsManually
    pub fn get_allow_adding_non_microsoft_accounts_manually(&self) -> Option<&i32> {
        self.allow_adding_non_microsoft_accounts_manually.as_ref()
    }

    /// Sets the value of AllowMicrosoftAccountConnection
    pub fn set_allow_microsoft_account_connection(&mut self, value: i32) {
        self.allow_microsoft_account_connection = Some(value);
    }

    /// Gets the value of AllowMicrosoftAccountConnection
    pub fn get_allow_microsoft_account_connection(&self) -> Option<&i32> {
        self.allow_microsoft_account_connection.as_ref()
    }

    /// Sets the value of AllowMicrosoftAccountSignInAssistant
    pub fn set_allow_microsoft_account_sign_in_assistant(&mut self, value: i32) {
        self.allow_microsoft_account_sign_in_assistant = Some(value);
    }

    /// Gets the value of AllowMicrosoftAccountSignInAssistant
    pub fn get_allow_microsoft_account_sign_in_assistant(&self) -> Option<&i32> {
        self.allow_microsoft_account_sign_in_assistant.as_ref()
    }

    /// Sets the value of DomainNamesForEmailSync
    pub fn set_domain_names_for_email_sync(&mut self, value: String) {
        self.domain_names_for_email_sync = Some(value);
    }

    /// Gets the value of DomainNamesForEmailSync
    pub fn get_domain_names_for_email_sync(&self) -> Option<&String> {
        self.domain_names_for_email_sync.as_ref()
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

