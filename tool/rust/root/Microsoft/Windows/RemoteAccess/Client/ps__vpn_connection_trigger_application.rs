// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PS_VpnConnectionTriggerApplication struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PS_VpnConnectionTriggerApplication {
}

impl PS_VpnConnectionTriggerApplication {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `application_id` -  (String[])
    /// * `connection_name` -  (String)
    /// * `force` -  (bool)
    /// * `pass_thru` -  (bool)

    /// * `cmdlet_output` -  (VpnConnectionTriggerApplication)
    /// * `return_value` -  (u32)
    pub fn add(&self, connection_name: &String, application_id: &Vec<String>, pass_thru: bool, force: bool, cmdlet_output: &mut VpnConnectionTriggerApplication) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ConnectionName".to_string(), value: connection_name.into() });
        args.push(MethodParameter { name: "ApplicationID".to_string(), value: application_id.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });

        let result = self.invoke_method("Add", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `application_id` -  (String[])
    /// * `connection_name` -  (String)
    /// * `force` -  (bool)
    /// * `pass_thru` -  (bool)

    /// * `cmdlet_output` -  (VpnConnectionTriggerApplication)
    /// * `return_value` -  (u32)
    pub fn remove(&self, connection_name: &String, application_id: &Vec<String>, pass_thru: bool, force: bool, cmdlet_output: &mut VpnConnectionTriggerApplication) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ConnectionName".to_string(), value: connection_name.into() });
        args.push(MethodParameter { name: "ApplicationID".to_string(), value: application_id.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });

        let result = self.invoke_method("Remove", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

