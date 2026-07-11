import type { HTMLAttributes, ReactNode } from "react";
import { cn } from "./cn";

export interface CardProps extends HTMLAttributes<HTMLDivElement> {
  children: ReactNode;
  as?: "div" | "section" | "article";
}

/** Surface container with border, radius, and subtle shadow. */
export function Card({ children, as = "div", className, ...rest }: CardProps) {
  const Tag = as as "div";
  return (
    <Tag className={cn("govui-card", className)} {...rest}>
      {children}
    </Tag>
  );
}

export interface ContainerProps extends HTMLAttributes<HTMLDivElement> {
  children: ReactNode;
}

/** Centered, max-width content wrapper. */
export function Container({ children, className, ...rest }: ContainerProps) {
  return (
    <div className={cn("govui-container", className)} {...rest}>
      {children}
    </div>
  );
}

export interface StackProps extends HTMLAttributes<HTMLDivElement> {
  children: ReactNode;
  tight?: boolean;
}

/** Vertical flex layout with consistent spacing. */
export function Stack({ children, tight, className, ...rest }: StackProps) {
  return (
    <div className={cn("govui-stack", tight && "govui-stack--tight", className)} {...rest}>
      {children}
    </div>
  );
}

export interface GridProps extends HTMLAttributes<HTMLDivElement> {
  children: ReactNode;
}

/** Responsive auto-fit grid. */
export function Grid({ children, className, ...rest }: GridProps) {
  return (
    <div className={cn("govui-grid", className)} {...rest}>
      {children}
    </div>
  );
}

export interface RowProps extends HTMLAttributes<HTMLDivElement> {
  children: ReactNode;
  between?: boolean;
}

/** Horizontal flex layout. */
export function Row({ children, between, className, ...rest }: RowProps) {
  return (
    <div className={cn("govui-row", between && "govui-row--between", className)} {...rest}>
      {children}
    </div>
  );
}
