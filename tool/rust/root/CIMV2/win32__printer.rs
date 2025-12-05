// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_Printer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_Printer {
    #[serde(flatten)]
    pub base: CIM_Printer,

/// The Attributes property indicates the attributes of the Win32 printing device. These attributes are represented through a combination of flags. Attributes of the printer include:
/// Queued  - Print jobs are buffered and queued.
///  Direct  - Specifies that the document should be sent directly to the printer.  This is used if print job are not being properly queued.
/// Default - The printer is the default printer on the computer.
/// Shared - Available as a shared network resource.
/// Network - Attached to the network.
/// Hidden - Hidden from some users on the network.
/// Local - Directly connected to this computer.
/// EnableDevQ - Enable the queue on the printer if available.
/// KeepPrintedJobs - Specifies that the spooler should not delete documents after they are printed.
/// DoCompleteFirst - Start jobs that are finished spooling first.
/// WorkOffline - Queue print jobs when printer is not available.
/// EnableBIDI - Enable bi-directional printing.
/// RawOnly - Allow only raw data type jobs to be spooled.
/// Published - Indicates whether the printer is published in the network directory service.
/// 
    #[serde(rename = "Attributes")]
    pub attributes: Option<u32>,

/// The AveragePagesPerMinute property specifies the rate (average number of pages per minute) that the printer is capable of sustaining.
    #[serde(rename = "AveragePagesPerMinute")]
    pub average_pages_per_minute: Option<u32>,

/// The Comment property specifies the comment of a print queue.
/// Example: Color printer
    #[serde(rename = "Comment")]
    pub comment: Option<String>,

/// The Default property indicates whether the printer is the default printer on the computer.
    #[serde(rename = "Default")]
    pub default: Option<bool>,

/// The DefaultPriority property specifies the default priority value assigned to each print job.
    #[serde(rename = "DefaultPriority")]
    pub default_priority: Option<u32>,

/// The Direct property indicates whether the print jobs should be sent directly to the printer.  This means that no spool files are created for the print jobs.
/// 
    #[serde(rename = "Direct")]
    pub direct: Option<bool>,

/// The DoCompleteFirst property indicates whether the printer should start jobs that have finished spooling as opposed to the order of the job received.
    #[serde(rename = "DoCompleteFirst")]
    pub do_complete_first: Option<bool>,

/// The DriverName property specifies the name of the Win32 printer driver.
/// Example: Windows NT Fax Driver
    #[serde(rename = "DriverName")]
    pub driver_name: Option<String>,

/// The EnableBIDI property indicates whether the printer can print bidirectionally.
    #[serde(rename = "EnableBIDI")]
    pub enable_bidi: Option<bool>,

/// The EnableDevQueryPrint property indicates whether to hold documents in the queue, if document and printer setups do not match
    #[serde(rename = "EnableDevQueryPrint")]
    pub enable_dev_query_print: Option<bool>,

/// The ExtendedDetectedErrorState property reports standard error information.  Any additional information should be recorded in the DetecteErrorState property.
    #[serde(rename = "ExtendedDetectedErrorState")]
    pub extended_detected_error_state: Option<Printer_ExtendedDetectedErrorState>,

/// Status information for a Printer, beyond that specified in the LogicalDevice Availability property. Values include "Idle" (3) and an indication that the Device is currently printing (4).
    #[serde(rename = "ExtendedPrinterStatus")]
    pub extended_printer_status: Option<Printer_ExtendedPrinterStatus>,

/// The Hidden property indicates whether the printer is hidden from network users.
    #[serde(rename = "Hidden")]
    pub hidden: Option<bool>,

/// The KeepPrintedJobs property indicates whether the print spooler should not delete the jobs after they are completed.
    #[serde(rename = "KeepPrintedJobs")]
    pub keep_printed_jobs: Option<bool>,

/// The Local property indicates whether the printer is attached to the network.  A masquerading printer is printer that is implemented as local printers but has a port that refers to a remote machine.  From the application perspective these hybrid printers should be viewed as printer connections since that is their intended behavior.
    #[serde(rename = "Local")]
    pub local: Option<bool>,

/// The Location property specifies the physical location of the printer.
/// Example: Bldg. 38, Room 1164
    #[serde(rename = "Location")]
    pub location: Option<String>,

/// The Network property indicates whether the printer is a network printer.
    #[serde(rename = "Network")]
    pub network: Option<bool>,

/// The Parameters property specifies optional parameters for the print processor.
/// Example: Copies=2
    #[serde(rename = "Parameters")]
    pub parameters: Option<String>,

/// The PortName property identifies the ports that can be used to transmit data to the printer. If a printer is connected to more than one port, the names of each port are separated by commas. Under Windows 95, only one port can be specified. 
/// Example: LPT1:, LPT2:, LPT3:
    #[serde(rename = "PortName")]
    pub port_name: Option<String>,

/// The PrinterPaperNames property indicates the list of paper sizes supported by the printer. The printer-specified names are used to represent supported paper sizes.
/// Example: B5 (JIS).
    #[serde(rename = "PrinterPaperNames")]
    pub printer_paper_names: Vec<String>,

/// This property has been deprecated in favor of PrinterStatus, DetectedErrorState and ErrorInformation CIM properties that more clearly indicate the state and error status of the printer. The PrinterState property specifies a values indicating one of the possible states relating to this printer.
    #[serde(rename = "PrinterState")]
    pub printer_state: Option<Printer_PrinterState>,

/// The PrintJobDataType property indicates the default data type that will be used for a print job.
    #[serde(rename = "PrintJobDataType")]
    pub print_job_data_type: Option<String>,

/// The PrintProcessor property specifies the name of the print spooler that handles print jobs.
/// Example: SPOOLSS.DLL.
    #[serde(rename = "PrintProcessor")]
    pub print_processor: Option<String>,

/// The Priority property specifies the priority of the  printer. The jobs on a higher priority printer are scheduled first.
    #[serde(rename = "Priority")]
    pub priority: Option<u32>,

/// The Published property indicates whether the printer is published in the network directory service.
    #[serde(rename = "Published")]
    pub published: Option<bool>,

/// The Queued property indicates whether the printer buffers and queues print jobs.
    #[serde(rename = "Queued")]
    pub queued: Option<bool>,

/// The RawOnly property indicates whether the printer accepts only raw data to be spooled.
    #[serde(rename = "RawOnly")]
    pub raw_only: Option<bool>,

/// The SeparatorFile property specifies the name of the file used to create a separator page. This page is used to separate print jobs sent to the printer.
    #[serde(rename = "SeparatorFile")]
    pub separator_file: Option<String>,

/// The ServerName property identifies the server that controls the printer. If this string is NULL, the printer is controlled locally. 
    #[serde(rename = "ServerName")]
    pub server_name: Option<String>,

/// The Shared property indicates whether the printer is available as a shared network resource.
    #[serde(rename = "Shared")]
    pub shared: Option<bool>,

/// The ShareName property indicates the share name of the Win32 printing device.
/// Example: \\PRINTSERVER1\PRINTER2
    #[serde(rename = "ShareName")]
    pub share_name: Option<String>,

/// The SpoolEnabled property shows whether spooling is enabled for this printer. 
/// Values:TRUE or FALSE. 
/// The SpoolEnabled property has been deprecated.  There is no replacementvalue and this property is now considered obsolete.
    #[serde(rename = "SpoolEnabled")]
    pub spool_enabled: Option<bool>,

/// The StartTime property specifies the earliest time the printer can print a job (if the printer has been limited to print only at certain times). This value is expressed as time elapsed since 12:00 AM GMT (Greenwich mean time).
    #[serde(rename = "StartTime")]
    pub start_time: Option<String>,

/// The UntilTime property specifies the latest time the printer can print a job (if the printer has been limited to print only at certain times). This value is expressed as time elapsed since 12:00 AM GMT (Greenwich mean time).
    #[serde(rename = "UntilTime")]
    pub until_time: Option<String>,

/// The WorkOffline property indicates whether to queue print jobs on the computer if the printer is offline.
    #[serde(rename = "WorkOffline")]
    pub work_offline: Option<bool>,
}

impl Win32_Printer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Printer::new(),
            attributes: None,
            average_pages_per_minute: None,
            comment: None,
            default: None,
            default_priority: None,
            direct: None,
            do_complete_first: None,
            driver_name: None,
            enable_bidi: None,
            enable_dev_query_print: None,
            extended_detected_error_state: None,
            extended_printer_status: None,
            hidden: None,
            keep_printed_jobs: None,
            local: None,
            location: None,
            network: None,
            parameters: None,
            port_name: None,
            printer_paper_names: Vec::new(),
            printer_state: None,
            print_job_data_type: None,
            print_processor: None,
            priority: None,
            published: None,
            queued: None,
            raw_only: None,
            separator_file: None,
            server_name: None,
            shared: None,
            share_name: None,
            spool_enabled: None,
            start_time: None,
            until_time: None,
            work_offline: None,
        }
    }


    /// Sets the value of Attributes
    pub fn set_attributes(&mut self, value: u32) {
        self.attributes = Some(value);
    }

    /// Gets the value of Attributes
    pub fn get_attributes(&self) -> Option<&u32> {
        self.attributes.as_ref()
    }

    /// Sets the value of AveragePagesPerMinute
    pub fn set_average_pages_per_minute(&mut self, value: u32) {
        self.average_pages_per_minute = Some(value);
    }

    /// Gets the value of AveragePagesPerMinute
    pub fn get_average_pages_per_minute(&self) -> Option<&u32> {
        self.average_pages_per_minute.as_ref()
    }

    /// Sets the value of Comment
    pub fn set_comment(&mut self, value: String) {
        self.comment = Some(value);
    }

    /// Gets the value of Comment
    pub fn get_comment(&self) -> Option<&String> {
        self.comment.as_ref()
    }

    /// Sets the value of Default
    pub fn set_default(&mut self, value: bool) {
        self.default = Some(value);
    }

    /// Gets the value of Default
    pub fn get_default(&self) -> Option<&bool> {
        self.default.as_ref()
    }

    /// Sets the value of DefaultPriority
    pub fn set_default_priority(&mut self, value: u32) {
        self.default_priority = Some(value);
    }

    /// Gets the value of DefaultPriority
    pub fn get_default_priority(&self) -> Option<&u32> {
        self.default_priority.as_ref()
    }

    /// Sets the value of Direct
    pub fn set_direct(&mut self, value: bool) {
        self.direct = Some(value);
    }

    /// Gets the value of Direct
    pub fn get_direct(&self) -> Option<&bool> {
        self.direct.as_ref()
    }

    /// Sets the value of DoCompleteFirst
    pub fn set_do_complete_first(&mut self, value: bool) {
        self.do_complete_first = Some(value);
    }

    /// Gets the value of DoCompleteFirst
    pub fn get_do_complete_first(&self) -> Option<&bool> {
        self.do_complete_first.as_ref()
    }

    /// Sets the value of DriverName
    pub fn set_driver_name(&mut self, value: String) {
        self.driver_name = Some(value);
    }

    /// Gets the value of DriverName
    pub fn get_driver_name(&self) -> Option<&String> {
        self.driver_name.as_ref()
    }

    /// Sets the value of EnableBIDI
    pub fn set_enable_bidi(&mut self, value: bool) {
        self.enable_bidi = Some(value);
    }

    /// Gets the value of EnableBIDI
    pub fn get_enable_bidi(&self) -> Option<&bool> {
        self.enable_bidi.as_ref()
    }

    /// Sets the value of EnableDevQueryPrint
    pub fn set_enable_dev_query_print(&mut self, value: bool) {
        self.enable_dev_query_print = Some(value);
    }

    /// Gets the value of EnableDevQueryPrint
    pub fn get_enable_dev_query_print(&self) -> Option<&bool> {
        self.enable_dev_query_print.as_ref()
    }

    /// Sets the value of ExtendedDetectedErrorState
    pub fn set_extended_detected_error_state(&mut self, value: Printer_ExtendedDetectedErrorState) {
        self.extended_detected_error_state = Some(value);
    }

    /// Gets the value of ExtendedDetectedErrorState
    pub fn get_extended_detected_error_state(&self) -> Option<&Printer_ExtendedDetectedErrorState> {
        self.extended_detected_error_state.as_ref()
    }

    /// Sets the value of ExtendedPrinterStatus
    pub fn set_extended_printer_status(&mut self, value: Printer_ExtendedPrinterStatus) {
        self.extended_printer_status = Some(value);
    }

    /// Gets the value of ExtendedPrinterStatus
    pub fn get_extended_printer_status(&self) -> Option<&Printer_ExtendedPrinterStatus> {
        self.extended_printer_status.as_ref()
    }

    /// Sets the value of Hidden
    pub fn set_hidden(&mut self, value: bool) {
        self.hidden = Some(value);
    }

    /// Gets the value of Hidden
    pub fn get_hidden(&self) -> Option<&bool> {
        self.hidden.as_ref()
    }

    /// Sets the value of KeepPrintedJobs
    pub fn set_keep_printed_jobs(&mut self, value: bool) {
        self.keep_printed_jobs = Some(value);
    }

    /// Gets the value of KeepPrintedJobs
    pub fn get_keep_printed_jobs(&self) -> Option<&bool> {
        self.keep_printed_jobs.as_ref()
    }

    /// Sets the value of Local
    pub fn set_local(&mut self, value: bool) {
        self.local = Some(value);
    }

    /// Gets the value of Local
    pub fn get_local(&self) -> Option<&bool> {
        self.local.as_ref()
    }

    /// Sets the value of Location
    pub fn set_location(&mut self, value: String) {
        self.location = Some(value);
    }

    /// Gets the value of Location
    pub fn get_location(&self) -> Option<&String> {
        self.location.as_ref()
    }

    /// Sets the value of Network
    pub fn set_network(&mut self, value: bool) {
        self.network = Some(value);
    }

    /// Gets the value of Network
    pub fn get_network(&self) -> Option<&bool> {
        self.network.as_ref()
    }

    /// Sets the value of Parameters
    pub fn set_parameters(&mut self, value: String) {
        self.parameters = Some(value);
    }

    /// Gets the value of Parameters
    pub fn get_parameters(&self) -> Option<&String> {
        self.parameters.as_ref()
    }

    /// Sets the value of PortName
    pub fn set_port_name(&mut self, value: String) {
        self.port_name = Some(value);
    }

    /// Gets the value of PortName
    pub fn get_port_name(&self) -> Option<&String> {
        self.port_name.as_ref()
    }

    /// Sets the value of PrinterPaperNames
    pub fn set_printer_paper_names(&mut self, value: Vec<String>) {
        self.printer_paper_names = value;
    }

    /// Gets the value of PrinterPaperNames
    pub fn get_printer_paper_names(&self) -> &Vec<String> {
        &self.printer_paper_names
    }

    /// Sets the value of PrinterState
    pub fn set_printer_state(&mut self, value: Printer_PrinterState) {
        self.printer_state = Some(value);
    }

    /// Gets the value of PrinterState
    pub fn get_printer_state(&self) -> Option<&Printer_PrinterState> {
        self.printer_state.as_ref()
    }

    /// Sets the value of PrintJobDataType
    pub fn set_print_job_data_type(&mut self, value: String) {
        self.print_job_data_type = Some(value);
    }

    /// Gets the value of PrintJobDataType
    pub fn get_print_job_data_type(&self) -> Option<&String> {
        self.print_job_data_type.as_ref()
    }

    /// Sets the value of PrintProcessor
    pub fn set_print_processor(&mut self, value: String) {
        self.print_processor = Some(value);
    }

    /// Gets the value of PrintProcessor
    pub fn get_print_processor(&self) -> Option<&String> {
        self.print_processor.as_ref()
    }

    /// Sets the value of Priority
    pub fn set_priority(&mut self, value: u32) {
        self.priority = Some(value);
    }

    /// Gets the value of Priority
    pub fn get_priority(&self) -> Option<&u32> {
        self.priority.as_ref()
    }

    /// Sets the value of Published
    pub fn set_published(&mut self, value: bool) {
        self.published = Some(value);
    }

    /// Gets the value of Published
    pub fn get_published(&self) -> Option<&bool> {
        self.published.as_ref()
    }

    /// Sets the value of Queued
    pub fn set_queued(&mut self, value: bool) {
        self.queued = Some(value);
    }

    /// Gets the value of Queued
    pub fn get_queued(&self) -> Option<&bool> {
        self.queued.as_ref()
    }

    /// Sets the value of RawOnly
    pub fn set_raw_only(&mut self, value: bool) {
        self.raw_only = Some(value);
    }

    /// Gets the value of RawOnly
    pub fn get_raw_only(&self) -> Option<&bool> {
        self.raw_only.as_ref()
    }

    /// Sets the value of SeparatorFile
    pub fn set_separator_file(&mut self, value: String) {
        self.separator_file = Some(value);
    }

    /// Gets the value of SeparatorFile
    pub fn get_separator_file(&self) -> Option<&String> {
        self.separator_file.as_ref()
    }

    /// Sets the value of ServerName
    pub fn set_server_name(&mut self, value: String) {
        self.server_name = Some(value);
    }

    /// Gets the value of ServerName
    pub fn get_server_name(&self) -> Option<&String> {
        self.server_name.as_ref()
    }

    /// Sets the value of Shared
    pub fn set_shared(&mut self, value: bool) {
        self.shared = Some(value);
    }

    /// Gets the value of Shared
    pub fn get_shared(&self) -> Option<&bool> {
        self.shared.as_ref()
    }

    /// Sets the value of ShareName
    pub fn set_share_name(&mut self, value: String) {
        self.share_name = Some(value);
    }

    /// Gets the value of ShareName
    pub fn get_share_name(&self) -> Option<&String> {
        self.share_name.as_ref()
    }

    /// Sets the value of SpoolEnabled
    pub fn set_spool_enabled(&mut self, value: bool) {
        self.spool_enabled = Some(value);
    }

    /// Gets the value of SpoolEnabled
    pub fn get_spool_enabled(&self) -> Option<&bool> {
        self.spool_enabled.as_ref()
    }

    /// Sets the value of StartTime
    pub fn set_start_time(&mut self, value: String) {
        self.start_time = Some(value);
    }

    /// Gets the value of StartTime
    pub fn get_start_time(&self) -> Option<&String> {
        self.start_time.as_ref()
    }

    /// Sets the value of UntilTime
    pub fn set_until_time(&mut self, value: String) {
        self.until_time = Some(value);
    }

    /// Gets the value of UntilTime
    pub fn get_until_time(&self) -> Option<&String> {
        self.until_time.as_ref()
    }

    /// Sets the value of WorkOffline
    pub fn set_work_offline(&mut self, value: bool) {
        self.work_offline = Some(value);
    }

    /// Gets the value of WorkOffline
    pub fn get_work_offline(&self) -> Option<&bool> {
        self.work_offline.as_ref()
    }

/// The Pause method pauses the print queue. No jobs can print anymore until the print queue is resumed. The method can return the following values:
/// 0 - Success.
/// 5 - Access denied.
/// Other - For integer values other than those listed above, refer to the documentation on the Win32 error codes.

    /// * `return_value` -  (u32)
    pub fn pause(&self) -> Result<(), WmiError> {
        self.invoke_method("Pause", &[])

    }


/// The Resume method resumes a paused print queue. The method can return the following values:
/// 0 - Success.
/// 5 - Access denied.
/// Other - For integer values other than those listed above, refer to the documentation on the Win32 error codes.

    /// * `return_value` -  (u32)
    pub fn resume(&self) -> Result<(), WmiError> {
        self.invoke_method("Resume", &[])

    }


/// The CancelAllJobs method cancels and removes all print jobs from the printer queue including the job currently printing. The method can return the following values:
/// 0 - Success.
/// 5 - Access denied.
/// Other - For integer values other than those listed above, refer to the documentation on the Win32 error codes.

    /// * `return_value` -  (u32)
    pub fn cancel_all_jobs(&self) -> Result<(), WmiError> {
        self.invoke_method("CancelAllJobs", &[])

    }


/// The AddPrinterConnection method provides a connection to an existing printer on the network and adds it to the list of available printers on the computer system. If successful, applications will be able to use this printer for print jobs.  If unsuccessful the printer is not installed. The method can return the following values:
/// 0 - Success.
/// 5 - Access denied.
/// 1801 - Invalid printer name.
/// 1930 - Incompatible printer driver.
/// Other - For integer values other than those listed above, refer to the documentation on the Win32 error codes.

    /// * `name` - The Name parameter specifies a friendly name for the printer.  This may be overridden if the name has alreadybeen set by the printer. (String)

    /// * `return_value` -  (u32)
    pub fn add_printer_connection(&self, name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        self.invoke_method("AddPrinterConnection", &args)

    }


/// The RenamePrinter method renames a printer. The method can return the following values:
/// 0 - Success.
/// 5 - Access denied.
/// 1801 - Invalid printer name.
/// Other - For integer values other than those listed above, refer to the documentation on the Win32 error codes.

    /// * `new_printer_name` - The NewPrinterName parameter specifies the new printer name. (String)

    /// * `return_value` -  (u32)
    pub fn rename_printer(&self, new_printer_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewPrinterName".to_string(), value: new_printer_name.into() });
        self.invoke_method("RenamePrinter", &args)

    }


/// The PrintTestPage method prints a test page. The method can return the following values:
/// 0 - Success.
/// 5 - Access denied.
/// Other - For integer values other than those listed above, refer to the documentation on the Win32 error codes.

    /// * `return_value` -  (u32)
    pub fn print_test_page(&self) -> Result<(), WmiError> {
        self.invoke_method("PrintTestPage", &[])

    }


/// The SetDefaultPrinter method sets the printer to be the default printer for the user who executes the method. The method can return the following values:
/// 0 - Success.
/// Other - For integer values other than those listed above, refer to the documentation on the Win32 error codes.

    /// * `return_value` -  (u32)
    pub fn set_default_printer(&self) -> Result<(), WmiError> {
        self.invoke_method("SetDefaultPrinter", &[])

    }


/// Retrieves a structural representation of the printer's security descriptor.
/// The method returns an integer value that can be interpreted as follows: 
/// 0 - Successful completion.
/// 2 - The user does not have access to the requested information.
/// 8 - Unknown failure.
/// 9 - The user does not have adequate privileges.
/// 21 - The specified parameter is invalid.
/// Other - For integer values other than those listed above, refer to Win32 error code documentation.

    /// * `descriptor` -  (Win32_SecurityDescriptor)
    /// * `return_value` -  (u32)
    pub fn get_security_descriptor(&self, descriptor: &mut Win32_SecurityDescriptor) -> Result<(), WmiError> {

        let result = self.invoke_method("GetSecurityDescriptor", &[])?;
        let descriptor = result.get_value("Descriptor")?;
        Ok(result.return_value)

    }


/// Sets security descriptor on the printer to the specified structure. 
/// The method returns an integer value that can be interpreted as follows: 
/// 0 - Successful completion.
/// 2 - The user does not have access to the requested information.
/// 8 - Unknown failure.
/// 9 - The user does not have adequate privileges.
/// 21 - The specified parameter is invalid.
/// Other - For integer values other than those listed above, refer to Win32 error code documentation.

    /// * `descriptor` -  (Win32_SecurityDescriptor)

    /// * `return_value` -  (u32)
    pub fn set_security_descriptor(&self, descriptor: Win32_SecurityDescriptor) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Descriptor".to_string(), value: descriptor.into() });
        self.invoke_method("SetSecurityDescriptor", &args)

    }

}

