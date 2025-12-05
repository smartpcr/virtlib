// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PS_VpnConnectionTriggerTrustedNetwork struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PS_VpnConnectionTriggerTrustedNetwork {
}

impl PS_VpnConnectionTriggerTrustedNetwork {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `connection_name` -  (String)
    /// * `dns_suffix` -  (String[])
    /// * `force` -  (bool)
    /// * `pass_thru` -  (bool)

    /// * `cmdlet_output` -  (VpnConnectionTriggerTrustedNetwork)
    /// * `return_value` -  (u32)
    pub fn add(&self, connection_name: &String, dns_suffix: &Vec<String>, pass_thru: bool, force: bool, cmdlet_output: &mut VpnConnectionTriggerTrustedNetwork) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ConnectionName".to_string(), value: connection_name.into() });
        args.push(MethodParameter { name: "DnsSuffix".to_string(), value: dns_suffix.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });

        let result = self.invoke_method("Add", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `connection_name` -  (String)
    /// * `dns_suffix` -  (String[])
    /// * `force` -  (bool)
    /// * `pass_thru` -  (bool)

    /// * `cmdlet_output` -  (VpnConnectionTriggerTrustedNetwork)
    /// * `return_value` -  (u32)
    pub fn remove(&self, connection_name: &String, dns_suffix: &Vec<String>, pass_thru: bool, force: bool, cmdlet_output: &mut VpnConnectionTriggerTrustedNetwork) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ConnectionName".to_string(), value: connection_name.into() });
        args.push(MethodParameter { name: "DnsSuffix".to_string(), value: dns_suffix.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });

        let result = self.invoke_method("Remove", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `connection_name` -  (String)
    /// * `default_dns_suffixes` -  (bool)
    /// * `force` -  (bool)
    /// * `pass_thru` -  (bool)

    /// * `cmdlet_output` -  (VpnConnectionTriggerTrustedNetwork)
    /// * `return_value` -  (u32)
    pub fn set(&self, connection_name: &String, default_dns_suffixes: bool, pass_thru: bool, force: bool, cmdlet_output: &mut VpnConnectionTriggerTrustedNetwork) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ConnectionName".to_string(), value: connection_name.into() });
        args.push(MethodParameter { name: "DefaultDnsSuffixes".to_string(), value: default_dns_suffixes.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });

        let result = self.invoke_method("Set", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

