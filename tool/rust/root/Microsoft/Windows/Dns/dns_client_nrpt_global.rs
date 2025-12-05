// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Dns
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DnsClientNrptGlobal struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DnsClientNrptGlobal {

/// 
    #[serde(rename = "EnableDAForAllNetworks")]
    pub enable_dafor_all_networks: Option<String>,

/// 
    #[serde(rename = "QueryPolicy")]
    pub query_policy: Option<String>,

/// 
    #[serde(rename = "SecureNameQueryFallback")]
    pub secure_name_query_fallback: Option<String>,
}

impl DnsClientNrptGlobal {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            enable_dafor_all_networks: None,
            query_policy: None,
            secure_name_query_fallback: None,
        }
    }


    /// Sets the value of EnableDAForAllNetworks
    pub fn set_enable_dafor_all_networks(&mut self, value: String) {
        self.enable_dafor_all_networks = Some(value);
    }

    /// Gets the value of EnableDAForAllNetworks
    pub fn get_enable_dafor_all_networks(&self) -> Option<&String> {
        self.enable_dafor_all_networks.as_ref()
    }

    /// Sets the value of QueryPolicy
    pub fn set_query_policy(&mut self, value: String) {
        self.query_policy = Some(value);
    }

    /// Gets the value of QueryPolicy
    pub fn get_query_policy(&self) -> Option<&String> {
        self.query_policy.as_ref()
    }

    /// Sets the value of SecureNameQueryFallback
    pub fn set_secure_name_query_fallback(&mut self, value: String) {
        self.secure_name_query_fallback = Some(value);
    }

    /// Gets the value of SecureNameQueryFallback
    pub fn get_secure_name_query_fallback(&self) -> Option<&String> {
        self.secure_name_query_fallback.as_ref()
    }
}

