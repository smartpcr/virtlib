// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetGPO struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetGPO {
    #[serde(flatten)]
    pub base: CIM_SettingData,
}

impl MSFT_NetGPO {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
        }
    }


/// 

    /// * `domain_controller` -  (String)
    /// * `policy_store` -  (String)

    /// * `gposession` -  (String)
    /// * `return_value` -  (u32)
    pub fn open(&self, policy_store: &String, domain_controller: &String, gposession: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PolicyStore".to_string(), value: policy_store.into() });
        args.push(MethodParameter { name: "DomainController".to_string(), value: domain_controller.into() });

        let result = self.invoke_method("Open", &args)?;
        let gposession = result.get_value("GPOSession")?;
        Ok(result.return_value)

    }


/// 

    /// * `gposession` -  (String)

    /// * `return_value` -  (u32)
    pub fn save(&self, gposession: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "GPOSession".to_string(), value: gposession.into() });
        self.invoke_method("Save", &args)

    }

}

