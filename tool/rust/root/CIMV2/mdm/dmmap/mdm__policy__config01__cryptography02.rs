// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_Cryptography02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_Cryptography02 {

/// 
    #[serde(rename = "AllowFipsAlgorithmPolicy")]
    pub allow_fips_algorithm_policy: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "TLSCipherSuites")]
    pub tlscipher_suites: Option<String>,
}

impl MDM_Policy_Config01_Cryptography02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_fips_algorithm_policy: None,
            instance_id: None,
            parent_id: None,
            tlscipher_suites: None,
        }
    }


    /// Sets the value of AllowFipsAlgorithmPolicy
    pub fn set_allow_fips_algorithm_policy(&mut self, value: i32) {
        self.allow_fips_algorithm_policy = Some(value);
    }

    /// Gets the value of AllowFipsAlgorithmPolicy
    pub fn get_allow_fips_algorithm_policy(&self) -> Option<&i32> {
        self.allow_fips_algorithm_policy.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of TLSCipherSuites
    pub fn set_tlscipher_suites(&mut self, value: String) {
        self.tlscipher_suites = Some(value);
    }

    /// Gets the value of TLSCipherSuites
    pub fn get_tlscipher_suites(&self) -> Option<&String> {
        self.tlscipher_suites.as_ref()
    }
}

