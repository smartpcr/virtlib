// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_SecuritySystemWideStatistics struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_SecuritySystemWideStatistics {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "ActiveSchannelSessionCacheEntries")]
    pub active_schannel_session_cache_entries: Option<u32>,

/// 
    #[serde(rename = "DigestAuthentications")]
    pub digest_authentications: Option<u32>,

/// 
    #[serde(rename = "ForwardedKerberosRequests")]
    pub forwarded_kerberos_requests: Option<u32>,

/// 
    #[serde(rename = "KDCarmoredASRequests")]
    pub kdcarmored_asrequests: Option<u32>,

/// 
    #[serde(rename = "KDCarmoredTGSRequests")]
    pub kdcarmored_tgsrequests: Option<u32>,

/// 
    #[serde(rename = "KDCASRequests")]
    pub kdcasrequests: Option<u32>,

/// 
    #[serde(rename = "KDCclaimsawareASRequests")]
    pub kdcclaimsaware_asrequests: Option<u32>,

/// 
    #[serde(rename = "KDCclaimsawareserviceassertedidentityTGSrequests")]
    pub kdcclaimsawareserviceassertedidentity_tgsrequests: Option<u32>,

/// 
    #[serde(rename = "KDCclaimsawareTGSRequests")]
    pub kdcclaimsaware_tgsrequests: Option<u32>,

/// 
    #[serde(rename = "KDCclassictypeconstraineddelegationTGSRequests")]
    pub kdcclassictypeconstraineddelegation_tgsrequests: Option<u32>,

/// 
    #[serde(rename = "KDCkeytrustASRequests")]
    pub kdckeytrust_asrequests: Option<u32>,

/// 
    #[serde(rename = "KDCresourcetypeconstraineddelegationTGSRequests")]
    pub kdcresourcetypeconstraineddelegation_tgsrequests: Option<u32>,

/// 
    #[serde(rename = "KDCTGSRequests")]
    pub kdctgsrequests: Option<u32>,

/// 
    #[serde(rename = "KerberosAuthentications")]
    pub kerberos_authentications: Option<u32>,

/// 
    #[serde(rename = "NTLMAuthentications")]
    pub ntlmauthentications: Option<u32>,

/// 
    #[serde(rename = "SchannelSessionCacheEntries")]
    pub schannel_session_cache_entries: Option<u32>,

/// 
    #[serde(rename = "SSLClientSideFullHandshakes")]
    pub sslclient_side_full_handshakes: Option<u32>,

/// 
    #[serde(rename = "SSLClientSideReconnectHandshakes")]
    pub sslclient_side_reconnect_handshakes: Option<u32>,

/// 
    #[serde(rename = "SSLServerSideFullHandshakes")]
    pub sslserver_side_full_handshakes: Option<u32>,

/// 
    #[serde(rename = "SSLServerSideReconnectHandshakes")]
    pub sslserver_side_reconnect_handshakes: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_SecuritySystemWideStatistics {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            active_schannel_session_cache_entries: None,
            digest_authentications: None,
            forwarded_kerberos_requests: None,
            kdcarmored_asrequests: None,
            kdcarmored_tgsrequests: None,
            kdcasrequests: None,
            kdcclaimsaware_asrequests: None,
            kdcclaimsawareserviceassertedidentity_tgsrequests: None,
            kdcclaimsaware_tgsrequests: None,
            kdcclassictypeconstraineddelegation_tgsrequests: None,
            kdckeytrust_asrequests: None,
            kdcresourcetypeconstraineddelegation_tgsrequests: None,
            kdctgsrequests: None,
            kerberos_authentications: None,
            ntlmauthentications: None,
            schannel_session_cache_entries: None,
            sslclient_side_full_handshakes: None,
            sslclient_side_reconnect_handshakes: None,
            sslserver_side_full_handshakes: None,
            sslserver_side_reconnect_handshakes: None,
        }
    }


    /// Sets the value of ActiveSchannelSessionCacheEntries
    pub fn set_active_schannel_session_cache_entries(&mut self, value: u32) {
        self.active_schannel_session_cache_entries = Some(value);
    }

    /// Gets the value of ActiveSchannelSessionCacheEntries
    pub fn get_active_schannel_session_cache_entries(&self) -> Option<&u32> {
        self.active_schannel_session_cache_entries.as_ref()
    }

    /// Sets the value of DigestAuthentications
    pub fn set_digest_authentications(&mut self, value: u32) {
        self.digest_authentications = Some(value);
    }

    /// Gets the value of DigestAuthentications
    pub fn get_digest_authentications(&self) -> Option<&u32> {
        self.digest_authentications.as_ref()
    }

    /// Sets the value of ForwardedKerberosRequests
    pub fn set_forwarded_kerberos_requests(&mut self, value: u32) {
        self.forwarded_kerberos_requests = Some(value);
    }

    /// Gets the value of ForwardedKerberosRequests
    pub fn get_forwarded_kerberos_requests(&self) -> Option<&u32> {
        self.forwarded_kerberos_requests.as_ref()
    }

    /// Sets the value of KDCarmoredASRequests
    pub fn set_kdcarmored_asrequests(&mut self, value: u32) {
        self.kdcarmored_asrequests = Some(value);
    }

    /// Gets the value of KDCarmoredASRequests
    pub fn get_kdcarmored_asrequests(&self) -> Option<&u32> {
        self.kdcarmored_asrequests.as_ref()
    }

    /// Sets the value of KDCarmoredTGSRequests
    pub fn set_kdcarmored_tgsrequests(&mut self, value: u32) {
        self.kdcarmored_tgsrequests = Some(value);
    }

    /// Gets the value of KDCarmoredTGSRequests
    pub fn get_kdcarmored_tgsrequests(&self) -> Option<&u32> {
        self.kdcarmored_tgsrequests.as_ref()
    }

    /// Sets the value of KDCASRequests
    pub fn set_kdcasrequests(&mut self, value: u32) {
        self.kdcasrequests = Some(value);
    }

    /// Gets the value of KDCASRequests
    pub fn get_kdcasrequests(&self) -> Option<&u32> {
        self.kdcasrequests.as_ref()
    }

    /// Sets the value of KDCclaimsawareASRequests
    pub fn set_kdcclaimsaware_asrequests(&mut self, value: u32) {
        self.kdcclaimsaware_asrequests = Some(value);
    }

    /// Gets the value of KDCclaimsawareASRequests
    pub fn get_kdcclaimsaware_asrequests(&self) -> Option<&u32> {
        self.kdcclaimsaware_asrequests.as_ref()
    }

    /// Sets the value of KDCclaimsawareserviceassertedidentityTGSrequests
    pub fn set_kdcclaimsawareserviceassertedidentity_tgsrequests(&mut self, value: u32) {
        self.kdcclaimsawareserviceassertedidentity_tgsrequests = Some(value);
    }

    /// Gets the value of KDCclaimsawareserviceassertedidentityTGSrequests
    pub fn get_kdcclaimsawareserviceassertedidentity_tgsrequests(&self) -> Option<&u32> {
        self.kdcclaimsawareserviceassertedidentity_tgsrequests.as_ref()
    }

    /// Sets the value of KDCclaimsawareTGSRequests
    pub fn set_kdcclaimsaware_tgsrequests(&mut self, value: u32) {
        self.kdcclaimsaware_tgsrequests = Some(value);
    }

    /// Gets the value of KDCclaimsawareTGSRequests
    pub fn get_kdcclaimsaware_tgsrequests(&self) -> Option<&u32> {
        self.kdcclaimsaware_tgsrequests.as_ref()
    }

    /// Sets the value of KDCclassictypeconstraineddelegationTGSRequests
    pub fn set_kdcclassictypeconstraineddelegation_tgsrequests(&mut self, value: u32) {
        self.kdcclassictypeconstraineddelegation_tgsrequests = Some(value);
    }

    /// Gets the value of KDCclassictypeconstraineddelegationTGSRequests
    pub fn get_kdcclassictypeconstraineddelegation_tgsrequests(&self) -> Option<&u32> {
        self.kdcclassictypeconstraineddelegation_tgsrequests.as_ref()
    }

    /// Sets the value of KDCkeytrustASRequests
    pub fn set_kdckeytrust_asrequests(&mut self, value: u32) {
        self.kdckeytrust_asrequests = Some(value);
    }

    /// Gets the value of KDCkeytrustASRequests
    pub fn get_kdckeytrust_asrequests(&self) -> Option<&u32> {
        self.kdckeytrust_asrequests.as_ref()
    }

    /// Sets the value of KDCresourcetypeconstraineddelegationTGSRequests
    pub fn set_kdcresourcetypeconstraineddelegation_tgsrequests(&mut self, value: u32) {
        self.kdcresourcetypeconstraineddelegation_tgsrequests = Some(value);
    }

    /// Gets the value of KDCresourcetypeconstraineddelegationTGSRequests
    pub fn get_kdcresourcetypeconstraineddelegation_tgsrequests(&self) -> Option<&u32> {
        self.kdcresourcetypeconstraineddelegation_tgsrequests.as_ref()
    }

    /// Sets the value of KDCTGSRequests
    pub fn set_kdctgsrequests(&mut self, value: u32) {
        self.kdctgsrequests = Some(value);
    }

    /// Gets the value of KDCTGSRequests
    pub fn get_kdctgsrequests(&self) -> Option<&u32> {
        self.kdctgsrequests.as_ref()
    }

    /// Sets the value of KerberosAuthentications
    pub fn set_kerberos_authentications(&mut self, value: u32) {
        self.kerberos_authentications = Some(value);
    }

    /// Gets the value of KerberosAuthentications
    pub fn get_kerberos_authentications(&self) -> Option<&u32> {
        self.kerberos_authentications.as_ref()
    }

    /// Sets the value of NTLMAuthentications
    pub fn set_ntlmauthentications(&mut self, value: u32) {
        self.ntlmauthentications = Some(value);
    }

    /// Gets the value of NTLMAuthentications
    pub fn get_ntlmauthentications(&self) -> Option<&u32> {
        self.ntlmauthentications.as_ref()
    }

    /// Sets the value of SchannelSessionCacheEntries
    pub fn set_schannel_session_cache_entries(&mut self, value: u32) {
        self.schannel_session_cache_entries = Some(value);
    }

    /// Gets the value of SchannelSessionCacheEntries
    pub fn get_schannel_session_cache_entries(&self) -> Option<&u32> {
        self.schannel_session_cache_entries.as_ref()
    }

    /// Sets the value of SSLClientSideFullHandshakes
    pub fn set_sslclient_side_full_handshakes(&mut self, value: u32) {
        self.sslclient_side_full_handshakes = Some(value);
    }

    /// Gets the value of SSLClientSideFullHandshakes
    pub fn get_sslclient_side_full_handshakes(&self) -> Option<&u32> {
        self.sslclient_side_full_handshakes.as_ref()
    }

    /// Sets the value of SSLClientSideReconnectHandshakes
    pub fn set_sslclient_side_reconnect_handshakes(&mut self, value: u32) {
        self.sslclient_side_reconnect_handshakes = Some(value);
    }

    /// Gets the value of SSLClientSideReconnectHandshakes
    pub fn get_sslclient_side_reconnect_handshakes(&self) -> Option<&u32> {
        self.sslclient_side_reconnect_handshakes.as_ref()
    }

    /// Sets the value of SSLServerSideFullHandshakes
    pub fn set_sslserver_side_full_handshakes(&mut self, value: u32) {
        self.sslserver_side_full_handshakes = Some(value);
    }

    /// Gets the value of SSLServerSideFullHandshakes
    pub fn get_sslserver_side_full_handshakes(&self) -> Option<&u32> {
        self.sslserver_side_full_handshakes.as_ref()
    }

    /// Sets the value of SSLServerSideReconnectHandshakes
    pub fn set_sslserver_side_reconnect_handshakes(&mut self, value: u32) {
        self.sslserver_side_reconnect_handshakes = Some(value);
    }

    /// Gets the value of SSLServerSideReconnectHandshakes
    pub fn get_sslserver_side_reconnect_handshakes(&self) -> Option<&u32> {
        self.sslserver_side_reconnect_handshakes.as_ref()
    }
}

