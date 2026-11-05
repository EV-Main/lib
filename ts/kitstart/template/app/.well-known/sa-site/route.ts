import { siteReportRoute } from "@evinvest/kitstart/next";
import { site } from "@/shared/config/site";

/** What the live sources answer now, for the conductor. */
export const dynamic = "force-dynamic";

export const GET = siteReportRoute(site);
