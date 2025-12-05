// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_SecurityDescriptorHelper struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_SecurityDescriptorHelper {
}

impl Win32_SecurityDescriptorHelper {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `descriptor` -  (__SecurityDescriptor)

    /// * `return_value` -  (u32)
    /// * `sddl` -  (String)
    pub fn win32_sdto_sddl(&self, descriptor: __SecurityDescriptor, sddl: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Descriptor".to_string(), value: descriptor.into() });

        let result = self.invoke_method("Win32SDToSDDL", &args)?;
        let sddl = result.get_value("SDDL")?;
        Ok(result.return_value)

    }


/// 

    /// * `descriptor` -  (__SecurityDescriptor)

    /// * `binary_sd` -  (u8[])
    /// * `return_value` -  (u32)
    pub fn win32_sdto_binary_sd(&self, descriptor: __SecurityDescriptor, binary_sd: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Descriptor".to_string(), value: descriptor.into() });

        let result = self.invoke_method("Win32SDToBinarySD", &args)?;
        let binary_sd = result.get_value("BinarySD")?;
        Ok(result.return_value)

    }


/// 

    /// * `sddl` -  (String)

    /// * `descriptor` -  (__SecurityDescriptor)
    /// * `return_value` -  (u32)
    pub fn sddlto_win32_sd(&self, sddl: &String, descriptor: &mut __SecurityDescriptor) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SDDL".to_string(), value: sddl.into() });

        let result = self.invoke_method("SDDLToWin32SD", &args)?;
        let descriptor = result.get_value("Descriptor")?;
        Ok(result.return_value)

    }


/// 

    /// * `sddl` -  (String)

    /// * `binary_sd` -  (u8[])
    /// * `return_value` -  (u32)
    pub fn sddlto_binary_sd(&self, sddl: &String, binary_sd: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SDDL".to_string(), value: sddl.into() });

        let result = self.invoke_method("SDDLToBinarySD", &args)?;
        let binary_sd = result.get_value("BinarySD")?;
        Ok(result.return_value)

    }


/// 

    /// * `binary_sd` -  (u8[])

    /// * `descriptor` -  (__SecurityDescriptor)
    /// * `return_value` -  (u32)
    pub fn binary_sdto_win32_sd(&self, binary_sd: &Vec<u8>, descriptor: &mut __SecurityDescriptor) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "BinarySD".to_string(), value: binary_sd.into() });

        let result = self.invoke_method("BinarySDToWin32SD", &args)?;
        let descriptor = result.get_value("Descriptor")?;
        Ok(result.return_value)

    }


/// 

    /// * `binary_sd` -  (u8[])

    /// * `return_value` -  (u32)
    /// * `sddl` -  (String)
    pub fn binary_sdto_sddl(&self, binary_sd: &Vec<u8>, sddl: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "BinarySD".to_string(), value: binary_sd.into() });

        let result = self.invoke_method("BinarySDToSDDL", &args)?;
        let sddl = result.get_value("SDDL")?;
        Ok(result.return_value)

    }

}

