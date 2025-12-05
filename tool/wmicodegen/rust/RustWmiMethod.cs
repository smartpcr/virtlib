// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

using System;
using System.Globalization;
using System.Management;
using System.Text;
using Microsoft.wmi.Common;

namespace Microsoft.wmi.Rust
{
    public class RustWmiMethod : WmiMethod
    {
        public RustWmiMethod() { }

        public RustWmiMethod(MethodData mData, RustWmiClass wClass)
            : base(mData, wClass)
        {
            HeaderComments.Add(RustWmiSource.GetHeaderCommentText(mData.Qualifiers));
            if (mData.InParameters != null)
                HeaderComments.Add(GetWmiMethodComment(mData.InParameters.Properties));
            if (mData.OutParameters != null)
                HeaderComments.Add(GetWmiMethodComment(mData.OutParameters.Properties));
        }

        public override string GetSourceCode()
        {
            StringBuilder sb = new StringBuilder();

            // Add documentation comments
            foreach (var item in HeaderComments)
            {
                sb.AppendLine(item);
            }

            string methodName = ToSnakeCase(FixName(Name));
            string rustReturnType = ConvertToRustReturnType(ReturnType);

            // Method signature
            sb.AppendFormat(CultureInfo.InvariantCulture, "    pub fn {0}(&self", methodName);

            // Add parameters
            if (Params.Count > 0)
            {
                foreach (var item in Params)
                {
                    sb.AppendFormat(CultureInfo.InvariantCulture, ", {0}", item.GetSourceCode());
                }
            }

            sb.AppendFormat(") -> Result<{0}, WmiError> {{\n", rustReturnType);
            sb.AppendLine(BodyText);
            sb.AppendLine("    }");

            return sb.ToString();
        }

        private string ConvertToRustReturnType(string returnType)
        {
            if (string.IsNullOrEmpty(returnType) || returnType.Equals("void", StringComparison.OrdinalIgnoreCase))
            {
                return "()";
            }

            // Convert common types
            switch (returnType.ToLowerInvariant())
            {
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
                case "boolean":
                case "bool":
                    return "bool";
                case "string":
                    return "String";
                default:
                    return returnType;
            }
        }

        private string GetWmiMethodComment(PropertyDataCollection collection)
        {
            StringBuilder sbComment = new StringBuilder();
            if (collection != null)
            {
                Type tmpType;
                foreach (var pData in collection)
                {
                    sbComment.AppendFormat(CultureInfo.InvariantCulture,
                            "\n    /// * `{0}` - {1} ({2}{3})",
                            ToSnakeCase(pData.Name),
                            IFormat.GetDescription(pData.Qualifiers).Replace("\n", " "),
                            Parent.Parent.GetType(pData, out tmpType),
                            pData.IsArray ? "[]" : "");
                }
            }
            return sbComment.ToString();
        }

        protected override WmiMethodParam GetWmiMethodParam(PropertyData pData, ParamType pType, WmiMethod wMethod, bool optional)
        {
            return new RustWmiMethodParam(pData, pType, wMethod as RustWmiMethod, optional);
        }

        protected override WmiMethodParam GetWmiMethodParam(string comment, ParamType pType, string Type, string Name, bool custom)
        {
            return new RustWmiMethodParam(comment, pType, Type, Name, custom);
        }

        protected override string GetMethodBodyText(MethodData mData)
        {
            StringBuilder sb = new StringBuilder();

            // Build method invocation arguments
            if (mData.InParameters != null)
            {
                sb.AppendLine("        let mut args = Vec::new();");

                foreach (var item in Params.FindAll(p => p.ParamterType == ParamType.Input))
                {
                    string paramName = ToSnakeCase(WmiMethod.FixName(item.Name));
                    if (item.Optional)
                    {
                        sb.AppendFormat(CultureInfo.InvariantCulture,
                            "        if let Some(val) = {0} {{\n", paramName);
                        sb.AppendFormat(CultureInfo.InvariantCulture,
                            "            args.push(MethodParameter {{ name: \"{0}\".to_string(), value: val.into() }});\n",
                            item.Name);
                        sb.AppendLine("        }");
                    }
                    else
                    {
                        sb.AppendFormat(CultureInfo.InvariantCulture,
                            "        args.push(MethodParameter {{ name: \"{0}\".to_string(), value: {1}.into() }});\n",
                            item.Name, paramName);
                    }
                }
            }

            // Invoke method
            if (IFormat.HasOutParams(mData))
            {
                if (IFormat.HasJobOutputParams(mData))
                {
                    sb.AppendFormat(CultureInfo.InvariantCulture,
                        "\n        let result = self.invoke_method_with_job(\"{0}\", {1})?;\n",
                        mData.Name, mData.InParameters != null ? "&args" : "&[]");
                }
                else
                {
                    sb.AppendFormat(CultureInfo.InvariantCulture,
                        "\n        let result = self.invoke_method(\"{0}\", {1})?;\n",
                        mData.Name, mData.InParameters != null ? "&args" : "&[]");
                }

                // Extract output parameters
                Type tmpType;
                foreach (var pData in mData.OutParameters.Properties)
                {
                    if (!pData.Name.Equals("ReturnValue", StringComparison.OrdinalIgnoreCase))
                    {
                        string typeString = Parent.Parent.GetType(pData, out tmpType);
                        string paramName = ToSnakeCase(pData.Name);
                        sb.AppendFormat(CultureInfo.InvariantCulture,
                            "        let {0} = result.get_value(\"{1}\")?;\n",
                            paramName, pData.Name);
                    }
                }

                if (!ReturnType.Equals("void", StringComparison.OrdinalIgnoreCase))
                {
                    sb.AppendLine("        Ok(result.return_value)");
                }
                else
                {
                    sb.AppendLine("        Ok(())");
                }
            }
            else
            {
                sb.AppendFormat(CultureInfo.InvariantCulture,
                    "        self.invoke_method(\"{0}\", {1})\n",
                    mData.Name, mData.InParameters != null ? "&args" : "&[]");
            }

            return sb.ToString();
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
