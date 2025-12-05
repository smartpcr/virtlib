// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.PEH
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_Engine struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_Engine {

/// 
    #[serde(rename = "InstanceId")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl MSFT_Engine {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            name: None,
        }
    }


    /// Sets the value of InstanceId
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceId
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

/// 

    /// * `runspace_name` -  (String)

    /// * `return_value` -  (u32)
    /// * `runspace` -  (MSFT_Runspace)
    pub fn create_default_runspace(&self, runspace_name: &String, runspace: &mut MSFT_Runspace) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "runspaceName".to_string(), value: runspace_name.into() });

        let result = self.invoke_method("CreateDefaultRunspace", &args)?;
        let runspace = result.get_value("runspace")?;
        Ok(result.return_value)

    }

}

