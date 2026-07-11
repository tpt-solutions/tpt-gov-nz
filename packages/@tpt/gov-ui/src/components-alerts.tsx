import type { ButtonHTMLAttributes, ReactNode } from "react";
import { cn } from "./cn";

export type ButtonVariant = "primary" | "ghost" | "danger";
export type ButtonSize = "default" | "small";

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  children: ReactNode;
  variant?: ButtonVariant;
  size?: ButtonSize;
  block?: boolean;
}

/** Primary action button shared across portals. */
export function Button({
  children,
  variant = "primary",
  size = "default",
  block,
  className,
  type = "button",
  ...rest
}: ButtonProps) {
  return (
    <button
      type={type}
      className={cn(
        "govui-btn",
        variant === "ghost" && "govui-btn--ghost",
        variant === "danger" && "govui-btn--danger",
        size === "small" && "govui-btn--small",
        block && "govui-btn--block",
        className,
      )}
      {...rest}
    >
      {children}
    </button>
  );
}

export type AlertTone = "info" | "warn" | "danger" | "success";

export interface AlertProps {
  tone?: AlertTone;
  children: ReactNode;
  role?: "alert" | "status";
  className?: string;
}

/** Inline status / notice banner. */
export function Alert({ tone = "info", children, role = "alert", className }: AlertProps) {
  return (
    <div
      className={cn("govui-alert", `govui-alert--${tone}`, className)}
      role={role}
    >
      {children}
    </div>
  );
}

export type BadgeTone = "default" | "accent" | "warn" | "danger";

export interface BadgeProps {
  tone?: BadgeTone;
  children: ReactNode;
  className?: string;
}

/** Small status pill. */
export function Badge({ tone = "default", children, className }: BadgeProps) {
  return (
    <span className={cn("govui-badge", tone !== "default" && `govui-badge--${tone}`, className)}>
      {children}
    </span>
  );
}
