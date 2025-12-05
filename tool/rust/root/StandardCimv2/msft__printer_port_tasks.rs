// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_PrinterPortTasks struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_PrinterPortTasks {
}

impl MSFT_PrinterPortTasks {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `computer_name` -  (String)
    /// * `name` -  (String)

    /// * `return_value` -  (u32)
    pub fn add_by_local_port(&self, computer_name: &String, name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerName".to_string(), value: computer_name.into() });
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        self.invoke_method("AddByLocalPort", &args)

    }


/// 

    /// * `computer_name` -  (String)
    /// * `host_name` -  (String)
    /// * `printer_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn add_by_lpr_port(&self, computer_name: &String, host_name: &String, printer_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerName".to_string(), value: computer_name.into() });
        args.push(MethodParameter { name: "HostName".to_string(), value: host_name.into() });
        args.push(MethodParameter { name: "PrinterName".to_string(), value: printer_name.into() });
        self.invoke_method("AddByLprPort", &args)

    }


/// 

    /// * `computer_name` -  (String)
    /// * `name` -  (String)
    /// * `port_number` -  (u32)
    /// * `printer_host_address` -  (String)
    /// * `snmp` -  (u32)
    /// * `snmpcommunity` -  (String)

    /// * `return_value` -  (u32)
    pub fn add_by_tcp_port(&self, computer_name: &String, name: &String, port_number: u32, printer_host_address: &String, snmp: u32, snmpcommunity: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerName".to_string(), value: computer_name.into() });
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "PortNumber".to_string(), value: port_number.into() });
        args.push(MethodParameter { name: "PrinterHostAddress".to_string(), value: printer_host_address.into() });
        args.push(MethodParameter { name: "SNMP".to_string(), value: snmp.into() });
        args.push(MethodParameter { name: "SNMPCommunity".to_string(), value: snmpcommunity.into() });
        self.invoke_method("AddByTcpPort", &args)

    }


/// 

    /// * `computer_name` -  (String)
    /// * `lpr_byte_counting` -  (bool)
    /// * `lpr_host_address` -  (String)
    /// * `lpr_queue_name` -  (String)
    /// * `name` -  (String)
    /// * `port_number` -  (u32)
    /// * `snmp` -  (u32)
    /// * `snmpcommunity` -  (String)

    /// * `return_value` -  (u32)
    pub fn add_by_tcp_port_lpr_mode(&self, computer_name: &String, lpr_byte_counting: bool, lpr_host_address: &String, lpr_queue_name: &String, name: &String, port_number: u32, snmp: u32, snmpcommunity: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerName".to_string(), value: computer_name.into() });
        args.push(MethodParameter { name: "LprByteCounting".to_string(), value: lpr_byte_counting.into() });
        args.push(MethodParameter { name: "LprHostAddress".to_string(), value: lpr_host_address.into() });
        args.push(MethodParameter { name: "LprQueueName".to_string(), value: lpr_queue_name.into() });
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "PortNumber".to_string(), value: port_number.into() });
        args.push(MethodParameter { name: "SNMP".to_string(), value: snmp.into() });
        args.push(MethodParameter { name: "SNMPCommunity".to_string(), value: snmpcommunity.into() });
        self.invoke_method("AddByTcpPortLprMode", &args)

    }

}

