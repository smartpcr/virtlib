// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

using System;
using System.Globalization;
using System.Management;
using System.Text;
using Microsoft.wmi.Common;

namespace Microsoft.wmi.Rust
{
    public class RustWmiProperty : WmiProperty
    {
        public RustWmiProperty(PropertyData pData, RustWmiClass wClass) : base(pData, wClass)
        {
            HeaderComment = RustWmiSource.GetHeaderCommentText(pData.Qualifiers);
            Setter = GetPropertySetter(pData);
            Getter = GetPropertyGetter(pData);
            Name = pData.Name;
        }

        public override string GetSourceCode()
        {
            StringBuilder sb = new StringBuilder();
            sb.AppendLine(HeaderComment);

            string fieldName = ToSnakeCase(Name);
            string rustType = GetRustFieldType();

            // Add serde rename to preserve original WMI property name
            sb.AppendFormat(CultureInfo.InvariantCulture,
                "    #[serde(rename = \"{0}\")]\n", Name);
            sb.AppendFormat(CultureInfo.InvariantCulture,
                "    pub {0}: {1},", fieldName, rustType);
            return sb.ToString();
        }

        private string GetRustFieldType()
        {
            string baseType = Type;
            if (IsArray)
            {
                return $"Vec<{baseType}>";
            }
            // Make all fields Option to handle nullable WMI values
            return $"Option<{baseType}>";
        }

        public string GetDefaultValue()
        {
            if (IsArray)
            {
                return "Vec::new()";
            }
            return "None";
        }

        public override string ToString()
        {
            return string.Format(CultureInfo.InvariantCulture,
                "WmiProperty [Name:{0}] [Type:{1}]", Name, Type);
        }

        private string GetPropertySetter(PropertyData pData)
        {
            string fieldName = ToSnakeCase(pData.Name);
            string rustType = IsArray ? $"Vec<{Type}>" : Type;
            string paramType = IsArray ? $"Vec<{Type}>" : $"{Type}";

            return String.Format(CultureInfo.InvariantCulture,
@"
    /// Sets the value of {1}
    pub fn set_{2}(&mut self, value: {3}) {{
        self.{2} = {4};
    }}", Type, pData.Name, fieldName, paramType, IsArray ? "value" : "Some(value)");
        }

        private string GetPropertyGetter(PropertyData pData)
        {
            string fieldName = ToSnakeCase(pData.Name);
            string returnType = IsArray ? $"&Vec<{Type}>" : $"Option<&{Type}>";

            return String.Format(CultureInfo.InvariantCulture,
@"
    /// Gets the value of {1}
    pub fn get_{2}(&self) -> {3} {{
        {4}
    }}", Type, pData.Name, fieldName, returnType,
                IsArray ? $"&self.{fieldName}" : $"self.{fieldName}.as_ref()");
        }

        private static string ToSnakeCase(string name)
        {
            if (string.IsNullOrEmpty(name))
                return name;

            var result = new StringBuilder();
            for (int i = 0; i < name.Length; i++)
            {
                char c = name[i];
                if (char.IsUpper(c))
                {
                    if (i > 0 && !char.IsUpper(name[i - 1]))
                        result.Append('_');
                    result.Append(char.ToLowerInvariant(c));
                }
                else
                {
                    result.Append(c);
                }
            }
            return result.ToString().ToLowerInvariant();
        }
    }
}
