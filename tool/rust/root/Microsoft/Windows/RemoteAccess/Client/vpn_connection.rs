// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// VpnConnection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VpnConnection {
    #[serde(flatten)]
    pub base: VpnCommonConfig,

/// 
    #[serde(rename = "AllUserConnection")]
    pub all_user_connection: Option<bool>,

/// 
    #[serde(rename = "AuthenticationMethod")]
    pub authentication_method: Vec<String>,

/// 
    #[serde(rename = "EapConfigXmlStream")]
    pub eap_config_xml_stream: Option<String>,

/// 
    #[serde(rename = "EncryptionLevel")]
    pub encryption_level: Option<String>,

/// 
    #[serde(rename = "IPSecCustomPolicy")]
    pub ipsec_custom_policy: Vec<VpnConnectionIPsecConfiguration>,

/// 
    #[serde(rename = "L2tpIPsecAuth")]
    pub l2tp_ipsec_auth: Option<String>,

/// 
    #[serde(rename = "MachineCertificateEKUFilter")]
    pub machine_certificate_ekufilter: Vec<String>,

/// 
    #[serde(rename = "MachineCertificateIssuerFilter")]
    pub machine_certificate_issuer_filter: Vec<u8>,

/// 
    #[serde(rename = "NapState")]
    pub nap_state: Option<String>,

/// 
    #[serde(rename = "TunnelType")]
    pub tunnel_type: Option<String>,

/// 
    #[serde(rename = "UseWinlogonCredential")]
    pub use_winlogon_credential: Option<bool>,

/// 
    #[serde(rename = "VpnConfigurationXml")]
    pub vpn_configuration_xml: Option<String>,
}

impl VpnConnection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: VpnCommonConfig::new(),
            all_user_connection: None,
            authentication_method: Vec::new(),
            eap_config_xml_stream: None,
            encryption_level: None,
            ipsec_custom_policy: Vec::new(),
            l2tp_ipsec_auth: None,
            machine_certificate_ekufilter: Vec::new(),
            machine_certificate_issuer_filter: Vec::new(),
            nap_state: None,
            tunnel_type: None,
            use_winlogon_credential: None,
            vpn_configuration_xml: None,
        }
    }


    /// Sets the value of AllUserConnection
    pub fn set_all_user_connection(&mut self, value: bool) {
        self.all_user_connection = Some(value);
    }

    /// Gets the value of AllUserConnection
    pub fn get_all_user_connection(&self) -> Option<&bool> {
        self.all_user_connection.as_ref()
    }

    /// Sets the value of AuthenticationMethod
    pub fn set_authentication_method(&mut self, value: Vec<String>) {
        self.authentication_method = value;
    }

    /// Gets the value of AuthenticationMethod
    pub fn get_authentication_method(&self) -> &Vec<String> {
        &self.authentication_method
    }

    /// Sets the value of EapConfigXmlStream
    pub fn set_eap_config_xml_stream(&mut self, value: String) {
        self.eap_config_xml_stream = Some(value);
    }

    /// Gets the value of EapConfigXmlStream
    pub fn get_eap_config_xml_stream(&self) -> Option<&String> {
        self.eap_config_xml_stream.as_ref()
    }

    /// Sets the value of EncryptionLevel
    pub fn set_encryption_level(&mut self, value: String) {
        self.encryption_level = Some(value);
    }

    /// Gets the value of EncryptionLevel
    pub fn get_encryption_level(&self) -> Option<&String> {
        self.encryption_level.as_ref()
    }

    /// Sets the value of IPSecCustomPolicy
    pub fn set_ipsec_custom_policy(&mut self, value: Vec<VpnConnectionIPsecConfiguration>) {
        self.ipsec_custom_policy = value;
    }

    /// Gets the value of IPSecCustomPolicy
    pub fn get_ipsec_custom_policy(&self) -> &Vec<VpnConnectionIPsecConfiguration> {
        &self.ipsec_custom_policy
    }

    /// Sets the value of L2tpIPsecAuth
    pub fn set_l2tp_ipsec_auth(&mut self, value: String) {
        self.l2tp_ipsec_auth = Some(value);
    }

    /// Gets the value of L2tpIPsecAuth
    pub fn get_l2tp_ipsec_auth(&self) -> Option<&String> {
        self.l2tp_ipsec_auth.as_ref()
    }

    /// Sets the value of MachineCertificateEKUFilter
    pub fn set_machine_certificate_ekufilter(&mut self, value: Vec<String>) {
        self.machine_certificate_ekufilter = value;
    }

    /// Gets the value of MachineCertificateEKUFilter
    pub fn get_machine_certificate_ekufilter(&self) -> &Vec<String> {
        &self.machine_certificate_ekufilter
    }

    /// Sets the value of MachineCertificateIssuerFilter
    pub fn set_machine_certificate_issuer_filter(&mut self, value: Vec<u8>) {
        self.machine_certificate_issuer_filter = value;
    }

    /// Gets the value of MachineCertificateIssuerFilter
    pub fn get_machine_certificate_issuer_filter(&self) -> &Vec<u8> {
        &self.machine_certificate_issuer_filter
    }

    /// Sets the value of NapState
    pub fn set_nap_state(&mut self, value: String) {
        self.nap_state = Some(value);
    }

    /// Gets the value of NapState
    pub fn get_nap_state(&self) -> Option<&String> {
        self.nap_state.as_ref()
    }

    /// Sets the value of TunnelType
    pub fn set_tunnel_type(&mut self, value: String) {
        self.tunnel_type = Some(value);
    }

    /// Gets the value of TunnelType
    pub fn get_tunnel_type(&self) -> Option<&String> {
        self.tunnel_type.as_ref()
    }

    /// Sets the value of UseWinlogonCredential
    pub fn set_use_winlogon_credential(&mut self, value: bool) {
        self.use_winlogon_credential = Some(value);
    }

    /// Gets the value of UseWinlogonCredential
    pub fn get_use_winlogon_credential(&self) -> Option<&bool> {
        self.use_winlogon_credential.as_ref()
    }

    /// Sets the value of VpnConfigurationXml
    pub fn set_vpn_configuration_xml(&mut self, value: String) {
        self.vpn_configuration_xml = Some(value);
    }

    /// Gets the value of VpnConfigurationXml
    pub fn get_vpn_configuration_xml(&self) -> Option<&String> {
        self.vpn_configuration_xml.as_ref()
    }
}

