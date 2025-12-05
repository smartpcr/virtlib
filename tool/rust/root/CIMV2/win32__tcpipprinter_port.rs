// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TCPIPPrinterPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TCPIPPrinterPort {
    #[serde(flatten)]
    pub base: CIM_ServiceAccessPoint,

/// The ByteCount property, when true, causes the computer to count the number of bytes in a document before sending them to the printer and the printer to report back the number of bytes actually read.  This is used for diagnostics when one discovers that bytes are missing from the print output.
    #[serde(rename = "ByteCount")]
    pub byte_count: Option<bool>,

/// The HostAddress property indicates the address of device or print server
    #[serde(rename = "HostAddress")]
    pub host_address: Option<String>,

/// The PortNumber property indicates the number of the TCP port used by the port monitor to communitcate with the device.
    #[serde(rename = "PortNumber")]
    pub port_number: Option<u32>,

/// The Protocol property has two values: 'Raw' indicates printing directly to a device and 'Lpr' indicates printing to device or print server; LPR is a legacy protocol, which will eventually be replaced by RAW. Some printers support only LPR.
    #[serde(rename = "Protocol")]
    pub protocol: Option<TCPIPPrinterPort_Protocol>,

/// The Queue property is used with the LPR protocol to indicate the name of the print queue on the server.
    #[serde(rename = "Queue")]
    pub queue: Option<String>,

/// The SNMPCommunity property contains a security level value for the device.  For example 'public'.
    #[serde(rename = "SNMPCommunity")]
    pub snmpcommunity: Option<String>,

/// The property SNMPDevIndex indicates the SNMP index number of this device for the SNMP agent.
    #[serde(rename = "SNMPDevIndex")]
    pub snmpdev_index: Option<u32>,

/// The SNMPEnabled property, when true, indicates that this printer supports RFC1759 (Simple Network Management Protocol) and can provide rich status information from the device.
    #[serde(rename = "SNMPEnabled")]
    pub snmpenabled: Option<bool>,
}

impl Win32_TCPIPPrinterPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ServiceAccessPoint::new(),
            byte_count: None,
            host_address: None,
            port_number: None,
            protocol: None,
            queue: None,
            snmpcommunity: None,
            snmpdev_index: None,
            snmpenabled: None,
        }
    }


    /// Sets the value of ByteCount
    pub fn set_byte_count(&mut self, value: bool) {
        self.byte_count = Some(value);
    }

    /// Gets the value of ByteCount
    pub fn get_byte_count(&self) -> Option<&bool> {
        self.byte_count.as_ref()
    }

    /// Sets the value of HostAddress
    pub fn set_host_address(&mut self, value: String) {
        self.host_address = Some(value);
    }

    /// Gets the value of HostAddress
    pub fn get_host_address(&self) -> Option<&String> {
        self.host_address.as_ref()
    }

    /// Sets the value of PortNumber
    pub fn set_port_number(&mut self, value: u32) {
        self.port_number = Some(value);
    }

    /// Gets the value of PortNumber
    pub fn get_port_number(&self) -> Option<&u32> {
        self.port_number.as_ref()
    }

    /// Sets the value of Protocol
    pub fn set_protocol(&mut self, value: TCPIPPrinterPort_Protocol) {
        self.protocol = Some(value);
    }

    /// Gets the value of Protocol
    pub fn get_protocol(&self) -> Option<&TCPIPPrinterPort_Protocol> {
        self.protocol.as_ref()
    }

    /// Sets the value of Queue
    pub fn set_queue(&mut self, value: String) {
        self.queue = Some(value);
    }

    /// Gets the value of Queue
    pub fn get_queue(&self) -> Option<&String> {
        self.queue.as_ref()
    }

    /// Sets the value of SNMPCommunity
    pub fn set_snmpcommunity(&mut self, value: String) {
        self.snmpcommunity = Some(value);
    }

    /// Gets the value of SNMPCommunity
    pub fn get_snmpcommunity(&self) -> Option<&String> {
        self.snmpcommunity.as_ref()
    }

    /// Sets the value of SNMPDevIndex
    pub fn set_snmpdev_index(&mut self, value: u32) {
        self.snmpdev_index = Some(value);
    }

    /// Gets the value of SNMPDevIndex
    pub fn get_snmpdev_index(&self) -> Option<&u32> {
        self.snmpdev_index.as_ref()
    }

    /// Sets the value of SNMPEnabled
    pub fn set_snmpenabled(&mut self, value: bool) {
        self.snmpenabled = Some(value);
    }

    /// Gets the value of SNMPEnabled
    pub fn get_snmpenabled(&self) -> Option<&bool> {
        self.snmpenabled.as_ref()
    }
}

