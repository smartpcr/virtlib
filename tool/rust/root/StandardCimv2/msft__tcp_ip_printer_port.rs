// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_TcpIpPrinterPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_TcpIpPrinterPort {
    #[serde(flatten)]
    pub base: MSFT_PrinterPort,

/// 
    #[serde(rename = "LprByteCounting")]
    pub lpr_byte_counting: Option<bool>,

/// 
    #[serde(rename = "LprQueueName")]
    pub lpr_queue_name: Option<String>,

/// 
    #[serde(rename = "PortNumber")]
    pub port_number: Option<u32>,

/// 
    #[serde(rename = "PrinterHostAddress")]
    pub printer_host_address: Option<String>,

/// 
    #[serde(rename = "PrinterHostIP")]
    pub printer_host_ip: Option<String>,

/// 
    #[serde(rename = "Protocol")]
    pub protocol: Option<u32>,

/// 
    #[serde(rename = "SNMPCommunity")]
    pub snmpcommunity: Option<String>,

/// 
    #[serde(rename = "SNMPEnabled")]
    pub snmpenabled: Option<bool>,

/// 
    #[serde(rename = "SNMPIndex")]
    pub snmpindex: Option<u32>,
}

impl MSFT_TcpIpPrinterPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_PrinterPort::new(),
            lpr_byte_counting: None,
            lpr_queue_name: None,
            port_number: None,
            printer_host_address: None,
            printer_host_ip: None,
            protocol: None,
            snmpcommunity: None,
            snmpenabled: None,
            snmpindex: None,
        }
    }


    /// Sets the value of LprByteCounting
    pub fn set_lpr_byte_counting(&mut self, value: bool) {
        self.lpr_byte_counting = Some(value);
    }

    /// Gets the value of LprByteCounting
    pub fn get_lpr_byte_counting(&self) -> Option<&bool> {
        self.lpr_byte_counting.as_ref()
    }

    /// Sets the value of LprQueueName
    pub fn set_lpr_queue_name(&mut self, value: String) {
        self.lpr_queue_name = Some(value);
    }

    /// Gets the value of LprQueueName
    pub fn get_lpr_queue_name(&self) -> Option<&String> {
        self.lpr_queue_name.as_ref()
    }

    /// Sets the value of PortNumber
    pub fn set_port_number(&mut self, value: u32) {
        self.port_number = Some(value);
    }

    /// Gets the value of PortNumber
    pub fn get_port_number(&self) -> Option<&u32> {
        self.port_number.as_ref()
    }

    /// Sets the value of PrinterHostAddress
    pub fn set_printer_host_address(&mut self, value: String) {
        self.printer_host_address = Some(value);
    }

    /// Gets the value of PrinterHostAddress
    pub fn get_printer_host_address(&self) -> Option<&String> {
        self.printer_host_address.as_ref()
    }

    /// Sets the value of PrinterHostIP
    pub fn set_printer_host_ip(&mut self, value: String) {
        self.printer_host_ip = Some(value);
    }

    /// Gets the value of PrinterHostIP
    pub fn get_printer_host_ip(&self) -> Option<&String> {
        self.printer_host_ip.as_ref()
    }

    /// Sets the value of Protocol
    pub fn set_protocol(&mut self, value: u32) {
        self.protocol = Some(value);
    }

    /// Gets the value of Protocol
    pub fn get_protocol(&self) -> Option<&u32> {
        self.protocol.as_ref()
    }

    /// Sets the value of SNMPCommunity
    pub fn set_snmpcommunity(&mut self, value: String) {
        self.snmpcommunity = Some(value);
    }

    /// Gets the value of SNMPCommunity
    pub fn get_snmpcommunity(&self) -> Option<&String> {
        self.snmpcommunity.as_ref()
    }

    /// Sets the value of SNMPEnabled
    pub fn set_snmpenabled(&mut self, value: bool) {
        self.snmpenabled = Some(value);
    }

    /// Gets the value of SNMPEnabled
    pub fn get_snmpenabled(&self) -> Option<&bool> {
        self.snmpenabled.as_ref()
    }

    /// Sets the value of SNMPIndex
    pub fn set_snmpindex(&mut self, value: u32) {
        self.snmpindex = Some(value);
    }

    /// Gets the value of SNMPIndex
    pub fn get_snmpindex(&self) -> Option<&u32> {
        self.snmpindex.as_ref()
    }
}

