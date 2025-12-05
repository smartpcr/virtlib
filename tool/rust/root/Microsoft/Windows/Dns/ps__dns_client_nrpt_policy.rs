// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Dns
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PS_DnsClientNrptPolicy struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PS_DnsClientNrptPolicy {
}

impl PS_DnsClientNrptPolicy {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `effective` -  (bool)
    /// * `namespace` -  (String)

    /// * `cmdlet_output` -  (DnsClientPolicyConfiguration[])
    /// * `return_value` -  (u32)
    pub fn get(&self, effective: bool, namespace: &String, cmdlet_output: &mut Vec<DnsClientPolicyConfiguration>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Effective".to_string(), value: effective.into() });
        args.push(MethodParameter { name: "Namespace".to_string(), value: namespace.into() });

        let result = self.invoke_method("Get", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

