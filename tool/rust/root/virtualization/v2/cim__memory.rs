// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Memory struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Memory {
    #[serde(flatten)]
    pub base: CIM_StorageExtent,

/// An array of octets holding additional error information. An example is ECC Syndrome or the return of the check bits if a CRC-based ErrorMethodology is used. In the latter case, if a single bit error is recognized and the CRC algorithm is known, it is possible to determine the exact bit that failed. This type of data (ECC Syndrome, Check Bit or Parity Bit data, or other vendor supplied information) is included in this field. If the ErrorInfo property is equal to 3, "OK", then AdditionalErrorData has no meaning.
    #[serde(rename = "AdditionalErrorData")]
    pub additional_error_data: Vec<u8>,

/// Boolean indicating that the most recent error was correctable. If the ErrorInfo property is equal to 3, "OK", then this property has no meaning.
    #[serde(rename = "CorrectableError")]
    pub correctable_error: Option<bool>,

/// The ending address, referenced by an application or operating system and mapped by a memory controller, for this Memory object. The ending address is specified in KBytes.
    #[serde(rename = "EndingAddress")]
    pub ending_address: Option<u64>,

/// An integer enumeration indicating the memory access operation that caused the last error. The type of error is described by the ErrorInfo property. If the ErrorInfo property is equal to 3, "OK", then this property has no meaning.
    #[serde(rename = "ErrorAccess")]
    pub error_access: Option<Memory_ErrorAccess>,

/// Specifies the address of the last memory error. The type of error is described by the ErrorInfo property. If the ErrorInfo property is equal to 3, "OK", then this property has no meaning.
    #[serde(rename = "ErrorAddress")]
    pub error_address: Option<u64>,

/// Data captured during the last erroneous mebmory access. The data occupies the first n octets of the array necessary to hold the number of bits specified by the ErrorTransferSize property. If ErrorTransferSize is 0, then this property has no meaning.
    #[serde(rename = "ErrorData")]
    pub error_data: Vec<u8>,

/// The ordering for data stored in the ErrorData property. "Least Significant Byte First" (value=1) or "Most Significant Byte First" (2) can be specified. If ErrorTransferSize is 0, then this property has no meaning.
    #[serde(rename = "ErrorDataOrder")]
    pub error_data_order: Option<Memory_ErrorDataOrder>,

/// An integer enumeration describing the type of error that occurred most recently. For example, single (value=6) or double bit errors (7) can be specified using this property. The values, 12-14, are undefined in the CIM Schema since in DMI, they mix the semantics of the type of error and whether it was correctable or not. The latter is indicated in the property, CorrectableError.
    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<Memory_ErrorInfo>,

/// Specifies the range, in bytes, to which the last error can be resolved. For example, if error addresses are resolved to bit 11 (ie, on a typical page basis), then errors can be resolved to 4K boundaries and this property is set to 4000. If the ErrorInfo property is equal to 3, "OK", then this property has no meaning.
    #[serde(rename = "ErrorResolution")]
    pub error_resolution: Option<u64>,

/// The time that the last memory error occurred. The type of error is described by the ErrorInfo property. If the Error Info property is equal to 3, "OK", then this property has no meaning.
    #[serde(rename = "ErrorTime")]
    pub error_time: Option<String>,

/// The size of the data transfer in bits that caused the last error. 0 indicates no error. If the ErrorInfo property is equal to 3, "OK", then this property should be set to 0.
    #[serde(rename = "ErrorTransferSize")]
    pub error_transfer_size: Option<u32>,

/// Free form string providing more information if the Error Type property is set to 1, "Other". If not set to 1, this string has no meaning.
    #[serde(rename = "OtherErrorDescription")]
    pub other_error_description: Option<String>,

/// The beginning address, referenced by an application or operating system and mapped by a memory controller, for this Memory object. The starting address is specified in KBytes.
    #[serde(rename = "StartingAddress")]
    pub starting_address: Option<u64>,

/// Boolean indicating whether the address information in the property, ErrorAddress, is a system-level address (TRUE) or a physical address (FALSE). If the ErrorInfo property is equal to 3, "OK", then this property has no meaning.
    #[serde(rename = "SystemLevelAddress")]
    pub system_level_address: Option<bool>,

/// Volatile is a property that indicates whether this memory is volatile or not.
    #[serde(rename = "volatile")]
    pub volatile: Option<bool>,
}

impl CIM_Memory {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_StorageExtent::new(),
            additional_error_data: Vec::new(),
            correctable_error: None,
            ending_address: None,
            error_access: None,
            error_address: None,
            error_data: Vec::new(),
            error_data_order: None,
            error_info: None,
            error_resolution: None,
            error_time: None,
            error_transfer_size: None,
            other_error_description: None,
            starting_address: None,
            system_level_address: None,
            volatile: None,
        }
    }


    /// Sets the value of AdditionalErrorData
    pub fn set_additional_error_data(&mut self, value: Vec<u8>) {
        self.additional_error_data = value;
    }

    /// Gets the value of AdditionalErrorData
    pub fn get_additional_error_data(&self) -> &Vec<u8> {
        &self.additional_error_data
    }

    /// Sets the value of CorrectableError
    pub fn set_correctable_error(&mut self, value: bool) {
        self.correctable_error = Some(value);
    }

    /// Gets the value of CorrectableError
    pub fn get_correctable_error(&self) -> Option<&bool> {
        self.correctable_error.as_ref()
    }

    /// Sets the value of EndingAddress
    pub fn set_ending_address(&mut self, value: u64) {
        self.ending_address = Some(value);
    }

    /// Gets the value of EndingAddress
    pub fn get_ending_address(&self) -> Option<&u64> {
        self.ending_address.as_ref()
    }

    /// Sets the value of ErrorAccess
    pub fn set_error_access(&mut self, value: Memory_ErrorAccess) {
        self.error_access = Some(value);
    }

    /// Gets the value of ErrorAccess
    pub fn get_error_access(&self) -> Option<&Memory_ErrorAccess> {
        self.error_access.as_ref()
    }

    /// Sets the value of ErrorAddress
    pub fn set_error_address(&mut self, value: u64) {
        self.error_address = Some(value);
    }

    /// Gets the value of ErrorAddress
    pub fn get_error_address(&self) -> Option<&u64> {
        self.error_address.as_ref()
    }

    /// Sets the value of ErrorData
    pub fn set_error_data(&mut self, value: Vec<u8>) {
        self.error_data = value;
    }

    /// Gets the value of ErrorData
    pub fn get_error_data(&self) -> &Vec<u8> {
        &self.error_data
    }

    /// Sets the value of ErrorDataOrder
    pub fn set_error_data_order(&mut self, value: Memory_ErrorDataOrder) {
        self.error_data_order = Some(value);
    }

    /// Gets the value of ErrorDataOrder
    pub fn get_error_data_order(&self) -> Option<&Memory_ErrorDataOrder> {
        self.error_data_order.as_ref()
    }

    /// Sets the value of ErrorInfo
    pub fn set_error_info(&mut self, value: Memory_ErrorInfo) {
        self.error_info = Some(value);
    }

    /// Gets the value of ErrorInfo
    pub fn get_error_info(&self) -> Option<&Memory_ErrorInfo> {
        self.error_info.as_ref()
    }

    /// Sets the value of ErrorResolution
    pub fn set_error_resolution(&mut self, value: u64) {
        self.error_resolution = Some(value);
    }

    /// Gets the value of ErrorResolution
    pub fn get_error_resolution(&self) -> Option<&u64> {
        self.error_resolution.as_ref()
    }

    /// Sets the value of ErrorTime
    pub fn set_error_time(&mut self, value: String) {
        self.error_time = Some(value);
    }

    /// Gets the value of ErrorTime
    pub fn get_error_time(&self) -> Option<&String> {
        self.error_time.as_ref()
    }

    /// Sets the value of ErrorTransferSize
    pub fn set_error_transfer_size(&mut self, value: u32) {
        self.error_transfer_size = Some(value);
    }

    /// Gets the value of ErrorTransferSize
    pub fn get_error_transfer_size(&self) -> Option<&u32> {
        self.error_transfer_size.as_ref()
    }

    /// Sets the value of OtherErrorDescription
    pub fn set_other_error_description(&mut self, value: String) {
        self.other_error_description = Some(value);
    }

    /// Gets the value of OtherErrorDescription
    pub fn get_other_error_description(&self) -> Option<&String> {
        self.other_error_description.as_ref()
    }

    /// Sets the value of StartingAddress
    pub fn set_starting_address(&mut self, value: u64) {
        self.starting_address = Some(value);
    }

    /// Gets the value of StartingAddress
    pub fn get_starting_address(&self) -> Option<&u64> {
        self.starting_address.as_ref()
    }

    /// Sets the value of SystemLevelAddress
    pub fn set_system_level_address(&mut self, value: bool) {
        self.system_level_address = Some(value);
    }

    /// Gets the value of SystemLevelAddress
    pub fn get_system_level_address(&self) -> Option<&bool> {
        self.system_level_address.as_ref()
    }

    /// Sets the value of volatile
    pub fn set_volatile(&mut self, value: bool) {
        self.volatile = Some(value);
    }

    /// Gets the value of volatile
    pub fn get_volatile(&self) -> Option<&bool> {
        self.volatile.as_ref()
    }
}

