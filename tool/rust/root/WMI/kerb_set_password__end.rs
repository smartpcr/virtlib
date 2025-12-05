// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// KerbSetPassword_End struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KerbSetPassword_End {
    #[serde(flatten)]
    pub base: KerbSetPassword,

/// Account Name
    #[serde(rename = "AccountName")]
    pub account_name: Option<String>,

/// Account Realm
    #[serde(rename = "AccountRealm")]
    pub account_realm: Option<String>,

/// Client Name
    #[serde(rename = "ClientName")]
    pub client_name: Option<String>,

/// Client Realm
    #[serde(rename = "ClientRealm")]
    pub client_realm: Option<String>,

/// KDC Address
    #[serde(rename = "KdcAddress")]
    pub kdc_address: Option<String>,

/// Status
    #[serde(rename = "Status")]
    pub status: Option<u32>,
}

impl KerbSetPassword_End {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: KerbSetPassword::new(),
            account_name: None,
            account_realm: None,
            client_name: None,
            client_realm: None,
            kdc_address: None,
            status: None,
        }
    }


    /// Sets the value of AccountName
    pub fn set_account_name(&mut self, value: String) {
        self.account_name = Some(value);
    }

    /// Gets the value of AccountName
    pub fn get_account_name(&self) -> Option<&String> {
        self.account_name.as_ref()
    }

    /// Sets the value of AccountRealm
    pub fn set_account_realm(&mut self, value: String) {
        self.account_realm = Some(value);
    }

    /// Gets the value of AccountRealm
    pub fn get_account_realm(&self) -> Option<&String> {
        self.account_realm.as_ref()
    }

    /// Sets the value of ClientName
    pub fn set_client_name(&mut self, value: String) {
        self.client_name = Some(value);
    }

    /// Gets the value of ClientName
    pub fn get_client_name(&self) -> Option<&String> {
        self.client_name.as_ref()
    }

    /// Sets the value of ClientRealm
    pub fn set_client_realm(&mut self, value: String) {
        self.client_realm = Some(value);
    }

    /// Gets the value of ClientRealm
    pub fn get_client_realm(&self) -> Option<&String> {
        self.client_realm.as_ref()
    }

    /// Sets the value of KdcAddress
    pub fn set_kdc_address(&mut self, value: String) {
        self.kdc_address = Some(value);
    }

    /// Gets the value of KdcAddress
    pub fn get_kdc_address(&self) -> Option<&String> {
        self.kdc_address.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u32> {
        self.status.as_ref()
    }
}

