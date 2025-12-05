// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SMB
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SmbMultichannelConstraint struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SmbMultichannelConstraint {

/// 
    #[serde(rename = "InterfaceAlias")]
    pub interface_alias: Option<String>,

/// 
    #[serde(rename = "InterfaceGuid")]
    pub interface_guid: Option<String>,

/// 
    #[serde(rename = "InterfaceIndex")]
    pub interface_index: Option<u32>,

/// 
    #[serde(rename = "ServerName")]
    pub server_name: Option<String>,
}

impl MSFT_SmbMultichannelConstraint {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            interface_alias: None,
            interface_guid: None,
            interface_index: None,
            server_name: None,
        }
    }


    /// Sets the value of InterfaceAlias
    pub fn set_interface_alias(&mut self, value: String) {
        self.interface_alias = Some(value);
    }

    /// Gets the value of InterfaceAlias
    pub fn get_interface_alias(&self) -> Option<&String> {
        self.interface_alias.as_ref()
    }

    /// Sets the value of InterfaceGuid
    pub fn set_interface_guid(&mut self, value: String) {
        self.interface_guid = Some(value);
    }

    /// Gets the value of InterfaceGuid
    pub fn get_interface_guid(&self) -> Option<&String> {
        self.interface_guid.as_ref()
    }

    /// Sets the value of InterfaceIndex
    pub fn set_interface_index(&mut self, value: u32) {
        self.interface_index = Some(value);
    }

    /// Gets the value of InterfaceIndex
    pub fn get_interface_index(&self) -> Option<&u32> {
        self.interface_index.as_ref()
    }

    /// Sets the value of ServerName
    pub fn set_server_name(&mut self, value: String) {
        self.server_name = Some(value);
    }

    /// Gets the value of ServerName
    pub fn get_server_name(&self) -> Option<&String> {
        self.server_name.as_ref()
    }

/// 

    /// * `interface_alias` -  (String[])
    /// * `interface_index` -  (u32[])
    /// * `server_name` -  (String)

    /// * `output` -  (MSFT_SmbMultichannelConstraint[])
    /// * `return_value` -  (u32)
    pub fn create_constraint(&self, server_name: &String, interface_index: &Vec<u32>, interface_alias: &Vec<String>, output: &mut Vec<MSFT_SmbMultichannelConstraint>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ServerName".to_string(), value: server_name.into() });
        args.push(MethodParameter { name: "InterfaceIndex".to_string(), value: interface_index.into() });
        args.push(MethodParameter { name: "InterfaceAlias".to_string(), value: interface_alias.into() });

        let result = self.invoke_method("CreateConstraint", &args)?;
        let output = result.get_value("Output")?;
        Ok(result.return_value)

    }

}

