import { colors } from "./colors";
import { spacing } from "./spacing";
import { typography } from "./typography";
import { shadows } from "./shadows";
import { borderRadius } from "./border-radius";

type VariantEntry = Record<string, string>;
type FlatEntry = Record<string, string>;

function flattenVariants(prefix: string, obj: VariantEntry): FlatEntry {
  const result: FlatEntry = {};
  for (const [key, value] of Object.entries(obj)) {
    result[`${prefix}-${key}`] = value;
  }
  return result;
}

function flattenObject(prefix: string, obj: Record<string, string | VariantEntry>): FlatEntry {
  const result: FlatEntry = {};
  for (const [key, value] of Object.entries(obj)) {
    if (typeof value === "string") {
      result[`${prefix}-${key}`] = value;
    } else if (typeof value === "object" && value !== null) {
      const nested = flattenVariants(`${prefix}-${key}`, value as VariantEntry);
      Object.assign(result, nested);
    }
  }
  return result;
}

function prefixTokens(prefix: string, obj: Record<string, string>): FlatEntry {
  const result: FlatEntry = {};
  for (const [key, value] of Object.entries(obj)) {
    result[`${prefix}-${key}`] = value;
  }
  return result;
}

export function generateCssVars(): string {
  const allVars: FlatEntry = {
    ...flattenObject("color", colors),
    ...prefixTokens("spacing", spacing as unknown as Record<string, string>),
    ...prefixTokens("font-family", typography.fontFamily),
    ...prefixTokens("font-size", typography.fontSize),
    ...prefixTokens("font-weight", typography.fontWeight),
    ...prefixTokens("line-height", typography.lineHeight),
    ...prefixTokens("shadow", shadows as unknown as Record<string, string>),
    ...prefixTokens("radius", borderRadius as unknown as Record<string, string>),
  };

  const lines = [":root {"];
  for (const [name, value] of Object.entries(allVars)) {
    lines.push(`  --${name}: ${value};`);
  }
  lines.push("}");

  return lines.join("\n");
}
