// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.PEH
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_Runspace struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_Runspace {

/// 
    #[serde(rename = "InstanceId")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl MSFT_Runspace {
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

    /// * `args` -  (serde_json::Value)
    /// * `pipeline_definition_instance_id` -  (String)
    /// * `pipeline_name` -  (String)

    /// * `pipeline` -  (MSFT_Pipeline)
    /// * `return_value` -  (u32)
    pub fn create_pipeline(&self, pipeline_definition_instance_id: &String, pipeline_name: &String, args: serde_json::Value, pipeline: &mut MSFT_Pipeline) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "pipelineDefinitionInstanceId".to_string(), value: pipeline_definition_instance_id.into() });
        args.push(MethodParameter { name: "pipelineName".to_string(), value: pipeline_name.into() });
        args.push(MethodParameter { name: "args".to_string(), value: args.into() });

        let result = self.invoke_method("CreatePipeline", &args)?;
        let pipeline = result.get_value("pipeline")?;
        Ok(result.return_value)

    }


/// 

    /// * `modules` -  (String[])

    /// * `return_value` -  (u32)
    pub fn import_module(&self, modules: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "modules".to_string(), value: modules.into() });
        self.invoke_method("ImportModule", &args)

    }

}

