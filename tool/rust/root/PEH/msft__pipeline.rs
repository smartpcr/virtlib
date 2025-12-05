// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.PEH
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_Pipeline struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_Pipeline {

/// 
    #[serde(rename = "InstanceId")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl MSFT_Pipeline {
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

    /// * `pipeline_execution_name` -  (String)

    /// * `pipeline_execution` -  (MSFT_PipelineExecution)
    /// * `return_value` -  (u32)
    pub fn execute(&self, pipeline_execution_name: &String, pipeline_execution: &mut MSFT_PipelineExecution) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "pipelineExecutionName".to_string(), value: pipeline_execution_name.into() });

        let result = self.invoke_method("Execute", &args)?;
        let pipeline_execution = result.get_value("pipelineExecution")?;
        Ok(result.return_value)

    }

}

