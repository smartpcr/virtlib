// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_MetricServiceSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_MetricServiceSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "MetricsFlushInterval")]
    pub metrics_flush_interval: Option<String>,
}

impl Msvm_MetricServiceSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            metrics_flush_interval: None,
        }
    }


    /// Sets the value of MetricsFlushInterval
    pub fn set_metrics_flush_interval(&mut self, value: String) {
        self.metrics_flush_interval = Some(value);
    }

    /// Gets the value of MetricsFlushInterval
    pub fn get_metrics_flush_interval(&self) -> Option<&String> {
        self.metrics_flush_interval.as_ref()
    }
}

impl Msvm_MetricServiceSettingData {
    /// Gets the related Msvm_MetricService object(s)
    pub fn get_related__metric_service(&self) -> Result<Msvm_MetricService, WmiError> {
        self.get_related("Msvm_MetricService")
    }

}

