// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageQoSPolicyStore struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageQoSPolicyStore {

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "IOPSNormalizationSize")]
    pub iopsnormalization_size: Option<u32>,
}

impl MSFT_StorageQoSPolicyStore {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            id: None,
            iopsnormalization_size: None,
        }
    }


    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of IOPSNormalizationSize
    pub fn set_iopsnormalization_size(&mut self, value: u32) {
        self.iopsnormalization_size = Some(value);
    }

    /// Gets the value of IOPSNormalizationSize
    pub fn get_iopsnormalization_size(&self) -> Option<&u32> {
        self.iopsnormalization_size.as_ref()
    }

/// 

    /// * `policy` -  (MSFT_StorageQoSPolicy)

    /// * `policy` -  (MSFT_StorageQoSPolicy)
    /// * `return_value` -  (i32)
    pub fn create_policy(&self, policy: &mut MSFT_StorageQoSPolicy) -> Result<(), WmiError> {
        let mut args = Vec::new();

        let result = self.invoke_method("CreatePolicy", &args)?;
        let policy = result.get_value("Policy")?;
        Ok(result.return_value)

    }


/// 

    /// * `iopsnormalization_size` -  (u32)

    /// * `return_value` -  (i32)
    pub fn set_attributes(&self, iopsnormalization_size: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IOPSNormalizationSize".to_string(), value: iopsnormalization_size.into() });
        self.invoke_method("SetAttributes", &args)

    }

}

