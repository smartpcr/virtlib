// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Attestation
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_AttestationResult struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_AttestationResult {

/// 
    #[serde(rename = "AttestationStatus")]
    pub attestation_status: Option<u16>,

/// 
    #[serde(rename = "AttestationSubstatus")]
    pub attestation_substatus: Option<u64>,

/// 
    #[serde(rename = "Data")]
    pub data: Vec<u8>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u16>,

/// 
    #[serde(rename = "Url")]
    pub url: Option<String>,
}

impl MSFT_AttestationResult {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            attestation_status: None,
            attestation_substatus: None,
            data: Vec::new(),
            type: None,
            url: None,
        }
    }


    /// Sets the value of AttestationStatus
    pub fn set_attestation_status(&mut self, value: u16) {
        self.attestation_status = Some(value);
    }

    /// Gets the value of AttestationStatus
    pub fn get_attestation_status(&self) -> Option<&u16> {
        self.attestation_status.as_ref()
    }

    /// Sets the value of AttestationSubstatus
    pub fn set_attestation_substatus(&mut self, value: u64) {
        self.attestation_substatus = Some(value);
    }

    /// Gets the value of AttestationSubstatus
    pub fn get_attestation_substatus(&self) -> Option<&u64> {
        self.attestation_substatus.as_ref()
    }

    /// Sets the value of Data
    pub fn set_data(&mut self, value: Vec<u8>) {
        self.data = value;
    }

    /// Gets the value of Data
    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u16) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u16> {
        self.type.as_ref()
    }

    /// Sets the value of Url
    pub fn set_url(&mut self, value: String) {
        self.url = Some(value);
    }

    /// Gets the value of Url
    pub fn get_url(&self) -> Option<&String> {
        self.url.as_ref()
    }
}

