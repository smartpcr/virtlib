// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetConSecRule struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetConSecRule {
    #[serde(flatten)]
    pub base: MSFT_NetSARule,

/// 
    #[serde(rename = "AllowSetKey")]
    pub allow_set_key: Option<bool>,

/// 
    #[serde(rename = "AllowWatchKey")]
    pub allow_watch_key: Option<bool>,

/// 
    #[serde(rename = "BypassTunnelIfEncrypted")]
    pub bypass_tunnel_if_encrypted: Option<bool>,

/// 
    #[serde(rename = "InboundSecurity")]
    pub inbound_security: Option<u16>,

/// 
    #[serde(rename = "KeyModule")]
    pub key_module: Option<u16>,

/// 
    #[serde(rename = "LocalTunnelEndpoint")]
    pub local_tunnel_endpoint: Vec<String>,

/// 
    #[serde(rename = "Machines")]
    pub machines: Option<String>,

/// 
    #[serde(rename = "MaxReturnPathLifetimeSeconds")]
    pub max_return_path_lifetime_seconds: Option<u32>,

/// 
    #[serde(rename = "Mode")]
    pub mode: Option<u16>,

/// 
    #[serde(rename = "OutboundSecurity")]
    pub outbound_security: Option<u16>,

/// 
    #[serde(rename = "RemoteTunnelEndpoint")]
    pub remote_tunnel_endpoint: Vec<String>,

/// 
    #[serde(rename = "RemoteTunnelEndpointDNSName")]
    pub remote_tunnel_endpoint_dnsname: Option<String>,

/// 
    #[serde(rename = "RequireAuthorization")]
    pub require_authorization: Option<bool>,

/// 
    #[serde(rename = "Users")]
    pub users: Option<String>,
}

impl MSFT_NetConSecRule {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSARule::new(),
            allow_set_key: None,
            allow_watch_key: None,
            bypass_tunnel_if_encrypted: None,
            inbound_security: None,
            key_module: None,
            local_tunnel_endpoint: Vec::new(),
            machines: None,
            max_return_path_lifetime_seconds: None,
            mode: None,
            outbound_security: None,
            remote_tunnel_endpoint: Vec::new(),
            remote_tunnel_endpoint_dnsname: None,
            require_authorization: None,
            users: None,
        }
    }


    /// Sets the value of AllowSetKey
    pub fn set_allow_set_key(&mut self, value: bool) {
        self.allow_set_key = Some(value);
    }

    /// Gets the value of AllowSetKey
    pub fn get_allow_set_key(&self) -> Option<&bool> {
        self.allow_set_key.as_ref()
    }

    /// Sets the value of AllowWatchKey
    pub fn set_allow_watch_key(&mut self, value: bool) {
        self.allow_watch_key = Some(value);
    }

    /// Gets the value of AllowWatchKey
    pub fn get_allow_watch_key(&self) -> Option<&bool> {
        self.allow_watch_key.as_ref()
    }

    /// Sets the value of BypassTunnelIfEncrypted
    pub fn set_bypass_tunnel_if_encrypted(&mut self, value: bool) {
        self.bypass_tunnel_if_encrypted = Some(value);
    }

    /// Gets the value of BypassTunnelIfEncrypted
    pub fn get_bypass_tunnel_if_encrypted(&self) -> Option<&bool> {
        self.bypass_tunnel_if_encrypted.as_ref()
    }

    /// Sets the value of InboundSecurity
    pub fn set_inbound_security(&mut self, value: u16) {
        self.inbound_security = Some(value);
    }

    /// Gets the value of InboundSecurity
    pub fn get_inbound_security(&self) -> Option<&u16> {
        self.inbound_security.as_ref()
    }

    /// Sets the value of KeyModule
    pub fn set_key_module(&mut self, value: u16) {
        self.key_module = Some(value);
    }

    /// Gets the value of KeyModule
    pub fn get_key_module(&self) -> Option<&u16> {
        self.key_module.as_ref()
    }

    /// Sets the value of LocalTunnelEndpoint
    pub fn set_local_tunnel_endpoint(&mut self, value: Vec<String>) {
        self.local_tunnel_endpoint = value;
    }

    /// Gets the value of LocalTunnelEndpoint
    pub fn get_local_tunnel_endpoint(&self) -> &Vec<String> {
        &self.local_tunnel_endpoint
    }

    /// Sets the value of Machines
    pub fn set_machines(&mut self, value: String) {
        self.machines = Some(value);
    }

    /// Gets the value of Machines
    pub fn get_machines(&self) -> Option<&String> {
        self.machines.as_ref()
    }

    /// Sets the value of MaxReturnPathLifetimeSeconds
    pub fn set_max_return_path_lifetime_seconds(&mut self, value: u32) {
        self.max_return_path_lifetime_seconds = Some(value);
    }

    /// Gets the value of MaxReturnPathLifetimeSeconds
    pub fn get_max_return_path_lifetime_seconds(&self) -> Option<&u32> {
        self.max_return_path_lifetime_seconds.as_ref()
    }

    /// Sets the value of Mode
    pub fn set_mode(&mut self, value: u16) {
        self.mode = Some(value);
    }

    /// Gets the value of Mode
    pub fn get_mode(&self) -> Option<&u16> {
        self.mode.as_ref()
    }

    /// Sets the value of OutboundSecurity
    pub fn set_outbound_security(&mut self, value: u16) {
        self.outbound_security = Some(value);
    }

    /// Gets the value of OutboundSecurity
    pub fn get_outbound_security(&self) -> Option<&u16> {
        self.outbound_security.as_ref()
    }

    /// Sets the value of RemoteTunnelEndpoint
    pub fn set_remote_tunnel_endpoint(&mut self, value: Vec<String>) {
        self.remote_tunnel_endpoint = value;
    }

    /// Gets the value of RemoteTunnelEndpoint
    pub fn get_remote_tunnel_endpoint(&self) -> &Vec<String> {
        &self.remote_tunnel_endpoint
    }

    /// Sets the value of RemoteTunnelEndpointDNSName
    pub fn set_remote_tunnel_endpoint_dnsname(&mut self, value: String) {
        self.remote_tunnel_endpoint_dnsname = Some(value);
    }

    /// Gets the value of RemoteTunnelEndpointDNSName
    pub fn get_remote_tunnel_endpoint_dnsname(&self) -> Option<&String> {
        self.remote_tunnel_endpoint_dnsname.as_ref()
    }

    /// Sets the value of RequireAuthorization
    pub fn set_require_authorization(&mut self, value: bool) {
        self.require_authorization = Some(value);
    }

    /// Gets the value of RequireAuthorization
    pub fn get_require_authorization(&self) -> Option<&bool> {
        self.require_authorization.as_ref()
    }

    /// Sets the value of Users
    pub fn set_users(&mut self, value: String) {
        self.users = Some(value);
    }

    /// Gets the value of Users
    pub fn get_users(&self) -> Option<&String> {
        self.users.as_ref()
    }

/// 

    /// * `address_type` -  (u16)
    /// * `dns_servers` -  (String[])
    /// * `domains` -  (String[])
    /// * `endpoint_type` -  (u16)
    /// * `servers` -  (String[])

    /// * `output` -  (MSFT_NetSecDeltaCollection[])
    /// * `return_value` -  (u32)
    pub fn sync_policy_delta(&self, servers: &Vec<String>, domains: &Vec<String>, endpoint_type: u16, address_type: u16, dns_servers: &Vec<String>, output: &mut Vec<MSFT_NetSecDeltaCollection>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Servers".to_string(), value: servers.into() });
        args.push(MethodParameter { name: "Domains".to_string(), value: domains.into() });
        args.push(MethodParameter { name: "EndpointType".to_string(), value: endpoint_type.into() });
        args.push(MethodParameter { name: "AddressType".to_string(), value: address_type.into() });
        args.push(MethodParameter { name: "DnsServers".to_string(), value: dns_servers.into() });

        let result = self.invoke_method("SyncPolicyDelta", &args)?;
        let output = result.get_value("Output")?;
        Ok(result.return_value)

    }


/// 

    /// * `action` -  (u16)
    /// * `endpoint_type` -  (u16)
    /// * `ipv4_addresses` -  (String[])
    /// * `ipv6_addresses` -  (String[])
    /// * `pass_thru` -  (bool)

    /// * `output` -  (MSFT_NetConSecRule[])
    /// * `return_value` -  (u32)
    pub fn set_policy_delta(&self, action: u16, ipv6_addresses: &Vec<String>, ipv4_addresses: &Vec<String>, endpoint_type: u16, pass_thru: bool, output: &mut Vec<MSFT_NetConSecRule>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Action".to_string(), value: action.into() });
        args.push(MethodParameter { name: "IPv6Addresses".to_string(), value: ipv6_addresses.into() });
        args.push(MethodParameter { name: "IPv4Addresses".to_string(), value: ipv4_addresses.into() });
        args.push(MethodParameter { name: "EndpointType".to_string(), value: endpoint_type.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("SetPolicyDelta", &args)?;
        let output = result.get_value("Output")?;
        Ok(result.return_value)

    }


/// 

    /// * `dependents` -  (CIM_ManagedSystemElement[])
    /// * `return_value` -  (u32)
    pub fn enumerate_full(&self, dependents: &mut Vec<CIM_ManagedSystemElement>) -> Result<(), WmiError> {

        let result = self.invoke_method("EnumerateFull", &[])?;
        let dependents = result.get_value("Dependents")?;
        Ok(result.return_value)

    }


/// 

    /// * `local_address` -  (String)
    /// * `local_port` -  (u16)
    /// * `protocol` -  (String)
    /// * `remote_address` -  (String)
    /// * `remote_port` -  (u16)

    /// * `cmdlet_output` -  (MSFT_NetConSecRule[])
    /// * `return_value` -  (u32)
    pub fn find(&self, local_address: &String, remote_address: &String, protocol: &String, local_port: u16, remote_port: u16, cmdlet_output: &mut Vec<MSFT_NetConSecRule>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "LocalAddress".to_string(), value: local_address.into() });
        args.push(MethodParameter { name: "RemoteAddress".to_string(), value: remote_address.into() });
        args.push(MethodParameter { name: "Protocol".to_string(), value: protocol.into() });
        args.push(MethodParameter { name: "LocalPort".to_string(), value: local_port.into() });
        args.push(MethodParameter { name: "RemotePort".to_string(), value: remote_port.into() });

        let result = self.invoke_method("Find", &args)?;
        let cmdlet_output = result.get_value("CmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn enable(&self) -> Result<(), WmiError> {
        self.invoke_method("Enable", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn disable(&self) -> Result<(), WmiError> {
        self.invoke_method("Disable", &[])

    }


/// 

    /// * `new_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn rename(&self, new_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewName".to_string(), value: new_name.into() });
        self.invoke_method("Rename", &args)

    }


/// 

    /// * `new_gposession` -  (String)
    /// * `new_name` -  (String)
    /// * `new_policy_store` -  (String)

    /// * `return_value` -  (u32)
    pub fn clone_object(&self, new_name: &String, new_policy_store: &String, new_gposession: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewName".to_string(), value: new_name.into() });
        args.push(MethodParameter { name: "NewPolicyStore".to_string(), value: new_policy_store.into() });
        args.push(MethodParameter { name: "NewGPOSession".to_string(), value: new_gposession.into() });
        self.invoke_method("CloneObject", &args)

    }

}

