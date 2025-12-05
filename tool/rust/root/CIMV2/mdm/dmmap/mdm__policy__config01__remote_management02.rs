// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_RemoteManagement02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_RemoteManagement02 {

/// 
    #[serde(rename = "AllowBasicAuthentication_Client")]
    pub allow_basic_authentication__client: Option<String>,

/// 
    #[serde(rename = "AllowBasicAuthentication_Service")]
    pub allow_basic_authentication__service: Option<String>,

/// 
    #[serde(rename = "AllowCredSSPAuthenticationClient")]
    pub allow_cred_sspauthentication_client: Option<String>,

/// 
    #[serde(rename = "AllowCredSSPAuthenticationService")]
    pub allow_cred_sspauthentication_service: Option<String>,

/// 
    #[serde(rename = "AllowRemoteServerManagement")]
    pub allow_remote_server_management: Option<String>,

/// 
    #[serde(rename = "AllowUnencryptedTraffic_Client")]
    pub allow_unencrypted_traffic__client: Option<String>,

/// 
    #[serde(rename = "AllowUnencryptedTraffic_Service")]
    pub allow_unencrypted_traffic__service: Option<String>,

/// 
    #[serde(rename = "DisallowDigestAuthentication")]
    pub disallow_digest_authentication: Option<String>,

/// 
    #[serde(rename = "DisallowNegotiateAuthenticationClient")]
    pub disallow_negotiate_authentication_client: Option<String>,

/// 
    #[serde(rename = "DisallowNegotiateAuthenticationService")]
    pub disallow_negotiate_authentication_service: Option<String>,

/// 
    #[serde(rename = "DisallowStoringOfRunAsCredentials")]
    pub disallow_storing_of_run_as_credentials: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "SpecifyChannelBindingTokenHardeningLevel")]
    pub specify_channel_binding_token_hardening_level: Option<String>,

/// 
    #[serde(rename = "TrustedHosts")]
    pub trusted_hosts: Option<String>,

/// 
    #[serde(rename = "TurnOnCompatibilityHTTPListener")]
    pub turn_on_compatibility_httplistener: Option<String>,

/// 
    #[serde(rename = "TurnOnCompatibilityHTTPSListener")]
    pub turn_on_compatibility_httpslistener: Option<String>,
}

impl MDM_Policy_Config01_RemoteManagement02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_basic_authentication__client: None,
            allow_basic_authentication__service: None,
            allow_cred_sspauthentication_client: None,
            allow_cred_sspauthentication_service: None,
            allow_remote_server_management: None,
            allow_unencrypted_traffic__client: None,
            allow_unencrypted_traffic__service: None,
            disallow_digest_authentication: None,
            disallow_negotiate_authentication_client: None,
            disallow_negotiate_authentication_service: None,
            disallow_storing_of_run_as_credentials: None,
            instance_id: None,
            parent_id: None,
            specify_channel_binding_token_hardening_level: None,
            trusted_hosts: None,
            turn_on_compatibility_httplistener: None,
            turn_on_compatibility_httpslistener: None,
        }
    }


    /// Sets the value of AllowBasicAuthentication_Client
    pub fn set_allow_basic_authentication__client(&mut self, value: String) {
        self.allow_basic_authentication__client = Some(value);
    }

    /// Gets the value of AllowBasicAuthentication_Client
    pub fn get_allow_basic_authentication__client(&self) -> Option<&String> {
        self.allow_basic_authentication__client.as_ref()
    }

    /// Sets the value of AllowBasicAuthentication_Service
    pub fn set_allow_basic_authentication__service(&mut self, value: String) {
        self.allow_basic_authentication__service = Some(value);
    }

    /// Gets the value of AllowBasicAuthentication_Service
    pub fn get_allow_basic_authentication__service(&self) -> Option<&String> {
        self.allow_basic_authentication__service.as_ref()
    }

    /// Sets the value of AllowCredSSPAuthenticationClient
    pub fn set_allow_cred_sspauthentication_client(&mut self, value: String) {
        self.allow_cred_sspauthentication_client = Some(value);
    }

    /// Gets the value of AllowCredSSPAuthenticationClient
    pub fn get_allow_cred_sspauthentication_client(&self) -> Option<&String> {
        self.allow_cred_sspauthentication_client.as_ref()
    }

    /// Sets the value of AllowCredSSPAuthenticationService
    pub fn set_allow_cred_sspauthentication_service(&mut self, value: String) {
        self.allow_cred_sspauthentication_service = Some(value);
    }

    /// Gets the value of AllowCredSSPAuthenticationService
    pub fn get_allow_cred_sspauthentication_service(&self) -> Option<&String> {
        self.allow_cred_sspauthentication_service.as_ref()
    }

    /// Sets the value of AllowRemoteServerManagement
    pub fn set_allow_remote_server_management(&mut self, value: String) {
        self.allow_remote_server_management = Some(value);
    }

    /// Gets the value of AllowRemoteServerManagement
    pub fn get_allow_remote_server_management(&self) -> Option<&String> {
        self.allow_remote_server_management.as_ref()
    }

    /// Sets the value of AllowUnencryptedTraffic_Client
    pub fn set_allow_unencrypted_traffic__client(&mut self, value: String) {
        self.allow_unencrypted_traffic__client = Some(value);
    }

    /// Gets the value of AllowUnencryptedTraffic_Client
    pub fn get_allow_unencrypted_traffic__client(&self) -> Option<&String> {
        self.allow_unencrypted_traffic__client.as_ref()
    }

    /// Sets the value of AllowUnencryptedTraffic_Service
    pub fn set_allow_unencrypted_traffic__service(&mut self, value: String) {
        self.allow_unencrypted_traffic__service = Some(value);
    }

    /// Gets the value of AllowUnencryptedTraffic_Service
    pub fn get_allow_unencrypted_traffic__service(&self) -> Option<&String> {
        self.allow_unencrypted_traffic__service.as_ref()
    }

    /// Sets the value of DisallowDigestAuthentication
    pub fn set_disallow_digest_authentication(&mut self, value: String) {
        self.disallow_digest_authentication = Some(value);
    }

    /// Gets the value of DisallowDigestAuthentication
    pub fn get_disallow_digest_authentication(&self) -> Option<&String> {
        self.disallow_digest_authentication.as_ref()
    }

    /// Sets the value of DisallowNegotiateAuthenticationClient
    pub fn set_disallow_negotiate_authentication_client(&mut self, value: String) {
        self.disallow_negotiate_authentication_client = Some(value);
    }

    /// Gets the value of DisallowNegotiateAuthenticationClient
    pub fn get_disallow_negotiate_authentication_client(&self) -> Option<&String> {
        self.disallow_negotiate_authentication_client.as_ref()
    }

    /// Sets the value of DisallowNegotiateAuthenticationService
    pub fn set_disallow_negotiate_authentication_service(&mut self, value: String) {
        self.disallow_negotiate_authentication_service = Some(value);
    }

    /// Gets the value of DisallowNegotiateAuthenticationService
    pub fn get_disallow_negotiate_authentication_service(&self) -> Option<&String> {
        self.disallow_negotiate_authentication_service.as_ref()
    }

    /// Sets the value of DisallowStoringOfRunAsCredentials
    pub fn set_disallow_storing_of_run_as_credentials(&mut self, value: String) {
        self.disallow_storing_of_run_as_credentials = Some(value);
    }

    /// Gets the value of DisallowStoringOfRunAsCredentials
    pub fn get_disallow_storing_of_run_as_credentials(&self) -> Option<&String> {
        self.disallow_storing_of_run_as_credentials.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of SpecifyChannelBindingTokenHardeningLevel
    pub fn set_specify_channel_binding_token_hardening_level(&mut self, value: String) {
        self.specify_channel_binding_token_hardening_level = Some(value);
    }

    /// Gets the value of SpecifyChannelBindingTokenHardeningLevel
    pub fn get_specify_channel_binding_token_hardening_level(&self) -> Option<&String> {
        self.specify_channel_binding_token_hardening_level.as_ref()
    }

    /// Sets the value of TrustedHosts
    pub fn set_trusted_hosts(&mut self, value: String) {
        self.trusted_hosts = Some(value);
    }

    /// Gets the value of TrustedHosts
    pub fn get_trusted_hosts(&self) -> Option<&String> {
        self.trusted_hosts.as_ref()
    }

    /// Sets the value of TurnOnCompatibilityHTTPListener
    pub fn set_turn_on_compatibility_httplistener(&mut self, value: String) {
        self.turn_on_compatibility_httplistener = Some(value);
    }

    /// Gets the value of TurnOnCompatibilityHTTPListener
    pub fn get_turn_on_compatibility_httplistener(&self) -> Option<&String> {
        self.turn_on_compatibility_httplistener.as_ref()
    }

    /// Sets the value of TurnOnCompatibilityHTTPSListener
    pub fn set_turn_on_compatibility_httpslistener(&mut self, value: String) {
        self.turn_on_compatibility_httpslistener = Some(value);
    }

    /// Gets the value of TurnOnCompatibilityHTTPSListener
    pub fn get_turn_on_compatibility_httpslistener(&self) -> Option<&String> {
        self.turn_on_compatibility_httpslistener.as_ref()
    }
}

