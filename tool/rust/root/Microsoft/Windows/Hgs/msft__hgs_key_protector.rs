// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Hgs
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_HgsKeyProtector struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_HgsKeyProtector {

/// 
    #[serde(rename = "Guardians")]
    pub guardians: Vec<MSFT_HgsGuardian>,

/// 
    #[serde(rename = "Owner")]
    pub owner: Option<MSFT_HgsGuardian>,

/// 
    #[serde(rename = "RawData")]
    pub raw_data: Vec<u8>,
}

impl MSFT_HgsKeyProtector {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            guardians: Vec::new(),
            owner: None,
            raw_data: Vec::new(),
        }
    }


    /// Sets the value of Guardians
    pub fn set_guardians(&mut self, value: Vec<MSFT_HgsGuardian>) {
        self.guardians = value;
    }

    /// Gets the value of Guardians
    pub fn get_guardians(&self) -> &Vec<MSFT_HgsGuardian> {
        &self.guardians
    }

    /// Sets the value of Owner
    pub fn set_owner(&mut self, value: MSFT_HgsGuardian) {
        self.owner = Some(value);
    }

    /// Gets the value of Owner
    pub fn get_owner(&self) -> Option<&MSFT_HgsGuardian> {
        self.owner.as_ref()
    }

    /// Sets the value of RawData
    pub fn set_raw_data(&mut self, value: Vec<u8>) {
        self.raw_data = value;
    }

    /// Gets the value of RawData
    pub fn get_raw_data(&self) -> &Vec<u8> {
        &self.raw_data
    }

/// 

    /// * `allow_expired` -  (bool)
    /// * `allow_untrusted_root` -  (bool)
    /// * `guardian` -  (MSFT_HgsGuardian[])
    /// * `owner` -  (MSFT_HgsGuardian)

    /// * `cmdlet_output` -  (MSFT_HgsKeyProtector)
    /// * `return_value` -  (u32)
    pub fn new_by_guardians(&self, allow_untrusted_root: bool, allow_expired: bool, owner: MSFT_HgsGuardian, guardian: &Vec<MSFT_HgsGuardian>, cmdlet_output: &mut MSFT_HgsKeyProtector) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AllowUntrustedRoot".to_string(), value: allow_untrusted_root.into() });
        args.push(MethodParameter { name: "AllowExpired".to_string(), value: allow_expired.into() });
        args.push(MethodParameter { name: "Owner".to_string(), value: owner.into() });
        args.push(MethodParameter { name: "Guardian".to_string(), value: guardian.into() });

        let result = self.invoke_method("NewByGuardians", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `allow_expired` -  (bool)
    /// * `allow_untrusted_root` -  (bool)
    /// * `guardian` -  (MSFT_HgsGuardian)
    /// * `key_protector` -  (MSFT_HgsKeyProtector)

    /// * `cmdlet_output` -  (MSFT_HgsKeyProtector)
    /// * `return_value` -  (u32)
    pub fn grant(&self, key_protector: MSFT_HgsKeyProtector, guardian: MSFT_HgsGuardian, allow_expired: bool, allow_untrusted_root: bool, cmdlet_output: &mut MSFT_HgsKeyProtector) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "KeyProtector".to_string(), value: key_protector.into() });
        args.push(MethodParameter { name: "Guardian".to_string(), value: guardian.into() });
        args.push(MethodParameter { name: "AllowExpired".to_string(), value: allow_expired.into() });
        args.push(MethodParameter { name: "AllowUntrustedRoot".to_string(), value: allow_untrusted_root.into() });

        let result = self.invoke_method("Grant", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `guardian` -  (MSFT_HgsGuardian)
    /// * `key_protector` -  (MSFT_HgsKeyProtector)

    /// * `cmdlet_output` -  (MSFT_HgsKeyProtector)
    /// * `return_value` -  (u32)
    pub fn revoke(&self, key_protector: MSFT_HgsKeyProtector, guardian: MSFT_HgsGuardian, cmdlet_output: &mut MSFT_HgsKeyProtector) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "KeyProtector".to_string(), value: key_protector.into() });
        args.push(MethodParameter { name: "Guardian".to_string(), value: guardian.into() });

        let result = self.invoke_method("Revoke", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `bytes` -  (u8[])

    /// * `cmdlet_output` -  (MSFT_HgsKeyProtector)
    /// * `return_value` -  (u32)
    pub fn convert_to_by_raw_bytes(&self, bytes: &Vec<u8>, cmdlet_output: &mut MSFT_HgsKeyProtector) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Bytes".to_string(), value: bytes.into() });

        let result = self.invoke_method("ConvertToByRawBytes", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

