// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

using System;
using System.Globalization;
using System.Management;
using System.Text;
using Microsoft.wmi.Common;

namespace Microsoft.wmi.Rust
{
    public class RustWmiMethodParam : WmiMethodParam
    {
        public RustWmiMethodParam(WmiMethod wMethod, string sourceCode) : base(wMethod)
        {
            SourceCode = sourceCode;
        }

        public RustWmiMethodParam(PropertyData pData, ParamType pType, RustWmiMethod wMethod, bool optional) : base(pData, pType, wMethod, optional)
        {
            string rustType = ConvertToRustType(Type);
            string paramName = ToSnakeCase(WmiMethod.FixName(Name));

            if (IsArray)
            {
                rustType = $"Vec<{rustType}>";
            }

            // For optional parameters, wrap in Option
            if (optional)
            {
                rustType = $"Option<{rustType}>";
            }

            // For output parameters, use mutable reference
            if (pType == ParamType.Output || pType == ParamType.Reference)
            {
                SourceCode = string.Format(CultureInfo.InvariantCulture,
                    "{0}: &mut {1}",
                    paramName,
                    rustType);
            }
            else
            {
                // For input parameters that are large types, take by reference
                if (ShouldTakeByReference(rustType))
                {
                    SourceCode = string.Format(CultureInfo.InvariantCulture,
                        "{0}: &{1}",
                        paramName,
                        rustType);
                }
                else
                {
                    SourceCode = string.Format(CultureInfo.InvariantCulture,
                        "{0}: {1}",
                        paramName,
                        rustType);
                }
            }
        }

        public RustWmiMethodParam(string Comment, ParamType pType, string Type, string Name, bool custom)
            : base(Comment, pType, Type, Name, custom)
        {
            string rustType = ConvertToRustType(Type);
            string paramName = ToSnakeCase(WmiMethod.FixName(Name));

            if (pType == ParamType.Output)
            {
                SourceCode = string.Format(CultureInfo.InvariantCulture,
                    "{0}: &mut {1}",
                    paramName,
                    rustType);
            }
            else
            {
                SourceCode = string.Format(CultureInfo.InvariantCulture,
                    "{0}: {1}",
                    paramName,
                    rustType);
            }
        }

        public string SourceCode { get; set; }

        public override string GetSourceCode()
        {
            return SourceCode;
        }

        private string ConvertToRustType(string type)
        {
            if (string.IsNullOrEmpty(type))
                return "serde_json::Value";

            switch (type.ToLowerInvariant())
            {
                case "boolean":
                case "bool":
                    return "bool";
                case "string":
                    return "String";
                case "sbyte":
                case "sint8":
                    return "i8";
                case "byte":
                case "uint8":
                    return "u8";
                case "int16":
                case "sint16":
                    return "i16";
                case "uint16":
                    return "u16";
                case "int32":
                case "sint32":
                    return "i32";
                case "uint32":
                    return "u32";
                case "int64":
                case "sint64":
                    return "i64";
                case "uint64":
                    return "u64";
                case "single":
                case "real32":
                    return "f32";
                case "double":
                case "real64":
                    return "f64";
                case "char":
                    return "char";
                case "datetime":
                    return "String";
                case "object":
                    return "serde_json::Value";
                default:
                    return type;
            }
        }

        private bool ShouldTakeByReference(string rustType)
        {
            // Take String and Vec types by reference
            return rustType.StartsWith("String") ||
                   rustType.StartsWith("Vec<") ||
                   rustType.StartsWith("Option<String") ||
                   rustType.StartsWith("Option<Vec<");
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
