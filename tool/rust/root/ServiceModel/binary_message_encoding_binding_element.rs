// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// BinaryMessageEncodingBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BinaryMessageEncodingBindingElement {
    #[serde(flatten)]
    pub base: MessageEncodingBindingElement,

/// The compression format to apply to the messages.
    #[serde(rename = "CompressionFormat")]
    pub compression_format: Option<String>,

/// An integer that defines how many messages can be read simultaneously without allocating new readers. 
    #[serde(rename = "MaxReadPoolSize")]
    pub max_read_pool_size: Option<i32>,

/// A value that specifies the size, in bytes, of the buffer used for encoding.
    #[serde(rename = "MaxSessionSize")]
    pub max_session_size: Option<i32>,

/// An integer that defines how many messages can be sent simultaneously without allocating new writers.
    #[serde(rename = "MaxWritePoolSize")]
    pub max_write_pool_size: Option<i32>,

/// The quotas of the readers.
    #[serde(rename = "ReaderQuotas")]
    pub reader_quotas: Option<XmlDictionaryReaderQuotas>,
}

impl BinaryMessageEncodingBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MessageEncodingBindingElement::new(),
            compression_format: None,
            max_read_pool_size: None,
            max_session_size: None,
            max_write_pool_size: None,
            reader_quotas: None,
        }
    }


    /// Sets the value of CompressionFormat
    pub fn set_compression_format(&mut self, value: String) {
        self.compression_format = Some(value);
    }

    /// Gets the value of CompressionFormat
    pub fn get_compression_format(&self) -> Option<&String> {
        self.compression_format.as_ref()
    }

    /// Sets the value of MaxReadPoolSize
    pub fn set_max_read_pool_size(&mut self, value: i32) {
        self.max_read_pool_size = Some(value);
    }

    /// Gets the value of MaxReadPoolSize
    pub fn get_max_read_pool_size(&self) -> Option<&i32> {
        self.max_read_pool_size.as_ref()
    }

    /// Sets the value of MaxSessionSize
    pub fn set_max_session_size(&mut self, value: i32) {
        self.max_session_size = Some(value);
    }

    /// Gets the value of MaxSessionSize
    pub fn get_max_session_size(&self) -> Option<&i32> {
        self.max_session_size.as_ref()
    }

    /// Sets the value of MaxWritePoolSize
    pub fn set_max_write_pool_size(&mut self, value: i32) {
        self.max_write_pool_size = Some(value);
    }

    /// Gets the value of MaxWritePoolSize
    pub fn get_max_write_pool_size(&self) -> Option<&i32> {
        self.max_write_pool_size.as_ref()
    }

    /// Sets the value of ReaderQuotas
    pub fn set_reader_quotas(&mut self, value: XmlDictionaryReaderQuotas) {
        self.reader_quotas = Some(value);
    }

    /// Gets the value of ReaderQuotas
    pub fn get_reader_quotas(&self) -> Option<&XmlDictionaryReaderQuotas> {
        self.reader_quotas.as_ref()
    }
}

