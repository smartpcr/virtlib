// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAddressFilter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAddressFilter {
    #[serde(flatten)]
    pub base: CIM_FilterEntryBase,

/// 
    #[serde(rename = "LocalAddress")]
    pub local_address: Vec<String>,

/// 
    #[serde(rename = "RemoteAddress")]
    pub remote_address: Vec<String>,
}

impl MSFT_NetAddressFilter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_FilterEntryBase::new(),
            local_address: Vec::new(),
            remote_address: Vec::new(),
        }
    }


    /// Sets the value of LocalAddress
    pub fn set_local_address(&mut self, value: Vec<String>) {
        self.local_address = value;
    }

    /// Gets the value of LocalAddress
    pub fn get_local_address(&self) -> &Vec<String> {
        &self.local_address
    }

    /// Sets the value of RemoteAddress
    pub fn set_remote_address(&mut self, value: Vec<String>) {
        self.remote_address = value;
    }

    /// Gets the value of RemoteAddress
    pub fn get_remote_address(&self) -> &Vec<String> {
        &self.remote_address
    }

/// 

    /// * `interface_index` -  (u32)
    /// * `remote_address` -  (String)

    /// * `isolation_type` -  (u32)
    /// * `return_value` -  (u32)
    pub fn query_isolation_type(&self, interface_index: u32, remote_address: &String, isolation_type: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InterfaceIndex".to_string(), value: interface_index.into() });
        args.push(MethodParameter { name: "RemoteAddress".to_string(), value: remote_address.into() });

        let result = self.invoke_method("QueryIsolationType", &args)?;
        let isolation_type = result.get_value("IsolationType")?;
        Ok(result.return_value)

    }

}

