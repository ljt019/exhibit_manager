import { ChevronRight } from "lucide-react";

import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import {
  SidebarGroup,
  SidebarMenu,
  SidebarMenuAction,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarMenuSub,
  SidebarMenuSubButton,
  SidebarMenuSubItem,
} from "@/components/ui/sidebar";
import { useNavigate } from "react-router-dom";

import type { RouteConfig } from "@/routes";

interface NavMainProps {
  items: RouteConfig[];
}

export function NavMain({ items }: NavMainProps) {
  const navigate = useNavigate();

  const isCurrentRoute = (routePath: string) => {
    // Remove the leading '#' from the hash
    const currentRoute = window.location.hash.substring(1);
    return currentRoute === routePath;
  };

  return (
    <SidebarGroup>
      <SidebarMenu>
        {items.map((item) => (
          <div
            className={`${
              isCurrentRoute(item.url)
                ? "text-foreground"
                : "text-muted-foreground"
            }`}
          >
            {item.sidebar ? (
              <Collapsible key={item.title} asChild defaultOpen={item.isActive}>
                <SidebarMenuItem>
                  <SidebarMenuButton
                    asChild
                    tooltip={item.title}
                    className="hover:cursor-pointer"
                  >
                    <a onClick={() => navigate(item.url)}>
                      {item.icon && <item.icon />}
                      <span className="text-lg">{item.title}</span>
                    </a>
                  </SidebarMenuButton>
                  {item.items?.length ? (
                    <>
                      <CollapsibleTrigger asChild>
                        <SidebarMenuAction className="data-[state=open]:rotate-90">
                          <ChevronRight />
                          <span className="sr-only">Toggle</span>
                        </SidebarMenuAction>
                      </CollapsibleTrigger>
                      <CollapsibleContent>
                        <SidebarMenuSub>
                          {item.items?.map((subItem) => (
                            <SidebarMenuSubItem key={subItem.title}>
                              <SidebarMenuSubButton asChild>
                                <a onClick={() => navigate(subItem.url)}>
                                  <span>{subItem.title}</span>
                                </a>
                              </SidebarMenuSubButton>
                            </SidebarMenuSubItem>
                          ))}
                        </SidebarMenuSub>
                      </CollapsibleContent>
                    </>
                  ) : null}
                </SidebarMenuItem>
              </Collapsible>
            ) : null}
          </div>
        ))}
      </SidebarMenu>
    </SidebarGroup>
  );
}
