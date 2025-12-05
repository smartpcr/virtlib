// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_EASPolicy struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_EASPolicy {

/// 
    #[serde(rename = "key")]
    pub key: Option<u32>,
}

impl MDM_EASPolicy {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            key: None,
        }
    }


    /// Sets the value of key
    pub fn set_key(&mut self, value: u32) {
        self.key = Some(value);
    }

    /// Gets the value of key
    pub fn get_key(&self) -> Option<&u32> {
        self.key.as_ref()
    }

/// 

    /// * `named_values_list` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_values(&self, named_values_list: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NamedValuesList".to_string(), value: named_values_list.into() });
        self.invoke_method("SetValues", &args)

    }

}

