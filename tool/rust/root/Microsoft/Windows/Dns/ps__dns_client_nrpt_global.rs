// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Dns
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PS_DnsClientNrptGlobal struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PS_DnsClientNrptGlobal {
}

impl PS_DnsClientNrptGlobal {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `enable_dafor_all_networks` -  (String)
    /// * `gpo_name` -  (String)
    /// * `pass_thru` -  (bool)
    /// * `query_policy` -  (String)
    /// * `secure_name_query_fallback` -  (String)
    /// * `server` -  (String)

    /// * `cmdlet_output` -  (DnsClientNrptGlobal)
    /// * `return_value` -  (u32)
    pub fn set(&self, enable_dafor_all_networks: &String, gpo_name: &String, secure_name_query_fallback: &String, query_policy: &String, server: &String, pass_thru: bool, cmdlet_output: &mut DnsClientNrptGlobal) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "EnableDAForAllNetworks".to_string(), value: enable_dafor_all_networks.into() });
        args.push(MethodParameter { name: "GpoName".to_string(), value: gpo_name.into() });
        args.push(MethodParameter { name: "SecureNameQueryFallback".to_string(), value: secure_name_query_fallback.into() });
        args.push(MethodParameter { name: "QueryPolicy".to_string(), value: query_policy.into() });
        args.push(MethodParameter { name: "Server".to_string(), value: server.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("Set", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `gpo_name` -  (String)
    /// * `server` -  (String)

    /// * `cmdlet_output` -  (DnsClientNrptGlobal)
    /// * `return_value` -  (u32)
    pub fn get(&self, server: &String, gpo_name: &String, cmdlet_output: &mut DnsClientNrptGlobal) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Server".to_string(), value: server.into() });
        args.push(MethodParameter { name: "GpoName".to_string(), value: gpo_name.into() });

        let result = self.invoke_method("Get", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

