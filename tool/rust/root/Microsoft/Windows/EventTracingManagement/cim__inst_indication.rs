// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.EventTracingManagement
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_InstIndication struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_InstIndication {
    #[serde(flatten)]
    pub base: CIM_Indication,

/// A copy of the instance that changed to generate the Indication. SourceInstance contains the current values of the properties selected by the Indication Filter's Query. In the case of CIM_InstDeletion, the property values are copied before the instance is deleted.
    #[serde(rename = "SourceInstance")]
    pub source_instance: Option<serde_json::Value>,

/// The host name or IP address of the SourceInstance.
    #[serde(rename = "SourceInstanceHost")]
    pub source_instance_host: Option<String>,

/// The Model Path of the SourceInstance. The following format MUST be used to encode the Model Path: 
/// <NamespacePath>:<ClassName>.<Prop1>="<Value1>", 
/// <Prop2>="<Value2>", ...
    #[serde(rename = "SourceInstanceModelPath")]
    pub source_instance_model_path: Option<String>,
}

impl CIM_InstIndication {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Indication::new(),
            source_instance: None,
            source_instance_host: None,
            source_instance_model_path: None,
        }
    }


    /// Sets the value of SourceInstance
    pub fn set_source_instance(&mut self, value: serde_json::Value) {
        self.source_instance = Some(value);
    }

    /// Gets the value of SourceInstance
    pub fn get_source_instance(&self) -> Option<&serde_json::Value> {
        self.source_instance.as_ref()
    }

    /// Sets the value of SourceInstanceHost
    pub fn set_source_instance_host(&mut self, value: String) {
        self.source_instance_host = Some(value);
    }

    /// Gets the value of SourceInstanceHost
    pub fn get_source_instance_host(&self) -> Option<&String> {
        self.source_instance_host.as_ref()
    }

    /// Sets the value of SourceInstanceModelPath
    pub fn set_source_instance_model_path(&mut self, value: String) {
        self.source_instance_model_path = Some(value);
    }

    /// Gets the value of SourceInstanceModelPath
    pub fn get_source_instance_model_path(&self) -> Option<&String> {
        self.source_instance_model_path.as_ref()
    }
}

