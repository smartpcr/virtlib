// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetTeredoConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetTeredoConfiguration {
    #[serde(flatten)]
    pub base: MSFT_NetSettingData,

/// 
    #[serde(rename = "ClientPort")]
    pub client_port: Option<u32>,

/// 
    #[serde(rename = "DefaultQualified")]
    pub default_qualified: Option<bool>,

/// 
    #[serde(rename = "PolicyStore")]
    pub policy_store: Option<String>,

/// 
    #[serde(rename = "RefreshInterval")]
    pub refresh_interval: Option<u32>,

/// 
    #[serde(rename = "ServerName")]
    pub server_name: Option<String>,

/// 
    #[serde(rename = "ServerShunt")]
    pub server_shunt: Option<bool>,

/// 
    #[serde(rename = "ServerVirtualIP")]
    pub server_virtual_ip: Option<String>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u32>,
}

impl MSFT_NetTeredoConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSettingData::new(),
            client_port: None,
            default_qualified: None,
            policy_store: None,
            refresh_interval: None,
            server_name: None,
            server_shunt: None,
            server_virtual_ip: None,
            type: None,
        }
    }


    /// Sets the value of ClientPort
    pub fn set_client_port(&mut self, value: u32) {
        self.client_port = Some(value);
    }

    /// Gets the value of ClientPort
    pub fn get_client_port(&self) -> Option<&u32> {
        self.client_port.as_ref()
    }

    /// Sets the value of DefaultQualified
    pub fn set_default_qualified(&mut self, value: bool) {
        self.default_qualified = Some(value);
    }

    /// Gets the value of DefaultQualified
    pub fn get_default_qualified(&self) -> Option<&bool> {
        self.default_qualified.as_ref()
    }

    /// Sets the value of PolicyStore
    pub fn set_policy_store(&mut self, value: String) {
        self.policy_store = Some(value);
    }

    /// Gets the value of PolicyStore
    pub fn get_policy_store(&self) -> Option<&String> {
        self.policy_store.as_ref()
    }

    /// Sets the value of RefreshInterval
    pub fn set_refresh_interval(&mut self, value: u32) {
        self.refresh_interval = Some(value);
    }

    /// Gets the value of RefreshInterval
    pub fn get_refresh_interval(&self) -> Option<&u32> {
        self.refresh_interval.as_ref()
    }

    /// Sets the value of ServerName
    pub fn set_server_name(&mut self, value: String) {
        self.server_name = Some(value);
    }

    /// Gets the value of ServerName
    pub fn get_server_name(&self) -> Option<&String> {
        self.server_name.as_ref()
    }

    /// Sets the value of ServerShunt
    pub fn set_server_shunt(&mut self, value: bool) {
        self.server_shunt = Some(value);
    }

    /// Gets the value of ServerShunt
    pub fn get_server_shunt(&self) -> Option<&bool> {
        self.server_shunt.as_ref()
    }

    /// Sets the value of ServerVirtualIP
    pub fn set_server_virtual_ip(&mut self, value: String) {
        self.server_virtual_ip = Some(value);
    }

    /// Gets the value of ServerVirtualIP
    pub fn get_server_virtual_ip(&self) -> Option<&String> {
        self.server_virtual_ip.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u32) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u32> {
        self.type.as_ref()
    }

/// 

    /// * `client_port` -  (bool)
    /// * `default_qualified` -  (bool)
    /// * `pass_thru` -  (bool)
    /// * `refresh_interval` -  (bool)
    /// * `server_name` -  (bool)
    /// * `server_shunt` -  (bool)
    /// * `server_virtual_ip` -  (bool)
    /// * `type` -  (bool)

    /// * `output_object` -  (MSFT_NetTeredoConfiguration)
    /// * `return_value` -  (u32)
    pub fn reset(&self, type: bool, server_name: bool, refresh_interval: bool, client_port: bool, server_virtual_ip: bool, default_qualified: bool, server_shunt: bool, pass_thru: bool, output_object: &mut MSFT_NetTeredoConfiguration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });
        args.push(MethodParameter { name: "ServerName".to_string(), value: server_name.into() });
        args.push(MethodParameter { name: "RefreshInterval".to_string(), value: refresh_interval.into() });
        args.push(MethodParameter { name: "ClientPort".to_string(), value: client_port.into() });
        args.push(MethodParameter { name: "ServerVirtualIP".to_string(), value: server_virtual_ip.into() });
        args.push(MethodParameter { name: "DefaultQualified".to_string(), value: default_qualified.into() });
        args.push(MethodParameter { name: "ServerShunt".to_string(), value: server_shunt.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("Reset", &args)?;
        let output_object = result.get_value("OutputObject")?;
        Ok(result.return_value)

    }

}

