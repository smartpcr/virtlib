// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_PrinterConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_PrinterConfiguration {

/// 
    #[serde(rename = "Collate")]
    pub collate: Option<bool>,

/// 
    #[serde(rename = "Color")]
    pub color: Option<bool>,

/// 
    #[serde(rename = "ComputerName")]
    pub computer_name: Option<String>,

/// 
    #[serde(rename = "DuplexingMode")]
    pub duplexing_mode: Option<u32>,

/// 
    #[serde(rename = "PaperSize")]
    pub paper_size: Option<u32>,

/// 
    #[serde(rename = "PrintCapabilitiesXML")]
    pub print_capabilities_xml: Option<String>,

/// 
    #[serde(rename = "PrinterName")]
    pub printer_name: Option<String>,

/// 
    #[serde(rename = "PrintTicketXML")]
    pub print_ticket_xml: Option<String>,
}

impl MSFT_PrinterConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            collate: None,
            color: None,
            computer_name: None,
            duplexing_mode: None,
            paper_size: None,
            print_capabilities_xml: None,
            printer_name: None,
            print_ticket_xml: None,
        }
    }


    /// Sets the value of Collate
    pub fn set_collate(&mut self, value: bool) {
        self.collate = Some(value);
    }

    /// Gets the value of Collate
    pub fn get_collate(&self) -> Option<&bool> {
        self.collate.as_ref()
    }

    /// Sets the value of Color
    pub fn set_color(&mut self, value: bool) {
        self.color = Some(value);
    }

    /// Gets the value of Color
    pub fn get_color(&self) -> Option<&bool> {
        self.color.as_ref()
    }

    /// Sets the value of ComputerName
    pub fn set_computer_name(&mut self, value: String) {
        self.computer_name = Some(value);
    }

    /// Gets the value of ComputerName
    pub fn get_computer_name(&self) -> Option<&String> {
        self.computer_name.as_ref()
    }

    /// Sets the value of DuplexingMode
    pub fn set_duplexing_mode(&mut self, value: u32) {
        self.duplexing_mode = Some(value);
    }

    /// Gets the value of DuplexingMode
    pub fn get_duplexing_mode(&self) -> Option<&u32> {
        self.duplexing_mode.as_ref()
    }

    /// Sets the value of PaperSize
    pub fn set_paper_size(&mut self, value: u32) {
        self.paper_size = Some(value);
    }

    /// Gets the value of PaperSize
    pub fn get_paper_size(&self) -> Option<&u32> {
        self.paper_size.as_ref()
    }

    /// Sets the value of PrintCapabilitiesXML
    pub fn set_print_capabilities_xml(&mut self, value: String) {
        self.print_capabilities_xml = Some(value);
    }

    /// Gets the value of PrintCapabilitiesXML
    pub fn get_print_capabilities_xml(&self) -> Option<&String> {
        self.print_capabilities_xml.as_ref()
    }

    /// Sets the value of PrinterName
    pub fn set_printer_name(&mut self, value: String) {
        self.printer_name = Some(value);
    }

    /// Gets the value of PrinterName
    pub fn get_printer_name(&self) -> Option<&String> {
        self.printer_name.as_ref()
    }

    /// Sets the value of PrintTicketXML
    pub fn set_print_ticket_xml(&mut self, value: String) {
        self.print_ticket_xml = Some(value);
    }

    /// Gets the value of PrintTicketXML
    pub fn get_print_ticket_xml(&self) -> Option<&String> {
        self.print_ticket_xml.as_ref()
    }

/// 

    /// * `computer_name` -  (String)
    /// * `printer_name` -  (String)

    /// * `cmdlet_output` -  (MSFT_PrinterConfiguration)
    /// * `return_value` -  (u32)
    pub fn get_by_printer_name(&self, computer_name: &String, printer_name: &String, cmdlet_output: &mut MSFT_PrinterConfiguration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerName".to_string(), value: computer_name.into() });
        args.push(MethodParameter { name: "PrinterName".to_string(), value: printer_name.into() });

        let result = self.invoke_method("GetByPrinterName", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `printer_object` -  (MSFT_Printer)

    /// * `cmdlet_output` -  (MSFT_PrinterConfiguration)
    /// * `return_value` -  (u32)
    pub fn get_by_printer_object(&self, printer_object: MSFT_Printer, cmdlet_output: &mut MSFT_PrinterConfiguration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PrinterObject".to_string(), value: printer_object.into() });

        let result = self.invoke_method("GetByPrinterObject", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `collate` -  (bool)
    /// * `color` -  (bool)
    /// * `computer_name` -  (String)
    /// * `duplexing_mode` -  (u32)
    /// * `paper_size` -  (u32)
    /// * `printer_name` -  (String)
    /// * `print_ticket_xml` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_by_printer_name(&self, collate: bool, color: bool, duplexing_mode: u32, paper_size: u32, print_ticket_xml: &String, computer_name: &String, printer_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collate".to_string(), value: collate.into() });
        args.push(MethodParameter { name: "Color".to_string(), value: color.into() });
        args.push(MethodParameter { name: "DuplexingMode".to_string(), value: duplexing_mode.into() });
        args.push(MethodParameter { name: "PaperSize".to_string(), value: paper_size.into() });
        args.push(MethodParameter { name: "PrintTicketXML".to_string(), value: print_ticket_xml.into() });
        args.push(MethodParameter { name: "ComputerName".to_string(), value: computer_name.into() });
        args.push(MethodParameter { name: "PrinterName".to_string(), value: printer_name.into() });
        self.invoke_method("SetByPrinterName", &args)

    }


/// 

    /// * `collate` -  (bool)
    /// * `color` -  (bool)
    /// * `duplexing_mode` -  (u32)
    /// * `paper_size` -  (u32)
    /// * `printer_object` -  (MSFT_Printer)
    /// * `print_ticket_xml` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_by_printer_object(&self, collate: bool, color: bool, duplexing_mode: u32, paper_size: u32, print_ticket_xml: &String, printer_object: MSFT_Printer) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collate".to_string(), value: collate.into() });
        args.push(MethodParameter { name: "Color".to_string(), value: color.into() });
        args.push(MethodParameter { name: "DuplexingMode".to_string(), value: duplexing_mode.into() });
        args.push(MethodParameter { name: "PaperSize".to_string(), value: paper_size.into() });
        args.push(MethodParameter { name: "PrintTicketXML".to_string(), value: print_ticket_xml.into() });
        args.push(MethodParameter { name: "PrinterObject".to_string(), value: printer_object.into() });
        self.invoke_method("SetByPrinterObject", &args)

    }


/// 

    /// * `input_object` -  (MSFT_PrinterConfiguration)

    /// * `return_value` -  (u32)
    pub fn set_by_print_config_object(&self, input_object: MSFT_PrinterConfiguration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });
        self.invoke_method("SetByPrintConfigObject", &args)

    }

}

