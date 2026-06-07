import * as React from "react";
import { cva, type VariantProps } from "class-variance-authority";
import { cn } from "@/lib/utils";

const buttonVariants = cva(
  "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-[var(--color-primary-base)] disabled:pointer-events-none disabled:opacity-50",
  {
    variants: {
      variant: {
        primary:
          "bg-[var(--color-primary-base)] text-white hover:bg-[var(--color-primary-hover)] active:bg-[var(--color-primary-active)]",
        secondary:
          "bg-[var(--color-secondary-base)] text-white hover:bg-[var(--color-secondary-hover)] active:bg-[var(--color-secondary-active)]",
        outline:
          "border border-[var(--color-border-base)] bg-transparent text-[var(--color-text-base)] hover:bg-[var(--color-surface-hover)] active:bg-[var(--color-surface-active)]",
        ghost:
          "bg-transparent text-[var(--color-text-base)] hover:bg-[var(--color-surface-hover)] active:bg-[var(--color-surface-active)]",
      },
      size: {
        sm: "h-9 ps-3 pe-3 py-1.5 text-sm",
        md: "h-10 ps-4 pe-4 py-2 text-base",
        lg: "h-11 ps-6 pe-6 py-3 text-lg",
      },
    },
    defaultVariants: {
      variant: "primary",
      size: "md",
    },
  },
);

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonVariants> {}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant, size, ...props }, ref) => {
    return (
      <button
        className={cn(buttonVariants({ variant, size, className }))}
        ref={ref}
        {...props}
      />
    );
  },
);
Button.displayName = "Button";

export { Button, buttonVariants };
