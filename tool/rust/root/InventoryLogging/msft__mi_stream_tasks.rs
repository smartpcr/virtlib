// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.InventoryLogging
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msft_MiStreamTasks struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msft_MiStreamTasks {
}

impl Msft_MiStreamTasks {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `filename` -  (String)

    /// * `results` -  (serde_json::Value[])
    /// * `return_value` -  (u32)
    pub fn collect(&self, filename: &String, results: &mut Vec<serde_json::Value>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Filename".to_string(), value: filename.into() });

        let result = self.invoke_method("Collect", &args)?;
        let results = result.get_value("Results")?;
        Ok(result.return_value)

    }


/// 

    /// * `check_collection_history` -  (bool)
    /// * `filename` -  (String)

    /// * `return_value` -  (u32)
    pub fn push(&self, filename: &String, check_collection_history: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Filename".to_string(), value: filename.into() });
        args.push(MethodParameter { name: "CheckCollectionHistory".to_string(), value: check_collection_history.into() });
        self.invoke_method("Push", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn flush(&self) -> Result<(), WmiError> {
        self.invoke_method("Flush", &[])

    }

}

