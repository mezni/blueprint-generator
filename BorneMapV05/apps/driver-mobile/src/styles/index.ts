import { colors } from './theme/tokens';
import { spacing } from './theme/tokens';
import { shadows } from './theme/tokens';
import { borderRadius } from './theme/tokens';
import { fontFamily } from './theme/tokens';

// CSS Custom Properties for Design Tokens
const generateCssVars = () => {
  const allVars = {
    // Colors
    ...Object.fromEntries(
      Object.entries(colors).map(([key, value]) => [`--color-${key}`, value.base])
    ),
    ...Object.fromEntries(
      Object.entries(colors).map(([key, value]) => [`--color-${key}-hover`, value.hover])
    ),
    ...Object.fromEntries(
      Object.entries(colors).map(([key, value]) => [`--color-${key}-active`, value.active])
    ),
    ...Object.fromEntries(
      Object.entries(colors).map(([key, value]) => [`--color-${key}-muted`, value.muted])
    ),
    
    // Spacing
    ...Object.fromEntries(
      Object.entries(spacing).map(([key, value]) => [`--spacing-${key}`, value])
    ),
    
    // Font Family
    ...Object.fromEntries(
      Object.entries(fontFamily).map(([key, value]) => [`--font-${key}`, value])
    ),
    
    // Shadows
    ...Object.fromEntries(
      Object.entries(shadows).map(([key, value]) => [`--shadow-${key}`, value])
    ),
    
    // Border Radius
    ...Object.fromEntries(
      Object.entries(borderRadius).map(([key, value]) => [`--radius-${key}`, value])
    ),
  };

  return Object.entries(allVars)
    .map(([name, value]) => `  --${name}: ${value};`)
    .join('\n');
};

export const appStyles = {
  root: {
    fontFamily: fontFamily.sans,
    color: colors.text.base,
    backgroundColor: colors.surface.base,
  },
  body: {
    fontFamily: fontFamily.sans,
    color: colors.text.base,
    backgroundColor: colors.surface.base,
    ...spacing, // Add spacing utilities
  },
  text: {
    base: colors.text.base,
    muted: colors.text.muted,
    hover: colors.text.hover,
    active: colors.text.active,
  },
  surface: {
    base: colors.surface.base,
    hover: colors.surface.hover,
    active: colors.surface.active,
    muted: colors.surface.muted,
  },
  border: {
    base: colors.border.base,
    hover: colors.border.hover,
    active: colors.border.active,
    muted: colors.border.muted,
  },
  primary: {
    base: colors.primary.base,
    hover: colors.primary.hover,
    active: colors.primary.active,
    muted: colors.primary.muted,
  },
  success: {
    base: colors.success.base,
    hover: colors.success.hover,
    active: colors.success.active,
    muted: colors.success.muted,
  },
  error: {
    base: colors.error.base,
    hover: colors.error.hover,
    active: colors.error.active,
    muted: colors.error.muted,
  },
};

export const generateStyles = () => {
  const cssVars = generateCssVars();
  return `:root {\n${cssVars}\n}`;
};
