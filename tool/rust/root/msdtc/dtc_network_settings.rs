// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.msdtc
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DtcNetworkSettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DtcNetworkSettings {

/// 
    #[serde(rename = "AuthenticationLevel")]
    pub authentication_level: Option<String>,

/// 
    #[serde(rename = "InboundTransactionsEnabled")]
    pub inbound_transactions_enabled: Option<bool>,

/// 
    #[serde(rename = "LUTransactionsEnabled")]
    pub lutransactions_enabled: Option<bool>,

/// 
    #[serde(rename = "OutboundTransactionsEnabled")]
    pub outbound_transactions_enabled: Option<bool>,

/// 
    #[serde(rename = "RemoteAdministrationAccessEnabled")]
    pub remote_administration_access_enabled: Option<bool>,

/// 
    #[serde(rename = "RemoteClientAccessEnabled")]
    pub remote_client_access_enabled: Option<bool>,

/// 
    #[serde(rename = "XATransactionsEnabled")]
    pub xatransactions_enabled: Option<bool>,
}

impl DtcNetworkSettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            authentication_level: None,
            inbound_transactions_enabled: None,
            lutransactions_enabled: None,
            outbound_transactions_enabled: None,
            remote_administration_access_enabled: None,
            remote_client_access_enabled: None,
            xatransactions_enabled: None,
        }
    }


    /// Sets the value of AuthenticationLevel
    pub fn set_authentication_level(&mut self, value: String) {
        self.authentication_level = Some(value);
    }

    /// Gets the value of AuthenticationLevel
    pub fn get_authentication_level(&self) -> Option<&String> {
        self.authentication_level.as_ref()
    }

    /// Sets the value of InboundTransactionsEnabled
    pub fn set_inbound_transactions_enabled(&mut self, value: bool) {
        self.inbound_transactions_enabled = Some(value);
    }

    /// Gets the value of InboundTransactionsEnabled
    pub fn get_inbound_transactions_enabled(&self) -> Option<&bool> {
        self.inbound_transactions_enabled.as_ref()
    }

    /// Sets the value of LUTransactionsEnabled
    pub fn set_lutransactions_enabled(&mut self, value: bool) {
        self.lutransactions_enabled = Some(value);
    }

    /// Gets the value of LUTransactionsEnabled
    pub fn get_lutransactions_enabled(&self) -> Option<&bool> {
        self.lutransactions_enabled.as_ref()
    }

    /// Sets the value of OutboundTransactionsEnabled
    pub fn set_outbound_transactions_enabled(&mut self, value: bool) {
        self.outbound_transactions_enabled = Some(value);
    }

    /// Gets the value of OutboundTransactionsEnabled
    pub fn get_outbound_transactions_enabled(&self) -> Option<&bool> {
        self.outbound_transactions_enabled.as_ref()
    }

    /// Sets the value of RemoteAdministrationAccessEnabled
    pub fn set_remote_administration_access_enabled(&mut self, value: bool) {
        self.remote_administration_access_enabled = Some(value);
    }

    /// Gets the value of RemoteAdministrationAccessEnabled
    pub fn get_remote_administration_access_enabled(&self) -> Option<&bool> {
        self.remote_administration_access_enabled.as_ref()
    }

    /// Sets the value of RemoteClientAccessEnabled
    pub fn set_remote_client_access_enabled(&mut self, value: bool) {
        self.remote_client_access_enabled = Some(value);
    }

    /// Gets the value of RemoteClientAccessEnabled
    pub fn get_remote_client_access_enabled(&self) -> Option<&bool> {
        self.remote_client_access_enabled.as_ref()
    }

    /// Sets the value of XATransactionsEnabled
    pub fn set_xatransactions_enabled(&mut self, value: bool) {
        self.xatransactions_enabled = Some(value);
    }

    /// Gets the value of XATransactionsEnabled
    pub fn get_xatransactions_enabled(&self) -> Option<&bool> {
        self.xatransactions_enabled.as_ref()
    }
}

