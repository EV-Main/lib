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
export const LayoutIcon = glyph("M3 4h18v16H3zM9 4v16M9 12h12");
export const MessageIcon = glyph("M21 12a8 8 0 0 1-11.6 7.1L4 20l1-4.8A8 8 0 1 1 21 12z");
export const CameraIcon = glyph("M4 8h3l2-3h6l2 3h3v11H4zM12 17a4 4 0 1 0 0-8 4 4 0 0 0 0 8z");
export const EyeIcon = glyph("M2 12s4-7 10-7 10 7 10 7-4 7-10 7S2 12 2 12zM12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z");
export const SidebarIcon = glyph("M3 4h18v16H3zM15 4v16");
export const StarIcon = glyph("M12 3l2.7 5.6 6.1.9-4.4 4.3 1 6.1L12 17l-5.4 2.9 1-6.1L3.2 9.5l6.1-.9z");
export const HistoryIcon = glyph("M3 12a9 9 0 1 0 3-6.7L3 8M3 3v5h5M12 7v5l3 2");
export const BookmarkIcon = glyph("M6 3h12v18l-6-4-6 4z");
export const ChevronLeftIcon = glyph("M15 6l-6 6 6 6");
export const ChevronRightIcon = glyph("M9 6l6 6-6 6");
export const ChevronUpIcon = glyph("M6 15l6-6 6 6");
export const BotIcon = glyph("M12 3v3M5 8h14v10H5zM9 13h.01M15 13h.01M3 13v2M21 13v2");
export const SlidersIcon = glyph("M4 6h10M18 6h2M4 12h4M12 12h8M4 18h12M20 18h0M14 4v4M8 10v4M16 16v4");
export const SwapIcon = glyph("M7 4L3 8l4 4M3 8h14M17 20l4-4-4-4M21 16H7");
export const LinkIcon = glyph("M10 14a4 4 0 0 0 5.7 0l3-3a4 4 0 0 0-5.7-5.7l-1 1M14 10a4 4 0 0 0-5.7 0l-3 3a4 4 0 0 0 5.7 5.7l1-1");
export const UsersIcon = glyph("M16 20v-1a4 4 0 0 0-8 0v1M12 12a4 4 0 1 0 0-8 4 4 0 0 0 0 8zM20 20v-1a4 4 0 0 0-3-3.9M17 4.1a4 4 0 0 1 0 7.8");
export const HelpIcon = glyph("M12 21a9 9 0 1 0 0-18 9 9 0 0 0 0 18zM9.5 9a2.5 2.5 0 1 1 3.5 2.3c-.6.3-1 .9-1 1.7M12 17h.01");
export const KeyboardIcon = glyph("M3 6h18v12H3zM7 10h.01M11 10h.01M15 10h.01M7 14h10");
export const GlobeIcon = glyph("M12 21a9 9 0 1 0 0-18 9 9 0 0 0 0 18zM3 12h18M12 3c3 3 3 15 0 18M12 3c-3 3-3 15 0 18");
export const FileTextIcon = glyph("M6 3h8l4 4v14H6zM14 3v4h4M9 13h6M9 17h6");
export const ShieldIcon = glyph("M12 3l8 3v6c0 5-3.5 8-8 9-4.5-1-8-4-8-9V6z");
export const KeyIcon = glyph("M14 10a4 4 0 1 1-3.4-4 4 4 0 0 1 3.4 4zM11 11l-8 8M6 16l2 2");
export const PencilIcon = glyph("M4 20h4L19 9l-4-4L4 16z");
export const UploadIcon = glyph("M12 16V4M7 9l5-5 5 5M4 20h16");
export const MoreIcon = glyph("M5 12h.01M12 12h.01M19 12h.01");
export const SquareIcon = glyph("M6 6h12v12H6z");
export const GridIcon = glyph("M4 4h7v7H4zM13 4h7v7h-7zM4 13h7v7H4zM13 13h7v7h-7z");
export const ActivityIcon = glyph("M3 12h4l3-8 4 16 3-8h4");
export const AlertIcon = glyph("M12 3l10 18H2zM12 10v5M12 18h.01");
export const LogOutIcon = glyph("M9 4H4v16h5M16 8l4 4-4 4M20 12H9");
export const MoonIcon = glyph("M20 14A8 8 0 1 1 10 4a6 6 0 0 0 10 10z");
export const TrendingUpIcon = glyph("M3 17l6-6 4 4 8-8M15 7h6v6");
export const CreditCardIcon = glyph("M3 6h18v12H3zM3 10h18");
