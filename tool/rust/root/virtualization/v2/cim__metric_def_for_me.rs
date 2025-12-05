// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_MetricDefForME struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_MetricDefForME {
    #[serde(flatten)]
    pub base: CIM_Dependency,

/// MetricCollectionEnabled indicates whether the metric defined by the referenced CIM_BaseMetricDefinition is being collected for the referenced CIM_ManagedElement. A value of 2 "Enabled" shall indicate the metric is being collected. A value of 3 "Disabled" shall indicate the metric is not being collected. When collection of a metric is re-enabled, the metric is re-initialized such that any values for a current access metric reflect data collected after the time at which collection was re-enabled.
    #[serde(rename = "MetricCollectionEnabled")]
    pub metric_collection_enabled: Option<MetricDefForME_MetricCollectionEnabled>,
}

impl CIM_MetricDefForME {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Dependency::new(),
            metric_collection_enabled: None,
        }
    }


    /// Sets the value of MetricCollectionEnabled
    pub fn set_metric_collection_enabled(&mut self, value: MetricDefForME_MetricCollectionEnabled) {
        self.metric_collection_enabled = Some(value);
    }

    /// Gets the value of MetricCollectionEnabled
    pub fn get_metric_collection_enabled(&self) -> Option<&MetricDefForME_MetricCollectionEnabled> {
        self.metric_collection_enabled.as_ref()
    }
}

