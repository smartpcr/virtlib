// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_IKESAEndpoint struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_IKESAEndpoint {
    #[serde(flatten)]
    pub base: CIM_SecurityAssociationEndpoint,

/// 
    #[serde(rename = "AuthenticationMethod")]
    pub authentication_method: Option<u16>,

/// 
    #[serde(rename = "CipherAlgorithm")]
    pub cipher_algorithm: Option<u16>,

/// 
    #[serde(rename = "GroupId")]
    pub group_id: Option<u16>,

/// 
    #[serde(rename = "HashAlgorithm")]
    pub hash_algorithm: Option<u16>,

/// 
    #[serde(rename = "InitiatorCookie")]
    pub initiator_cookie: Option<u64>,

/// 
    #[serde(rename = "OtherAuthenticationMethod")]
    pub other_authentication_method: Option<String>,

/// 
    #[serde(rename = "OtherCipherAlgorithm")]
    pub other_cipher_algorithm: Option<String>,

/// 
    #[serde(rename = "OtherHashAlgorithm")]
    pub other_hash_algorithm: Option<String>,

/// 
    #[serde(rename = "ResponderCookie")]
    pub responder_cookie: Option<u64>,

/// 
    #[serde(rename = "VendorID")]
    pub vendor_id: Option<String>,
}

impl CIM_IKESAEndpoint {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SecurityAssociationEndpoint::new(),
            authentication_method: None,
            cipher_algorithm: None,
            group_id: None,
            hash_algorithm: None,
            initiator_cookie: None,
            other_authentication_method: None,
            other_cipher_algorithm: None,
            other_hash_algorithm: None,
            responder_cookie: None,
            vendor_id: None,
        }
    }


    /// Sets the value of AuthenticationMethod
    pub fn set_authentication_method(&mut self, value: u16) {
        self.authentication_method = Some(value);
    }

    /// Gets the value of AuthenticationMethod
    pub fn get_authentication_method(&self) -> Option<&u16> {
        self.authentication_method.as_ref()
    }

    /// Sets the value of CipherAlgorithm
    pub fn set_cipher_algorithm(&mut self, value: u16) {
        self.cipher_algorithm = Some(value);
    }

    /// Gets the value of CipherAlgorithm
    pub fn get_cipher_algorithm(&self) -> Option<&u16> {
        self.cipher_algorithm.as_ref()
    }

    /// Sets the value of GroupId
    pub fn set_group_id(&mut self, value: u16) {
        self.group_id = Some(value);
    }

    /// Gets the value of GroupId
    pub fn get_group_id(&self) -> Option<&u16> {
        self.group_id.as_ref()
    }

    /// Sets the value of HashAlgorithm
    pub fn set_hash_algorithm(&mut self, value: u16) {
        self.hash_algorithm = Some(value);
    }

    /// Gets the value of HashAlgorithm
    pub fn get_hash_algorithm(&self) -> Option<&u16> {
        self.hash_algorithm.as_ref()
    }

    /// Sets the value of InitiatorCookie
    pub fn set_initiator_cookie(&mut self, value: u64) {
        self.initiator_cookie = Some(value);
    }

    /// Gets the value of InitiatorCookie
    pub fn get_initiator_cookie(&self) -> Option<&u64> {
        self.initiator_cookie.as_ref()
    }

    /// Sets the value of OtherAuthenticationMethod
    pub fn set_other_authentication_method(&mut self, value: String) {
        self.other_authentication_method = Some(value);
    }

    /// Gets the value of OtherAuthenticationMethod
    pub fn get_other_authentication_method(&self) -> Option<&String> {
        self.other_authentication_method.as_ref()
    }

    /// Sets the value of OtherCipherAlgorithm
    pub fn set_other_cipher_algorithm(&mut self, value: String) {
        self.other_cipher_algorithm = Some(value);
    }

    /// Gets the value of OtherCipherAlgorithm
    pub fn get_other_cipher_algorithm(&self) -> Option<&String> {
        self.other_cipher_algorithm.as_ref()
    }

    /// Sets the value of OtherHashAlgorithm
    pub fn set_other_hash_algorithm(&mut self, value: String) {
        self.other_hash_algorithm = Some(value);
    }

    /// Gets the value of OtherHashAlgorithm
    pub fn get_other_hash_algorithm(&self) -> Option<&String> {
        self.other_hash_algorithm.as_ref()
    }

    /// Sets the value of ResponderCookie
    pub fn set_responder_cookie(&mut self, value: u64) {
        self.responder_cookie = Some(value);
    }

    /// Gets the value of ResponderCookie
    pub fn get_responder_cookie(&self) -> Option<&u64> {
        self.responder_cookie.as_ref()
    }

    /// Sets the value of VendorID
    pub fn set_vendor_id(&mut self, value: String) {
        self.vendor_id = Some(value);
    }

    /// Gets the value of VendorID
    pub fn get_vendor_id(&self) -> Option<&String> {
        self.vendor_id.as_ref()
    }
}

