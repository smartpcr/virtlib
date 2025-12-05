// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PS_VpnConnectionTriggerDnsConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PS_VpnConnectionTriggerDnsConfiguration {

/// 
    #[serde(rename = "DnsIPAddress")]
    pub dns_ipaddress: Vec<String>,

/// 
    #[serde(rename = "DnsSuffix")]
    pub dns_suffix: Option<String>,
}

impl PS_VpnConnectionTriggerDnsConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            dns_ipaddress: Vec::new(),
            dns_suffix: None,
        }
    }


    /// Sets the value of DnsIPAddress
    pub fn set_dns_ipaddress(&mut self, value: Vec<String>) {
        self.dns_ipaddress = value;
    }

    /// Gets the value of DnsIPAddress
    pub fn get_dns_ipaddress(&self) -> &Vec<String> {
        &self.dns_ipaddress
    }

    /// Sets the value of DnsSuffix
    pub fn set_dns_suffix(&mut self, value: String) {
        self.dns_suffix = Some(value);
    }

    /// Gets the value of DnsSuffix
    pub fn get_dns_suffix(&self) -> Option<&String> {
        self.dns_suffix.as_ref()
    }

/// 

    /// * `connection_name` -  (String)
    /// * `dns_ipaddress` -  (String[])
    /// * `dns_suffix` -  (String)
    /// * `force` -  (bool)
    /// * `pass_thru` -  (bool)

    /// * `cmdlet_output` -  (VpnConnectionTriggerDnsConfiguration)
    /// * `return_value` -  (u32)
    pub fn add(&self, connection_name: &String, dns_suffix: &String, dns_ipaddress: &Vec<String>, pass_thru: bool, force: bool, cmdlet_output: &mut VpnConnectionTriggerDnsConfiguration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ConnectionName".to_string(), value: connection_name.into() });
        args.push(MethodParameter { name: "DnsSuffix".to_string(), value: dns_suffix.into() });
        args.push(MethodParameter { name: "DnsIPAddress".to_string(), value: dns_ipaddress.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });

        let result = self.invoke_method("Add", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `connection_name` -  (String)
    /// * `dns_ipaddress` -  (String[])
    /// * `dns_suffix` -  (String)
    /// * `dns_suffix_search_list` -  (String[])
    /// * `force` -  (bool)
    /// * `pass_thru` -  (bool)

    /// * `cmdlet_output` -  (VpnConnectionTriggerDnsConfiguration)
    /// * `return_value` -  (u32)
    pub fn set(&self, connection_name: &String, dns_suffix: &String, dns_ipaddress: &Vec<String>, dns_suffix_search_list: &Vec<String>, pass_thru: bool, force: bool, cmdlet_output: &mut VpnConnectionTriggerDnsConfiguration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ConnectionName".to_string(), value: connection_name.into() });
        args.push(MethodParameter { name: "DnsSuffix".to_string(), value: dns_suffix.into() });
        args.push(MethodParameter { name: "DnsIPAddress".to_string(), value: dns_ipaddress.into() });
        args.push(MethodParameter { name: "DnsSuffixSearchList".to_string(), value: dns_suffix_search_list.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });

        let result = self.invoke_method("Set", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `connection_name` -  (String)
    /// * `dns_suffix` -  (String[])
    /// * `force` -  (bool)
    /// * `pass_thru` -  (bool)

    /// * `cmdlet_output` -  (VpnConnectionTriggerDnsConfiguration[])
    /// * `return_value` -  (u32)
    pub fn remove(&self, connection_name: &String, dns_suffix: &Vec<String>, pass_thru: bool, force: bool, cmdlet_output: &mut Vec<VpnConnectionTriggerDnsConfiguration>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ConnectionName".to_string(), value: connection_name.into() });
        args.push(MethodParameter { name: "DnsSuffix".to_string(), value: dns_suffix.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });

        let result = self.invoke_method("Remove", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

