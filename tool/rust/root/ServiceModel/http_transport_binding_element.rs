// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HttpTransportBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HttpTransportBindingElement {
    #[serde(flatten)]
    pub base: TransportBindingElement,

/// A value that indicates whether the client accepts cookies and propagates them on future requests.
    #[serde(rename = "AllowCookies")]
    pub allow_cookies: Option<bool>,

/// The authentication scheme used to authenticate client requests being processed by an HTTP listener. 
    #[serde(rename = "AuthenticationScheme")]
    pub authentication_scheme: Option<String>,

/// A value that indicates whether proxies are ignored for local addresses.
    #[serde(rename = "BypassProxyOnLocal")]
    pub bypass_proxy_on_local: Option<bool>,

/// A value that indicates whether HTTP decompression (gzip/deflate) is enabled on the client.
    #[serde(rename = "DecompressionEnabled")]
    pub decompression_enabled: Option<bool>,

/// The extended protection policy used by the server to validate incoming client connections.
    #[serde(rename = "ExtendedProtectionPolicy")]
    pub extended_protection_policy: Option<ExtendedProtectionPolicy>,

/// A value that indicates whether the hostname is used to reach the service when matching on the URI.
    #[serde(rename = "HostNameComparisonMode")]
    pub host_name_comparison_mode: Option<String>,

/// When enabled http connections are kept alive regardless of activity level.
    #[serde(rename = "KeepAliveEnabled")]
    pub keep_alive_enabled: Option<bool>,

/// The maximum size of the buffer pool
    #[serde(rename = "MaxBufferSize")]
    pub max_buffer_size: Option<i32>,

/// A URI that contains the address of the proxy to use for HTTP requests.
    #[serde(rename = "ProxyAddress")]
    pub proxy_address: Option<String>,

/// The authentication scheme used to authenticate client requests being processed by an HTTP proxy.
    #[serde(rename = "ProxyAuthenticationScheme")]
    pub proxy_authentication_scheme: Option<String>,

/// The authentication realm.
    #[serde(rename = "Realm")]
    pub realm: Option<String>,

/// A value that specifies whether messages are buffered or streamed or a request or response.
    #[serde(rename = "TransferMode")]
    pub transfer_mode: Option<String>,

/// A value that indicates whether Unsafe Connection Sharing is enabled on the server.
    #[serde(rename = "UnsafeConnectionNtlmAuthentication")]
    pub unsafe_connection_ntlm_authentication: Option<bool>,

/// A value that indicates whether the machine-wide proxy settings are used rather than the user specific settings.
    #[serde(rename = "UseDefaultWebProxy")]
    pub use_default_web_proxy: Option<bool>,
}

impl HttpTransportBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: TransportBindingElement::new(),
            allow_cookies: None,
            authentication_scheme: None,
            bypass_proxy_on_local: None,
            decompression_enabled: None,
            extended_protection_policy: None,
            host_name_comparison_mode: None,
            keep_alive_enabled: None,
            max_buffer_size: None,
            proxy_address: None,
            proxy_authentication_scheme: None,
            realm: None,
            transfer_mode: None,
            unsafe_connection_ntlm_authentication: None,
            use_default_web_proxy: None,
        }
    }


    /// Sets the value of AllowCookies
    pub fn set_allow_cookies(&mut self, value: bool) {
        self.allow_cookies = Some(value);
    }

    /// Gets the value of AllowCookies
    pub fn get_allow_cookies(&self) -> Option<&bool> {
        self.allow_cookies.as_ref()
    }

    /// Sets the value of AuthenticationScheme
    pub fn set_authentication_scheme(&mut self, value: String) {
        self.authentication_scheme = Some(value);
    }

    /// Gets the value of AuthenticationScheme
    pub fn get_authentication_scheme(&self) -> Option<&String> {
        self.authentication_scheme.as_ref()
    }

    /// Sets the value of BypassProxyOnLocal
    pub fn set_bypass_proxy_on_local(&mut self, value: bool) {
        self.bypass_proxy_on_local = Some(value);
    }

    /// Gets the value of BypassProxyOnLocal
    pub fn get_bypass_proxy_on_local(&self) -> Option<&bool> {
        self.bypass_proxy_on_local.as_ref()
    }

    /// Sets the value of DecompressionEnabled
    pub fn set_decompression_enabled(&mut self, value: bool) {
        self.decompression_enabled = Some(value);
    }

    /// Gets the value of DecompressionEnabled
    pub fn get_decompression_enabled(&self) -> Option<&bool> {
        self.decompression_enabled.as_ref()
    }

    /// Sets the value of ExtendedProtectionPolicy
    pub fn set_extended_protection_policy(&mut self, value: ExtendedProtectionPolicy) {
        self.extended_protection_policy = Some(value);
    }

    /// Gets the value of ExtendedProtectionPolicy
    pub fn get_extended_protection_policy(&self) -> Option<&ExtendedProtectionPolicy> {
        self.extended_protection_policy.as_ref()
    }

    /// Sets the value of HostNameComparisonMode
    pub fn set_host_name_comparison_mode(&mut self, value: String) {
        self.host_name_comparison_mode = Some(value);
    }

    /// Gets the value of HostNameComparisonMode
    pub fn get_host_name_comparison_mode(&self) -> Option<&String> {
        self.host_name_comparison_mode.as_ref()
    }

    /// Sets the value of KeepAliveEnabled
    pub fn set_keep_alive_enabled(&mut self, value: bool) {
        self.keep_alive_enabled = Some(value);
    }

    /// Gets the value of KeepAliveEnabled
    pub fn get_keep_alive_enabled(&self) -> Option<&bool> {
        self.keep_alive_enabled.as_ref()
    }

    /// Sets the value of MaxBufferSize
    pub fn set_max_buffer_size(&mut self, value: i32) {
        self.max_buffer_size = Some(value);
    }

    /// Gets the value of MaxBufferSize
    pub fn get_max_buffer_size(&self) -> Option<&i32> {
        self.max_buffer_size.as_ref()
    }

    /// Sets the value of ProxyAddress
    pub fn set_proxy_address(&mut self, value: String) {
        self.proxy_address = Some(value);
    }

    /// Gets the value of ProxyAddress
    pub fn get_proxy_address(&self) -> Option<&String> {
        self.proxy_address.as_ref()
    }

    /// Sets the value of ProxyAuthenticationScheme
    pub fn set_proxy_authentication_scheme(&mut self, value: String) {
        self.proxy_authentication_scheme = Some(value);
    }

    /// Gets the value of ProxyAuthenticationScheme
    pub fn get_proxy_authentication_scheme(&self) -> Option<&String> {
        self.proxy_authentication_scheme.as_ref()
    }

    /// Sets the value of Realm
    pub fn set_realm(&mut self, value: String) {
        self.realm = Some(value);
    }

    /// Gets the value of Realm
    pub fn get_realm(&self) -> Option<&String> {
        self.realm.as_ref()
    }

    /// Sets the value of TransferMode
    pub fn set_transfer_mode(&mut self, value: String) {
        self.transfer_mode = Some(value);
    }

    /// Gets the value of TransferMode
    pub fn get_transfer_mode(&self) -> Option<&String> {
        self.transfer_mode.as_ref()
    }

    /// Sets the value of UnsafeConnectionNtlmAuthentication
    pub fn set_unsafe_connection_ntlm_authentication(&mut self, value: bool) {
        self.unsafe_connection_ntlm_authentication = Some(value);
    }

    /// Gets the value of UnsafeConnectionNtlmAuthentication
    pub fn get_unsafe_connection_ntlm_authentication(&self) -> Option<&bool> {
        self.unsafe_connection_ntlm_authentication.as_ref()
    }

    /// Sets the value of UseDefaultWebProxy
    pub fn set_use_default_web_proxy(&mut self, value: bool) {
        self.use_default_web_proxy = Some(value);
    }

    /// Gets the value of UseDefaultWebProxy
    pub fn get_use_default_web_proxy(&self) -> Option<&bool> {
        self.use_default_web_proxy.as_ref()
    }
}

