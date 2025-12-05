// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PS_VpnConnection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PS_VpnConnection {

/// 
    #[serde(rename = "AllUserConnection")]
    pub all_user_connection: Option<bool>,

/// 
    #[serde(rename = "ApplicationID")]
    pub application_id: Vec<String>,

/// 
    #[serde(rename = "AuthenticationMethod")]
    pub authentication_method: Vec<String>,

/// 
    #[serde(rename = "ConnectionStatus")]
    pub connection_status: Option<String>,

/// 
    #[serde(rename = "CustomConfiguration")]
    pub custom_configuration: Option<String>,

/// 
    #[serde(rename = "DnsConfig")]
    pub dns_config: Vec<PS_VpnConnectionTriggerDnsConfiguration>,

/// 
    #[serde(rename = "DnsSuffix")]
    pub dns_suffix: Option<String>,

/// 
    #[serde(rename = "DnsSuffixSearchList")]
    pub dns_suffix_search_list: Vec<String>,

/// 
    #[serde(rename = "EapConfigXmlStream")]
    pub eap_config_xml_stream: Option<String>,

/// 
    #[serde(rename = "EncryptionLevel")]
    pub encryption_level: Option<String>,

/// 
    #[serde(rename = "Guid")]
    pub guid: Option<String>,

/// 
    #[serde(rename = "IdleDisconnectSeconds")]
    pub idle_disconnect_seconds: Option<u32>,

/// 
    #[serde(rename = "IsAutoTriggerEnabled")]
    pub is_auto_trigger_enabled: Option<bool>,

/// 
    #[serde(rename = "L2tpIPsecAuth")]
    pub l2tp_ipsec_auth: Option<String>,

/// 
    #[serde(rename = "L2tpPsk")]
    pub l2tp_psk: Option<String>,

/// 
    #[serde(rename = "MachineCertificateEKUFilter")]
    pub machine_certificate_ekufilter: Vec<String>,

/// 
    #[serde(rename = "MachineCertificateIssuerFilter")]
    pub machine_certificate_issuer_filter: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "NapState")]
    pub nap_state: Option<String>,

/// 
    #[serde(rename = "PlugInApplicationID")]
    pub plug_in_application_id: Option<String>,

/// 
    #[serde(rename = "ProfileType")]
    pub profile_type: Option<String>,

/// 
    #[serde(rename = "ProvisioningAuthority")]
    pub provisioning_authority: Option<String>,

/// 
    #[serde(rename = "Proxy")]
    pub proxy: Option<PS_VpnConnectionProxy>,

/// 
    #[serde(rename = "RememberCredential")]
    pub remember_credential: Option<bool>,

/// 
    #[serde(rename = "Routes")]
    pub routes: Vec<PS_VpnConnectionRoute>,

/// 
    #[serde(rename = "ServerAddress")]
    pub server_address: Option<String>,

/// 
    #[serde(rename = "ServerList")]
    pub server_list: Vec<PS_VpnServerAddress>,

/// 
    #[serde(rename = "SplitTunneling")]
    pub split_tunneling: Option<bool>,

/// 
    #[serde(rename = "TrustedNetwork")]
    pub trusted_network: Vec<String>,

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

impl PS_VpnConnection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            all_user_connection: None,
            application_id: Vec::new(),
            authentication_method: Vec::new(),
            connection_status: None,
            custom_configuration: None,
            dns_config: Vec::new(),
            dns_suffix: None,
            dns_suffix_search_list: Vec::new(),
            eap_config_xml_stream: None,
            encryption_level: None,
            guid: None,
            idle_disconnect_seconds: None,
            is_auto_trigger_enabled: None,
            l2tp_ipsec_auth: None,
            l2tp_psk: None,
            machine_certificate_ekufilter: Vec::new(),
            machine_certificate_issuer_filter: None,
            name: None,
            nap_state: None,
            plug_in_application_id: None,
            profile_type: None,
            provisioning_authority: None,
            proxy: None,
            remember_credential: None,
            routes: Vec::new(),
            server_address: None,
            server_list: Vec::new(),
            split_tunneling: None,
            trusted_network: Vec::new(),
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

    /// Sets the value of ApplicationID
    pub fn set_application_id(&mut self, value: Vec<String>) {
        self.application_id = value;
    }

    /// Gets the value of ApplicationID
    pub fn get_application_id(&self) -> &Vec<String> {
        &self.application_id
    }

    /// Sets the value of AuthenticationMethod
    pub fn set_authentication_method(&mut self, value: Vec<String>) {
        self.authentication_method = value;
    }

    /// Gets the value of AuthenticationMethod
    pub fn get_authentication_method(&self) -> &Vec<String> {
        &self.authentication_method
    }

    /// Sets the value of ConnectionStatus
    pub fn set_connection_status(&mut self, value: String) {
        self.connection_status = Some(value);
    }

    /// Gets the value of ConnectionStatus
    pub fn get_connection_status(&self) -> Option<&String> {
        self.connection_status.as_ref()
    }

    /// Sets the value of CustomConfiguration
    pub fn set_custom_configuration(&mut self, value: String) {
        self.custom_configuration = Some(value);
    }

    /// Gets the value of CustomConfiguration
    pub fn get_custom_configuration(&self) -> Option<&String> {
        self.custom_configuration.as_ref()
    }

    /// Sets the value of DnsConfig
    pub fn set_dns_config(&mut self, value: Vec<PS_VpnConnectionTriggerDnsConfiguration>) {
        self.dns_config = value;
    }

    /// Gets the value of DnsConfig
    pub fn get_dns_config(&self) -> &Vec<PS_VpnConnectionTriggerDnsConfiguration> {
        &self.dns_config
    }

    /// Sets the value of DnsSuffix
    pub fn set_dns_suffix(&mut self, value: String) {
        self.dns_suffix = Some(value);
    }

    /// Gets the value of DnsSuffix
    pub fn get_dns_suffix(&self) -> Option<&String> {
        self.dns_suffix.as_ref()
    }

    /// Sets the value of DnsSuffixSearchList
    pub fn set_dns_suffix_search_list(&mut self, value: Vec<String>) {
        self.dns_suffix_search_list = value;
    }

    /// Gets the value of DnsSuffixSearchList
    pub fn get_dns_suffix_search_list(&self) -> &Vec<String> {
        &self.dns_suffix_search_list
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

    /// Sets the value of Guid
    pub fn set_guid(&mut self, value: String) {
        self.guid = Some(value);
    }

    /// Gets the value of Guid
    pub fn get_guid(&self) -> Option<&String> {
        self.guid.as_ref()
    }

    /// Sets the value of IdleDisconnectSeconds
    pub fn set_idle_disconnect_seconds(&mut self, value: u32) {
        self.idle_disconnect_seconds = Some(value);
    }

    /// Gets the value of IdleDisconnectSeconds
    pub fn get_idle_disconnect_seconds(&self) -> Option<&u32> {
        self.idle_disconnect_seconds.as_ref()
    }

    /// Sets the value of IsAutoTriggerEnabled
    pub fn set_is_auto_trigger_enabled(&mut self, value: bool) {
        self.is_auto_trigger_enabled = Some(value);
    }

    /// Gets the value of IsAutoTriggerEnabled
    pub fn get_is_auto_trigger_enabled(&self) -> Option<&bool> {
        self.is_auto_trigger_enabled.as_ref()
    }

    /// Sets the value of L2tpIPsecAuth
    pub fn set_l2tp_ipsec_auth(&mut self, value: String) {
        self.l2tp_ipsec_auth = Some(value);
    }

    /// Gets the value of L2tpIPsecAuth
    pub fn get_l2tp_ipsec_auth(&self) -> Option<&String> {
        self.l2tp_ipsec_auth.as_ref()
    }

    /// Sets the value of L2tpPsk
    pub fn set_l2tp_psk(&mut self, value: String) {
        self.l2tp_psk = Some(value);
    }

    /// Gets the value of L2tpPsk
    pub fn get_l2tp_psk(&self) -> Option<&String> {
        self.l2tp_psk.as_ref()
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
    pub fn set_machine_certificate_issuer_filter(&mut self, value: String) {
        self.machine_certificate_issuer_filter = Some(value);
    }

    /// Gets the value of MachineCertificateIssuerFilter
    pub fn get_machine_certificate_issuer_filter(&self) -> Option<&String> {
        self.machine_certificate_issuer_filter.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of NapState
    pub fn set_nap_state(&mut self, value: String) {
        self.nap_state = Some(value);
    }

    /// Gets the value of NapState
    pub fn get_nap_state(&self) -> Option<&String> {
        self.nap_state.as_ref()
    }

    /// Sets the value of PlugInApplicationID
    pub fn set_plug_in_application_id(&mut self, value: String) {
        self.plug_in_application_id = Some(value);
    }

    /// Gets the value of PlugInApplicationID
    pub fn get_plug_in_application_id(&self) -> Option<&String> {
        self.plug_in_application_id.as_ref()
    }

    /// Sets the value of ProfileType
    pub fn set_profile_type(&mut self, value: String) {
        self.profile_type = Some(value);
    }

    /// Gets the value of ProfileType
    pub fn get_profile_type(&self) -> Option<&String> {
        self.profile_type.as_ref()
    }

    /// Sets the value of ProvisioningAuthority
    pub fn set_provisioning_authority(&mut self, value: String) {
        self.provisioning_authority = Some(value);
    }

    /// Gets the value of ProvisioningAuthority
    pub fn get_provisioning_authority(&self) -> Option<&String> {
        self.provisioning_authority.as_ref()
    }

    /// Sets the value of Proxy
    pub fn set_proxy(&mut self, value: PS_VpnConnectionProxy) {
        self.proxy = Some(value);
    }

    /// Gets the value of Proxy
    pub fn get_proxy(&self) -> Option<&PS_VpnConnectionProxy> {
        self.proxy.as_ref()
    }

    /// Sets the value of RememberCredential
    pub fn set_remember_credential(&mut self, value: bool) {
        self.remember_credential = Some(value);
    }

    /// Gets the value of RememberCredential
    pub fn get_remember_credential(&self) -> Option<&bool> {
        self.remember_credential.as_ref()
    }

    /// Sets the value of Routes
    pub fn set_routes(&mut self, value: Vec<PS_VpnConnectionRoute>) {
        self.routes = value;
    }

    /// Gets the value of Routes
    pub fn get_routes(&self) -> &Vec<PS_VpnConnectionRoute> {
        &self.routes
    }

    /// Sets the value of ServerAddress
    pub fn set_server_address(&mut self, value: String) {
        self.server_address = Some(value);
    }

    /// Gets the value of ServerAddress
    pub fn get_server_address(&self) -> Option<&String> {
        self.server_address.as_ref()
    }

    /// Sets the value of ServerList
    pub fn set_server_list(&mut self, value: Vec<PS_VpnServerAddress>) {
        self.server_list = value;
    }

    /// Gets the value of ServerList
    pub fn get_server_list(&self) -> &Vec<PS_VpnServerAddress> {
        &self.server_list
    }

    /// Sets the value of SplitTunneling
    pub fn set_split_tunneling(&mut self, value: bool) {
        self.split_tunneling = Some(value);
    }

    /// Gets the value of SplitTunneling
    pub fn get_split_tunneling(&self) -> Option<&bool> {
        self.split_tunneling.as_ref()
    }

    /// Sets the value of TrustedNetwork
    pub fn set_trusted_network(&mut self, value: Vec<String>) {
        self.trusted_network = value;
    }

    /// Gets the value of TrustedNetwork
    pub fn get_trusted_network(&self) -> &Vec<String> {
        &self.trusted_network
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

/// 

    /// * `all_user_connection` -  (bool)
    /// * `force` -  (bool)
    /// * `name` -  (String[])
    /// * `pass_thru` -  (bool)

    /// * `cmdlet_output` -  (VpnCommonConfig[])
    /// * `return_value` -  (u32)
    pub fn remove(&self, name: &Vec<String>, force: bool, pass_thru: bool, all_user_connection: bool, cmdlet_output: &mut Vec<VpnCommonConfig>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "AllUserConnection".to_string(), value: all_user_connection.into() });

        let result = self.invoke_method("Remove", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `all_user_connection` -  (bool)
    /// * `name` -  (String[])

    /// * `cmdlet_output` -  (VpnCommonConfig[])
    /// * `return_value` -  (u32)
    pub fn get(&self, name: &Vec<String>, all_user_connection: bool, cmdlet_output: &mut Vec<VpnCommonConfig>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "AllUserConnection".to_string(), value: all_user_connection.into() });

        let result = self.invoke_method("Get", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `all_user_connection` -  (bool)
    /// * `authentication_method` -  (String[])
    /// * `dns_suffix` -  (String)
    /// * `eap_config_xml_stream` -  (String)
    /// * `encryption_level` -  (String)
    /// * `force` -  (bool)
    /// * `idle_disconnect_seconds` -  (u32)
    /// * `l2tp_psk` -  (String)
    /// * `machine_certificate_ekufilter` -  (String[])
    /// * `machine_certificate_issuer_filter` -  (u8[])
    /// * `name` -  (String)
    /// * `pass_thru` -  (bool)
    /// * `remember_credential` -  (bool)
    /// * `server_address` -  (String)
    /// * `server_list` -  (VpnServerAddress[])
    /// * `split_tunneling` -  (bool)
    /// * `tunnel_type` -  (String)
    /// * `use_winlogon_credential` -  (bool)

    /// * `cmdlet_output` -  (VpnConnection)
    /// * `return_value` -  (u32)
    pub fn set(&self, server_address: &String, all_user_connection: bool, split_tunneling: bool, remember_credential: bool, tunnel_type: &String, pass_thru: bool, force: bool, l2tp_psk: &String, authentication_method: &Vec<String>, eap_config_xml_stream: &String, name: &String, use_winlogon_credential: bool, encryption_level: &String, machine_certificate_ekufilter: &Vec<String>, machine_certificate_issuer_filter: &Vec<u8>, server_list: &Vec<VpnServerAddress>, idle_disconnect_seconds: u32, dns_suffix: &String, cmdlet_output: &mut VpnConnection) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ServerAddress".to_string(), value: server_address.into() });
        args.push(MethodParameter { name: "AllUserConnection".to_string(), value: all_user_connection.into() });
        args.push(MethodParameter { name: "SplitTunneling".to_string(), value: split_tunneling.into() });
        args.push(MethodParameter { name: "RememberCredential".to_string(), value: remember_credential.into() });
        args.push(MethodParameter { name: "TunnelType".to_string(), value: tunnel_type.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        args.push(MethodParameter { name: "L2tpPsk".to_string(), value: l2tp_psk.into() });
        args.push(MethodParameter { name: "AuthenticationMethod".to_string(), value: authentication_method.into() });
        args.push(MethodParameter { name: "EapConfigXmlStream".to_string(), value: eap_config_xml_stream.into() });
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "UseWinlogonCredential".to_string(), value: use_winlogon_credential.into() });
        args.push(MethodParameter { name: "EncryptionLevel".to_string(), value: encryption_level.into() });
        args.push(MethodParameter { name: "MachineCertificateEKUFilter".to_string(), value: machine_certificate_ekufilter.into() });
        args.push(MethodParameter { name: "MachineCertificateIssuerFilter".to_string(), value: machine_certificate_issuer_filter.into() });
        args.push(MethodParameter { name: "ServerList".to_string(), value: server_list.into() });
        args.push(MethodParameter { name: "IdleDisconnectSeconds".to_string(), value: idle_disconnect_seconds.into() });
        args.push(MethodParameter { name: "DnsSuffix".to_string(), value: dns_suffix.into() });

        let result = self.invoke_method("Set", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `custom_configuration` -  (String)
    /// * `dns_suffix` -  (String)
    /// * `force` -  (bool)
    /// * `idle_disconnect_seconds` -  (u32)
    /// * `name` -  (String)
    /// * `pass_thru` -  (bool)
    /// * `plug_in_application_id` -  (String)
    /// * `remember_credential` -  (bool)
    /// * `server_address` -  (String)
    /// * `server_list` -  (VpnServerAddress[])
    /// * `split_tunneling` -  (bool)
    /// * `third_party_vpn` -  (bool)

    /// * `cmdlet_output` -  (ThirdPartyVpnConnection)
    /// * `return_value` -  (u32)
    pub fn set_by_third_party(&self, name: &String, server_address: &String, remember_credential: bool, split_tunneling: bool, pass_thru: bool, force: bool, server_list: &Vec<VpnServerAddress>, dns_suffix: &String, idle_disconnect_seconds: u32, plug_in_application_id: &String, custom_configuration: &String, third_party_vpn: bool, cmdlet_output: &mut ThirdPartyVpnConnection) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "ServerAddress".to_string(), value: server_address.into() });
        args.push(MethodParameter { name: "RememberCredential".to_string(), value: remember_credential.into() });
        args.push(MethodParameter { name: "SplitTunneling".to_string(), value: split_tunneling.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        args.push(MethodParameter { name: "ServerList".to_string(), value: server_list.into() });
        args.push(MethodParameter { name: "DnsSuffix".to_string(), value: dns_suffix.into() });
        args.push(MethodParameter { name: "IdleDisconnectSeconds".to_string(), value: idle_disconnect_seconds.into() });
        args.push(MethodParameter { name: "PlugInApplicationID".to_string(), value: plug_in_application_id.into() });
        args.push(MethodParameter { name: "CustomConfiguration".to_string(), value: custom_configuration.into() });
        args.push(MethodParameter { name: "ThirdPartyVpn".to_string(), value: third_party_vpn.into() });

        let result = self.invoke_method("SetByThirdParty", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `all_user_connection` -  (bool)
    /// * `authentication_method` -  (String[])
    /// * `dns_suffix` -  (String)
    /// * `eap_config_xml_stream` -  (String)
    /// * `encryption_level` -  (String)
    /// * `force` -  (bool)
    /// * `idle_disconnect_seconds` -  (u32)
    /// * `l2tp_psk` -  (String)
    /// * `machine_certificate_ekufilter` -  (String[])
    /// * `machine_certificate_issuer_filter` -  (u8[])
    /// * `name` -  (String)
    /// * `pass_thru` -  (bool)
    /// * `remember_credential` -  (bool)
    /// * `server_address` -  (String)
    /// * `server_list` -  (VpnServerAddress[])
    /// * `split_tunneling` -  (bool)
    /// * `tunnel_type` -  (String)
    /// * `use_winlogon_credential` -  (bool)

    /// * `cmdlet_output` -  (VpnConnection)
    /// * `return_value` -  (u32)
    pub fn add(&self, name: &String, server_address: &String, tunnel_type: &String, all_user_connection: bool, remember_credential: bool, idle_disconnect_seconds: u32, dns_suffix: &String, split_tunneling: bool, force: bool, pass_thru: bool, l2tp_psk: &String, use_winlogon_credential: bool, eap_config_xml_stream: &String, authentication_method: &Vec<String>, encryption_level: &String, machine_certificate_issuer_filter: &Vec<u8>, machine_certificate_ekufilter: &Vec<String>, server_list: &Vec<VpnServerAddress>, cmdlet_output: &mut VpnConnection) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "ServerAddress".to_string(), value: server_address.into() });
        args.push(MethodParameter { name: "TunnelType".to_string(), value: tunnel_type.into() });
        args.push(MethodParameter { name: "AllUserConnection".to_string(), value: all_user_connection.into() });
        args.push(MethodParameter { name: "RememberCredential".to_string(), value: remember_credential.into() });
        args.push(MethodParameter { name: "IdleDisconnectSeconds".to_string(), value: idle_disconnect_seconds.into() });
        args.push(MethodParameter { name: "DnsSuffix".to_string(), value: dns_suffix.into() });
        args.push(MethodParameter { name: "SplitTunneling".to_string(), value: split_tunneling.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "L2tpPsk".to_string(), value: l2tp_psk.into() });
        args.push(MethodParameter { name: "UseWinlogonCredential".to_string(), value: use_winlogon_credential.into() });
        args.push(MethodParameter { name: "EapConfigXmlStream".to_string(), value: eap_config_xml_stream.into() });
        args.push(MethodParameter { name: "AuthenticationMethod".to_string(), value: authentication_method.into() });
        args.push(MethodParameter { name: "EncryptionLevel".to_string(), value: encryption_level.into() });
        args.push(MethodParameter { name: "MachineCertificateIssuerFilter".to_string(), value: machine_certificate_issuer_filter.into() });
        args.push(MethodParameter { name: "MachineCertificateEKUFilter".to_string(), value: machine_certificate_ekufilter.into() });
        args.push(MethodParameter { name: "ServerList".to_string(), value: server_list.into() });

        let result = self.invoke_method("Add", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `custom_configuration` -  (String)
    /// * `dns_suffix` -  (String)
    /// * `force` -  (bool)
    /// * `idle_disconnect_seconds` -  (u32)
    /// * `name` -  (String)
    /// * `pass_thru` -  (bool)
    /// * `plug_in_application_id` -  (String)
    /// * `remember_credential` -  (bool)
    /// * `server_address` -  (String)
    /// * `server_list` -  (VpnServerAddress[])
    /// * `split_tunneling` -  (bool)

    /// * `cmdlet_output` -  (ThirdPartyVpnConnection)
    /// * `return_value` -  (u32)
    pub fn new_by_third_party(&self, name: &String, server_address: &String, remember_credential: bool, split_tunneling: bool, pass_thru: bool, force: bool, server_list: &Vec<VpnServerAddress>, dns_suffix: &String, idle_disconnect_seconds: u32, plug_in_application_id: &String, custom_configuration: &String, cmdlet_output: &mut ThirdPartyVpnConnection) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "ServerAddress".to_string(), value: server_address.into() });
        args.push(MethodParameter { name: "RememberCredential".to_string(), value: remember_credential.into() });
        args.push(MethodParameter { name: "SplitTunneling".to_string(), value: split_tunneling.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        args.push(MethodParameter { name: "ServerList".to_string(), value: server_list.into() });
        args.push(MethodParameter { name: "DnsSuffix".to_string(), value: dns_suffix.into() });
        args.push(MethodParameter { name: "IdleDisconnectSeconds".to_string(), value: idle_disconnect_seconds.into() });
        args.push(MethodParameter { name: "PlugInApplicationID".to_string(), value: plug_in_application_id.into() });
        args.push(MethodParameter { name: "CustomConfiguration".to_string(), value: custom_configuration.into() });

        let result = self.invoke_method("NewByThirdParty", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

