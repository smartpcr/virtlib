// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IEProxySettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IEProxySettings {

/// 
    #[serde(rename = "enableProxy")]
    pub enable_proxy: Option<bool>,

/// 
    #[serde(rename = "ftpProxyServer")]
    pub ftp_proxy_server: Option<String>,

/// 
    #[serde(rename = "gopherProxyServer")]
    pub gopher_proxy_server: Option<String>,

/// 
    #[serde(rename = "httpProxyServer")]
    pub http_proxy_server: Option<String>,

/// 
    #[serde(rename = "proxyOverride")]
    pub proxy_override: Option<String>,

/// 
    #[serde(rename = "rsopID")]
    pub rsop_id: Option<String>,

/// 
    #[serde(rename = "rsopPrecedence")]
    pub rsop_precedence: Option<u32>,

/// 
    #[serde(rename = "secureProxyServer")]
    pub secure_proxy_server: Option<String>,

/// 
    #[serde(rename = "socksProxyServer")]
    pub socks_proxy_server: Option<String>,

/// 
    #[serde(rename = "useSameProxy")]
    pub use_same_proxy: Option<bool>,
}

impl RSOP_IEProxySettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            enable_proxy: None,
            ftp_proxy_server: None,
            gopher_proxy_server: None,
            http_proxy_server: None,
            proxy_override: None,
            rsop_id: None,
            rsop_precedence: None,
            secure_proxy_server: None,
            socks_proxy_server: None,
            use_same_proxy: None,
        }
    }


    /// Sets the value of enableProxy
    pub fn set_enable_proxy(&mut self, value: bool) {
        self.enable_proxy = Some(value);
    }

    /// Gets the value of enableProxy
    pub fn get_enable_proxy(&self) -> Option<&bool> {
        self.enable_proxy.as_ref()
    }

    /// Sets the value of ftpProxyServer
    pub fn set_ftp_proxy_server(&mut self, value: String) {
        self.ftp_proxy_server = Some(value);
    }

    /// Gets the value of ftpProxyServer
    pub fn get_ftp_proxy_server(&self) -> Option<&String> {
        self.ftp_proxy_server.as_ref()
    }

    /// Sets the value of gopherProxyServer
    pub fn set_gopher_proxy_server(&mut self, value: String) {
        self.gopher_proxy_server = Some(value);
    }

    /// Gets the value of gopherProxyServer
    pub fn get_gopher_proxy_server(&self) -> Option<&String> {
        self.gopher_proxy_server.as_ref()
    }

    /// Sets the value of httpProxyServer
    pub fn set_http_proxy_server(&mut self, value: String) {
        self.http_proxy_server = Some(value);
    }

    /// Gets the value of httpProxyServer
    pub fn get_http_proxy_server(&self) -> Option<&String> {
        self.http_proxy_server.as_ref()
    }

    /// Sets the value of proxyOverride
    pub fn set_proxy_override(&mut self, value: String) {
        self.proxy_override = Some(value);
    }

    /// Gets the value of proxyOverride
    pub fn get_proxy_override(&self) -> Option<&String> {
        self.proxy_override.as_ref()
    }

    /// Sets the value of rsopID
    pub fn set_rsop_id(&mut self, value: String) {
        self.rsop_id = Some(value);
    }

    /// Gets the value of rsopID
    pub fn get_rsop_id(&self) -> Option<&String> {
        self.rsop_id.as_ref()
    }

    /// Sets the value of rsopPrecedence
    pub fn set_rsop_precedence(&mut self, value: u32) {
        self.rsop_precedence = Some(value);
    }

    /// Gets the value of rsopPrecedence
    pub fn get_rsop_precedence(&self) -> Option<&u32> {
        self.rsop_precedence.as_ref()
    }

    /// Sets the value of secureProxyServer
    pub fn set_secure_proxy_server(&mut self, value: String) {
        self.secure_proxy_server = Some(value);
    }

    /// Gets the value of secureProxyServer
    pub fn get_secure_proxy_server(&self) -> Option<&String> {
        self.secure_proxy_server.as_ref()
    }

    /// Sets the value of socksProxyServer
    pub fn set_socks_proxy_server(&mut self, value: String) {
        self.socks_proxy_server = Some(value);
    }

    /// Gets the value of socksProxyServer
    pub fn get_socks_proxy_server(&self) -> Option<&String> {
        self.socks_proxy_server.as_ref()
    }

    /// Sets the value of useSameProxy
    pub fn set_use_same_proxy(&mut self, value: bool) {
        self.use_same_proxy = Some(value);
    }

    /// Gets the value of useSameProxy
    pub fn get_use_same_proxy(&self) -> Option<&bool> {
        self.use_same_proxy.as_ref()
    }
}

