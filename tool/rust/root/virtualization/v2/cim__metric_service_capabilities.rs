// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_MetricServiceCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_MetricServiceCapabilities {
    #[serde(flatten)]
    pub base: CIM_EnabledLogicalElementCapabilities,

/// ControllableManagedElements identifies the instances of CIM_ManagedElement that can be controlled by the associated CIM_MetricService instance. Each value shall be formatted as a WBEM URI defined according to DSP0207 identifying an instance of CIM_ManagedElement If a value corresponding to an instance of CIM_ManagedElement is included in the ControllableManagedElements property, the associated instance of CIM_MetricService shall support enabling and/or disabling at least one metric defined for the CIM_ManagedElement instance.
    #[serde(rename = "ControllableManagedElements")]
    pub controllable_managed_elements: Vec<String>,

/// ControllableMetrics identifies the instances of CIM_BaseMetricDefinition that can be controlled by the associated CIM_MetricService instance. Each string value shall be formatted as a WBEM URI defined as in accordance with DSP0207 that identifies an instance of CIM_BaseMetricDefinition. An instance of CIM_BaseMetricDefinition shall not be identified by a value of the ControllableMetrics property unless it is associated through CIM_ServiceAffectsElement to the associated instance of CIM_MetricService. If a value corresponding to an instance of CIM_BaseMetricDefinition is included in the ControllableMetrics property, the associated instance of CIM_MetricService shall support enabling and/or disabling at least one metric defined by the CIM_BaseMetricDefinition instance.
    #[serde(rename = "ControllableMetrics")]
    pub controllable_metrics: Vec<String>,

/// ManagedElementControlTypes identifies the type of control supported by the associated CIM_MetricService instance for the CIM_ManagedElement identified by the value at the same array index in the ControllableManagedElements property. A value of 2 "Discrete" shall indicate that individual metrics controlled by the associated instance of CIM_MetricService may be enabled and or disabled for the instance of CIM_ManagedElement identified at the corresponding array index of ControllableManagedElements.A value of 3 "Bulk" shall indicate that all metrics controlled by the associated instance of CIM_MetricService may be enabled and or disabled for the instance of CIM_ManagedElement identified at the corresponding array index of ControllableManagedElements. A value of 4 "Both" shall indicate that all metrics controlled by the associated instance of CIM_MetricService may be enabled and or disabled with a single operation or individually for the instance of CIM_ManagedElement identified by the value at the same array index of ControllableManagedElements.
    #[serde(rename = "ManagedElementControlTypes")]
    pub managed_element_control_types: Vec<MetricServiceCapabilities_ManagedElementControlTypes>,

/// MetricControlTypes identifies the type of control supported by the associated CIM_MetricService instance for the CIM_BaseMetricDefinition identified by the value at the same array index in the ControllableMetrics property. A value of 2 "Discrete" shall indicate that individual metrics defined by the instance of CIM_BaseMetricDefinition identified at the corresponding array index of ControllableMetrics may be enabled and or disabled by the associated instance of CIM_MetricService.A value of 3 "Bulk" shall indicate that all metrics defined by the instance of CIM_BaseMetricDefinition identified by the value at the same array index of ControllableMetrics may be enabled and or disabled with a single operation. A value of 4 "Both" shall indicate that all metrics defined by the instance of CIM_BaseMetricDefinition identified by the value at the same array index of ControllableMetrics may be enabled and or disabled individually or as a single operation.
    #[serde(rename = "MetricsControlTypes")]
    pub metrics_control_types: Vec<MetricServiceCapabilities_MetricsControlTypes>,

/// Each enumeration corresponds to support for the like-named method of the MetricService.
    #[serde(rename = "SupportedMethods")]
    pub supported_methods: Vec<MetricServiceCapabilities_SupportedMethods>,
}

impl CIM_MetricServiceCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_EnabledLogicalElementCapabilities::new(),
            controllable_managed_elements: Vec::new(),
            controllable_metrics: Vec::new(),
            managed_element_control_types: Vec::new(),
            metrics_control_types: Vec::new(),
            supported_methods: Vec::new(),
        }
    }


    /// Sets the value of ControllableManagedElements
    pub fn set_controllable_managed_elements(&mut self, value: Vec<String>) {
        self.controllable_managed_elements = value;
    }

    /// Gets the value of ControllableManagedElements
    pub fn get_controllable_managed_elements(&self) -> &Vec<String> {
        &self.controllable_managed_elements
    }

    /// Sets the value of ControllableMetrics
    pub fn set_controllable_metrics(&mut self, value: Vec<String>) {
        self.controllable_metrics = value;
    }

    /// Gets the value of ControllableMetrics
    pub fn get_controllable_metrics(&self) -> &Vec<String> {
        &self.controllable_metrics
    }

    /// Sets the value of ManagedElementControlTypes
    pub fn set_managed_element_control_types(&mut self, value: Vec<MetricServiceCapabilities_ManagedElementControlTypes>) {
        self.managed_element_control_types = value;
    }

    /// Gets the value of ManagedElementControlTypes
    pub fn get_managed_element_control_types(&self) -> &Vec<MetricServiceCapabilities_ManagedElementControlTypes> {
        &self.managed_element_control_types
    }

    /// Sets the value of MetricsControlTypes
    pub fn set_metrics_control_types(&mut self, value: Vec<MetricServiceCapabilities_MetricsControlTypes>) {
        self.metrics_control_types = value;
    }

    /// Gets the value of MetricsControlTypes
    pub fn get_metrics_control_types(&self) -> &Vec<MetricServiceCapabilities_MetricsControlTypes> {
        &self.metrics_control_types
    }

    /// Sets the value of SupportedMethods
    pub fn set_supported_methods(&mut self, value: Vec<MetricServiceCapabilities_SupportedMethods>) {
        self.supported_methods = value;
    }

    /// Gets the value of SupportedMethods
    pub fn get_supported_methods(&self) -> &Vec<MetricServiceCapabilities_SupportedMethods> {
        &self.supported_methods
    }
}

