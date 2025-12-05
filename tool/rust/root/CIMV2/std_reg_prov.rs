// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// StdRegProv struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StdRegProv {
}

impl StdRegProv {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `h_def_key` -  (u32)
    /// * `s_sub_key_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn create_key(&self, h_def_key: u32, s_sub_key_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "hDefKey".to_string(), value: h_def_key.into() });
        args.push(MethodParameter { name: "sSubKeyName".to_string(), value: s_sub_key_name.into() });
        self.invoke_method("CreateKey", &args)

    }


/// 

    /// * `h_def_key` -  (u32)
    /// * `s_sub_key_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn delete_key(&self, h_def_key: u32, s_sub_key_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "hDefKey".to_string(), value: h_def_key.into() });
        args.push(MethodParameter { name: "sSubKeyName".to_string(), value: s_sub_key_name.into() });
        self.invoke_method("DeleteKey", &args)

    }


/// 

    /// * `h_def_key` -  (u32)
    /// * `s_sub_key_name` -  (String)

    /// * `return_value` -  (u32)
    /// * `s_names` -  (String[])
    pub fn enum_key(&self, h_def_key: u32, s_sub_key_name: &String, s_names: &mut Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "hDefKey".to_string(), value: h_def_key.into() });
        args.push(MethodParameter { name: "sSubKeyName".to_string(), value: s_sub_key_name.into() });

        let result = self.invoke_method("EnumKey", &args)?;
        let s_names = result.get_value("sNames")?;
        Ok(result.return_value)

    }


/// 

    /// * `h_def_key` -  (u32)
    /// * `s_sub_key_name` -  (String)

    /// * `return_value` -  (u32)
    /// * `s_names` -  (String[])
    /// * `types` -  (i32[])
    pub fn enum_values(&self, h_def_key: u32, s_sub_key_name: &String, s_names: &mut Vec<String>, types: &mut Vec<i32>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "hDefKey".to_string(), value: h_def_key.into() });
        args.push(MethodParameter { name: "sSubKeyName".to_string(), value: s_sub_key_name.into() });

        let result = self.invoke_method("EnumValues", &args)?;
        let s_names = result.get_value("sNames")?;
        let types = result.get_value("Types")?;
        Ok(result.return_value)

    }


/// 

    /// * `h_def_key` -  (u32)
    /// * `s_sub_key_name` -  (String)
    /// * `s_value_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn delete_value(&self, h_def_key: u32, s_sub_key_name: &String, s_value_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "hDefKey".to_string(), value: h_def_key.into() });
        args.push(MethodParameter { name: "sSubKeyName".to_string(), value: s_sub_key_name.into() });
        args.push(MethodParameter { name: "sValueName".to_string(), value: s_value_name.into() });
        self.invoke_method("DeleteValue", &args)

    }


/// 

    /// * `h_def_key` -  (u32)
    /// * `s_sub_key_name` -  (String)
    /// * `s_value_name` -  (String)
    /// * `u_value` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_dwordvalue(&self, h_def_key: u32, s_sub_key_name: &String, s_value_name: &String, u_value: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "hDefKey".to_string(), value: h_def_key.into() });
        args.push(MethodParameter { name: "sSubKeyName".to_string(), value: s_sub_key_name.into() });
        args.push(MethodParameter { name: "sValueName".to_string(), value: s_value_name.into() });
        args.push(MethodParameter { name: "uValue".to_string(), value: u_value.into() });
        self.invoke_method("SetDWORDValue", &args)

    }


/// 

    /// * `h_def_key` -  (u32)
    /// * `s_sub_key_name` -  (String)
    /// * `s_value_name` -  (String)
    /// * `u_value` -  (u64)

    /// * `return_value` -  (u32)
    pub fn set_qwordvalue(&self, h_def_key: u32, s_sub_key_name: &String, s_value_name: &String, u_value: u64) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "hDefKey".to_string(), value: h_def_key.into() });
        args.push(MethodParameter { name: "sSubKeyName".to_string(), value: s_sub_key_name.into() });
        args.push(MethodParameter { name: "sValueName".to_string(), value: s_value_name.into() });
        args.push(MethodParameter { name: "uValue".to_string(), value: u_value.into() });
        self.invoke_method("SetQWORDValue", &args)

    }


/// 

    /// * `h_def_key` -  (u32)
    /// * `s_sub_key_name` -  (String)
    /// * `s_value_name` -  (String)

    /// * `return_value` -  (u32)
    /// * `u_value` -  (u32)
    pub fn get_dwordvalue(&self, h_def_key: u32, s_sub_key_name: &String, s_value_name: &String, u_value: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "hDefKey".to_string(), value: h_def_key.into() });
        args.push(MethodParameter { name: "sSubKeyName".to_string(), value: s_sub_key_name.into() });
        args.push(MethodParameter { name: "sValueName".to_string(), value: s_value_name.into() });

        let result = self.invoke_method("GetDWORDValue", &args)?;
        let u_value = result.get_value("uValue")?;
        Ok(result.return_value)

    }


/// 

    /// * `h_def_key` -  (u32)
    /// * `s_sub_key_name` -  (String)
    /// * `s_value_name` -  (String)

    /// * `return_value` -  (u32)
    /// * `u_value` -  (u64)
    pub fn get_qwordvalue(&self, h_def_key: u32, s_sub_key_name: &String, s_value_name: &String, u_value: &mut u64) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "hDefKey".to_string(), value: h_def_key.into() });
        args.push(MethodParameter { name: "sSubKeyName".to_string(), value: s_sub_key_name.into() });
        args.push(MethodParameter { name: "sValueName".to_string(), value: s_value_name.into() });

        let result = self.invoke_method("GetQWORDValue", &args)?;
        let u_value = result.get_value("uValue")?;
        Ok(result.return_value)

    }


/// 

    /// * `h_def_key` -  (u32)
    /// * `s_sub_key_name` -  (String)
    /// * `s_value` -  (String)
    /// * `s_value_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_string_value(&self, h_def_key: u32, s_sub_key_name: &String, s_value_name: &String, s_value: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "hDefKey".to_string(), value: h_def_key.into() });
        args.push(MethodParameter { name: "sSubKeyName".to_string(), value: s_sub_key_name.into() });
        args.push(MethodParameter { name: "sValueName".to_string(), value: s_value_name.into() });
        args.push(MethodParameter { name: "sValue".to_string(), value: s_value.into() });
        self.invoke_method("SetStringValue", &args)

    }


/// 

    /// * `h_def_key` -  (u32)
    /// * `s_sub_key_name` -  (String)
    /// * `s_value_name` -  (String)

    /// * `return_value` -  (u32)
    /// * `s_value` -  (String)
    pub fn get_string_value(&self, h_def_key: u32, s_sub_key_name: &String, s_value_name: &String, s_value: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "hDefKey".to_string(), value: h_def_key.into() });
        args.push(MethodParameter { name: "sSubKeyName".to_string(), value: s_sub_key_name.into() });
        args.push(MethodParameter { name: "sValueName".to_string(), value: s_value_name.into() });

        let result = self.invoke_method("GetStringValue", &args)?;
        let s_value = result.get_value("sValue")?;
        Ok(result.return_value)

    }


/// 

    /// * `h_def_key` -  (u32)
    /// * `s_sub_key_name` -  (String)
    /// * `s_value` -  (String[])
    /// * `s_value_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_multi_string_value(&self, h_def_key: u32, s_sub_key_name: &String, s_value_name: &String, s_value: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "hDefKey".to_string(), value: h_def_key.into() });
        args.push(MethodParameter { name: "sSubKeyName".to_string(), value: s_sub_key_name.into() });
        args.push(MethodParameter { name: "sValueName".to_string(), value: s_value_name.into() });
        args.push(MethodParameter { name: "sValue".to_string(), value: s_value.into() });
        self.invoke_method("SetMultiStringValue", &args)

    }


/// 

    /// * `h_def_key` -  (u32)
    /// * `s_sub_key_name` -  (String)
    /// * `s_value_name` -  (String)

    /// * `return_value` -  (u32)
    /// * `s_value` -  (String[])
    pub fn get_multi_string_value(&self, h_def_key: u32, s_sub_key_name: &String, s_value_name: &String, s_value: &mut Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "hDefKey".to_string(), value: h_def_key.into() });
        args.push(MethodParameter { name: "sSubKeyName".to_string(), value: s_sub_key_name.into() });
        args.push(MethodParameter { name: "sValueName".to_string(), value: s_value_name.into() });

        let result = self.invoke_method("GetMultiStringValue", &args)?;
        let s_value = result.get_value("sValue")?;
        Ok(result.return_value)

    }


/// 

    /// * `h_def_key` -  (u32)
    /// * `s_sub_key_name` -  (String)
    /// * `s_value` -  (String)
    /// * `s_value_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_expanded_string_value(&self, h_def_key: u32, s_sub_key_name: &String, s_value_name: &String, s_value: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "hDefKey".to_string(), value: h_def_key.into() });
        args.push(MethodParameter { name: "sSubKeyName".to_string(), value: s_sub_key_name.into() });
        args.push(MethodParameter { name: "sValueName".to_string(), value: s_value_name.into() });
        args.push(MethodParameter { name: "sValue".to_string(), value: s_value.into() });
        self.invoke_method("SetExpandedStringValue", &args)

    }


/// 

    /// * `h_def_key` -  (u32)
    /// * `s_sub_key_name` -  (String)
    /// * `s_value_name` -  (String)

    /// * `return_value` -  (u32)
    /// * `s_value` -  (String)
    pub fn get_expanded_string_value(&self, h_def_key: u32, s_sub_key_name: &String, s_value_name: &String, s_value: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "hDefKey".to_string(), value: h_def_key.into() });
        args.push(MethodParameter { name: "sSubKeyName".to_string(), value: s_sub_key_name.into() });
        args.push(MethodParameter { name: "sValueName".to_string(), value: s_value_name.into() });

        let result = self.invoke_method("GetExpandedStringValue", &args)?;
        let s_value = result.get_value("sValue")?;
        Ok(result.return_value)

    }


/// 

    /// * `h_def_key` -  (u32)
    /// * `s_sub_key_name` -  (String)
    /// * `s_value_name` -  (String)
    /// * `u_value` -  (u8[])

    /// * `return_value` -  (u32)
    pub fn set_binary_value(&self, h_def_key: u32, s_sub_key_name: &String, s_value_name: &String, u_value: &Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "hDefKey".to_string(), value: h_def_key.into() });
        args.push(MethodParameter { name: "sSubKeyName".to_string(), value: s_sub_key_name.into() });
        args.push(MethodParameter { name: "sValueName".to_string(), value: s_value_name.into() });
        args.push(MethodParameter { name: "uValue".to_string(), value: u_value.into() });
        self.invoke_method("SetBinaryValue", &args)

    }


/// 

    /// * `h_def_key` -  (u32)
    /// * `s_sub_key_name` -  (String)
    /// * `s_value_name` -  (String)

    /// * `return_value` -  (u32)
    /// * `u_value` -  (u8[])
    pub fn get_binary_value(&self, h_def_key: u32, s_sub_key_name: &String, s_value_name: &String, u_value: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "hDefKey".to_string(), value: h_def_key.into() });
        args.push(MethodParameter { name: "sSubKeyName".to_string(), value: s_sub_key_name.into() });
        args.push(MethodParameter { name: "sValueName".to_string(), value: s_value_name.into() });

        let result = self.invoke_method("GetBinaryValue", &args)?;
        let u_value = result.get_value("uValue")?;
        Ok(result.return_value)

    }


/// 

    /// * `h_def_key` -  (u32)
    /// * `s_sub_key_name` -  (String)
    /// * `u_required` -  (u32)

    /// * `b_granted` -  (bool)
    /// * `return_value` -  (u32)
    pub fn check_access(&self, h_def_key: u32, s_sub_key_name: &String, u_required: u32, b_granted: &mut bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "hDefKey".to_string(), value: h_def_key.into() });
        args.push(MethodParameter { name: "sSubKeyName".to_string(), value: s_sub_key_name.into() });
        args.push(MethodParameter { name: "uRequired".to_string(), value: u_required.into() });

        let result = self.invoke_method("CheckAccess", &args)?;
        let b_granted = result.get_value("bGranted")?;
        Ok(result.return_value)

    }


/// 

    /// * `descriptor` -  (__SecurityDescriptor)
    /// * `h_def_key` -  (u32)
    /// * `s_sub_key_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_security_descriptor(&self, h_def_key: u32, s_sub_key_name: &String, descriptor: __SecurityDescriptor) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "hDefKey".to_string(), value: h_def_key.into() });
        args.push(MethodParameter { name: "sSubKeyName".to_string(), value: s_sub_key_name.into() });
        args.push(MethodParameter { name: "Descriptor".to_string(), value: descriptor.into() });
        self.invoke_method("SetSecurityDescriptor", &args)

    }


/// 

    /// * `h_def_key` -  (u32)
    /// * `s_sub_key_name` -  (String)

    /// * `descriptor` -  (__SecurityDescriptor)
    /// * `return_value` -  (u32)
    pub fn get_security_descriptor(&self, h_def_key: u32, s_sub_key_name: &String, descriptor: &mut __SecurityDescriptor) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "hDefKey".to_string(), value: h_def_key.into() });
        args.push(MethodParameter { name: "sSubKeyName".to_string(), value: s_sub_key_name.into() });

        let result = self.invoke_method("GetSecurityDescriptor", &args)?;
        let descriptor = result.get_value("Descriptor")?;
        Ok(result.return_value)

    }

}

