// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PrintJob struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PrintJob {
    #[serde(flatten)]
    pub base: CIM_Job,

/// The Color property indicates whether the document is to be printed in color or monochrome.  Some color printers have the capability to print using true black instead of a combination of Yellow, Cyan, and Magenta.  This usually creates darker and sharper text for documents.  This option is only useful for color printers that support true black printing.
    #[serde(rename = "Color")]
    pub color: Option<String>,

/// The DataType property indicates the format of the data for this print job. This instructs the printer driver to eithertranslate the data (generic text, PostScript, or PCL) before printing, or to print in a raw format (for graphics and pictures).
/// Example: TEXT
    #[serde(rename = "DataType")]
    pub data_type: Option<String>,

/// The Document property specifies the name of the print job. The user sees this name when viewing documents waiting to be printed. 
/// Example: Microsoft Word - Review.doc
    #[serde(rename = "Document")]
    pub document: Option<String>,

/// The DriverName property indicates the name of the printer driver used for the print job.
    #[serde(rename = "DriverName")]
    pub driver_name: Option<String>,

/// The HostPrintQueue property contains the name of the computer on which the print job was created.
    #[serde(rename = "HostPrintQueue")]
    pub host_print_queue: Option<String>,

/// The JobId property indicates the identifier number of the job. It is used by other methods as a handle to a single job spooling to the printer.
    #[serde(rename = "JobId")]
    pub job_id: Option<u32>,

/// The PagesPrinted property specifies the number of pages that have been printed. This value may be zero if the print job does not contain page delimiting information.
    #[serde(rename = "PagesPrinted")]
    pub pages_printed: Option<u32>,

/// The PaperLength property indicates the length of the paper.
/// Example: 2794
    #[serde(rename = "PaperLength")]
    pub paper_length: Option<u32>,

/// The PaperSize property indicates the size of the paper.
/// Example: A4 or Letter
    #[serde(rename = "PaperSize")]
    pub paper_size: Option<String>,

/// The PaperWidth property indicates the width of the paper.
/// Example: 2159
    #[serde(rename = "PaperWidth")]
    pub paper_width: Option<u32>,

/// The Parameters property indicates optional parameters to send to the print processor. See the PrintProcessor member for more information.
    #[serde(rename = "Parameters")]
    pub parameters: Option<String>,

/// The PrintProcessor property indicates the print processor service used to process the print job. A printer processor works in conjunction with the printer driver to provide additional translation of printer data for the printer, and can also be used to provide special options such as a title page for the job.
    #[serde(rename = "PrintProcessor")]
    pub print_processor: Option<String>,

/// The Size property indicates the size of the print job.
    #[serde(rename = "Size")]
    pub size: Option<u32>,

/// The SizeHigh property indicates the size of the print job if the Size property exceeds 4,294,967,295 bytes.
    #[serde(rename = "SizeHigh")]
    pub size_high: Option<u32>,

/// The StatusMask property specifies a bitmap of the possible statuses relating to this print job.
    #[serde(rename = "StatusMask")]
    pub status_mask: Option<u32>,

/// The TotalPages property specifies the number of pages required to complete the job. This value may be zero if the print job does not contain page-delimiting information.
    #[serde(rename = "TotalPages")]
    pub total_pages: Option<u32>,
}

impl Win32_PrintJob {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Job::new(),
            color: None,
            data_type: None,
            document: None,
            driver_name: None,
            host_print_queue: None,
            job_id: None,
            pages_printed: None,
            paper_length: None,
            paper_size: None,
            paper_width: None,
            parameters: None,
            print_processor: None,
            size: None,
            size_high: None,
            status_mask: None,
            total_pages: None,
        }
    }


    /// Sets the value of Color
    pub fn set_color(&mut self, value: String) {
        self.color = Some(value);
    }

    /// Gets the value of Color
    pub fn get_color(&self) -> Option<&String> {
        self.color.as_ref()
    }

    /// Sets the value of DataType
    pub fn set_data_type(&mut self, value: String) {
        self.data_type = Some(value);
    }

    /// Gets the value of DataType
    pub fn get_data_type(&self) -> Option<&String> {
        self.data_type.as_ref()
    }

    /// Sets the value of Document
    pub fn set_document(&mut self, value: String) {
        self.document = Some(value);
    }

    /// Gets the value of Document
    pub fn get_document(&self) -> Option<&String> {
        self.document.as_ref()
    }

    /// Sets the value of DriverName
    pub fn set_driver_name(&mut self, value: String) {
        self.driver_name = Some(value);
    }

    /// Gets the value of DriverName
    pub fn get_driver_name(&self) -> Option<&String> {
        self.driver_name.as_ref()
    }

    /// Sets the value of HostPrintQueue
    pub fn set_host_print_queue(&mut self, value: String) {
        self.host_print_queue = Some(value);
    }

    /// Gets the value of HostPrintQueue
    pub fn get_host_print_queue(&self) -> Option<&String> {
        self.host_print_queue.as_ref()
    }

    /// Sets the value of JobId
    pub fn set_job_id(&mut self, value: u32) {
        self.job_id = Some(value);
    }

    /// Gets the value of JobId
    pub fn get_job_id(&self) -> Option<&u32> {
        self.job_id.as_ref()
    }

    /// Sets the value of PagesPrinted
    pub fn set_pages_printed(&mut self, value: u32) {
        self.pages_printed = Some(value);
    }

    /// Gets the value of PagesPrinted
    pub fn get_pages_printed(&self) -> Option<&u32> {
        self.pages_printed.as_ref()
    }

    /// Sets the value of PaperLength
    pub fn set_paper_length(&mut self, value: u32) {
        self.paper_length = Some(value);
    }

    /// Gets the value of PaperLength
    pub fn get_paper_length(&self) -> Option<&u32> {
        self.paper_length.as_ref()
    }

    /// Sets the value of PaperSize
    pub fn set_paper_size(&mut self, value: String) {
        self.paper_size = Some(value);
    }

    /// Gets the value of PaperSize
    pub fn get_paper_size(&self) -> Option<&String> {
        self.paper_size.as_ref()
    }

    /// Sets the value of PaperWidth
    pub fn set_paper_width(&mut self, value: u32) {
        self.paper_width = Some(value);
    }

    /// Gets the value of PaperWidth
    pub fn get_paper_width(&self) -> Option<&u32> {
        self.paper_width.as_ref()
    }

    /// Sets the value of Parameters
    pub fn set_parameters(&mut self, value: String) {
        self.parameters = Some(value);
    }

    /// Gets the value of Parameters
    pub fn get_parameters(&self) -> Option<&String> {
        self.parameters.as_ref()
    }

    /// Sets the value of PrintProcessor
    pub fn set_print_processor(&mut self, value: String) {
        self.print_processor = Some(value);
    }

    /// Gets the value of PrintProcessor
    pub fn get_print_processor(&self) -> Option<&String> {
        self.print_processor.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u32) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u32> {
        self.size.as_ref()
    }

    /// Sets the value of SizeHigh
    pub fn set_size_high(&mut self, value: u32) {
        self.size_high = Some(value);
    }

    /// Gets the value of SizeHigh
    pub fn get_size_high(&self) -> Option<&u32> {
        self.size_high.as_ref()
    }

    /// Sets the value of StatusMask
    pub fn set_status_mask(&mut self, value: u32) {
        self.status_mask = Some(value);
    }

    /// Gets the value of StatusMask
    pub fn get_status_mask(&self) -> Option<&u32> {
        self.status_mask.as_ref()
    }

    /// Sets the value of TotalPages
    pub fn set_total_pages(&mut self, value: u32) {
        self.total_pages = Some(value);
    }

    /// Gets the value of TotalPages
    pub fn get_total_pages(&self) -> Option<&u32> {
        self.total_pages.as_ref()
    }

/// The Pause method pauses a job in a print queue. If the job was currently printing, no other job will be printed. If the job wasn't printing yet, another unpaused print job may begin printing. The method can return the following values:
/// 0 - Success.
/// 5 - Access denied.
/// Other - For integer values other than those listed above, refer to the documentation on the Win32 error codes.

    /// * `return_value` -  (u32)
    pub fn pause(&self) -> Result<(), WmiError> {
        self.invoke_method("Pause", &[])

    }


/// The Resume method continues a paused print job. The method can return the following values:
/// 0 - Success.
/// 5 - Access denied.
/// Other - For integer values other than those listed above, refer to the documentation on the Win32 error codes.

    /// * `return_value` -  (u32)
    pub fn resume(&self) -> Result<(), WmiError> {
        self.invoke_method("Resume", &[])

    }

}

