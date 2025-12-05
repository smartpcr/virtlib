// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ISCSI_LoginOptions struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ISCSI_LoginOptions {

/// 
    #[serde(rename = "AuthType")]
    pub auth_type: Option<u32>,

/// 
    #[serde(rename = "DataDigest")]
    pub data_digest: Option<LoginOptions_DataDigest>,

/// 
    #[serde(rename = "DefaultTime2Retain")]
    pub default_time2_retain: Option<u32>,

/// 
    #[serde(rename = "DefaultTime2Wait")]
    pub default_time2_wait: Option<u32>,

/// 
    #[serde(rename = "HeaderDigest")]
    pub header_digest: Option<LoginOptions_HeaderDigest>,

/// 
    #[serde(rename = "InformationSpecified")]
    pub information_specified: Option<u32>,

/// 
    #[serde(rename = "LoginFlags")]
    pub login_flags: Option<u32>,

/// 
    #[serde(rename = "MaximumConnections")]
    pub maximum_connections: Option<u32>,
}

impl ISCSI_LoginOptions {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            auth_type: None,
            data_digest: None,
            default_time2_retain: None,
            default_time2_wait: None,
            header_digest: None,
            information_specified: None,
            login_flags: None,
            maximum_connections: None,
        }
    }


    /// Sets the value of AuthType
    pub fn set_auth_type(&mut self, value: u32) {
        self.auth_type = Some(value);
    }

    /// Gets the value of AuthType
    pub fn get_auth_type(&self) -> Option<&u32> {
        self.auth_type.as_ref()
    }

    /// Sets the value of DataDigest
    pub fn set_data_digest(&mut self, value: LoginOptions_DataDigest) {
        self.data_digest = Some(value);
    }

    /// Gets the value of DataDigest
    pub fn get_data_digest(&self) -> Option<&LoginOptions_DataDigest> {
        self.data_digest.as_ref()
    }

    /// Sets the value of DefaultTime2Retain
    pub fn set_default_time2_retain(&mut self, value: u32) {
        self.default_time2_retain = Some(value);
    }

    /// Gets the value of DefaultTime2Retain
    pub fn get_default_time2_retain(&self) -> Option<&u32> {
        self.default_time2_retain.as_ref()
    }

    /// Sets the value of DefaultTime2Wait
    pub fn set_default_time2_wait(&mut self, value: u32) {
        self.default_time2_wait = Some(value);
    }

    /// Gets the value of DefaultTime2Wait
    pub fn get_default_time2_wait(&self) -> Option<&u32> {
        self.default_time2_wait.as_ref()
    }

    /// Sets the value of HeaderDigest
    pub fn set_header_digest(&mut self, value: LoginOptions_HeaderDigest) {
        self.header_digest = Some(value);
    }

    /// Gets the value of HeaderDigest
    pub fn get_header_digest(&self) -> Option<&LoginOptions_HeaderDigest> {
        self.header_digest.as_ref()
    }

    /// Sets the value of InformationSpecified
    pub fn set_information_specified(&mut self, value: u32) {
        self.information_specified = Some(value);
    }

    /// Gets the value of InformationSpecified
    pub fn get_information_specified(&self) -> Option<&u32> {
        self.information_specified.as_ref()
    }

    /// Sets the value of LoginFlags
    pub fn set_login_flags(&mut self, value: u32) {
        self.login_flags = Some(value);
    }

    /// Gets the value of LoginFlags
    pub fn get_login_flags(&self) -> Option<&u32> {
        self.login_flags.as_ref()
    }

    /// Sets the value of MaximumConnections
    pub fn set_maximum_connections(&mut self, value: u32) {
        self.maximum_connections = Some(value);
    }

    /// Gets the value of MaximumConnections
    pub fn get_maximum_connections(&self) -> Option<&u32> {
        self.maximum_connections.as_ref()
    }
}

