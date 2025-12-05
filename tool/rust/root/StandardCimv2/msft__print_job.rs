// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_PrintJob struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_PrintJob {
    #[serde(flatten)]
    pub base: CIM_ManagedSystemElement,

/// 
    #[serde(rename = "ComputerName")]
    pub computer_name: Option<String>,

/// 
    #[serde(rename = "Datatype")]
    pub datatype: Option<String>,

/// 
    #[serde(rename = "DocumentName")]
    pub document_name: Option<String>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<u32>,

/// 
    #[serde(rename = "JobStatus")]
    pub job_status: Option<u32>,

/// 
    #[serde(rename = "JobTime")]
    pub job_time: Option<u32>,

/// 
    #[serde(rename = "PagesPrinted")]
    pub pages_printed: Option<u32>,

/// 
    #[serde(rename = "Position")]
    pub position: Option<u32>,

/// 
    #[serde(rename = "PrinterName")]
    pub printer_name: Option<String>,

/// 
    #[serde(rename = "Priority")]
    pub priority: Option<u32>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u32>,

/// 
    #[serde(rename = "SubmittedTime")]
    pub submitted_time: Option<String>,

/// 
    #[serde(rename = "TotalPages")]
    pub total_pages: Option<u32>,

/// 
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,
}

impl MSFT_PrintJob {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedSystemElement::new(),
            computer_name: None,
            datatype: None,
            document_name: None,
            id: None,
            job_status: None,
            job_time: None,
            pages_printed: None,
            position: None,
            printer_name: None,
            priority: None,
            size: None,
            submitted_time: None,
            total_pages: None,
            user_name: None,
        }
    }


    /// Sets the value of ComputerName
    pub fn set_computer_name(&mut self, value: String) {
        self.computer_name = Some(value);
    }

    /// Gets the value of ComputerName
    pub fn get_computer_name(&self) -> Option<&String> {
        self.computer_name.as_ref()
    }

    /// Sets the value of Datatype
    pub fn set_datatype(&mut self, value: String) {
        self.datatype = Some(value);
    }

    /// Gets the value of Datatype
    pub fn get_datatype(&self) -> Option<&String> {
        self.datatype.as_ref()
    }

    /// Sets the value of DocumentName
    pub fn set_document_name(&mut self, value: String) {
        self.document_name = Some(value);
    }

    /// Gets the value of DocumentName
    pub fn get_document_name(&self) -> Option<&String> {
        self.document_name.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: u32) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&u32> {
        self.id.as_ref()
    }

    /// Sets the value of JobStatus
    pub fn set_job_status(&mut self, value: u32) {
        self.job_status = Some(value);
    }

    /// Gets the value of JobStatus
    pub fn get_job_status(&self) -> Option<&u32> {
        self.job_status.as_ref()
    }

    /// Sets the value of JobTime
    pub fn set_job_time(&mut self, value: u32) {
        self.job_time = Some(value);
    }

    /// Gets the value of JobTime
    pub fn get_job_time(&self) -> Option<&u32> {
        self.job_time.as_ref()
    }

    /// Sets the value of PagesPrinted
    pub fn set_pages_printed(&mut self, value: u32) {
        self.pages_printed = Some(value);
    }

    /// Gets the value of PagesPrinted
    pub fn get_pages_printed(&self) -> Option<&u32> {
        self.pages_printed.as_ref()
    }

    /// Sets the value of Position
    pub fn set_position(&mut self, value: u32) {
        self.position = Some(value);
    }

    /// Gets the value of Position
    pub fn get_position(&self) -> Option<&u32> {
        self.position.as_ref()
    }

    /// Sets the value of PrinterName
    pub fn set_printer_name(&mut self, value: String) {
        self.printer_name = Some(value);
    }

    /// Gets the value of PrinterName
    pub fn get_printer_name(&self) -> Option<&String> {
        self.printer_name.as_ref()
    }

    /// Sets the value of Priority
    pub fn set_priority(&mut self, value: u32) {
        self.priority = Some(value);
    }

    /// Gets the value of Priority
    pub fn get_priority(&self) -> Option<&u32> {
        self.priority.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u32) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u32> {
        self.size.as_ref()
    }

    /// Sets the value of SubmittedTime
    pub fn set_submitted_time(&mut self, value: String) {
        self.submitted_time = Some(value);
    }

    /// Gets the value of SubmittedTime
    pub fn get_submitted_time(&self) -> Option<&String> {
        self.submitted_time.as_ref()
    }

    /// Sets the value of TotalPages
    pub fn set_total_pages(&mut self, value: u32) {
        self.total_pages = Some(value);
    }

    /// Gets the value of TotalPages
    pub fn get_total_pages(&self) -> Option<&u32> {
        self.total_pages.as_ref()
    }

    /// Sets the value of UserName
    pub fn set_user_name(&mut self, value: String) {
        self.user_name = Some(value);
    }

    /// Gets the value of UserName
    pub fn get_user_name(&self) -> Option<&String> {
        self.user_name.as_ref()
    }

/// 

    /// * `computer_name` -  (String)
    /// * `id` -  (u32)
    /// * `printer_name` -  (String)

    /// * `cmdlet_output` -  (MSFT_PrintJob[])
    /// * `return_value` -  (u32)
    pub fn get_by_name(&self, computer_name: &String, id: u32, printer_name: &String, cmdlet_output: &mut Vec<MSFT_PrintJob>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerName".to_string(), value: computer_name.into() });
        args.push(MethodParameter { name: "ID".to_string(), value: id.into() });
        args.push(MethodParameter { name: "PrinterName".to_string(), value: printer_name.into() });

        let result = self.invoke_method("GetByName", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `id` -  (u32)
    /// * `printer_object` -  (MSFT_Printer)

    /// * `cmdlet_output` -  (MSFT_PrintJob[])
    /// * `return_value` -  (u32)
    pub fn get_by_object(&self, id: u32, printer_object: MSFT_Printer, cmdlet_output: &mut Vec<MSFT_PrintJob>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ID".to_string(), value: id.into() });
        args.push(MethodParameter { name: "PrinterObject".to_string(), value: printer_object.into() });

        let result = self.invoke_method("GetByObject", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `input_object` -  (MSFT_PrintJob)

    /// * `return_value` -  (u32)
    pub fn delete_job_by_object(&self, input_object: MSFT_PrintJob) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });
        self.invoke_method("DeleteJobByObject", &args)

    }


/// 

    /// * `computer_name` -  (String)
    /// * `id` -  (u32)
    /// * `printer_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn delete_job_by_id(&self, computer_name: &String, id: u32, printer_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerName".to_string(), value: computer_name.into() });
        args.push(MethodParameter { name: "ID".to_string(), value: id.into() });
        args.push(MethodParameter { name: "PrinterName".to_string(), value: printer_name.into() });
        self.invoke_method("DeleteJobById", &args)

    }


/// 

    /// * `id` -  (u32)
    /// * `printer_object` -  (MSFT_Printer)

    /// * `return_value` -  (u32)
    pub fn delete_job_by_printer_object(&self, id: u32, printer_object: MSFT_Printer) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ID".to_string(), value: id.into() });
        args.push(MethodParameter { name: "PrinterObject".to_string(), value: printer_object.into() });
        self.invoke_method("DeleteJobByPrinterObject", &args)

    }


/// 

    /// * `input_object` -  (MSFT_PrintJob)

    /// * `return_value` -  (u32)
    pub fn restart_job_by_object(&self, input_object: MSFT_PrintJob) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });
        self.invoke_method("RestartJobByObject", &args)

    }


/// 

    /// * `computer_name` -  (String)
    /// * `id` -  (u32)
    /// * `printer_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn restart_job_by_id(&self, computer_name: &String, id: u32, printer_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerName".to_string(), value: computer_name.into() });
        args.push(MethodParameter { name: "ID".to_string(), value: id.into() });
        args.push(MethodParameter { name: "PrinterName".to_string(), value: printer_name.into() });
        self.invoke_method("RestartJobById", &args)

    }


/// 

    /// * `id` -  (u32)
    /// * `printer_object` -  (MSFT_Printer)

    /// * `return_value` -  (u32)
    pub fn restart_job_by_printer_object(&self, id: u32, printer_object: MSFT_Printer) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ID".to_string(), value: id.into() });
        args.push(MethodParameter { name: "PrinterObject".to_string(), value: printer_object.into() });
        self.invoke_method("RestartJobByPrinterObject", &args)

    }


/// 

    /// * `input_object` -  (MSFT_PrintJob)

    /// * `return_value` -  (u32)
    pub fn resume_job_by_object(&self, input_object: MSFT_PrintJob) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });
        self.invoke_method("ResumeJobByObject", &args)

    }


/// 

    /// * `computer_name` -  (String)
    /// * `id` -  (u32)
    /// * `printer_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn resume_job_by_id(&self, computer_name: &String, id: u32, printer_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerName".to_string(), value: computer_name.into() });
        args.push(MethodParameter { name: "ID".to_string(), value: id.into() });
        args.push(MethodParameter { name: "PrinterName".to_string(), value: printer_name.into() });
        self.invoke_method("ResumeJobById", &args)

    }


/// 

    /// * `id` -  (u32)
    /// * `printer_object` -  (MSFT_Printer)

    /// * `return_value` -  (u32)
    pub fn resume_job_by_printer_object(&self, id: u32, printer_object: MSFT_Printer) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ID".to_string(), value: id.into() });
        args.push(MethodParameter { name: "PrinterObject".to_string(), value: printer_object.into() });
        self.invoke_method("ResumeJobByPrinterObject", &args)

    }


/// 

    /// * `input_object` -  (MSFT_PrintJob)

    /// * `return_value` -  (u32)
    pub fn suspend_job_by_object(&self, input_object: MSFT_PrintJob) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });
        self.invoke_method("SuspendJobByObject", &args)

    }


/// 

    /// * `computer_name` -  (String)
    /// * `id` -  (u32)
    /// * `printer_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn suspend_job_by_id(&self, computer_name: &String, id: u32, printer_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerName".to_string(), value: computer_name.into() });
        args.push(MethodParameter { name: "ID".to_string(), value: id.into() });
        args.push(MethodParameter { name: "PrinterName".to_string(), value: printer_name.into() });
        self.invoke_method("SuspendJobById", &args)

    }


/// 

    /// * `id` -  (u32)
    /// * `printer_object` -  (MSFT_Printer)

    /// * `return_value` -  (u32)
    pub fn suspend_job_by_printer_object(&self, id: u32, printer_object: MSFT_Printer) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ID".to_string(), value: id.into() });
        args.push(MethodParameter { name: "PrinterObject".to_string(), value: printer_object.into() });
        self.invoke_method("SuspendJobByPrinterObject", &args)

    }

}

