// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Hgs
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_HgsKeyProtectorOperations struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_HgsKeyProtectorOperations {
}

impl MSFT_HgsKeyProtectorOperations {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `ingress_key_protector` -  (u8[])

    /// * `egress_key_protector` -  (u8[])
    /// * `encrypted_keys` -  (u8[])
    /// * `encrypted_transfer_key` -  (u8[])
    /// * `encrypted_wrapping_key` -  (u8[])
    /// * `return_value` -  (u32)
    pub fn unwrap_key_protector(&self, ingress_key_protector: &Vec<u8>, encrypted_transfer_key: &mut Vec<u8>, encrypted_wrapping_key: &mut Vec<u8>, encrypted_keys: &mut Vec<u8>, egress_key_protector: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IngressKeyProtector".to_string(), value: ingress_key_protector.into() });

        let result = self.invoke_method("UnwrapKeyProtector", &args)?;
        let egress_key_protector = result.get_value("EgressKeyProtector")?;
        let encrypted_keys = result.get_value("EncryptedKeys")?;
        let encrypted_transfer_key = result.get_value("EncryptedTransferKey")?;
        let encrypted_wrapping_key = result.get_value("EncryptedWrappingKey")?;
        Ok(result.return_value)

    }


/// 

    /// * `egress_key_protector` -  (u8[])
    /// * `encrypted_keys` -  (u8[])
    /// * `encrypted_transfer_key` -  (u8[])
    /// * `encrypted_wrapping_key` -  (u8[])
    /// * `return_value` -  (u32)
    pub fn create_key_protector(&self, encrypted_transfer_key: &mut Vec<u8>, encrypted_wrapping_key: &mut Vec<u8>, encrypted_keys: &mut Vec<u8>, egress_key_protector: &mut Vec<u8>) -> Result<(), WmiError> {

        let result = self.invoke_method("CreateKeyProtector", &[])?;
        let egress_key_protector = result.get_value("EgressKeyProtector")?;
        let encrypted_keys = result.get_value("EncryptedKeys")?;
        let encrypted_transfer_key = result.get_value("EncryptedTransferKey")?;
        let encrypted_wrapping_key = result.get_value("EncryptedWrappingKey")?;
        Ok(result.return_value)

    }


/// 

    /// * `base_key_protector` -  (u8[])
    /// * `plaintext_data` -  (u8[])
    /// * `roll_key_protector` -  (bool)
    /// * `unique_encryption_identifier` -  (u32)

    /// * `egress_key_protector` -  (u8[])
    /// * `encrypted_data` -  (u8[])
    /// * `return_value` -  (u32)
    pub fn encrypt_data_with_key_protector(&self, base_key_protector: &Vec<u8>, unique_encryption_identifier: u32, plaintext_data: &Vec<u8>, roll_key_protector: bool, egress_key_protector: &mut Vec<u8>, encrypted_data: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "BaseKeyProtector".to_string(), value: base_key_protector.into() });
        args.push(MethodParameter { name: "UniqueEncryptionIdentifier".to_string(), value: unique_encryption_identifier.into() });
        args.push(MethodParameter { name: "PlaintextData".to_string(), value: plaintext_data.into() });
        args.push(MethodParameter { name: "RollKeyProtector".to_string(), value: roll_key_protector.into() });

        let result = self.invoke_method("EncryptDataWithKeyProtector", &args)?;
        let egress_key_protector = result.get_value("EgressKeyProtector")?;
        let encrypted_data = result.get_value("EncryptedData")?;
        Ok(result.return_value)

    }


/// 

    /// * `base_key_protector` -  (u8[])
    /// * `encrypted_data` -  (u8[])

    /// * `plaintext_data` -  (u8[])
    /// * `return_value` -  (u32)
    pub fn decrypt_data_with_key_protector(&self, base_key_protector: &Vec<u8>, encrypted_data: &Vec<u8>, plaintext_data: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "BaseKeyProtector".to_string(), value: base_key_protector.into() });
        args.push(MethodParameter { name: "EncryptedData".to_string(), value: encrypted_data.into() });

        let result = self.invoke_method("DecryptDataWithKeyProtector", &args)?;
        let plaintext_data = result.get_value("PlaintextData")?;
        Ok(result.return_value)

    }

}

