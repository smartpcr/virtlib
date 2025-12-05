// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_MetricService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_MetricService {
    #[serde(flatten)]
    pub base: CIM_MetricService,
}

impl Msvm_MetricService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_MetricService::new(),
        }
    }


/// 

    /// * `setting_data` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn modify_service_settings(&self, setting_data: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SettingData".to_string(), value: setting_data.into() });

        let result = self.invoke_method_with_job("ModifyServiceSettings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }

}

impl Msvm_MetricService {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

    /// Gets the related Msvm_AggregationMetricDefinition object(s)
    pub fn get_related__aggregation_metric_definition(&self) -> Result<Vec<Msvm_AggregationMetricDefinition>, WmiError> {
        self.get_all_related("Msvm_AggregationMetricDefinition")
    }

    /// Gets the related Msvm_BaseMetricDefinition object(s)
    pub fn get_related__base_metric_definition(&self) -> Result<Vec<Msvm_BaseMetricDefinition>, WmiError> {
        self.get_all_related("Msvm_BaseMetricDefinition")
    }

    /// Gets the related Msvm_MetricServiceSettingData object(s)
    pub fn get_related__metric_service_setting_data(&self) -> Result<Msvm_MetricServiceSettingData, WmiError> {
        self.get_related("Msvm_MetricServiceSettingData")
    }

    /// Gets the related Msvm_MetricServiceCapabilities object(s)
    pub fn get_related__metric_service_capabilities(&self) -> Result<Msvm_MetricServiceCapabilities, WmiError> {
        self.get_related("Msvm_MetricServiceCapabilities")
    }

}

