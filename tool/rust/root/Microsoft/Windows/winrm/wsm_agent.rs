// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.winrm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WsmAgent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WsmAgent {
}

impl WsmAgent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `detail` -  (String)
    /// * `key` -  (String)

    /// * `data` -  (String)
    /// * `return_value` -  (u32)
    pub fn get_data(&self, key: &String, detail: &String, data: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "key".to_string(), value: key.into() });
        args.push(MethodParameter { name: "detail".to_string(), value: detail.into() });

        let result = self.invoke_method("GetData", &args)?;
        let data = result.get_value("data")?;
        Ok(result.return_value)

    }


/// 

    /// * `data` -  (String)
    /// * `detail` -  (String)
    /// * `key` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_data(&self, key: &String, detail: &String, data: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "key".to_string(), value: key.into() });
        args.push(MethodParameter { name: "detail".to_string(), value: detail.into() });
        args.push(MethodParameter { name: "data".to_string(), value: data.into() });
        self.invoke_method("SetData", &args)

    }


/// 

    /// * `detail` -  (String)
    /// * `key` -  (String)

    /// * `return_value` -  (u32)
    pub fn remove_data(&self, key: &String, detail: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "key".to_string(), value: key.into() });
        args.push(MethodParameter { name: "detail".to_string(), value: detail.into() });
        self.invoke_method("RemoveData", &args)

    }

}

