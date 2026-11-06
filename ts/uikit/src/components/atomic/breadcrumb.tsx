import * as React from "react";
import { cn } from "../../lib/cn";
import { Slot } from "../../primitives/slot";
import {
  BREADCRUMB_ELLIPSIS,
  BREADCRUMB_ITEM,
  BREADCRUMB_LINK,
  BREADCRUMB_LIST,
  BREADCRUMB_PAGE,
  BREADCRUMB_SEPARATOR,
} from "../../generated/breadcrumb";
import * as copy from "../../generated/copy";
import { iconShapes } from "../../icons/icon-shapes";

export function Breadcrumb({ ...props }: React.ComponentProps<"nav">) {
  return <nav aria-label={copy.BREADCRUMB} data-slot="breadcrumb" {...props} />;
}

export function BreadcrumbList({
  className,
  ...props
}: React.ComponentProps<"ol">) {
  return (
    <ol
      data-slot="breadcrumb-list"
      className={cn(BREADCRUMB_LIST, className)}
      {...props}
    />
  );
}

export function BreadcrumbItem({
  className,
  ...props
}: React.ComponentProps<"li">) {
  return (
    <li
      data-slot="breadcrumb-item"
      className={cn(BREADCRUMB_ITEM, className)}
      {...props}
    />
  );
}

export interface BreadcrumbLinkProps extends React.ComponentProps<"a"> {
  asChild?: boolean;
}

export function BreadcrumbLink({
  asChild,
  className,
  ...props
}: BreadcrumbLinkProps) {
  const Comp = asChild ? Slot : "a";
  return (
    <Comp
      data-slot="breadcrumb-link"
      className={cn(BREADCRUMB_LINK, className)}
      {...(props as Record<string, unknown>)}
    />
  );
}

export function BreadcrumbPage({
  className,
  ...props
}: React.ComponentProps<"span">) {
  return (
    <span
      data-slot="breadcrumb-page"
      role="link"
      aria-disabled="true"
      aria-current="page"
      className={cn(BREADCRUMB_PAGE, className)}
      {...props}
    />
  );
}

export function BreadcrumbSeparator({
  children,
  className,
  ...props
}: React.ComponentProps<"li">) {
  return (
    <li
      data-slot="breadcrumb-separator"
      role="presentation"
      aria-hidden="true"
      className={cn(BREADCRUMB_SEPARATOR, className)}
      {...props}
    >
      {children ?? (
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2}>
          {iconShapes("chevron-right")}
        </svg>
      )}
    </li>
  );
}

export function BreadcrumbEllipsis({
  className,
  ...props
}: React.ComponentProps<"span">) {
  return (
    <span
      data-slot="breadcrumb-ellipsis"
      role="presentation"
      aria-hidden="true"
      className={cn(BREADCRUMB_ELLIPSIS, className)}
      {...props}
    >
      <svg
        className="size-4"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth={2}
      >
        {iconShapes("ellipsis")}
      </svg>
      <span className="sr-only">{copy.BREADCRUMB_MORE}</span>
    </span>
  );
}
