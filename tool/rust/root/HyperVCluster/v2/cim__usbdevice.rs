// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_USBDevice struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_USBDevice {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// Indicates the USB class code.
    #[serde(rename = "ClassCode")]
    pub class_code: Option<u8>,

/// CommandTimeout is configurable by management applications supporting USB Redirections. When the Redirection Service redirects a USBDevice command to a remote device, and the remote device does not respond before CommandTimout times out, the Redirection Service will emulate a media eject event and re-try the command and/or try to re-establish the connection to the remote device. The timeout is expressed using the interval format of the datetime type.
    #[serde(rename = "CommandTimeout")]
    pub command_timeout: Option<String>,

/// An array of USB 'alternate settings' for each interface in the currently selected configuration (indicated by the CurrentConfigValue property). This array has one entry for each interface in the configuration. If the property, CurrentConfigValue, is zero (indicating the Device is not configured), the array is undefined. To understand how to parse this octet string, refer to the USB Specification.
    #[serde(rename = "CurrentAlternateSettings")]
    pub current_alternate_settings: Vec<u8>,

/// Indicates the configuration currently selected for the Device. If this value is zero, the Device is unconfigured.
    #[serde(rename = "CurrentConfigValue")]
    pub current_config_value: Option<u8>,

/// From the USB specification Device Descriptor, Device Release Number in Binary-Coded Decimal.
    #[serde(rename = "DeviceReleaseNumber")]
    pub device_release_number: Option<u16>,

/// From the USB specification Device Descriptior, Manufacturer string.
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// From the USB specification Device Descriptor, Maximum Packet size for the USB zero endpoint. Valid sizes are 8, 16, 32, 64.
    #[serde(rename = "MaxPacketSize")]
    pub max_packet_size: Option<u8>,

/// Number of device configurations that are defined for the Device.
    #[serde(rename = "NumberOfConfigs")]
    pub number_of_configs: Option<u8>,

/// From the USB specification Device Descriptor, Product String.
    #[serde(rename = "Product")]
    pub product: Option<String>,

/// From the USB specification Device Descriptor, Product ID assigned by manufacturer.
    #[serde(rename = "ProductID")]
    pub product_id: Option<u16>,

/// Indicates the USB protocol code.
    #[serde(rename = "ProtocolCode")]
    pub protocol_code: Option<u8>,

/// From the USB specification Device Descriptor, Serial Number String.
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,

/// Indicates the USB subclass code.
    #[serde(rename = "SubclassCode")]
    pub subclass_code: Option<u8>,

/// Indicates the latest USB Version supported by the USB Device. The property is expressed as a Binary-Coded Decimal (BCD) where a decimal point is implied between the 2nd and 3rd digits. For example, a value of 0x201 indicates that version 2.01 is supported.
    #[serde(rename = "USBVersion")]
    pub usbversion: Option<u16>,

/// From the USB specification Device Descriptor, where 'bcdUSB' is the USB Specification Number, in Binary-Coded Decimal format, that the device complies with.
    #[serde(rename = "USBVersionInBCD")]
    pub usbversion_in_bcd: Option<u16>,

/// From the USB specification Device Descriptor, Vendor ID assigned by USB.org.
    #[serde(rename = "VendorID")]
    pub vendor_id: Option<u16>,
}

impl CIM_USBDevice {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            class_code: None,
            command_timeout: None,
            current_alternate_settings: Vec::new(),
            current_config_value: None,
            device_release_number: None,
            manufacturer: None,
            max_packet_size: None,
            number_of_configs: None,
            product: None,
            product_id: None,
            protocol_code: None,
            serial_number: None,
            subclass_code: None,
            usbversion: None,
            usbversion_in_bcd: None,
            vendor_id: None,
        }
    }


    /// Sets the value of ClassCode
    pub fn set_class_code(&mut self, value: u8) {
        self.class_code = Some(value);
    }

    /// Gets the value of ClassCode
    pub fn get_class_code(&self) -> Option<&u8> {
        self.class_code.as_ref()
    }

    /// Sets the value of CommandTimeout
    pub fn set_command_timeout(&mut self, value: String) {
        self.command_timeout = Some(value);
    }

    /// Gets the value of CommandTimeout
    pub fn get_command_timeout(&self) -> Option<&String> {
        self.command_timeout.as_ref()
    }

    /// Sets the value of CurrentAlternateSettings
    pub fn set_current_alternate_settings(&mut self, value: Vec<u8>) {
        self.current_alternate_settings = value;
    }

    /// Gets the value of CurrentAlternateSettings
    pub fn get_current_alternate_settings(&self) -> &Vec<u8> {
        &self.current_alternate_settings
    }

    /// Sets the value of CurrentConfigValue
    pub fn set_current_config_value(&mut self, value: u8) {
        self.current_config_value = Some(value);
    }

    /// Gets the value of CurrentConfigValue
    pub fn get_current_config_value(&self) -> Option<&u8> {
        self.current_config_value.as_ref()
    }

    /// Sets the value of DeviceReleaseNumber
    pub fn set_device_release_number(&mut self, value: u16) {
        self.device_release_number = Some(value);
    }

    /// Gets the value of DeviceReleaseNumber
    pub fn get_device_release_number(&self) -> Option<&u16> {
        self.device_release_number.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of MaxPacketSize
    pub fn set_max_packet_size(&mut self, value: u8) {
        self.max_packet_size = Some(value);
    }

    /// Gets the value of MaxPacketSize
    pub fn get_max_packet_size(&self) -> Option<&u8> {
        self.max_packet_size.as_ref()
    }

    /// Sets the value of NumberOfConfigs
    pub fn set_number_of_configs(&mut self, value: u8) {
        self.number_of_configs = Some(value);
    }

    /// Gets the value of NumberOfConfigs
    pub fn get_number_of_configs(&self) -> Option<&u8> {
        self.number_of_configs.as_ref()
    }

    /// Sets the value of Product
    pub fn set_product(&mut self, value: String) {
        self.product = Some(value);
    }

    /// Gets the value of Product
    pub fn get_product(&self) -> Option<&String> {
        self.product.as_ref()
    }

    /// Sets the value of ProductID
    pub fn set_product_id(&mut self, value: u16) {
        self.product_id = Some(value);
    }

    /// Gets the value of ProductID
    pub fn get_product_id(&self) -> Option<&u16> {
        self.product_id.as_ref()
    }

    /// Sets the value of ProtocolCode
    pub fn set_protocol_code(&mut self, value: u8) {
        self.protocol_code = Some(value);
    }

    /// Gets the value of ProtocolCode
    pub fn get_protocol_code(&self) -> Option<&u8> {
        self.protocol_code.as_ref()
    }

    /// Sets the value of SerialNumber
    pub fn set_serial_number(&mut self, value: String) {
        self.serial_number = Some(value);
    }

    /// Gets the value of SerialNumber
    pub fn get_serial_number(&self) -> Option<&String> {
        self.serial_number.as_ref()
    }

    /// Sets the value of SubclassCode
    pub fn set_subclass_code(&mut self, value: u8) {
        self.subclass_code = Some(value);
    }

    /// Gets the value of SubclassCode
    pub fn get_subclass_code(&self) -> Option<&u8> {
        self.subclass_code.as_ref()
    }

    /// Sets the value of USBVersion
    pub fn set_usbversion(&mut self, value: u16) {
        self.usbversion = Some(value);
    }

    /// Gets the value of USBVersion
    pub fn get_usbversion(&self) -> Option<&u16> {
        self.usbversion.as_ref()
    }

    /// Sets the value of USBVersionInBCD
    pub fn set_usbversion_in_bcd(&mut self, value: u16) {
        self.usbversion_in_bcd = Some(value);
    }

    /// Gets the value of USBVersionInBCD
    pub fn get_usbversion_in_bcd(&self) -> Option<&u16> {
        self.usbversion_in_bcd.as_ref()
    }

    /// Sets the value of VendorID
    pub fn set_vendor_id(&mut self, value: u16) {
        self.vendor_id = Some(value);
    }

    /// Gets the value of VendorID
    pub fn get_vendor_id(&self) -> Option<&u16> {
        self.vendor_id.as_ref()
    }

/// This method returns the USBDevice Descriptor as specified by the input parameters. Each parameter is briefly described here with more detail in its Qualifier list. RequestType is an input parameter that defines whether the request is for standard, class or vendor-specific information, as well as specifying the recipient. RequestValue is also an input parameter and defines the USB Descriptor Type and Index. RequestIndex is an input parameter which describes the language used to return a string Descriptor. RequestLength is both an input and output parameter. It specifies the length of the Descriptor that should be returned (on input) and what is actually returned in the Buffer parameter (on output). Buffer is an output parameter, containing the Descriptor data. The GetDescriptor method returns an integer value of 0 if the USB Descriptor is successfully returned, 1 if the request is not supported and any other number to indicate an error. 
/// In a subclass, the set of possible return codes could be specified, using a ValueMap qualifier on the method. The strings to which the ValueMap contents are 'translated' may also be specified in the subclass as a Values array qualifier.

    /// * `request_index` - RequestIndex defines the 2 byte Language ID code used by the USBDevice when returning string Descriptor data. The parameter is typically 0 for non-string Descriptors. Refer to the USB Specification for more information. (u16)
    /// * `request_length` - On input, RequestLength is the length (in octets) of the Descriptor that should be returned. If this value is less than the actual length of the Descriptor, only the requested length will be returned. If it is more than the actual length, the actual length is returned. On output, this parameter is the length, in octets, of the Buffer being returned. If the requested Descriptor does not exist, the contents of this parameter are undefined. (u16)
    /// * `request_type` - RequestType is bit-mapped and identifies the type of Descriptor request and the recipient. The type of request may be 'standard', 'class' or 'vendor-specific'. The recipient may be 'device', 'interface', 'endpoint' or 'other'. Refer to the USB Specification for the appropriate values for each bit. (u8)
    /// * `request_value` - RequestValue contains the Descriptor Type in the high byte and the Descriptor Index (for example, index or offset into the Descriptor array) in the low byte. Refer to the USB Specification for more information. (u16)

    /// * `buffer` - Buffer returns the requested Descriptor information. If the Descriptor does not exist, the contents of the Buffer are undefined. (u8[])
    /// * `request_length` - On input, RequestLength is the length (in octets) of the Descriptor that should be returned. If this value is less than the actual length of the Descriptor, only the requested length will be returned. If it is more than the actual length, the actual length is returned. On output, this parameter is the length, in octets, of the Buffer being returned. If the requested Descriptor does not exist, the contents of this parameter are undefined. (u16)
    /// * `return_value` -  (u32)
    pub fn get_descriptor(&self, request_type: u8, request_value: u16, request_index: u16, request_length: &mut u16, buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RequestType".to_string(), value: request_type.into() });
        args.push(MethodParameter { name: "RequestValue".to_string(), value: request_value.into() });
        args.push(MethodParameter { name: "RequestIndex".to_string(), value: request_index.into() });

        let result = self.invoke_method("GetDescriptor", &args)?;
        let buffer = result.get_value("Buffer")?;
        let request_length = result.get_value("RequestLength")?;
        Ok(result.return_value)

    }

}

