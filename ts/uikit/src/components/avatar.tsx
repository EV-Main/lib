"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import {
  AVATAR,
  AVATAR_IMAGE,
  AVATAR_FALLBACK,
  avatarShapeClasses,
  avatarSizeClasses,
  type AvatarShape,
  type AvatarSize,
} from "../generated/avatar";

export type { AvatarShape, AvatarSize };

export interface AvatarProps extends React.ComponentProps<"div"> {
  /** `circle` is a true circle in every theme; `square` takes the chip radius. */
  shape?: AvatarShape;
  /** 20, 28 or 56 px; the fallback initial is typeset to match. */
  size?: AvatarSize;
}

export function Avatar({ className, shape = "circle", size = "md", ...props }: AvatarProps) {
  return (
    <div
      data-slot="avatar"
      data-size={size}
      className={cn(AVATAR, avatarShapeClasses[shape], avatarSizeClasses[size], className)}
      {...props}
    />
  );
}

export function AvatarImage({
  className,
  onError,
  ...props
}: React.ComponentProps<"img">) {
  const [errored, setErrored] = React.useState(false);
  if (errored) return null;
  return (
    <img
      data-slot="avatar-image"
      className={cn(AVATAR_IMAGE, className)}
      onError={(e) => {
        onError?.(e);
        setErrored(true);
      }}
      {...props}
    />
  );
}

export function AvatarFallback({
  className,
  ...props
}: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="avatar-fallback"
      className={cn(AVATAR_FALLBACK, className)}
      {...props}
    />
  );
}
