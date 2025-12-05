// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_LdapPerformanceCounters_LdapClient struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_LdapPerformanceCounters_LdapClient {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "BindsDigestBindsPersec")]
    pub binds_digest_binds_persec: Option<u32>,

/// 
    #[serde(rename = "BindsNegotiateBindsPersec")]
    pub binds_negotiate_binds_persec: Option<u32>,

/// 
    #[serde(rename = "BindsNTLMBindsPersec")]
    pub binds_ntlmbinds_persec: Option<u32>,

/// 
    #[serde(rename = "BindsSimpleBindsPersec")]
    pub binds_simple_binds_persec: Option<u32>,

/// 
    #[serde(rename = "BindsTotalBindsPersec")]
    pub binds_total_binds_persec: Option<u32>,

/// 
    #[serde(rename = "ConnectionsNewConnectionsPersec")]
    pub connections_new_connections_persec: Option<u32>,

/// 
    #[serde(rename = "ConnectionsNewTCPConnectionsPersec")]
    pub connections_new_tcpconnections_persec: Option<u32>,

/// 
    #[serde(rename = "ConnectionsNewTLSConnectionsPersec")]
    pub connections_new_tlsconnections_persec: Option<u32>,

/// 
    #[serde(rename = "ConnectionsNewUDPConnectionsPersec")]
    pub connections_new_udpconnections_persec: Option<u32>,

/// 
    #[serde(rename = "ConnectionsOpenConnections")]
    pub connections_open_connections: Option<u32>,

/// 
    #[serde(rename = "OperationsAbandonsPersec")]
    pub operations_abandons_persec: Option<u32>,

/// 
    #[serde(rename = "OperationsAddsPersec")]
    pub operations_adds_persec: Option<u32>,

/// 
    #[serde(rename = "OperationsDeletesPersec")]
    pub operations_deletes_persec: Option<u32>,

/// 
    #[serde(rename = "OperationsModifyPersec")]
    pub operations_modify_persec: Option<u32>,

/// 
    #[serde(rename = "RequestsNewRequestsPersec")]
    pub requests_new_requests_persec: Option<u32>,

/// 
    #[serde(rename = "RequestsRequestCount")]
    pub requests_request_count: Option<u32>,

/// 
    #[serde(rename = "ResponsesAverageResponseTime")]
    pub responses_average_response_time: Option<u32>,

/// 
    #[serde(rename = "ResponsesFailurePollingResponsesPersec")]
    pub responses_failure_polling_responses_persec: Option<u32>,

/// 
    #[serde(rename = "ResponsesFailureResponsesPersec")]
    pub responses_failure_responses_persec: Option<u32>,

/// 
    #[serde(rename = "ResponsesPendingResponses")]
    pub responses_pending_responses: Option<u32>,

/// 
    #[serde(rename = "ResponsesSuccessfulPollingResponsesPersec")]
    pub responses_successful_polling_responses_persec: Option<u32>,

/// 
    #[serde(rename = "ResponsesSuccessfulResponsesPersec")]
    pub responses_successful_responses_persec: Option<u32>,

/// 
    #[serde(rename = "SearchesBaseSearchesPersec")]
    pub searches_base_searches_persec: Option<u32>,

/// 
    #[serde(rename = "SearchesOnelevelSearchesPersec")]
    pub searches_onelevel_searches_persec: Option<u32>,

/// 
    #[serde(rename = "SearchesSubtreeSearchesPersec")]
    pub searches_subtree_searches_persec: Option<u32>,
}

impl Win32_PerfFormattedData_LdapPerformanceCounters_LdapClient {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            binds_digest_binds_persec: None,
            binds_negotiate_binds_persec: None,
            binds_ntlmbinds_persec: None,
            binds_simple_binds_persec: None,
            binds_total_binds_persec: None,
            connections_new_connections_persec: None,
            connections_new_tcpconnections_persec: None,
            connections_new_tlsconnections_persec: None,
            connections_new_udpconnections_persec: None,
            connections_open_connections: None,
            operations_abandons_persec: None,
            operations_adds_persec: None,
            operations_deletes_persec: None,
            operations_modify_persec: None,
            requests_new_requests_persec: None,
            requests_request_count: None,
            responses_average_response_time: None,
            responses_failure_polling_responses_persec: None,
            responses_failure_responses_persec: None,
            responses_pending_responses: None,
            responses_successful_polling_responses_persec: None,
            responses_successful_responses_persec: None,
            searches_base_searches_persec: None,
            searches_onelevel_searches_persec: None,
            searches_subtree_searches_persec: None,
        }
    }


    /// Sets the value of BindsDigestBindsPersec
    pub fn set_binds_digest_binds_persec(&mut self, value: u32) {
        self.binds_digest_binds_persec = Some(value);
    }

    /// Gets the value of BindsDigestBindsPersec
    pub fn get_binds_digest_binds_persec(&self) -> Option<&u32> {
        self.binds_digest_binds_persec.as_ref()
    }

    /// Sets the value of BindsNegotiateBindsPersec
    pub fn set_binds_negotiate_binds_persec(&mut self, value: u32) {
        self.binds_negotiate_binds_persec = Some(value);
    }

    /// Gets the value of BindsNegotiateBindsPersec
    pub fn get_binds_negotiate_binds_persec(&self) -> Option<&u32> {
        self.binds_negotiate_binds_persec.as_ref()
    }

    /// Sets the value of BindsNTLMBindsPersec
    pub fn set_binds_ntlmbinds_persec(&mut self, value: u32) {
        self.binds_ntlmbinds_persec = Some(value);
    }

    /// Gets the value of BindsNTLMBindsPersec
    pub fn get_binds_ntlmbinds_persec(&self) -> Option<&u32> {
        self.binds_ntlmbinds_persec.as_ref()
    }

    /// Sets the value of BindsSimpleBindsPersec
    pub fn set_binds_simple_binds_persec(&mut self, value: u32) {
        self.binds_simple_binds_persec = Some(value);
    }

    /// Gets the value of BindsSimpleBindsPersec
    pub fn get_binds_simple_binds_persec(&self) -> Option<&u32> {
        self.binds_simple_binds_persec.as_ref()
    }

    /// Sets the value of BindsTotalBindsPersec
    pub fn set_binds_total_binds_persec(&mut self, value: u32) {
        self.binds_total_binds_persec = Some(value);
    }

    /// Gets the value of BindsTotalBindsPersec
    pub fn get_binds_total_binds_persec(&self) -> Option<&u32> {
        self.binds_total_binds_persec.as_ref()
    }

    /// Sets the value of ConnectionsNewConnectionsPersec
    pub fn set_connections_new_connections_persec(&mut self, value: u32) {
        self.connections_new_connections_persec = Some(value);
    }

    /// Gets the value of ConnectionsNewConnectionsPersec
    pub fn get_connections_new_connections_persec(&self) -> Option<&u32> {
        self.connections_new_connections_persec.as_ref()
    }

    /// Sets the value of ConnectionsNewTCPConnectionsPersec
    pub fn set_connections_new_tcpconnections_persec(&mut self, value: u32) {
        self.connections_new_tcpconnections_persec = Some(value);
    }

    /// Gets the value of ConnectionsNewTCPConnectionsPersec
    pub fn get_connections_new_tcpconnections_persec(&self) -> Option<&u32> {
        self.connections_new_tcpconnections_persec.as_ref()
    }

    /// Sets the value of ConnectionsNewTLSConnectionsPersec
    pub fn set_connections_new_tlsconnections_persec(&mut self, value: u32) {
        self.connections_new_tlsconnections_persec = Some(value);
    }

    /// Gets the value of ConnectionsNewTLSConnectionsPersec
    pub fn get_connections_new_tlsconnections_persec(&self) -> Option<&u32> {
        self.connections_new_tlsconnections_persec.as_ref()
    }

    /// Sets the value of ConnectionsNewUDPConnectionsPersec
    pub fn set_connections_new_udpconnections_persec(&mut self, value: u32) {
        self.connections_new_udpconnections_persec = Some(value);
    }

    /// Gets the value of ConnectionsNewUDPConnectionsPersec
    pub fn get_connections_new_udpconnections_persec(&self) -> Option<&u32> {
        self.connections_new_udpconnections_persec.as_ref()
    }

    /// Sets the value of ConnectionsOpenConnections
    pub fn set_connections_open_connections(&mut self, value: u32) {
        self.connections_open_connections = Some(value);
    }

    /// Gets the value of ConnectionsOpenConnections
    pub fn get_connections_open_connections(&self) -> Option<&u32> {
        self.connections_open_connections.as_ref()
    }

    /// Sets the value of OperationsAbandonsPersec
    pub fn set_operations_abandons_persec(&mut self, value: u32) {
        self.operations_abandons_persec = Some(value);
    }

    /// Gets the value of OperationsAbandonsPersec
    pub fn get_operations_abandons_persec(&self) -> Option<&u32> {
        self.operations_abandons_persec.as_ref()
    }

    /// Sets the value of OperationsAddsPersec
    pub fn set_operations_adds_persec(&mut self, value: u32) {
        self.operations_adds_persec = Some(value);
    }

    /// Gets the value of OperationsAddsPersec
    pub fn get_operations_adds_persec(&self) -> Option<&u32> {
        self.operations_adds_persec.as_ref()
    }

    /// Sets the value of OperationsDeletesPersec
    pub fn set_operations_deletes_persec(&mut self, value: u32) {
        self.operations_deletes_persec = Some(value);
    }

    /// Gets the value of OperationsDeletesPersec
    pub fn get_operations_deletes_persec(&self) -> Option<&u32> {
        self.operations_deletes_persec.as_ref()
    }

    /// Sets the value of OperationsModifyPersec
    pub fn set_operations_modify_persec(&mut self, value: u32) {
        self.operations_modify_persec = Some(value);
    }

    /// Gets the value of OperationsModifyPersec
    pub fn get_operations_modify_persec(&self) -> Option<&u32> {
        self.operations_modify_persec.as_ref()
    }

    /// Sets the value of RequestsNewRequestsPersec
    pub fn set_requests_new_requests_persec(&mut self, value: u32) {
        self.requests_new_requests_persec = Some(value);
    }

    /// Gets the value of RequestsNewRequestsPersec
    pub fn get_requests_new_requests_persec(&self) -> Option<&u32> {
        self.requests_new_requests_persec.as_ref()
    }

    /// Sets the value of RequestsRequestCount
    pub fn set_requests_request_count(&mut self, value: u32) {
        self.requests_request_count = Some(value);
    }

    /// Gets the value of RequestsRequestCount
    pub fn get_requests_request_count(&self) -> Option<&u32> {
        self.requests_request_count.as_ref()
    }

    /// Sets the value of ResponsesAverageResponseTime
    pub fn set_responses_average_response_time(&mut self, value: u32) {
        self.responses_average_response_time = Some(value);
    }

    /// Gets the value of ResponsesAverageResponseTime
    pub fn get_responses_average_response_time(&self) -> Option<&u32> {
        self.responses_average_response_time.as_ref()
    }

    /// Sets the value of ResponsesFailurePollingResponsesPersec
    pub fn set_responses_failure_polling_responses_persec(&mut self, value: u32) {
        self.responses_failure_polling_responses_persec = Some(value);
    }

    /// Gets the value of ResponsesFailurePollingResponsesPersec
    pub fn get_responses_failure_polling_responses_persec(&self) -> Option<&u32> {
        self.responses_failure_polling_responses_persec.as_ref()
    }

    /// Sets the value of ResponsesFailureResponsesPersec
    pub fn set_responses_failure_responses_persec(&mut self, value: u32) {
        self.responses_failure_responses_persec = Some(value);
    }

    /// Gets the value of ResponsesFailureResponsesPersec
    pub fn get_responses_failure_responses_persec(&self) -> Option<&u32> {
        self.responses_failure_responses_persec.as_ref()
    }

    /// Sets the value of ResponsesPendingResponses
    pub fn set_responses_pending_responses(&mut self, value: u32) {
        self.responses_pending_responses = Some(value);
    }

    /// Gets the value of ResponsesPendingResponses
    pub fn get_responses_pending_responses(&self) -> Option<&u32> {
        self.responses_pending_responses.as_ref()
    }

    /// Sets the value of ResponsesSuccessfulPollingResponsesPersec
    pub fn set_responses_successful_polling_responses_persec(&mut self, value: u32) {
        self.responses_successful_polling_responses_persec = Some(value);
    }

    /// Gets the value of ResponsesSuccessfulPollingResponsesPersec
    pub fn get_responses_successful_polling_responses_persec(&self) -> Option<&u32> {
        self.responses_successful_polling_responses_persec.as_ref()
    }

    /// Sets the value of ResponsesSuccessfulResponsesPersec
    pub fn set_responses_successful_responses_persec(&mut self, value: u32) {
        self.responses_successful_responses_persec = Some(value);
    }

    /// Gets the value of ResponsesSuccessfulResponsesPersec
    pub fn get_responses_successful_responses_persec(&self) -> Option<&u32> {
        self.responses_successful_responses_persec.as_ref()
    }

    /// Sets the value of SearchesBaseSearchesPersec
    pub fn set_searches_base_searches_persec(&mut self, value: u32) {
        self.searches_base_searches_persec = Some(value);
    }

    /// Gets the value of SearchesBaseSearchesPersec
    pub fn get_searches_base_searches_persec(&self) -> Option<&u32> {
        self.searches_base_searches_persec.as_ref()
    }

    /// Sets the value of SearchesOnelevelSearchesPersec
    pub fn set_searches_onelevel_searches_persec(&mut self, value: u32) {
        self.searches_onelevel_searches_persec = Some(value);
    }

    /// Gets the value of SearchesOnelevelSearchesPersec
    pub fn get_searches_onelevel_searches_persec(&self) -> Option<&u32> {
        self.searches_onelevel_searches_persec.as_ref()
    }

    /// Sets the value of SearchesSubtreeSearchesPersec
    pub fn set_searches_subtree_searches_persec(&mut self, value: u32) {
        self.searches_subtree_searches_persec = Some(value);
    }

    /// Gets the value of SearchesSubtreeSearchesPersec
    pub fn get_searches_subtree_searches_persec(&self) -> Option<&u32> {
        self.searches_subtree_searches_persec.as_ref()
    }
}

