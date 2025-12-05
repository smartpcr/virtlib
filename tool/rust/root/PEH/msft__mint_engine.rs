// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.PEH
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MintEngine struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MintEngine {
    #[serde(flatten)]
    pub base: MSFT_Engine,
}

impl MSFT_MintEngine {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_Engine::new(),
        }
    }


/// 

    /// * `args` -  (serde_json::Value)
    /// * `engine` -  (MSFT_Engine)
    /// * `expression` -  (MSFT_Expression)
    /// * `modules` -  (String[])
    /// * `pipeline_execution_name` -  (String)

    /// * `pipeline_execution` -  (MSFT_PipelineExecution)
    /// * `return_value` -  (u32)
    pub fn execute_expression(&self, expression: MSFT_Expression, args: serde_json::Value, modules: &Vec<String>, engine: MSFT_Engine, pipeline_execution_name: &String, pipeline_execution: &mut MSFT_PipelineExecution) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "expression".to_string(), value: expression.into() });
        args.push(MethodParameter { name: "args".to_string(), value: args.into() });
        args.push(MethodParameter { name: "modules".to_string(), value: modules.into() });
        args.push(MethodParameter { name: "engine".to_string(), value: engine.into() });
        args.push(MethodParameter { name: "pipelineExecutionName".to_string(), value: pipeline_execution_name.into() });

        let result = self.invoke_method("ExecuteExpression", &args)?;
        let pipeline_execution = result.get_value("pipelineExecution")?;
        Ok(result.return_value)

    }


/// 

    /// * `flags` -  (u32)

    /// * `return_value` -  (u32)
    pub fn wakeup(&self, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("Wakeup", &args)

    }

}

