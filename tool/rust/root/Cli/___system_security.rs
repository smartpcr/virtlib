// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Cli
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __SystemSecurity struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __SystemSecurity {
}

impl __SystemSecurity {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `return_value` -  (u32)
    /// * `sd` -  (u8[])
    pub fn get_sd(&self, sd: &mut Vec<u8>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetSD", &[])?;
        let sd = result.get_value("SD")?;
        Ok(result.return_value)

    }


/// 

    /// * `descriptor` -  (__SecurityDescriptor)
    /// * `return_value` -  (u32)
    pub fn get_security_descriptor(&self, descriptor: &mut __SecurityDescriptor) -> Result<(), WmiError> {

        let result = self.invoke_method("GetSecurityDescriptor", &[])?;
        let descriptor = result.get_value("Descriptor")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    /// * `ul` -  (__NTLMUser9X[])
    pub fn get9_xuser_list(&self, ul: &mut Vec<__NTLMUser9X>) -> Result<(), WmiError> {

        let result = self.invoke_method("Get9XUserList", &[])?;
        let ul = result.get_value("ul")?;
        Ok(result.return_value)

    }


/// 

    /// * `sd` -  (u8[])

    /// * `return_value` -  (u32)
    pub fn set_sd(&self, sd: &Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SD".to_string(), value: sd.into() });
        self.invoke_method("SetSD", &args)

    }


/// 

    /// * `descriptor` -  (__SecurityDescriptor)

    /// * `return_value` -  (u32)
    pub fn set_security_descriptor(&self, descriptor: __SecurityDescriptor) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Descriptor".to_string(), value: descriptor.into() });
        self.invoke_method("SetSecurityDescriptor", &args)

    }


/// 

    /// * `ul` -  (__NTLMUser9X[])

    /// * `return_value` -  (u32)
    pub fn set9_xuser_list(&self, ul: &Vec<__NTLMUser9X>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ul".to_string(), value: ul.into() });
        self.invoke_method("Set9XUserList", &args)

    }


/// 

    /// * `return_value` -  (u32)
    /// * `rights` -  (i32)
    pub fn get_caller_access_rights(&self, rights: &mut i32) -> Result<(), WmiError> {

        let result = self.invoke_method("GetCallerAccessRights", &[])?;
        let rights = result.get_value("rights")?;
        Ok(result.return_value)

    }

}

