// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

using System.Globalization;
using System.IO;
using System.Text;
using Microsoft.wmi.Common;

namespace Microsoft.wmi.Rust
{
    public class RustWmiEnum : WmiEnum
    {
        public RustWmiEnum(string enumName, RustWmiSource wSource) : base(enumName, wSource)
        {
            CopyrightText = string.Format(CultureInfo.InvariantCulture,
@"// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on {1} using wmigen
//      Source {0}
//////////////////////////////////////////////",
                    Name,
                    System.DateTime.Now.ToShortDateString()
                    );
        }

        public override string GetSourceCode()
        {
            StringBuilder sbEnum = new StringBuilder();
            sbEnum.AppendFormat(CultureInfo.InvariantCulture, "\n/// {0} enumeration\n", Name);
            sbEnum.AppendLine("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]");
            sbEnum.AppendLine("#[repr(i32)]");
            sbEnum.AppendFormat(CultureInfo.InvariantCulture, "pub enum {0} {{\n", Name);

            foreach (var item in EnumValues)
            {
                string variantName = SanitizeEnumVariant(item.Key);
                sbEnum.AppendFormat(CultureInfo.InvariantCulture, "    /// {0}\n", item.Key);
                sbEnum.AppendFormat(CultureInfo.InvariantCulture, "    #[serde(rename = \"{0}\")]\n", item.Key);
                sbEnum.AppendFormat(CultureInfo.InvariantCulture, "    {0} = {1},\n",
                    variantName, item.Value);
            }

            sbEnum.AppendLine("}");

            // Add Default implementation
            sbEnum.AppendLine();
            sbEnum.AppendFormat(CultureInfo.InvariantCulture, "impl Default for {0} {{\n", Name);
            sbEnum.AppendLine("    fn default() -> Self {");
            if (EnumValues.Count > 0)
            {
                var firstValue = EnumValues.GetEnumerator();
                firstValue.MoveNext();
                sbEnum.AppendFormat(CultureInfo.InvariantCulture, "        Self::{0}\n",
                    SanitizeEnumVariant(firstValue.Current.Key));
            }
            else
            {
                sbEnum.AppendLine("        Self::Unknown");
            }
            sbEnum.AppendLine("    }");
            sbEnum.AppendLine("}");

            return sbEnum.ToString();
        }

        public override void GenerateSource(string outdir)
        {
            string fileName = ToSnakeCase(Name) + ".rs";
            string path = Path.Combine(outdir, fileName);

            StringBuilder sbEnum = new StringBuilder();
            sbEnum.AppendLine(CopyrightText);
            sbEnum.AppendLine(GetSourceCode());
            File.WriteAllText(path, sbEnum.ToString());
            Logger.Info("Enum {0}", path);
        }

        private string SanitizeEnumVariant(string name)
        {
            if (string.IsNullOrEmpty(name))
                return "Unknown";

            // Remove invalid characters and convert to PascalCase
            var result = new StringBuilder();
            bool capitalizeNext = true;

            foreach (char c in name)
            {
                if (char.IsLetterOrDigit(c))
                {
                    if (capitalizeNext)
                    {
                        result.Append(char.ToUpperInvariant(c));
                        capitalizeNext = false;
                    }
                    else
                    {
                        result.Append(c);
                    }
                }
                else if (c == ' ' || c == '_' || c == '-')
                {
                    capitalizeNext = true;
                }
            }

            string variantName = result.ToString();

            // Rust enum variants cannot start with a digit
            if (variantName.Length > 0 && char.IsDigit(variantName[0]))
            {
                variantName = "V" + variantName;
            }

            // Handle reserved keywords
            if (IsRustKeyword(variantName))
            {
                variantName = variantName + "Value";
            }

            return string.IsNullOrEmpty(variantName) ? "Unknown" : variantName;
        }

        private bool IsRustKeyword(string name)
        {
            string[] keywords = new string[]
            {
                "as", "break", "const", "continue", "crate", "else", "enum", "extern",
                "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
                "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct",
                "super", "trait", "true", "type", "unsafe", "use", "where", "while",
                "async", "await", "dyn", "abstract", "become", "box", "do", "final",
                "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try"
            };

            foreach (var keyword in keywords)
            {
                if (name.Equals(keyword, System.StringComparison.OrdinalIgnoreCase))
                    return true;
            }
            return false;
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
