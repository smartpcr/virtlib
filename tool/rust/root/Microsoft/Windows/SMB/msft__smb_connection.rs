// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SMB
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SmbConnection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SmbConnection {

/// 
    #[serde(rename = "ContinuouslyAvailable")]
    pub continuously_available: Option<bool>,

/// 
    #[serde(rename = "Credential")]
    pub credential: Option<String>,

/// 
    #[serde(rename = "Dialect")]
    pub dialect: Option<String>,

/// 
    #[serde(rename = "Encrypted")]
    pub encrypted: Option<bool>,

/// 
    #[serde(rename = "NumOpens")]
    pub num_opens: Option<u64>,

/// 
    #[serde(rename = "Redirected")]
    pub redirected: Option<bool>,

/// 
    #[serde(rename = "ServerName")]
    pub server_name: Option<String>,

/// 
    #[serde(rename = "ShareName")]
    pub share_name: Option<String>,

/// 
    #[serde(rename = "Signed")]
    pub signed: Option<bool>,

/// 
    #[serde(rename = "SmbInstance")]
    pub smb_instance: Option<SmbConnection_SmbInstance>,

/// 
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,
}

impl MSFT_SmbConnection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            continuously_available: None,
            credential: None,
            dialect: None,
            encrypted: None,
            num_opens: None,
            redirected: None,
            server_name: None,
            share_name: None,
            signed: None,
            smb_instance: None,
            user_name: None,
        }
    }


    /// Sets the value of ContinuouslyAvailable
    pub fn set_continuously_available(&mut self, value: bool) {
        self.continuously_available = Some(value);
    }

    /// Gets the value of ContinuouslyAvailable
    pub fn get_continuously_available(&self) -> Option<&bool> {
        self.continuously_available.as_ref()
    }

    /// Sets the value of Credential
    pub fn set_credential(&mut self, value: String) {
        self.credential = Some(value);
    }

    /// Gets the value of Credential
    pub fn get_credential(&self) -> Option<&String> {
        self.credential.as_ref()
    }

    /// Sets the value of Dialect
    pub fn set_dialect(&mut self, value: String) {
        self.dialect = Some(value);
    }

    /// Gets the value of Dialect
    pub fn get_dialect(&self) -> Option<&String> {
        self.dialect.as_ref()
    }

    /// Sets the value of Encrypted
    pub fn set_encrypted(&mut self, value: bool) {
        self.encrypted = Some(value);
    }

    /// Gets the value of Encrypted
    pub fn get_encrypted(&self) -> Option<&bool> {
        self.encrypted.as_ref()
    }

    /// Sets the value of NumOpens
    pub fn set_num_opens(&mut self, value: u64) {
        self.num_opens = Some(value);
    }

    /// Gets the value of NumOpens
    pub fn get_num_opens(&self) -> Option<&u64> {
        self.num_opens.as_ref()
    }

    /// Sets the value of Redirected
    pub fn set_redirected(&mut self, value: bool) {
        self.redirected = Some(value);
    }

    /// Gets the value of Redirected
    pub fn get_redirected(&self) -> Option<&bool> {
        self.redirected.as_ref()
    }

    /// Sets the value of ServerName
    pub fn set_server_name(&mut self, value: String) {
        self.server_name = Some(value);
    }

    /// Gets the value of ServerName
    pub fn get_server_name(&self) -> Option<&String> {
        self.server_name.as_ref()
    }

    /// Sets the value of ShareName
    pub fn set_share_name(&mut self, value: String) {
        self.share_name = Some(value);
    }

    /// Gets the value of ShareName
    pub fn get_share_name(&self) -> Option<&String> {
        self.share_name.as_ref()
    }

    /// Sets the value of Signed
    pub fn set_signed(&mut self, value: bool) {
        self.signed = Some(value);
    }

    /// Gets the value of Signed
    pub fn get_signed(&self) -> Option<&bool> {
        self.signed.as_ref()
    }

    /// Sets the value of SmbInstance
    pub fn set_smb_instance(&mut self, value: SmbConnection_SmbInstance) {
        self.smb_instance = Some(value);
    }

    /// Gets the value of SmbInstance
    pub fn get_smb_instance(&self) -> Option<&SmbConnection_SmbInstance> {
        self.smb_instance.as_ref()
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

