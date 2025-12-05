// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Cluster.Scaleout
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ClusterSetKey struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ClusterSetKey {

/// 
    #[serde(rename = "Cert")]
    pub cert: Vec<u8>,

/// 
    #[serde(rename = "key")]
    pub key: Vec<u8>,
}

impl MSFT_ClusterSetKey {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            cert: Vec::new(),
            key: Vec::new(),
        }
    }


    /// Sets the value of Cert
    pub fn set_cert(&mut self, value: Vec<u8>) {
        self.cert = value;
    }

    /// Gets the value of Cert
    pub fn get_cert(&self) -> &Vec<u8> {
        &self.cert
    }

    /// Sets the value of key
    pub fn set_key(&mut self, value: Vec<u8>) {
        self.key = value;
    }

    /// Gets the value of key
    pub fn get_key(&self) -> &Vec<u8> {
        &self.key
    }
}

