// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RsopLoggingModeProvider struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RsopLoggingModeProvider {
}

impl RsopLoggingModeProvider {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `flags` -  (u32)
    /// * `user_sid` -  (String)

    /// * `extended_info` -  (u32)
    /// * `h_result` -  (u32)
    /// * `name_space` -  (String)
    pub fn rsop_create_session(&self, flags: u32, user_sid: &String, name_space: &mut String, h_result: &mut u32, extended_info: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "flags".to_string(), value: flags.into() });
        args.push(MethodParameter { name: "userSid".to_string(), value: user_sid.into() });

        let result = self.invoke_method("RsopCreateSession", &args)?;
        let extended_info = result.get_value("ExtendedInfo")?;
        let h_result = result.get_value("hResult")?;
        let name_space = result.get_value("nameSpace")?;
        Ok(result.return_value)

    }


/// 

    /// * `name_space` -  (String)

    /// * `h_result` -  (u32)
    pub fn rsop_delete_session(&self, name_space: &String, h_result: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "nameSpace".to_string(), value: name_space.into() });

        let result = self.invoke_method("RsopDeleteSession", &args)?;
        let h_result = result.get_value("hResult")?;
        Ok(result.return_value)

    }


/// 

    /// * `h_result` -  (u32)
    /// * `user_sids` -  (String[])
    pub fn rsop_enumerate_users(&self, user_sids: &mut Vec<String>, h_result: &mut u32) -> Result<(), WmiError> {

        let result = self.invoke_method("RsopEnumerateUsers", &[])?;
        let h_result = result.get_value("hResult")?;
        let user_sids = result.get_value("userSids")?;
        Ok(result.return_value)

    }

}

