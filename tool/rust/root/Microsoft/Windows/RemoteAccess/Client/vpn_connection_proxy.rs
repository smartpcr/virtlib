// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// VpnConnectionProxy struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VpnConnectionProxy {

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
    #[serde(rename = "ConnectionName")]
    pub connection_name: Option<String>,

/// 
    #[serde(rename = "ExceptionPrefix")]
    pub exception_prefix: Vec<String>,

/// 
    #[serde(rename = "ProxyServer")]
    pub proxy_server: Option<String>,
}

impl VpnConnectionProxy {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            auto_configuration_script: None,
            auto_detect: None,
            bypass_proxy_for_local: None,
            connection_name: None,
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

    /// Sets the value of ConnectionName
    pub fn set_connection_name(&mut self, value: String) {
        self.connection_name = Some(value);
    }

    /// Gets the value of ConnectionName
    pub fn get_connection_name(&self) -> Option<&String> {
        self.connection_name.as_ref()
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
}

