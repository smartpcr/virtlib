// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_TeredoServer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_TeredoServer {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "InTeredoServerErrorPacketsAuthenticationError")]
    pub in_teredo_server_error_packets_authentication_error: Option<u32>,

/// 
    #[serde(rename = "InTeredoServerErrorPacketsDestinationError")]
    pub in_teredo_server_error_packets_destination_error: Option<u32>,

/// 
    #[serde(rename = "InTeredoServerErrorPacketsHeaderError")]
    pub in_teredo_server_error_packets_header_error: Option<u32>,

/// 
    #[serde(rename = "InTeredoServerErrorPacketsSourceError")]
    pub in_teredo_server_error_packets_source_error: Option<u32>,

/// 
    #[serde(rename = "InTeredoServerErrorPacketsTotal")]
    pub in_teredo_server_error_packets_total: Option<u32>,

/// 
    #[serde(rename = "InTeredoServerSuccessPacketsBubbles")]
    pub in_teredo_server_success_packets_bubbles: Option<u32>,

/// 
    #[serde(rename = "InTeredoServerSuccessPacketsEcho")]
    pub in_teredo_server_success_packets_echo: Option<u32>,

/// 
    #[serde(rename = "InTeredoServerSuccessPacketsRSPrimary")]
    pub in_teredo_server_success_packets_rsprimary: Option<u32>,

/// 
    #[serde(rename = "InTeredoServerSuccessPacketsRSSecondary")]
    pub in_teredo_server_success_packets_rssecondary: Option<u32>,

/// 
    #[serde(rename = "InTeredoServerSuccessPacketsTotal")]
    pub in_teredo_server_success_packets_total: Option<u32>,

/// 
    #[serde(rename = "InTeredoServerTotalPacketsSuccessError")]
    pub in_teredo_server_total_packets_success_error: Option<u32>,

/// 
    #[serde(rename = "InTeredoServerTotalPacketsSuccessErrorPersec")]
    pub in_teredo_server_total_packets_success_error_persec: Option<u32>,

/// 
    #[serde(rename = "OutTeredoServerRAPrimary")]
    pub out_teredo_server_raprimary: Option<u32>,

/// 
    #[serde(rename = "OutTeredoServerRASecondary")]
    pub out_teredo_server_rasecondary: Option<u32>,
}

impl Win32_PerfRawData_Counters_TeredoServer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            in_teredo_server_error_packets_authentication_error: None,
            in_teredo_server_error_packets_destination_error: None,
            in_teredo_server_error_packets_header_error: None,
            in_teredo_server_error_packets_source_error: None,
            in_teredo_server_error_packets_total: None,
            in_teredo_server_success_packets_bubbles: None,
            in_teredo_server_success_packets_echo: None,
            in_teredo_server_success_packets_rsprimary: None,
            in_teredo_server_success_packets_rssecondary: None,
            in_teredo_server_success_packets_total: None,
            in_teredo_server_total_packets_success_error: None,
            in_teredo_server_total_packets_success_error_persec: None,
            out_teredo_server_raprimary: None,
            out_teredo_server_rasecondary: None,
        }
    }


    /// Sets the value of InTeredoServerErrorPacketsAuthenticationError
    pub fn set_in_teredo_server_error_packets_authentication_error(&mut self, value: u32) {
        self.in_teredo_server_error_packets_authentication_error = Some(value);
    }

    /// Gets the value of InTeredoServerErrorPacketsAuthenticationError
    pub fn get_in_teredo_server_error_packets_authentication_error(&self) -> Option<&u32> {
        self.in_teredo_server_error_packets_authentication_error.as_ref()
    }

    /// Sets the value of InTeredoServerErrorPacketsDestinationError
    pub fn set_in_teredo_server_error_packets_destination_error(&mut self, value: u32) {
        self.in_teredo_server_error_packets_destination_error = Some(value);
    }

    /// Gets the value of InTeredoServerErrorPacketsDestinationError
    pub fn get_in_teredo_server_error_packets_destination_error(&self) -> Option<&u32> {
        self.in_teredo_server_error_packets_destination_error.as_ref()
    }

    /// Sets the value of InTeredoServerErrorPacketsHeaderError
    pub fn set_in_teredo_server_error_packets_header_error(&mut self, value: u32) {
        self.in_teredo_server_error_packets_header_error = Some(value);
    }

    /// Gets the value of InTeredoServerErrorPacketsHeaderError
    pub fn get_in_teredo_server_error_packets_header_error(&self) -> Option<&u32> {
        self.in_teredo_server_error_packets_header_error.as_ref()
    }

    /// Sets the value of InTeredoServerErrorPacketsSourceError
    pub fn set_in_teredo_server_error_packets_source_error(&mut self, value: u32) {
        self.in_teredo_server_error_packets_source_error = Some(value);
    }

    /// Gets the value of InTeredoServerErrorPacketsSourceError
    pub fn get_in_teredo_server_error_packets_source_error(&self) -> Option<&u32> {
        self.in_teredo_server_error_packets_source_error.as_ref()
    }

    /// Sets the value of InTeredoServerErrorPacketsTotal
    pub fn set_in_teredo_server_error_packets_total(&mut self, value: u32) {
        self.in_teredo_server_error_packets_total = Some(value);
    }

    /// Gets the value of InTeredoServerErrorPacketsTotal
    pub fn get_in_teredo_server_error_packets_total(&self) -> Option<&u32> {
        self.in_teredo_server_error_packets_total.as_ref()
    }

    /// Sets the value of InTeredoServerSuccessPacketsBubbles
    pub fn set_in_teredo_server_success_packets_bubbles(&mut self, value: u32) {
        self.in_teredo_server_success_packets_bubbles = Some(value);
    }

    /// Gets the value of InTeredoServerSuccessPacketsBubbles
    pub fn get_in_teredo_server_success_packets_bubbles(&self) -> Option<&u32> {
        self.in_teredo_server_success_packets_bubbles.as_ref()
    }

    /// Sets the value of InTeredoServerSuccessPacketsEcho
    pub fn set_in_teredo_server_success_packets_echo(&mut self, value: u32) {
        self.in_teredo_server_success_packets_echo = Some(value);
    }

    /// Gets the value of InTeredoServerSuccessPacketsEcho
    pub fn get_in_teredo_server_success_packets_echo(&self) -> Option<&u32> {
        self.in_teredo_server_success_packets_echo.as_ref()
    }

    /// Sets the value of InTeredoServerSuccessPacketsRSPrimary
    pub fn set_in_teredo_server_success_packets_rsprimary(&mut self, value: u32) {
        self.in_teredo_server_success_packets_rsprimary = Some(value);
    }

    /// Gets the value of InTeredoServerSuccessPacketsRSPrimary
    pub fn get_in_teredo_server_success_packets_rsprimary(&self) -> Option<&u32> {
        self.in_teredo_server_success_packets_rsprimary.as_ref()
    }

    /// Sets the value of InTeredoServerSuccessPacketsRSSecondary
    pub fn set_in_teredo_server_success_packets_rssecondary(&mut self, value: u32) {
        self.in_teredo_server_success_packets_rssecondary = Some(value);
    }

    /// Gets the value of InTeredoServerSuccessPacketsRSSecondary
    pub fn get_in_teredo_server_success_packets_rssecondary(&self) -> Option<&u32> {
        self.in_teredo_server_success_packets_rssecondary.as_ref()
    }

    /// Sets the value of InTeredoServerSuccessPacketsTotal
    pub fn set_in_teredo_server_success_packets_total(&mut self, value: u32) {
        self.in_teredo_server_success_packets_total = Some(value);
    }

    /// Gets the value of InTeredoServerSuccessPacketsTotal
    pub fn get_in_teredo_server_success_packets_total(&self) -> Option<&u32> {
        self.in_teredo_server_success_packets_total.as_ref()
    }

    /// Sets the value of InTeredoServerTotalPacketsSuccessError
    pub fn set_in_teredo_server_total_packets_success_error(&mut self, value: u32) {
        self.in_teredo_server_total_packets_success_error = Some(value);
    }

    /// Gets the value of InTeredoServerTotalPacketsSuccessError
    pub fn get_in_teredo_server_total_packets_success_error(&self) -> Option<&u32> {
        self.in_teredo_server_total_packets_success_error.as_ref()
    }

    /// Sets the value of InTeredoServerTotalPacketsSuccessErrorPersec
    pub fn set_in_teredo_server_total_packets_success_error_persec(&mut self, value: u32) {
        self.in_teredo_server_total_packets_success_error_persec = Some(value);
    }

    /// Gets the value of InTeredoServerTotalPacketsSuccessErrorPersec
    pub fn get_in_teredo_server_total_packets_success_error_persec(&self) -> Option<&u32> {
        self.in_teredo_server_total_packets_success_error_persec.as_ref()
    }

    /// Sets the value of OutTeredoServerRAPrimary
    pub fn set_out_teredo_server_raprimary(&mut self, value: u32) {
        self.out_teredo_server_raprimary = Some(value);
    }

    /// Gets the value of OutTeredoServerRAPrimary
    pub fn get_out_teredo_server_raprimary(&self) -> Option<&u32> {
        self.out_teredo_server_raprimary.as_ref()
    }

    /// Sets the value of OutTeredoServerRASecondary
    pub fn set_out_teredo_server_rasecondary(&mut self, value: u32) {
        self.out_teredo_server_rasecondary = Some(value);
    }

    /// Gets the value of OutTeredoServerRASecondary
    pub fn get_out_teredo_server_rasecondary(&self) -> Option<&u32> {
        self.out_teredo_server_rasecondary.as_ref()
    }
}

