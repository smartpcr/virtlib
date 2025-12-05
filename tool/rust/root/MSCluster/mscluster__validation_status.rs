// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_ValidationStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_ValidationStatus {

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,
}

impl MSCluster_ValidationStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            id: None,
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

/// 

    /// * `status` -  (u32)
    pub fn get_status(&self, status: &mut u32) -> Result<(), WmiError> {

        let result = self.invoke_method("GetStatus", &[])?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (bool)
    pub fn is_validation_successful(&self) -> Result<(), WmiError> {
        self.invoke_method("IsValidationSuccessful", &[])

    }


/// 

    /// * `node_names` -  (String[])

    /// * `status` -  (u32)
    pub fn get_node_status(&self, node_names: &Vec<String>, status: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NodeNames".to_string(), value: node_names.into() });

        let result = self.invoke_method("GetNodeStatus", &args)?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }

}

