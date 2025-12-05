// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Dns
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PS_DnsClientNrptRule struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PS_DnsClientNrptRule {
}

impl PS_DnsClientNrptRule {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `comment` -  (String)
    /// * `daenable` -  (bool)
    /// * `daipsec_encryption_type` -  (String)
    /// * `daipsec_required` -  (bool)
    /// * `daname_servers` -  (String[])
    /// * `daproxy_server_name` -  (String)
    /// * `daproxy_type` -  (String)
    /// * `display_name` -  (String)
    /// * `dns_sec_enable` -  (bool)
    /// * `dns_sec_ipsec_encryption_type` -  (String)
    /// * `dns_sec_ipsec_required` -  (bool)
    /// * `dns_sec_validation_required` -  (bool)
    /// * `gpo_name` -  (String)
    /// * `ipsec_trust_authority` -  (String)
    /// * `name_encoding` -  (String)
    /// * `name_servers` -  (String[])
    /// * `namespace` -  (String[])
    /// * `pass_thru` -  (bool)
    /// * `server` -  (String)

    /// * `cmdlet_output` -  (DnsClientNrptRule)
    /// * `return_value` -  (u32)
    pub fn add(&self, gpo_name: &String, daname_servers: &Vec<String>, daipsec_required: bool, daipsec_encryption_type: &String, daproxy_server_name: &String, dns_sec_enable: bool, pass_thru: bool, daproxy_type: &String, dns_sec_validation_required: bool, daenable: bool, ipsec_trust_authority: &String, comment: &String, display_name: &String, dns_sec_ipsec_required: bool, dns_sec_ipsec_encryption_type: &String, name_servers: &Vec<String>, name_encoding: &String, namespace: &Vec<String>, server: &String, cmdlet_output: &mut DnsClientNrptRule) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "GpoName".to_string(), value: gpo_name.into() });
        args.push(MethodParameter { name: "DANameServers".to_string(), value: daname_servers.into() });
        args.push(MethodParameter { name: "DAIPsecRequired".to_string(), value: daipsec_required.into() });
        args.push(MethodParameter { name: "DAIPsecEncryptionType".to_string(), value: daipsec_encryption_type.into() });
        args.push(MethodParameter { name: "DAProxyServerName".to_string(), value: daproxy_server_name.into() });
        args.push(MethodParameter { name: "DnsSecEnable".to_string(), value: dns_sec_enable.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "DAProxyType".to_string(), value: daproxy_type.into() });
        args.push(MethodParameter { name: "DnsSecValidationRequired".to_string(), value: dns_sec_validation_required.into() });
        args.push(MethodParameter { name: "DAEnable".to_string(), value: daenable.into() });
        args.push(MethodParameter { name: "IPsecTrustAuthority".to_string(), value: ipsec_trust_authority.into() });
        args.push(MethodParameter { name: "Comment".to_string(), value: comment.into() });
        args.push(MethodParameter { name: "DisplayName".to_string(), value: display_name.into() });
        args.push(MethodParameter { name: "DnsSecIPsecRequired".to_string(), value: dns_sec_ipsec_required.into() });
        args.push(MethodParameter { name: "DnsSecIPsecEncryptionType".to_string(), value: dns_sec_ipsec_encryption_type.into() });
        args.push(MethodParameter { name: "NameServers".to_string(), value: name_servers.into() });
        args.push(MethodParameter { name: "NameEncoding".to_string(), value: name_encoding.into() });
        args.push(MethodParameter { name: "Namespace".to_string(), value: namespace.into() });
        args.push(MethodParameter { name: "Server".to_string(), value: server.into() });

        let result = self.invoke_method("Add", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `force` -  (bool)
    /// * `gpo_name` -  (String)
    /// * `name` -  (String)
    /// * `pass_thru` -  (bool)
    /// * `server` -  (String)

    /// * `cmdlet_output` -  (DnsClientNrptRule)
    /// * `return_value` -  (u32)
    pub fn remove(&self, gpo_name: &String, name: &String, pass_thru: bool, server: &String, force: bool, cmdlet_output: &mut DnsClientNrptRule) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "GpoName".to_string(), value: gpo_name.into() });
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "Server".to_string(), value: server.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });

        let result = self.invoke_method("Remove", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `gpo_name` -  (String)
    /// * `name` -  (String[])
    /// * `server` -  (String)

    /// * `cmdlet_output` -  (DnsClientNrptRule[])
    /// * `return_value` -  (u32)
    pub fn get(&self, gpo_name: &String, name: &Vec<String>, server: &String, cmdlet_output: &mut Vec<DnsClientNrptRule>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "GpoName".to_string(), value: gpo_name.into() });
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Server".to_string(), value: server.into() });

        let result = self.invoke_method("Get", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `comment` -  (String)
    /// * `daenable` -  (bool)
    /// * `daipsec_encryption_type` -  (String)
    /// * `daipsec_required` -  (bool)
    /// * `daname_servers` -  (String[])
    /// * `daproxy_server_name` -  (String)
    /// * `daproxy_type` -  (String)
    /// * `display_name` -  (String)
    /// * `dns_sec_enable` -  (bool)
    /// * `dns_sec_ipsec_encryption_type` -  (String)
    /// * `dns_sec_ipsec_required` -  (bool)
    /// * `dns_sec_validation_required` -  (bool)
    /// * `gpo_name` -  (String)
    /// * `ipsec_trust_authority` -  (String)
    /// * `name` -  (String)
    /// * `name_encoding` -  (String)
    /// * `name_servers` -  (String[])
    /// * `namespace` -  (String[])
    /// * `pass_thru` -  (bool)
    /// * `server` -  (String)

    /// * `cmdlet_output` -  (DnsClientNrptRule)
    /// * `return_value` -  (u32)
    pub fn set(&self, daenable: bool, daipsec_encryption_type: &String, daipsec_required: bool, daname_servers: &Vec<String>, daproxy_server_name: &String, daproxy_type: &String, display_name: &String, pass_thru: bool, ipsec_trust_authority: &String, name: &String, name_encoding: &String, name_servers: &Vec<String>, namespace: &Vec<String>, server: &String, comment: &String, dns_sec_enable: bool, dns_sec_ipsec_encryption_type: &String, dns_sec_ipsec_required: bool, dns_sec_validation_required: bool, gpo_name: &String, cmdlet_output: &mut DnsClientNrptRule) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DAEnable".to_string(), value: daenable.into() });
        args.push(MethodParameter { name: "DAIPsecEncryptionType".to_string(), value: daipsec_encryption_type.into() });
        args.push(MethodParameter { name: "DAIPsecRequired".to_string(), value: daipsec_required.into() });
        args.push(MethodParameter { name: "DANameServers".to_string(), value: daname_servers.into() });
        args.push(MethodParameter { name: "DAProxyServerName".to_string(), value: daproxy_server_name.into() });
        args.push(MethodParameter { name: "DAProxyType".to_string(), value: daproxy_type.into() });
        args.push(MethodParameter { name: "DisplayName".to_string(), value: display_name.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "IPsecTrustAuthority".to_string(), value: ipsec_trust_authority.into() });
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "NameEncoding".to_string(), value: name_encoding.into() });
        args.push(MethodParameter { name: "NameServers".to_string(), value: name_servers.into() });
        args.push(MethodParameter { name: "Namespace".to_string(), value: namespace.into() });
        args.push(MethodParameter { name: "Server".to_string(), value: server.into() });
        args.push(MethodParameter { name: "Comment".to_string(), value: comment.into() });
        args.push(MethodParameter { name: "DnsSecEnable".to_string(), value: dns_sec_enable.into() });
        args.push(MethodParameter { name: "DnsSecIPsecEncryptionType".to_string(), value: dns_sec_ipsec_encryption_type.into() });
        args.push(MethodParameter { name: "DnsSecIPsecRequired".to_string(), value: dns_sec_ipsec_required.into() });
        args.push(MethodParameter { name: "DnsSecValidationRequired".to_string(), value: dns_sec_validation_required.into() });
        args.push(MethodParameter { name: "GpoName".to_string(), value: gpo_name.into() });

        let result = self.invoke_method("Set", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

