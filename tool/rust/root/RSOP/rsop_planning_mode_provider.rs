// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RsopPlanningModeProvider struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RsopPlanningModeProvider {
}

impl RsopPlanningModeProvider {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `computer_gpofilters` -  (String[])
    /// * `computer_name` -  (String)
    /// * `computer_security_groups` -  (String[])
    /// * `computer_som` -  (String)
    /// * `flags` -  (u32)
    /// * `site` -  (String)
    /// * `user_gpofilters` -  (String[])
    /// * `user_name` -  (String)
    /// * `user_security_groups` -  (String[])
    /// * `user_som` -  (String)

    /// * `extended_info` -  (u32)
    /// * `h_result` -  (u32)
    /// * `name_space` -  (String)
    pub fn rsop_create_session(&self, flags: u32, computer_name: &String, computer_som: &String, computer_security_groups: &Vec<String>, computer_gpofilters: &Vec<String>, user_name: &String, user_som: &String, user_security_groups: &Vec<String>, user_gpofilters: &Vec<String>, site: &String, name_space: &mut String, h_result: &mut u32, extended_info: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "flags".to_string(), value: flags.into() });
        args.push(MethodParameter { name: "computerName".to_string(), value: computer_name.into() });
        args.push(MethodParameter { name: "computerSOM".to_string(), value: computer_som.into() });
        args.push(MethodParameter { name: "computerSecurityGroups".to_string(), value: computer_security_groups.into() });
        args.push(MethodParameter { name: "computerGPOFilters".to_string(), value: computer_gpofilters.into() });
        args.push(MethodParameter { name: "userName".to_string(), value: user_name.into() });
        args.push(MethodParameter { name: "userSOM".to_string(), value: user_som.into() });
        args.push(MethodParameter { name: "userSecurityGroups".to_string(), value: user_security_groups.into() });
        args.push(MethodParameter { name: "userGPOFilters".to_string(), value: user_gpofilters.into() });
        args.push(MethodParameter { name: "site".to_string(), value: site.into() });

        let result = self.invoke_method("RsopCreateSession", &args)?;
        let extended_info = result.get_value("ExtendedInfo")?;
        let h_result = result.get_value("hResult")?;
        let name_space = result.get_value("nameSpace")?;
        Ok(result.return_value)

    }


/// 

    /// * `namespace` -  (String)

    /// * `h_result` -  (u32)
    pub fn rsop_delete_session(&self, namespace: &String, h_result: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "namespace".to_string(), value: namespace.into() });

        let result = self.invoke_method("RsopDeleteSession", &args)?;
        let h_result = result.get_value("hResult")?;
        Ok(result.return_value)

    }

}

