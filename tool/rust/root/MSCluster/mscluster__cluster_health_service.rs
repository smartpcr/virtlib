// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_ClusterHealthService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_ClusterHealthService {

/// 
    #[serde(rename = "ElementName")]
    pub element_name: Option<String>,
}

impl MSCluster_ClusterHealthService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            element_name: None,
        }
    }


    /// Sets the value of ElementName
    pub fn set_element_name(&mut self, value: String) {
        self.element_name = Some(value);
    }

    /// Gets the value of ElementName
    pub fn get_element_name(&self) -> Option<&String> {
        self.element_name.as_ref()
    }

/// 

    /// * `flags` -  (u32)
    /// * `metric_name` -  (String[])
    /// * `stream_name` -  (String)

    /// * `datapoints` -  (MSCluster_HealthMetric[])
    /// * `return_value` -  (u32)
    pub fn get_metric(&self, metric_name: &Vec<String>, stream_name: &String, flags: u32, datapoints: &mut Vec<MSCluster_HealthMetric>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "MetricName".to_string(), value: metric_name.into() });
        args.push(MethodParameter { name: "StreamName".to_string(), value: stream_name.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });

        let result = self.invoke_method("GetMetric", &args)?;
        let datapoints = result.get_value("Datapoints")?;
        Ok(result.return_value)

    }


/// 

    /// * `reporting_key` -  (String)
    /// * `reporting_type` -  (String)

    /// * `faults` -  (MSCluster_HealthFault[])
    /// * `return_value` -  (u32)
    pub fn get_fault(&self, reporting_key: &String, reporting_type: &String, faults: &mut Vec<MSCluster_HealthFault>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ReportingKey".to_string(), value: reporting_key.into() });
        args.push(MethodParameter { name: "ReportingType".to_string(), value: reporting_type.into() });

        let result = self.invoke_method("GetFault", &args)?;
        let faults = result.get_value("Faults")?;
        Ok(result.return_value)

    }


/// 

    /// * `flags` -  (u32)
    /// * `name` -  (String)
    /// * `parameters` -  (String[])
    /// * `timeout` -  (u32)

    /// * `command_id` -  (String)
    /// * `return_value` -  (u32)
    /// * `status` -  (i32)
    pub fn invoke_command(&self, name: &String, flags: u32, parameters: &Vec<String>, timeout: u32, command_id: &mut String, status: &mut i32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        args.push(MethodParameter { name: "Parameters".to_string(), value: parameters.into() });
        args.push(MethodParameter { name: "Timeout".to_string(), value: timeout.into() });

        let result = self.invoke_method("InvokeCommand", &args)?;
        let command_id = result.get_value("CommandId")?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }

}

