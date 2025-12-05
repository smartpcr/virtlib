// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_ProcessorPool struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_ProcessorPool {
    #[serde(flatten)]
    pub base: CIM_ResourcePool,
}

impl Msvm_ProcessorPool {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ResourcePool::new(),
        }
    }


/// 

    /// * `processor_count` -  (u16)

    /// * `return_value` -  (u32)
    pub fn calculate_possible_reserve(&self, processor_count: u16) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ProcessorCount".to_string(), value: processor_count.into() });
        self.invoke_method("CalculatePossibleReserve", &args)

    }

}

impl Msvm_ProcessorPool {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

    /// Gets the related Msvm_ResourcePoolSettingData object(s)
    pub fn get_related__resource_pool_setting_data(&self) -> Result<Msvm_ResourcePoolSettingData, WmiError> {
        self.get_related("Msvm_ResourcePoolSettingData")
    }

    /// Gets the related Msvm_ResourcePoolConfigurationService object(s)
    pub fn get_related__resource_pool_configuration_service(&self) -> Result<Msvm_ResourcePoolConfigurationService, WmiError> {
        self.get_related("Msvm_ResourcePoolConfigurationService")
    }

    /// Gets the related Msvm_AllocationCapabilities object(s)
    pub fn get_related__allocation_capabilities(&self) -> Result<Msvm_AllocationCapabilities, WmiError> {
        self.get_related("Msvm_AllocationCapabilities")
    }

    /// Gets the related Msvm_AggregationMetricDefinition object(s)
    pub fn get_related__aggregation_metric_definition(&self) -> Result<Msvm_AggregationMetricDefinition, WmiError> {
        self.get_related("Msvm_AggregationMetricDefinition")
    }

    /// Gets the related Msvm_Processor object(s)
    pub fn get_related__processor(&self) -> Result<Vec<Msvm_Processor>, WmiError> {
        self.get_all_related("Msvm_Processor")
    }

}

