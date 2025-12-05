// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_SID struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_SID {

/// 
    #[serde(rename = "AccountName")]
    pub account_name: Option<String>,

/// 
    #[serde(rename = "BinaryRepresentation")]
    pub binary_representation: Vec<u8>,

/// 
    #[serde(rename = "ReferencedDomainName")]
    pub referenced_domain_name: Option<String>,

/// 
    #[serde(rename = "SID")]
    pub sid: Option<String>,

/// 
    #[serde(rename = "SidLength")]
    pub sid_length: Option<u32>,
}

impl Win32_SID {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            account_name: None,
            binary_representation: Vec::new(),
            referenced_domain_name: None,
            sid: None,
            sid_length: None,
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

    /// Sets the value of BinaryRepresentation
    pub fn set_binary_representation(&mut self, value: Vec<u8>) {
        self.binary_representation = value;
    }

    /// Gets the value of BinaryRepresentation
    pub fn get_binary_representation(&self) -> &Vec<u8> {
        &self.binary_representation
    }

    /// Sets the value of ReferencedDomainName
    pub fn set_referenced_domain_name(&mut self, value: String) {
        self.referenced_domain_name = Some(value);
    }

    /// Gets the value of ReferencedDomainName
    pub fn get_referenced_domain_name(&self) -> Option<&String> {
        self.referenced_domain_name.as_ref()
    }

    /// Sets the value of SID
    pub fn set_sid(&mut self, value: String) {
        self.sid = Some(value);
    }

    /// Gets the value of SID
    pub fn get_sid(&self) -> Option<&String> {
        self.sid.as_ref()
    }

    /// Sets the value of SidLength
    pub fn set_sid_length(&mut self, value: u32) {
        self.sid_length = Some(value);
    }

    /// Gets the value of SidLength
    pub fn get_sid_length(&self) -> Option<&u32> {
        self.sid_length.as_ref()
    }
}

