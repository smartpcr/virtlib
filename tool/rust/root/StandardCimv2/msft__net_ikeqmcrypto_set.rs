// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetIKEQMCryptoSet struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetIKEQMCryptoSet {
    #[serde(flatten)]
    pub base: MSFT_NetIKECryptoSet,

/// 
    #[serde(rename = "PfsGroupID")]
    pub pfs_group_id: Option<u16>,
}

impl MSFT_NetIKEQMCryptoSet {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetIKECryptoSet::new(),
            pfs_group_id: None,
        }
    }


    /// Sets the value of PfsGroupID
    pub fn set_pfs_group_id(&mut self, value: u16) {
        self.pfs_group_id = Some(value);
    }

    /// Gets the value of PfsGroupID
    pub fn get_pfs_group_id(&self) -> Option<&u16> {
        self.pfs_group_id.as_ref()
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

