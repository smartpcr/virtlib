// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// TraceListener struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TraceListener {

/// The name of the trace listener.
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// The arguments of the trace listener.
    #[serde(rename = "TraceListenerArguments")]
    pub trace_listener_arguments: Vec<TraceListenerArgument>,
}

impl TraceListener {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            name: None,
            trace_listener_arguments: Vec::new(),
        }
    }


    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of TraceListenerArguments
    pub fn set_trace_listener_arguments(&mut self, value: Vec<TraceListenerArgument>) {
        self.trace_listener_arguments = value;
    }

    /// Gets the value of TraceListenerArguments
    pub fn get_trace_listener_arguments(&self) -> &Vec<TraceListenerArgument> {
        &self.trace_listener_arguments
    }
}

