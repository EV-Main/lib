import type { ComponentProps } from "react";

const glyph = (d: string) =>
  function Glyph({ className = "size-4", ...props }: ComponentProps<"svg">) {
    return (
      <svg
        className={className}
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth={1.75}
        strokeLinecap="round"
        strokeLinejoin="round"
        aria-hidden
        {...props}
      >
        <path d={d} />
      </svg>
    );
  };

export const PlusIcon = glyph("M12 5v14M5 12h14");
export const SearchIcon = glyph("M21 21l-4.3-4.3M17 11a6 6 0 1 1-12 0 6 6 0 0 1 12 0z");
export const UserIcon = glyph("M20 21a8 8 0 0 0-16 0M12 13a5 5 0 1 0 0-10 5 5 0 0 0 0 10z");
export const BellIcon = glyph("M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9M10 21h4");
export const CogIcon = glyph("M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6zM19 12a7 7 0 0 0-.1-1.2l2-1.5-2-3.4-2.3 1a7 7 0 0 0-2-1.2L14.2 3h-4l-.4 2.7a7 7 0 0 0-2 1.2l-2.3-1-2 3.4 2 1.5A7 7 0 0 0 5 12c0 .4 0 .8.1 1.2l-2 1.5 2 3.4 2.3-1a7 7 0 0 0 2 1.2l.4 2.7h4l.4-2.7a7 7 0 0 0 2-1.2l2.3 1 2-3.4-2-1.5c.1-.4.1-.8.1-1.2z");
export const TrashIcon = glyph("M4 7h16M10 11v6M14 11v6M6 7l1 13h10l1-13M9 7V4h6v3");
export const CopyIcon = glyph("M9 9h11v11H9zM5 15V4h11");
export const CheckIcon = glyph("M5 12.5l4.5 4.5L19 7.5");
export const CloseIcon = glyph("M6 6l12 12M18 6L6 18");
export const ChevronDownIcon = glyph("M6 9l6 6 6-6");
export const HomeIcon = glyph("M3 10.5 12 3l9 7.5V21H3z");
export const ChartIcon = glyph("M3 3v18h18M7 15l4-4 3 3 5-6");
export const WalletIcon = glyph("M3 7h18v12H3zM16 13h2");
export const LayersIcon = glyph("M12 3l9 5-9 5-9-5 9-5zM3 13l9 5 9-5");
export const ExternalIcon = glyph("M14 4h6v6M20 4l-9 9M18 14v6H4V6h6");
export const BoldIcon = glyph("M7 5h6a3.5 3.5 0 0 1 0 7H7zM7 12h7a3.5 3.5 0 0 1 0 7H7z");
export const ItalicIcon = glyph("M10 5h8M6 19h8M14 5l-4 14");
