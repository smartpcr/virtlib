// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// BcdObject struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BcdObject {

/// This is the guid id of this object, unique to this store.
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// This is the file path of the store that this object belongs to.
    #[serde(rename = "StoreFilePath")]
    pub store_file_path: Option<String>,

/// The upper 4 bits (28-31) represent the object type. The meaning of the lower 28 bits (0-27) is dependent on the object type.
    #[serde(rename = "Type")]
    pub type: Option<u32>,
}

impl BcdObject {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            id: None,
            store_file_path: None,
            type: None,
        }
    }


    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of StoreFilePath
    pub fn set_store_file_path(&mut self, value: String) {
        self.store_file_path = Some(value);
    }

    /// Gets the value of StoreFilePath
    pub fn get_store_file_path(&self) -> Option<&String> {
        self.store_file_path.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u32) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u32> {
        self.type.as_ref()
    }

/// 

    /// * `return_value` -  (bool)
    /// * `types` -  (u32[])
    pub fn enumerate_element_types(&self, types: &mut Vec<u32>) -> Result<(), WmiError> {

        let result = self.invoke_method("EnumerateElementTypes", &[])?;
        let types = result.get_value("Types")?;
        Ok(result.return_value)

    }


/// 

    /// * `elements` -  (BcdElement[])
    /// * `return_value` -  (bool)
    pub fn enumerate_elements(&self, elements: &mut Vec<BcdElement>) -> Result<(), WmiError> {

        let result = self.invoke_method("EnumerateElements", &[])?;
        let elements = result.get_value("Elements")?;
        Ok(result.return_value)

    }


/// 

    /// * `type` -  (u32)

    /// * `element` -  (BcdElement)
    /// * `return_value` -  (bool)
    pub fn get_element(&self, type: u32, element: &mut BcdElement) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });

        let result = self.invoke_method("GetElement", &args)?;
        let element = result.get_value("Element")?;
        Ok(result.return_value)

    }


/// 

    /// * `flags` -  (u32)
    /// * `type` -  (u32)

    /// * `element` -  (BcdElement)
    /// * `return_value` -  (bool)
    pub fn get_element_with_flags(&self, type: u32, flags: u32, element: &mut BcdElement) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });

        let result = self.invoke_method("GetElementWithFlags", &args)?;
        let element = result.get_value("Element")?;
        Ok(result.return_value)

    }


/// 

    /// * `additional_options` -  (String)
    /// * `device_type` -  (BcdObject_DeviceType)
    /// * `type` -  (u32)

    /// * `return_value` -  (bool)
    pub fn set_device_element(&self, type: u32, device_type: BcdObject_DeviceType, additional_options: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });
        args.push(MethodParameter { name: "DeviceType".to_string(), value: device_type.into() });
        args.push(MethodParameter { name: "AdditionalOptions".to_string(), value: additional_options.into() });
        self.invoke_method("SetDeviceElement", &args)

    }


/// 

    /// * `additional_options` -  (String)
    /// * `device_type` -  (BcdObject_DeviceType)
    /// * `path` -  (String)
    /// * `type` -  (u32)

    /// * `return_value` -  (bool)
    pub fn set_partition_device_element(&self, type: u32, device_type: BcdObject_DeviceType, additional_options: &String, path: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });
        args.push(MethodParameter { name: "DeviceType".to_string(), value: device_type.into() });
        args.push(MethodParameter { name: "AdditionalOptions".to_string(), value: additional_options.into() });
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        self.invoke_method("SetPartitionDeviceElement", &args)

    }


/// 

    /// * `additional_options` -  (String)
    /// * `device_type` -  (BcdObject_DeviceType)
    /// * `flags` -  (u32)
    /// * `path` -  (String)
    /// * `type` -  (u32)

    /// * `return_value` -  (bool)
    pub fn set_partition_device_element_with_flags(&self, type: u32, device_type: BcdObject_DeviceType, additional_options: &String, path: &String, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });
        args.push(MethodParameter { name: "DeviceType".to_string(), value: device_type.into() });
        args.push(MethodParameter { name: "AdditionalOptions".to_string(), value: additional_options.into() });
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("SetPartitionDeviceElementWithFlags", &args)

    }


/// 

    /// * `additional_options` -  (String)
    /// * `device_type` -  (BcdObject_DeviceType)
    /// * `parent_additional_options` -  (String)
    /// * `parent_device_type` -  (u32)
    /// * `parent_path` -  (String)
    /// * `path` -  (String)
    /// * `type` -  (u32)

    /// * `return_value` -  (bool)
    pub fn set_file_device_element(&self, type: u32, device_type: BcdObject_DeviceType, additional_options: &String, path: &String, parent_device_type: u32, parent_additional_options: &String, parent_path: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });
        args.push(MethodParameter { name: "DeviceType".to_string(), value: device_type.into() });
        args.push(MethodParameter { name: "AdditionalOptions".to_string(), value: additional_options.into() });
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "ParentDeviceType".to_string(), value: parent_device_type.into() });
        args.push(MethodParameter { name: "ParentAdditionalOptions".to_string(), value: parent_additional_options.into() });
        args.push(MethodParameter { name: "ParentPath".to_string(), value: parent_path.into() });
        self.invoke_method("SetFileDeviceElement", &args)

    }


/// 

    /// * `disk_signature` -  (String)
    /// * `partition_identifier` -  (String)
    /// * `partition_style` -  (BcdObject_PartitionStyle)
    /// * `type` -  (u32)

    /// * `return_value` -  (bool)
    pub fn set_qualified_partition_device_element(&self, type: u32, partition_style: BcdObject_PartitionStyle, disk_signature: &String, partition_identifier: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });
        args.push(MethodParameter { name: "PartitionStyle".to_string(), value: partition_style.into() });
        args.push(MethodParameter { name: "DiskSignature".to_string(), value: disk_signature.into() });
        args.push(MethodParameter { name: "PartitionIdentifier".to_string(), value: partition_identifier.into() });
        self.invoke_method("SetQualifiedPartitionDeviceElement", &args)

    }


/// 

    /// * `custom_locate` -  (u32)
    /// * `parent_additional_options` -  (String)
    /// * `parent_device_type` -  (u32)
    /// * `parent_path` -  (String)
    /// * `path` -  (String)
    /// * `type` -  (u32)

    /// * `return_value` -  (bool)
    pub fn set_vhd_device_element(&self, type: u32, path: &String, parent_device_type: u32, parent_additional_options: &String, parent_path: &String, custom_locate: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "ParentDeviceType".to_string(), value: parent_device_type.into() });
        args.push(MethodParameter { name: "ParentAdditionalOptions".to_string(), value: parent_additional_options.into() });
        args.push(MethodParameter { name: "ParentPath".to_string(), value: parent_path.into() });
        args.push(MethodParameter { name: "CustomLocate".to_string(), value: custom_locate.into() });
        self.invoke_method("SetVhdDeviceElement", &args)

    }


/// 

    /// * `string` -  (String)
    /// * `type` -  (u32)

    /// * `return_value` -  (bool)
    pub fn set_string_element(&self, type: u32, string: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });
        args.push(MethodParameter { name: "String".to_string(), value: string.into() });
        self.invoke_method("SetStringElement", &args)

    }


/// 

    /// * `id` -  (String)
    /// * `type` -  (u32)

    /// * `return_value` -  (bool)
    pub fn set_object_element(&self, type: u32, id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });
        args.push(MethodParameter { name: "Id".to_string(), value: id.into() });
        self.invoke_method("SetObjectElement", &args)

    }


/// 

    /// * `ids` -  (String[])
    /// * `type` -  (u32)

    /// * `return_value` -  (bool)
    pub fn set_object_list_element(&self, type: u32, ids: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });
        args.push(MethodParameter { name: "Ids".to_string(), value: ids.into() });
        self.invoke_method("SetObjectListElement", &args)

    }


/// 

    /// * `integer` -  (u64)
    /// * `type` -  (u32)

    /// * `return_value` -  (bool)
    pub fn set_integer_element(&self, type: u32, integer: u64) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });
        args.push(MethodParameter { name: "Integer".to_string(), value: integer.into() });
        self.invoke_method("SetIntegerElement", &args)

    }


/// 

    /// * `integers` -  (u64[])
    /// * `type` -  (u32)

    /// * `return_value` -  (bool)
    pub fn set_integer_list_element(&self, type: u32, integers: &Vec<u64>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });
        args.push(MethodParameter { name: "Integers".to_string(), value: integers.into() });
        self.invoke_method("SetIntegerListElement", &args)

    }


/// 

    /// * `boolean` -  (bool)
    /// * `type` -  (u32)

    /// * `return_value` -  (bool)
    pub fn set_boolean_element(&self, type: u32, boolean: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });
        args.push(MethodParameter { name: "Boolean".to_string(), value: boolean.into() });
        self.invoke_method("SetBooleanElement", &args)

    }


/// 

    /// * `type` -  (u32)

    /// * `return_value` -  (bool)
    pub fn delete_element(&self, type: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });
        self.invoke_method("DeleteElement", &args)

    }

}

