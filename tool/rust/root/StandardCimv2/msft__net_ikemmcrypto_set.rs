// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetIKEMMCryptoSet struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetIKEMMCryptoSet {
    #[serde(flatten)]
    pub base: MSFT_NetIKECryptoSet,

/// 
    #[serde(rename = "ForceDiffieHellman")]
    pub force_diffie_hellman: Option<bool>,

/// 
    #[serde(rename = "MaxLifetimeMinutes")]
    pub max_lifetime_minutes: Option<u32>,

/// 
    #[serde(rename = "MaxLifetimeSessions")]
    pub max_lifetime_sessions: Option<u32>,
}

impl MSFT_NetIKEMMCryptoSet {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetIKECryptoSet::new(),
            force_diffie_hellman: None,
            max_lifetime_minutes: None,
            max_lifetime_sessions: None,
        }
    }


    /// Sets the value of ForceDiffieHellman
    pub fn set_force_diffie_hellman(&mut self, value: bool) {
        self.force_diffie_hellman = Some(value);
    }

    /// Gets the value of ForceDiffieHellman
    pub fn get_force_diffie_hellman(&self) -> Option<&bool> {
        self.force_diffie_hellman.as_ref()
    }

    /// Sets the value of MaxLifetimeMinutes
    pub fn set_max_lifetime_minutes(&mut self, value: u32) {
        self.max_lifetime_minutes = Some(value);
    }

    /// Gets the value of MaxLifetimeMinutes
    pub fn get_max_lifetime_minutes(&self) -> Option<&u32> {
        self.max_lifetime_minutes.as_ref()
    }

    /// Sets the value of MaxLifetimeSessions
    pub fn set_max_lifetime_sessions(&mut self, value: u32) {
        self.max_lifetime_sessions = Some(value);
    }

    /// Gets the value of MaxLifetimeSessions
    pub fn get_max_lifetime_sessions(&self) -> Option<&u32> {
        self.max_lifetime_sessions.as_ref()
    }

/// 

    /// * `new_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn rename(&self, new_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewName".to_string(), value: new_name.into() });
        self.invoke_method("Rename", &args)

    }


/// 

    /// * `new_gposession` -  (String)
    /// * `new_id` -  (String)
    /// * `new_name` -  (String)
    /// * `new_policy_store` -  (String)

    /// * `return_value` -  (u32)
    pub fn clone_object(&self, new_name: &String, new_id: &String, new_policy_store: &String, new_gposession: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewName".to_string(), value: new_name.into() });
        args.push(MethodParameter { name: "NewID".to_string(), value: new_id.into() });
        args.push(MethodParameter { name: "NewPolicyStore".to_string(), value: new_policy_store.into() });
        args.push(MethodParameter { name: "NewGPOSession".to_string(), value: new_gposession.into() });
        self.invoke_method("CloneObject", &args)

    }

}

