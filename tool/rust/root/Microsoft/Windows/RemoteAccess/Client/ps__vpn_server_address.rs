// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PS_VpnServerAddress struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PS_VpnServerAddress {

/// 
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// 
    #[serde(rename = "ServerAddress")]
    pub server_address: Option<String>,
}

impl PS_VpnServerAddress {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            friendly_name: None,
            server_address: None,
        }
    }


    /// Sets the value of FriendlyName
    pub fn set_friendly_name(&mut self, value: String) {
        self.friendly_name = Some(value);
    }

    /// Gets the value of FriendlyName
    pub fn get_friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    /// Sets the value of ServerAddress
    pub fn set_server_address(&mut self, value: String) {
        self.server_address = Some(value);
    }

    /// Gets the value of ServerAddress
    pub fn get_server_address(&self) -> Option<&String> {
        self.server_address.as_ref()
    }

/// 

    /// * `friendly_name` -  (String)
    /// * `pass_thru` -  (bool)
    /// * `server_address` -  (String)

    /// * `cmdlet_output` -  (VpnServerAddress)
    /// * `return_value` -  (u32)
    pub fn new(&self, server_address: &String, friendly_name: &String, pass_thru: bool, cmdlet_output: &mut VpnServerAddress) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ServerAddress".to_string(), value: server_address.into() });
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("New", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

