// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// AppDomainInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppDomainInfo {

/// The Id of the appdomain.
    #[serde(rename = "AppDomainId")]
    pub app_domain_id: Option<i32>,

/// Indicates if the appdomain is the default appdomain.
    #[serde(rename = "IsDefault")]
    pub is_default: Option<bool>,

/// A value that specifies whether malformed messages are logged.
    #[serde(rename = "LogMalformedMessages")]
    pub log_malformed_messages: Option<bool>,

/// A value that specifies whether messages are traced at the service level (before encryption and transport-related transforms).
    #[serde(rename = "LogMessagesAtServiceLevel")]
    pub log_messages_at_service_level: Option<bool>,

/// A value that specifies whether messages are traced at the transport level.
    #[serde(rename = "LogMessagesAtTransportLevel")]
    pub log_messages_at_transport_level: Option<bool>,

/// The collection trace listeners that listen to the System.ServiceModel.MessageLogging trace source.
    #[serde(rename = "MessageLoggingTraceListeners")]
    pub message_logging_trace_listeners: Vec<TraceListener>,

/// The name of the appdomain.
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// The scope of active performance counters in the appdomain.
    #[serde(rename = "PerformanceCounters")]
    pub performance_counters: Option<String>,

/// The process Id.
    #[serde(rename = "ProcessId")]
    pub process_id: Option<i32>,

/// The path to the configuration of the service.
    #[serde(rename = "ServiceConfigPath")]
    pub service_config_path: Option<String>,

/// The collection trace listeners that listen to the System.ServiceModel trace source.
    #[serde(rename = "ServiceModelTraceListeners")]
    pub service_model_trace_listeners: Vec<TraceListener>,

/// The trace level of the System.ServiceModel trace source.
    #[serde(rename = "TraceLevel")]
    pub trace_level: Option<String>,
}

impl AppDomainInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            app_domain_id: None,
            is_default: None,
            log_malformed_messages: None,
            log_messages_at_service_level: None,
            log_messages_at_transport_level: None,
            message_logging_trace_listeners: Vec::new(),
            name: None,
            performance_counters: None,
            process_id: None,
            service_config_path: None,
            service_model_trace_listeners: Vec::new(),
            trace_level: None,
        }
    }


    /// Sets the value of AppDomainId
    pub fn set_app_domain_id(&mut self, value: i32) {
        self.app_domain_id = Some(value);
    }

    /// Gets the value of AppDomainId
    pub fn get_app_domain_id(&self) -> Option<&i32> {
        self.app_domain_id.as_ref()
    }

    /// Sets the value of IsDefault
    pub fn set_is_default(&mut self, value: bool) {
        self.is_default = Some(value);
    }

    /// Gets the value of IsDefault
    pub fn get_is_default(&self) -> Option<&bool> {
        self.is_default.as_ref()
    }

    /// Sets the value of LogMalformedMessages
    pub fn set_log_malformed_messages(&mut self, value: bool) {
        self.log_malformed_messages = Some(value);
    }

    /// Gets the value of LogMalformedMessages
    pub fn get_log_malformed_messages(&self) -> Option<&bool> {
        self.log_malformed_messages.as_ref()
    }

    /// Sets the value of LogMessagesAtServiceLevel
    pub fn set_log_messages_at_service_level(&mut self, value: bool) {
        self.log_messages_at_service_level = Some(value);
    }

    /// Gets the value of LogMessagesAtServiceLevel
    pub fn get_log_messages_at_service_level(&self) -> Option<&bool> {
        self.log_messages_at_service_level.as_ref()
    }

    /// Sets the value of LogMessagesAtTransportLevel
    pub fn set_log_messages_at_transport_level(&mut self, value: bool) {
        self.log_messages_at_transport_level = Some(value);
    }

    /// Gets the value of LogMessagesAtTransportLevel
    pub fn get_log_messages_at_transport_level(&self) -> Option<&bool> {
        self.log_messages_at_transport_level.as_ref()
    }

    /// Sets the value of MessageLoggingTraceListeners
    pub fn set_message_logging_trace_listeners(&mut self, value: Vec<TraceListener>) {
        self.message_logging_trace_listeners = value;
    }

    /// Gets the value of MessageLoggingTraceListeners
    pub fn get_message_logging_trace_listeners(&self) -> &Vec<TraceListener> {
        &self.message_logging_trace_listeners
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of PerformanceCounters
    pub fn set_performance_counters(&mut self, value: String) {
        self.performance_counters = Some(value);
    }

    /// Gets the value of PerformanceCounters
    pub fn get_performance_counters(&self) -> Option<&String> {
        self.performance_counters.as_ref()
    }

    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: i32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&i32> {
        self.process_id.as_ref()
    }

    /// Sets the value of ServiceConfigPath
    pub fn set_service_config_path(&mut self, value: String) {
        self.service_config_path = Some(value);
    }

    /// Gets the value of ServiceConfigPath
    pub fn get_service_config_path(&self) -> Option<&String> {
        self.service_config_path.as_ref()
    }

    /// Sets the value of ServiceModelTraceListeners
    pub fn set_service_model_trace_listeners(&mut self, value: Vec<TraceListener>) {
        self.service_model_trace_listeners = value;
    }

    /// Gets the value of ServiceModelTraceListeners
    pub fn get_service_model_trace_listeners(&self) -> &Vec<TraceListener> {
        &self.service_model_trace_listeners
    }

    /// Sets the value of TraceLevel
    pub fn set_trace_level(&mut self, value: String) {
        self.trace_level = Some(value);
    }

    /// Gets the value of TraceLevel
    pub fn get_trace_level(&self) -> Option<&String> {
        self.trace_level.as_ref()
    }
}

