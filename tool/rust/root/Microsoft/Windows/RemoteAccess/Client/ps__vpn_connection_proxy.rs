// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PS_VpnConnectionProxy struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PS_VpnConnectionProxy {

/// 
    #[serde(rename = "AutoConfigurationScript")]
    pub auto_configuration_script: Option<String>,

/// 
    #[serde(rename = "AutoDetect")]
    pub auto_detect: Option<bool>,

/// 
    #[serde(rename = "BypassProxyForLocal")]
    pub bypass_proxy_for_local: Option<bool>,

/// 
    #[serde(rename = "ExceptionPrefix")]
    pub exception_prefix: Vec<String>,

/// 
    #[serde(rename = "ProxyServer")]
    pub proxy_server: Option<String>,
}

impl PS_VpnConnectionProxy {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            auto_configuration_script: None,
            auto_detect: None,
            bypass_proxy_for_local: None,
            exception_prefix: Vec::new(),
            proxy_server: None,
        }
    }


    /// Sets the value of AutoConfigurationScript
    pub fn set_auto_configuration_script(&mut self, value: String) {
        self.auto_configuration_script = Some(value);
    }

    /// Gets the value of AutoConfigurationScript
    pub fn get_auto_configuration_script(&self) -> Option<&String> {
        self.auto_configuration_script.as_ref()
    }

    /// Sets the value of AutoDetect
    pub fn set_auto_detect(&mut self, value: bool) {
        self.auto_detect = Some(value);
    }

    /// Gets the value of AutoDetect
    pub fn get_auto_detect(&self) -> Option<&bool> {
        self.auto_detect.as_ref()
    }

    /// Sets the value of BypassProxyForLocal
    pub fn set_bypass_proxy_for_local(&mut self, value: bool) {
        self.bypass_proxy_for_local = Some(value);
    }

    /// Gets the value of BypassProxyForLocal
    pub fn get_bypass_proxy_for_local(&self) -> Option<&bool> {
        self.bypass_proxy_for_local.as_ref()
    }

    /// Sets the value of ExceptionPrefix
    pub fn set_exception_prefix(&mut self, value: Vec<String>) {
        self.exception_prefix = value;
    }

    /// Gets the value of ExceptionPrefix
    pub fn get_exception_prefix(&self) -> &Vec<String> {
        &self.exception_prefix
    }

    /// Sets the value of ProxyServer
    pub fn set_proxy_server(&mut self, value: String) {
        self.proxy_server = Some(value);
    }

    /// Gets the value of ProxyServer
    pub fn get_proxy_server(&self) -> Option<&String> {
        self.proxy_server.as_ref()
    }

/// 

    /// * `auto_configuration_script` -  (String)
    /// * `auto_detect` -  (bool)
    /// * `bypass_proxy_for_local` -  (bool)
    /// * `connection_name` -  (String)
    /// * `exception_prefix` -  (String[])
    /// * `pass_thru` -  (bool)
    /// * `proxy_server` -  (String)

    /// * `cmdlet_output` -  (VpnConnectionProxy)
    /// * `return_value` -  (u32)
    pub fn set(&self, auto_detect: bool, auto_configuration_script: &String, proxy_server: &String, bypass_proxy_for_local: bool, exception_prefix: &Vec<String>, connection_name: &String, pass_thru: bool, cmdlet_output: &mut VpnConnectionProxy) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AutoDetect".to_string(), value: auto_detect.into() });
        args.push(MethodParameter { name: "AutoConfigurationScript".to_string(), value: auto_configuration_script.into() });
        args.push(MethodParameter { name: "ProxyServer".to_string(), value: proxy_server.into() });
        args.push(MethodParameter { name: "BypassProxyForLocal".to_string(), value: bypass_proxy_for_local.into() });
        args.push(MethodParameter { name: "ExceptionPrefix".to_string(), value: exception_prefix.into() });
        args.push(MethodParameter { name: "ConnectionName".to_string(), value: connection_name.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("Set", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

