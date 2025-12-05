// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Attestation
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_VsmIdkEncryptionCertificate struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_VsmIdkEncryptionCertificate {
}

impl MSFT_VsmIdkEncryptionCertificate {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `attestation_server_url` -  (String)
    /// * `force` -  (bool)

    /// * `cmdlet_output` -  (MSFT_AttestationResult)
    /// * `return_value` -  (u32)
    pub fn get(&self, attestation_server_url: &String, force: bool, cmdlet_output: &mut MSFT_AttestationResult) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AttestationServerUrl".to_string(), value: attestation_server_url.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });

        let result = self.invoke_method("Get", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

