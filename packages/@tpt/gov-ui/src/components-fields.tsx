import type { InputHTMLAttributes, LabelHTMLAttributes, ReactNode, SelectHTMLAttributes, TextareaHTMLAttributes } from "react";
import { cn } from "./cn";

export interface FieldProps {
  label: ReactNode;
  htmlFor?: string;
  hint?: ReactNode;
  className?: string;
  children: ReactNode;
}

/** Labelled form control wrapper. */
export function Field({ label, htmlFor, hint, className, children }: FieldProps) {
  return (
    <div className={cn("govui-field", className)}>
      <label htmlFor={htmlFor}>{label}</label>
      {children}
      {hint ? <p className="govui-sr-only">{hint}</p> : null}
    </div>
  );
}

export interface TextFieldProps extends InputHTMLAttributes<HTMLInputElement> {
  label: ReactNode;
  hint?: ReactNode;
}

/** Labelled text input. */
export function TextField({ label, hint, id, className, ...rest }: TextFieldProps) {
  return (
    <Field label={label} hint={hint} {...(id ? { htmlFor: id } : {})}>
      <input id={id} className={className} {...rest} />
    </Field>
  );
}

export interface SelectFieldProps extends SelectHTMLAttributes<HTMLSelectElement> {
  label: ReactNode;
  hint?: ReactNode;
  children: ReactNode;
}

/** Labelled select. */
export function SelectField({ label, hint, id, className, children, ...rest }: SelectFieldProps) {
  return (
    <Field label={label} hint={hint} {...(id ? { htmlFor: id } : {})}>
      <select id={id} className={className} {...rest}>
        {children}
      </select>
    </Field>
  );
}

export interface TextAreaFieldProps extends TextareaHTMLAttributes<HTMLTextAreaElement> {
  label: ReactNode;
  hint?: ReactNode;
}

/** Labelled textarea. */
export function TextAreaField({ label, hint, id, className, ...rest }: TextAreaFieldProps) {
  return (
    <Field label={label} hint={hint} {...(id ? { htmlFor: id } : {})}>
      <textarea id={id} className={className} {...rest} />
    </Field>
  );
}

export interface DefinitionListProps {
  items: { term: ReactNode; value: ReactNode }[];
  className?: string;
}

/** Two-column term/definition list for displaying record details. */
export function DefinitionList({ items, className }: DefinitionListProps) {
  return (
    <dl className={cn("govui-dl", className)}>
      {items.map((item, i) => (
        <div key={i} style={{ display: "contents" }}>
          <dt>{item.term}</dt>
          <dd>{item.value}</dd>
        </div>
      ))}
    </dl>
  );
}

export type { LabelHTMLAttributes };
