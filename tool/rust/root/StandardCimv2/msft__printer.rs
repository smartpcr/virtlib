// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_Printer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_Printer {
    #[serde(flatten)]
    pub base: CIM_ManagedSystemElement,

/// 
    #[serde(rename = "BranchOfficeOfflineLogSizeMB")]
    pub branch_office_offline_log_size_mb: Option<u32>,

/// 
    #[serde(rename = "Comment")]
    pub comment: Option<String>,

/// 
    #[serde(rename = "ComputerName")]
    pub computer_name: Option<String>,

/// 
    #[serde(rename = "Datatype")]
    pub datatype: Option<String>,

/// 
    #[serde(rename = "DefaultJobPriority")]
    pub default_job_priority: Option<u32>,

/// 
    #[serde(rename = "DeviceType")]
    pub device_type: Option<u32>,

/// 
    #[serde(rename = "DisableBranchOfficeLogging")]
    pub disable_branch_office_logging: Option<bool>,

/// 
    #[serde(rename = "DriverName")]
    pub driver_name: Option<String>,

/// 
    #[serde(rename = "JobCount")]
    pub job_count: Option<u32>,

/// 
    #[serde(rename = "KeepPrintedJobs")]
    pub keep_printed_jobs: Option<bool>,

/// 
    #[serde(rename = "Location")]
    pub location: Option<String>,

/// 
    #[serde(rename = "PermissionSDDL")]
    pub permission_sddl: Option<String>,

/// 
    #[serde(rename = "PortName")]
    pub port_name: Option<String>,

/// 
    #[serde(rename = "PrinterStatus")]
    pub printer_status: Option<u32>,

/// 
    #[serde(rename = "PrintProcessor")]
    pub print_processor: Option<String>,

/// 
    #[serde(rename = "Priority")]
    pub priority: Option<u32>,

/// 
    #[serde(rename = "Published")]
    pub published: Option<bool>,

/// 
    #[serde(rename = "RenderingMode")]
    pub rendering_mode: Option<u32>,

/// 
    #[serde(rename = "SeparatorPageFile")]
    pub separator_page_file: Option<String>,

/// 
    #[serde(rename = "Shared")]
    pub shared: Option<bool>,

/// 
    #[serde(rename = "ShareName")]
    pub share_name: Option<String>,

/// 
    #[serde(rename = "StartTime")]
    pub start_time: Option<u32>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u32>,

/// 
    #[serde(rename = "UntilTime")]
    pub until_time: Option<u32>,

/// 
    #[serde(rename = "WorkflowPolicy")]
    pub workflow_policy: Option<u32>,
}

impl MSFT_Printer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedSystemElement::new(),
            branch_office_offline_log_size_mb: None,
            comment: None,
            computer_name: None,
            datatype: None,
            default_job_priority: None,
            device_type: None,
            disable_branch_office_logging: None,
            driver_name: None,
            job_count: None,
            keep_printed_jobs: None,
            location: None,
            permission_sddl: None,
            port_name: None,
            printer_status: None,
            print_processor: None,
            priority: None,
            published: None,
            rendering_mode: None,
            separator_page_file: None,
            shared: None,
            share_name: None,
            start_time: None,
            type: None,
            until_time: None,
            workflow_policy: None,
        }
    }


    /// Sets the value of BranchOfficeOfflineLogSizeMB
    pub fn set_branch_office_offline_log_size_mb(&mut self, value: u32) {
        self.branch_office_offline_log_size_mb = Some(value);
    }

    /// Gets the value of BranchOfficeOfflineLogSizeMB
    pub fn get_branch_office_offline_log_size_mb(&self) -> Option<&u32> {
        self.branch_office_offline_log_size_mb.as_ref()
    }

    /// Sets the value of Comment
    pub fn set_comment(&mut self, value: String) {
        self.comment = Some(value);
    }

    /// Gets the value of Comment
    pub fn get_comment(&self) -> Option<&String> {
        self.comment.as_ref()
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

    /// Sets the value of DefaultJobPriority
    pub fn set_default_job_priority(&mut self, value: u32) {
        self.default_job_priority = Some(value);
    }

    /// Gets the value of DefaultJobPriority
    pub fn get_default_job_priority(&self) -> Option<&u32> {
        self.default_job_priority.as_ref()
    }

    /// Sets the value of DeviceType
    pub fn set_device_type(&mut self, value: u32) {
        self.device_type = Some(value);
    }

    /// Gets the value of DeviceType
    pub fn get_device_type(&self) -> Option<&u32> {
        self.device_type.as_ref()
    }

    /// Sets the value of DisableBranchOfficeLogging
    pub fn set_disable_branch_office_logging(&mut self, value: bool) {
        self.disable_branch_office_logging = Some(value);
    }

    /// Gets the value of DisableBranchOfficeLogging
    pub fn get_disable_branch_office_logging(&self) -> Option<&bool> {
        self.disable_branch_office_logging.as_ref()
    }

    /// Sets the value of DriverName
    pub fn set_driver_name(&mut self, value: String) {
        self.driver_name = Some(value);
    }

    /// Gets the value of DriverName
    pub fn get_driver_name(&self) -> Option<&String> {
        self.driver_name.as_ref()
    }

    /// Sets the value of JobCount
    pub fn set_job_count(&mut self, value: u32) {
        self.job_count = Some(value);
    }

    /// Gets the value of JobCount
    pub fn get_job_count(&self) -> Option<&u32> {
        self.job_count.as_ref()
    }

    /// Sets the value of KeepPrintedJobs
    pub fn set_keep_printed_jobs(&mut self, value: bool) {
        self.keep_printed_jobs = Some(value);
    }

    /// Gets the value of KeepPrintedJobs
    pub fn get_keep_printed_jobs(&self) -> Option<&bool> {
        self.keep_printed_jobs.as_ref()
    }

    /// Sets the value of Location
    pub fn set_location(&mut self, value: String) {
        self.location = Some(value);
    }

    /// Gets the value of Location
    pub fn get_location(&self) -> Option<&String> {
        self.location.as_ref()
    }

    /// Sets the value of PermissionSDDL
    pub fn set_permission_sddl(&mut self, value: String) {
        self.permission_sddl = Some(value);
    }

    /// Gets the value of PermissionSDDL
    pub fn get_permission_sddl(&self) -> Option<&String> {
        self.permission_sddl.as_ref()
    }

    /// Sets the value of PortName
    pub fn set_port_name(&mut self, value: String) {
        self.port_name = Some(value);
    }

    /// Gets the value of PortName
    pub fn get_port_name(&self) -> Option<&String> {
        self.port_name.as_ref()
    }

    /// Sets the value of PrinterStatus
    pub fn set_printer_status(&mut self, value: u32) {
        self.printer_status = Some(value);
    }

    /// Gets the value of PrinterStatus
    pub fn get_printer_status(&self) -> Option<&u32> {
        self.printer_status.as_ref()
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

    /// Sets the value of RenderingMode
    pub fn set_rendering_mode(&mut self, value: u32) {
        self.rendering_mode = Some(value);
    }

    /// Gets the value of RenderingMode
    pub fn get_rendering_mode(&self) -> Option<&u32> {
        self.rendering_mode.as_ref()
    }

    /// Sets the value of SeparatorPageFile
    pub fn set_separator_page_file(&mut self, value: String) {
        self.separator_page_file = Some(value);
    }

    /// Gets the value of SeparatorPageFile
    pub fn get_separator_page_file(&self) -> Option<&String> {
        self.separator_page_file.as_ref()
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

    /// Sets the value of StartTime
    pub fn set_start_time(&mut self, value: u32) {
        self.start_time = Some(value);
    }

    /// Gets the value of StartTime
    pub fn get_start_time(&self) -> Option<&u32> {
        self.start_time.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u32) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u32> {
        self.type.as_ref()
    }

    /// Sets the value of UntilTime
    pub fn set_until_time(&mut self, value: u32) {
        self.until_time = Some(value);
    }

    /// Gets the value of UntilTime
    pub fn get_until_time(&self) -> Option<&u32> {
        self.until_time.as_ref()
    }

    /// Sets the value of WorkflowPolicy
    pub fn set_workflow_policy(&mut self, value: u32) {
        self.workflow_policy = Some(value);
    }

    /// Gets the value of WorkflowPolicy
    pub fn get_workflow_policy(&self) -> Option<&u32> {
        self.workflow_policy.as_ref()
    }

/// 

    /// * `connection_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn add_connection(&self, connection_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ConnectionName".to_string(), value: connection_name.into() });
        self.invoke_method("AddConnection", &args)

    }


/// 

    /// * `branch_office_offline_log_size_mb` -  (u32)
    /// * `comment` -  (String)
    /// * `computer_name` -  (String)
    /// * `datatype` -  (String)
    /// * `disable_branch_office_logging` -  (bool)
    /// * `driver_name` -  (String)
    /// * `keep_printed_jobs` -  (bool)
    /// * `location` -  (String)
    /// * `name` -  (String)
    /// * `permission_sddl` -  (String)
    /// * `port_name` -  (String)
    /// * `print_processor` -  (String)
    /// * `priority` -  (u32)
    /// * `published` -  (bool)
    /// * `rendering_mode` -  (u32)
    /// * `separator_page_file` -  (String)
    /// * `shared` -  (bool)
    /// * `share_name` -  (String)
    /// * `start_time` -  (u32)
    /// * `until_time` -  (u32)
    /// * `workflow_policy` -  (u32)

    /// * `return_value` -  (u32)
    pub fn add_by_existing_port(&self, comment: &String, datatype: &String, driver_name: &String, until_time: u32, keep_printed_jobs: bool, location: &String, name: &String, permission_sddl: &String, port_name: &String, print_processor: &String, priority: u32, published: bool, rendering_mode: u32, separator_page_file: &String, computer_name: &String, share_name: &String, shared: bool, start_time: u32, disable_branch_office_logging: bool, branch_office_offline_log_size_mb: u32, workflow_policy: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Comment".to_string(), value: comment.into() });
        args.push(MethodParameter { name: "Datatype".to_string(), value: datatype.into() });
        args.push(MethodParameter { name: "DriverName".to_string(), value: driver_name.into() });
        args.push(MethodParameter { name: "UntilTime".to_string(), value: until_time.into() });
        args.push(MethodParameter { name: "KeepPrintedJobs".to_string(), value: keep_printed_jobs.into() });
        args.push(MethodParameter { name: "Location".to_string(), value: location.into() });
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "PermissionSDDL".to_string(), value: permission_sddl.into() });
        args.push(MethodParameter { name: "PortName".to_string(), value: port_name.into() });
        args.push(MethodParameter { name: "PrintProcessor".to_string(), value: print_processor.into() });
        args.push(MethodParameter { name: "Priority".to_string(), value: priority.into() });
        args.push(MethodParameter { name: "Published".to_string(), value: published.into() });
        args.push(MethodParameter { name: "RenderingMode".to_string(), value: rendering_mode.into() });
        args.push(MethodParameter { name: "SeparatorPageFile".to_string(), value: separator_page_file.into() });
        args.push(MethodParameter { name: "ComputerName".to_string(), value: computer_name.into() });
        args.push(MethodParameter { name: "ShareName".to_string(), value: share_name.into() });
        args.push(MethodParameter { name: "Shared".to_string(), value: shared.into() });
        args.push(MethodParameter { name: "StartTime".to_string(), value: start_time.into() });
        args.push(MethodParameter { name: "DisableBranchOfficeLogging".to_string(), value: disable_branch_office_logging.into() });
        args.push(MethodParameter { name: "BranchOfficeOfflineLogSizeMB".to_string(), value: branch_office_offline_log_size_mb.into() });
        args.push(MethodParameter { name: "WorkflowPolicy".to_string(), value: workflow_policy.into() });
        self.invoke_method("AddByExistingPort", &args)

    }


/// 

    /// * `branch_office_offline_log_size_mb` -  (u32)
    /// * `comment` -  (String)
    /// * `computer_name` -  (String)
    /// * `datatype` -  (String)
    /// * `device_url` -  (String)
    /// * `device_uuid` -  (String)
    /// * `disable_branch_office_logging` -  (bool)
    /// * `ipp_url` -  (String)
    /// * `keep_printed_jobs` -  (bool)
    /// * `location` -  (String)
    /// * `name` -  (String)
    /// * `permission_sddl` -  (String)
    /// * `print_processor` -  (String)
    /// * `priority` -  (u32)
    /// * `published` -  (bool)
    /// * `rendering_mode` -  (u32)
    /// * `separator_page_file` -  (String)
    /// * `shared` -  (bool)
    /// * `share_name` -  (String)
    /// * `start_time` -  (u32)
    /// * `until_time` -  (u32)
    /// * `workflow_policy` -  (u32)

    /// * `return_value` -  (u32)
    pub fn add_by_adaptive_port(&self, comment: &String, datatype: &String, device_url: &String, until_time: u32, device_uuid: &String, ipp_url: &String, keep_printed_jobs: bool, location: &String, name: &String, permission_sddl: &String, print_processor: &String, priority: u32, published: bool, rendering_mode: u32, separator_page_file: &String, computer_name: &String, share_name: &String, shared: bool, start_time: u32, disable_branch_office_logging: bool, branch_office_offline_log_size_mb: u32, workflow_policy: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Comment".to_string(), value: comment.into() });
        args.push(MethodParameter { name: "Datatype".to_string(), value: datatype.into() });
        args.push(MethodParameter { name: "DeviceURL".to_string(), value: device_url.into() });
        args.push(MethodParameter { name: "UntilTime".to_string(), value: until_time.into() });
        args.push(MethodParameter { name: "DeviceUUID".to_string(), value: device_uuid.into() });
        args.push(MethodParameter { name: "IppUrl".to_string(), value: ipp_url.into() });
        args.push(MethodParameter { name: "KeepPrintedJobs".to_string(), value: keep_printed_jobs.into() });
        args.push(MethodParameter { name: "Location".to_string(), value: location.into() });
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "PermissionSDDL".to_string(), value: permission_sddl.into() });
        args.push(MethodParameter { name: "PrintProcessor".to_string(), value: print_processor.into() });
        args.push(MethodParameter { name: "Priority".to_string(), value: priority.into() });
        args.push(MethodParameter { name: "Published".to_string(), value: published.into() });
        args.push(MethodParameter { name: "RenderingMode".to_string(), value: rendering_mode.into() });
        args.push(MethodParameter { name: "SeparatorPageFile".to_string(), value: separator_page_file.into() });
        args.push(MethodParameter { name: "ComputerName".to_string(), value: computer_name.into() });
        args.push(MethodParameter { name: "ShareName".to_string(), value: share_name.into() });
        args.push(MethodParameter { name: "Shared".to_string(), value: shared.into() });
        args.push(MethodParameter { name: "StartTime".to_string(), value: start_time.into() });
        args.push(MethodParameter { name: "DisableBranchOfficeLogging".to_string(), value: disable_branch_office_logging.into() });
        args.push(MethodParameter { name: "BranchOfficeOfflineLogSizeMB".to_string(), value: branch_office_offline_log_size_mb.into() });
        args.push(MethodParameter { name: "WorkflowPolicy".to_string(), value: workflow_policy.into() });
        self.invoke_method("AddByAdaptivePort", &args)

    }


/// 

    /// * `computer_name` -  (String)
    /// * `name` -  (String)
    /// * `new_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn rename_by_name(&self, name: &String, new_name: &String, computer_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "NewName".to_string(), value: new_name.into() });
        args.push(MethodParameter { name: "ComputerName".to_string(), value: computer_name.into() });
        self.invoke_method("RenameByName", &args)

    }


/// 

    /// * `input_object` -  (MSFT_Printer)
    /// * `new_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn rename_by_object(&self, input_object: MSFT_Printer, new_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });
        args.push(MethodParameter { name: "NewName".to_string(), value: new_name.into() });
        self.invoke_method("RenameByObject", &args)

    }

}

