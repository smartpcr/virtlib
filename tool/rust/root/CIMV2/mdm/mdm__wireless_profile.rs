// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_WirelessProfile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_WirelessProfile {

/// 
    #[serde(rename = "AutoConnect")]
    pub auto_connect: Option<bool>,

/// 
    #[serde(rename = "ConnectionType")]
    pub connection_type: Option<u8>,

/// 
    #[serde(rename = "ConnectToMorePreferedNetwork")]
    pub connect_to_more_prefered_network: Option<bool>,

/// 
    #[serde(rename = "ConnectWhenNotBroadcasting")]
    pub connect_when_not_broadcasting: Option<bool>,

/// 
    #[serde(rename = "EnableFIPSCompliance")]
    pub enable_fipscompliance: Option<bool>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "OneXAuthenticationMode")]
    pub one_xauthentication_mode: Option<u8>,

/// 
    #[serde(rename = "OneXCacheUserData")]
    pub one_xcache_user_data: Option<bool>,

/// 
    #[serde(rename = "OneXEAPType")]
    pub one_xeaptype: Option<u8>,

/// 
    #[serde(rename = "OneXEAPXml")]
    pub one_xeapxml: Option<String>,

/// 
    #[serde(rename = "OneXSingleSignOnAllowAdditionalDialogs")]
    pub one_xsingle_sign_on_allow_additional_dialogs: Option<bool>,

/// 
    #[serde(rename = "OneXSingleSignOnMaxDelay")]
    pub one_xsingle_sign_on_max_delay: Option<u32>,

/// 
    #[serde(rename = "OneXSingleSignOnType")]
    pub one_xsingle_sign_on_type: Option<u8>,

/// 
    #[serde(rename = "OneXSingleSignOnUserBasedVirtualLAN")]
    pub one_xsingle_sign_on_user_based_virtual_lan: Option<bool>,

/// 
    #[serde(rename = "PMKCacheMode")]
    pub pmkcache_mode: Option<bool>,

/// 
    #[serde(rename = "PMKCacheSize")]
    pub pmkcache_size: Option<u32>,

/// 
    #[serde(rename = "PMKCacheTTL")]
    pub pmkcache_ttl: Option<u32>,

/// 
    #[serde(rename = "PreAuthMode")]
    pub pre_auth_mode: Option<bool>,

/// 
    #[serde(rename = "PreAuthThrottle")]
    pub pre_auth_throttle: Option<u32>,

/// 
    #[serde(rename = "SecurityAuthentication")]
    pub security_authentication: Option<u8>,

/// 
    #[serde(rename = "SecurityEncryption")]
    pub security_encryption: Option<u8>,

/// 
    #[serde(rename = "SharedKeyMaterial")]
    pub shared_key_material: Option<String>,

/// 
    #[serde(rename = "SharedKeyProtected")]
    pub shared_key_protected: Option<bool>,

/// 
    #[serde(rename = "SharedKeyType")]
    pub shared_key_type: Option<u8>,

/// 
    #[serde(rename = "SSID")]
    pub ssid: Option<String>,
}

impl MDM_WirelessProfile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            auto_connect: None,
            connection_type: None,
            connect_to_more_prefered_network: None,
            connect_when_not_broadcasting: None,
            enable_fipscompliance: None,
            name: None,
            one_xauthentication_mode: None,
            one_xcache_user_data: None,
            one_xeaptype: None,
            one_xeapxml: None,
            one_xsingle_sign_on_allow_additional_dialogs: None,
            one_xsingle_sign_on_max_delay: None,
            one_xsingle_sign_on_type: None,
            one_xsingle_sign_on_user_based_virtual_lan: None,
            pmkcache_mode: None,
            pmkcache_size: None,
            pmkcache_ttl: None,
            pre_auth_mode: None,
            pre_auth_throttle: None,
            security_authentication: None,
            security_encryption: None,
            shared_key_material: None,
            shared_key_protected: None,
            shared_key_type: None,
            ssid: None,
        }
    }


    /// Sets the value of AutoConnect
    pub fn set_auto_connect(&mut self, value: bool) {
        self.auto_connect = Some(value);
    }

    /// Gets the value of AutoConnect
    pub fn get_auto_connect(&self) -> Option<&bool> {
        self.auto_connect.as_ref()
    }

    /// Sets the value of ConnectionType
    pub fn set_connection_type(&mut self, value: u8) {
        self.connection_type = Some(value);
    }

    /// Gets the value of ConnectionType
    pub fn get_connection_type(&self) -> Option<&u8> {
        self.connection_type.as_ref()
    }

    /// Sets the value of ConnectToMorePreferedNetwork
    pub fn set_connect_to_more_prefered_network(&mut self, value: bool) {
        self.connect_to_more_prefered_network = Some(value);
    }

    /// Gets the value of ConnectToMorePreferedNetwork
    pub fn get_connect_to_more_prefered_network(&self) -> Option<&bool> {
        self.connect_to_more_prefered_network.as_ref()
    }

    /// Sets the value of ConnectWhenNotBroadcasting
    pub fn set_connect_when_not_broadcasting(&mut self, value: bool) {
        self.connect_when_not_broadcasting = Some(value);
    }

    /// Gets the value of ConnectWhenNotBroadcasting
    pub fn get_connect_when_not_broadcasting(&self) -> Option<&bool> {
        self.connect_when_not_broadcasting.as_ref()
    }

    /// Sets the value of EnableFIPSCompliance
    pub fn set_enable_fipscompliance(&mut self, value: bool) {
        self.enable_fipscompliance = Some(value);
    }

    /// Gets the value of EnableFIPSCompliance
    pub fn get_enable_fipscompliance(&self) -> Option<&bool> {
        self.enable_fipscompliance.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of OneXAuthenticationMode
    pub fn set_one_xauthentication_mode(&mut self, value: u8) {
        self.one_xauthentication_mode = Some(value);
    }

    /// Gets the value of OneXAuthenticationMode
    pub fn get_one_xauthentication_mode(&self) -> Option<&u8> {
        self.one_xauthentication_mode.as_ref()
    }

    /// Sets the value of OneXCacheUserData
    pub fn set_one_xcache_user_data(&mut self, value: bool) {
        self.one_xcache_user_data = Some(value);
    }

    /// Gets the value of OneXCacheUserData
    pub fn get_one_xcache_user_data(&self) -> Option<&bool> {
        self.one_xcache_user_data.as_ref()
    }

    /// Sets the value of OneXEAPType
    pub fn set_one_xeaptype(&mut self, value: u8) {
        self.one_xeaptype = Some(value);
    }

    /// Gets the value of OneXEAPType
    pub fn get_one_xeaptype(&self) -> Option<&u8> {
        self.one_xeaptype.as_ref()
    }

    /// Sets the value of OneXEAPXml
    pub fn set_one_xeapxml(&mut self, value: String) {
        self.one_xeapxml = Some(value);
    }

    /// Gets the value of OneXEAPXml
    pub fn get_one_xeapxml(&self) -> Option<&String> {
        self.one_xeapxml.as_ref()
    }

    /// Sets the value of OneXSingleSignOnAllowAdditionalDialogs
    pub fn set_one_xsingle_sign_on_allow_additional_dialogs(&mut self, value: bool) {
        self.one_xsingle_sign_on_allow_additional_dialogs = Some(value);
    }

    /// Gets the value of OneXSingleSignOnAllowAdditionalDialogs
    pub fn get_one_xsingle_sign_on_allow_additional_dialogs(&self) -> Option<&bool> {
        self.one_xsingle_sign_on_allow_additional_dialogs.as_ref()
    }

    /// Sets the value of OneXSingleSignOnMaxDelay
    pub fn set_one_xsingle_sign_on_max_delay(&mut self, value: u32) {
        self.one_xsingle_sign_on_max_delay = Some(value);
    }

    /// Gets the value of OneXSingleSignOnMaxDelay
    pub fn get_one_xsingle_sign_on_max_delay(&self) -> Option<&u32> {
        self.one_xsingle_sign_on_max_delay.as_ref()
    }

    /// Sets the value of OneXSingleSignOnType
    pub fn set_one_xsingle_sign_on_type(&mut self, value: u8) {
        self.one_xsingle_sign_on_type = Some(value);
    }

    /// Gets the value of OneXSingleSignOnType
    pub fn get_one_xsingle_sign_on_type(&self) -> Option<&u8> {
        self.one_xsingle_sign_on_type.as_ref()
    }

    /// Sets the value of OneXSingleSignOnUserBasedVirtualLAN
    pub fn set_one_xsingle_sign_on_user_based_virtual_lan(&mut self, value: bool) {
        self.one_xsingle_sign_on_user_based_virtual_lan = Some(value);
    }

    /// Gets the value of OneXSingleSignOnUserBasedVirtualLAN
    pub fn get_one_xsingle_sign_on_user_based_virtual_lan(&self) -> Option<&bool> {
        self.one_xsingle_sign_on_user_based_virtual_lan.as_ref()
    }

    /// Sets the value of PMKCacheMode
    pub fn set_pmkcache_mode(&mut self, value: bool) {
        self.pmkcache_mode = Some(value);
    }

    /// Gets the value of PMKCacheMode
    pub fn get_pmkcache_mode(&self) -> Option<&bool> {
        self.pmkcache_mode.as_ref()
    }

    /// Sets the value of PMKCacheSize
    pub fn set_pmkcache_size(&mut self, value: u32) {
        self.pmkcache_size = Some(value);
    }

    /// Gets the value of PMKCacheSize
    pub fn get_pmkcache_size(&self) -> Option<&u32> {
        self.pmkcache_size.as_ref()
    }

    /// Sets the value of PMKCacheTTL
    pub fn set_pmkcache_ttl(&mut self, value: u32) {
        self.pmkcache_ttl = Some(value);
    }

    /// Gets the value of PMKCacheTTL
    pub fn get_pmkcache_ttl(&self) -> Option<&u32> {
        self.pmkcache_ttl.as_ref()
    }

    /// Sets the value of PreAuthMode
    pub fn set_pre_auth_mode(&mut self, value: bool) {
        self.pre_auth_mode = Some(value);
    }

    /// Gets the value of PreAuthMode
    pub fn get_pre_auth_mode(&self) -> Option<&bool> {
        self.pre_auth_mode.as_ref()
    }

    /// Sets the value of PreAuthThrottle
    pub fn set_pre_auth_throttle(&mut self, value: u32) {
        self.pre_auth_throttle = Some(value);
    }

    /// Gets the value of PreAuthThrottle
    pub fn get_pre_auth_throttle(&self) -> Option<&u32> {
        self.pre_auth_throttle.as_ref()
    }

    /// Sets the value of SecurityAuthentication
    pub fn set_security_authentication(&mut self, value: u8) {
        self.security_authentication = Some(value);
    }

    /// Gets the value of SecurityAuthentication
    pub fn get_security_authentication(&self) -> Option<&u8> {
        self.security_authentication.as_ref()
    }

    /// Sets the value of SecurityEncryption
    pub fn set_security_encryption(&mut self, value: u8) {
        self.security_encryption = Some(value);
    }

    /// Gets the value of SecurityEncryption
    pub fn get_security_encryption(&self) -> Option<&u8> {
        self.security_encryption.as_ref()
    }

    /// Sets the value of SharedKeyMaterial
    pub fn set_shared_key_material(&mut self, value: String) {
        self.shared_key_material = Some(value);
    }

    /// Gets the value of SharedKeyMaterial
    pub fn get_shared_key_material(&self) -> Option<&String> {
        self.shared_key_material.as_ref()
    }

    /// Sets the value of SharedKeyProtected
    pub fn set_shared_key_protected(&mut self, value: bool) {
        self.shared_key_protected = Some(value);
    }

    /// Gets the value of SharedKeyProtected
    pub fn get_shared_key_protected(&self) -> Option<&bool> {
        self.shared_key_protected.as_ref()
    }

    /// Sets the value of SharedKeyType
    pub fn set_shared_key_type(&mut self, value: u8) {
        self.shared_key_type = Some(value);
    }

    /// Gets the value of SharedKeyType
    pub fn get_shared_key_type(&self) -> Option<&u8> {
        self.shared_key_type.as_ref()
    }

    /// Sets the value of SSID
    pub fn set_ssid(&mut self, value: String) {
        self.ssid = Some(value);
    }

    /// Gets the value of SSID
    pub fn get_ssid(&self) -> Option<&String> {
        self.ssid.as_ref()
    }
}

