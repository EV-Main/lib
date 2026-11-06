import * as React from "react";
import { cn } from "../../lib/cn";
import { buttonVariants, type ButtonSize } from "./button";
import * as copy from "../../generated/copy";
import { PAGINATION_CONTENT, PAGINATION_ELLIPSIS, PAGINATION_LINK_LABEL, PAGINATION_NEXT_LINK, PAGINATION_PREVIOUS_LINK, PAGINATION_ROOT } from "../../generated/pagination";
import { iconShapes } from "../../icons/icon-shapes";

export function Pagination({ className, ...props }: React.ComponentProps<"nav">) {
  return (
    <nav
      role="navigation"
      aria-label={copy.PAGINATION}
      data-slot="pagination"
      className={cn(PAGINATION_ROOT, className)}
      {...props}
    />
  );
}

export function PaginationContent({
  className,
  ...props
}: React.ComponentProps<"ul">) {
  return (
    <ul
      data-slot="pagination-content"
      className={cn(PAGINATION_CONTENT, className)}
      {...props}
    />
  );
}

export function PaginationItem(props: React.ComponentProps<"li">) {
  return <li data-slot="pagination-item" {...props} />;
}

export type PaginationLinkProps = {
  isActive?: boolean;
  size?: ButtonSize;
  /** Page numbers are square glyphs; a "Previous"/"Next" label is not. */
  icon?: boolean;
} & React.ComponentProps<"a">;

export function PaginationLink({
  className,
  isActive,
  size = "md",
  icon = true,
  ...props
}: PaginationLinkProps) {
  return (
    <a
      aria-current={isActive ? "page" : undefined}
      data-slot="pagination-link"
      data-active={isActive}
      className={cn(
        buttonVariants({
          variant: isActive ? "outline" : "ghost",
          size,
          icon,
        }),
        className,
      )}
      {...props}
    />
  );
}

const ChevronLeft = () => (
  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2}>
    {iconShapes("chevron-left")}
  </svg>
);

const ChevronRight = () => (
  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2}>
    {iconShapes("chevron-right")}
  </svg>
);

const MoreHorizontal = () => (
  <svg
    className="size-4"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth={2}
  >
    {iconShapes("ellipsis")}
  </svg>
);

export function PaginationPrevious({
  className,
  ...props
}: React.ComponentProps<typeof PaginationLink>) {
  return (
    <PaginationLink
      aria-label={copy.PAGINATION_PREVIOUS_PAGE}
      size="md"
      className={cn(PAGINATION_PREVIOUS_LINK, className)}
      {...props}
    >
      <ChevronLeft />
      <span className={PAGINATION_LINK_LABEL}>{copy.PAGINATION_PREVIOUS}</span>
    </PaginationLink>
  );
}

export function PaginationNext({
  className,
  ...props
}: React.ComponentProps<typeof PaginationLink>) {
  return (
    <PaginationLink
      aria-label={copy.PAGINATION_NEXT_PAGE}
      size="md"
      className={cn(PAGINATION_NEXT_LINK, className)}
      {...props}
    >
      <span className={PAGINATION_LINK_LABEL}>{copy.PAGINATION_NEXT}</span>
      <ChevronRight />
    </PaginationLink>
  );
}

export function PaginationEllipsis({
  className,
  ...props
}: React.ComponentProps<"span">) {
  return (
    <span
      aria-hidden
      data-slot="pagination-ellipsis"
      className={cn(PAGINATION_ELLIPSIS, className)}
      {...props}
    >
      <MoreHorizontal />
      <span className="sr-only">{copy.PAGINATION_MORE}</span>
    </span>
  );
}
