// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFC_DH_Chap_Parameters struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFC_DH_Chap_Parameters {

/// 
    #[serde(rename = "SecretEncoding")]
    pub secret_encoding: Option<Parameters_SecretEncoding>,

/// 
    #[serde(rename = "SharedSecret")]
    pub shared_secret: Vec<u8>,

/// 
    #[serde(rename = "SharedSecretLength")]
    pub shared_secret_length: Option<u32>,
}

impl MSFC_DH_Chap_Parameters {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            secret_encoding: None,
            shared_secret: Vec::new(),
            shared_secret_length: None,
        }
    }


    /// Sets the value of SecretEncoding
    pub fn set_secret_encoding(&mut self, value: Parameters_SecretEncoding) {
        self.secret_encoding = Some(value);
    }

    /// Gets the value of SecretEncoding
    pub fn get_secret_encoding(&self) -> Option<&Parameters_SecretEncoding> {
        self.secret_encoding.as_ref()
    }

    /// Sets the value of SharedSecret
    pub fn set_shared_secret(&mut self, value: Vec<u8>) {
        self.shared_secret = value;
    }

    /// Gets the value of SharedSecret
    pub fn get_shared_secret(&self) -> &Vec<u8> {
        &self.shared_secret
    }

    /// Sets the value of SharedSecretLength
    pub fn set_shared_secret_length(&mut self, value: u32) {
        self.shared_secret_length = Some(value);
    }

    /// Gets the value of SharedSecretLength
    pub fn get_shared_secret_length(&self) -> Option<&u32> {
        self.shared_secret_length.as_ref()
    }
}

