import * as React from "react";
import { type LucideIcon } from "lucide-react";
import { isDev } from "@/lib/is-dev";

import {
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";
import { BugReportDialog } from "@/components/bug-report-dialog";

export function NavSecondary({
  items,
  ...props
}: {
  items: {
    title: string;
    url: string;
    icon: LucideIcon;
  }[];
} & React.ComponentPropsWithoutRef<typeof SidebarGroup>) {
  return (
    <SidebarGroup {...props}>
      <SidebarGroupContent>
        <SidebarMenu>
          {isDev && <DevTools />}
          <BugReportDialog />
          {items.map((item) => (
            <SidebarMenuItem key={item.title}>
              <SidebarMenuButton
                asChild
                size="sm"
                className="text-muted-foreground hover:text-foreground"
              >
                <a href={item.url}>
                  <item.icon />
                  <span>{item.title}</span>
                </a>
              </SidebarMenuButton>
            </SidebarMenuItem>
          ))}
          <SidebarMenuItem></SidebarMenuItem>
        </SidebarMenu>
      </SidebarGroupContent>
    </SidebarGroup>
  );
}

import { axiosInstance } from "@/api/axiosInstance";
import { DatabaseZap, DatabaseBackup } from "lucide-react";

function DevTools() {
  return (
    <>
      <SidebarMenuItem>
        <SidebarMenuButton
          size="sm"
          className="text-muted-foreground hover:text-foreground"
          onClick={() => axiosInstance.get("/create-dummy-exhibits")}
        >
          <DatabaseBackup />
          Fill DB
        </SidebarMenuButton>
      </SidebarMenuItem>
      <SidebarMenuItem>
        <SidebarMenuButton
          size="sm"
          className="text-muted-foreground hover:text-foreground"
          onClick={() => axiosInstance.get("/reset")}
        >
          <DatabaseZap />
          Flush DB
        </SidebarMenuButton>
      </SidebarMenuItem>
    </>
  );
}
