// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSIInitiator_IKEPresharedKeyAuthenticationInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSIInitiator_IKEPresharedKeyAuthenticationInfo {

/// 
    #[serde(rename = "AuthMethod")]
    pub auth_method: Option<IKEPresharedKeyAuthenticationInfo_AuthMethod>,

/// 
    #[serde(rename = "Id")]
    pub id: Vec<u8>,

/// 
    #[serde(rename = "IdType")]
    pub id_type: Option<IKEPresharedKeyAuthenticationInfo_IdType>,

/// 
    #[serde(rename = "key")]
    pub key: Vec<u8>,

/// 
    #[serde(rename = "SecurityFlags")]
    pub security_flags: Option<u64>,
}

impl MSiSCSIInitiator_IKEPresharedKeyAuthenticationInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            auth_method: None,
            id: Vec::new(),
            id_type: None,
            key: Vec::new(),
            security_flags: None,
        }
    }


    /// Sets the value of AuthMethod
    pub fn set_auth_method(&mut self, value: IKEPresharedKeyAuthenticationInfo_AuthMethod) {
        self.auth_method = Some(value);
    }

    /// Gets the value of AuthMethod
    pub fn get_auth_method(&self) -> Option<&IKEPresharedKeyAuthenticationInfo_AuthMethod> {
        self.auth_method.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: Vec<u8>) {
        self.id = value;
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> &Vec<u8> {
        &self.id
    }

    /// Sets the value of IdType
    pub fn set_id_type(&mut self, value: IKEPresharedKeyAuthenticationInfo_IdType) {
        self.id_type = Some(value);
    }

    /// Gets the value of IdType
    pub fn get_id_type(&self) -> Option<&IKEPresharedKeyAuthenticationInfo_IdType> {
        self.id_type.as_ref()
    }

    /// Sets the value of key
    pub fn set_key(&mut self, value: Vec<u8>) {
        self.key = value;
    }

    /// Gets the value of key
    pub fn get_key(&self) -> &Vec<u8> {
        &self.key
    }

    /// Sets the value of SecurityFlags
    pub fn set_security_flags(&mut self, value: u64) {
        self.security_flags = Some(value);
    }

    /// Gets the value of SecurityFlags
    pub fn get_security_flags(&self) -> Option<&u64> {
        self.security_flags.as_ref()
    }
}

