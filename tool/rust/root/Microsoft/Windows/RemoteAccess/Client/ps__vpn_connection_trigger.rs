// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PS_VpnConnectionTrigger struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PS_VpnConnectionTrigger {
}

impl PS_VpnConnectionTrigger {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `connection_name` -  (String)

    /// * `cmdlet_output` -  (VpnConnectionTrigger)
    /// * `return_value` -  (u32)
    pub fn get(&self, connection_name: &String, cmdlet_output: &mut VpnConnectionTrigger) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ConnectionName".to_string(), value: connection_name.into() });

        let result = self.invoke_method("Get", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

