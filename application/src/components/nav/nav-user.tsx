import { useEffect } from "react";
import { BadgeCheck, Bell, ChevronsUpDown, LogOut } from "lucide-react";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";
import { useGetUserProfile } from "@/hooks/data/queries/useGetProfileInfo";
import { invoke } from "@tauri-apps/api";
import useListen from "@/hooks/util/useListen";
import { useNavigate } from "react-router-dom";

export function NavUser() {
  const navigate = useNavigate();

  useListen({
    event: "sign_out_complete",
    callback: () => {
      navigate("/");
    },
  });

  const {
    data: userProfile,
    isError: isUserProfileError,
    isPending: isUserProfilePending,
  } = useGetUserProfile();

  useEffect(() => {
    if (isUserProfileError) {
      navigate("/");
    }
  }, [isUserProfileError, navigate]);

  const renderAvatar = () => (
    <Avatar className="h-8 w-8 rounded-lg">
      {!isUserProfilePending && userProfile?.picture ? (
        <AvatarImage src={userProfile.picture} alt={userProfile.name} />
      ) : (
        <AvatarFallback className="rounded-lg">
          {isUserProfilePending ? "EM" : userProfile?.name?.charAt(0) || "?"}
        </AvatarFallback>
      )}
    </Avatar>
  );

  const renderName = () => (
    <span className="truncate font-semibold">
      {isUserProfilePending
        ? "Loading..."
        : userProfile?.name || "Unknown User"}
    </span>
  );

  return (
    <SidebarMenu>
      <SidebarMenuItem>
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <SidebarMenuButton
              size="lg"
              className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
            >
              {renderAvatar()}
              <div className="grid flex-1 text-left text-sm leading-tight">
                {renderName()}
              </div>
              <ChevronsUpDown className="ml-auto size-4" />
            </SidebarMenuButton>
          </DropdownMenuTrigger>
          <DropdownMenuContent
            className="w-[--radix-dropdown-menu-trigger-width] min-w-56 rounded-lg"
            side={"right"}
            align="end"
            sideOffset={4}
          >
            <DropdownMenuLabel className="p-0 font-normal">
              <div className="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
                {renderAvatar()}
                <div className="grid flex-1 text-left text-sm leading-tight">
                  {renderName()}
                </div>
              </div>
            </DropdownMenuLabel>
            <DropdownMenuGroup>
              <DropdownMenuItem>
                <BadgeCheck />
                Account
              </DropdownMenuItem>
              <DropdownMenuItem disabled={true}>
                <Bell />
                <p className="line-through">Notifications </p>
                <p>(coming soon)</p>
              </DropdownMenuItem>
            </DropdownMenuGroup>
            <DropdownMenuSeparator />
            <DropdownMenuItem
              onClick={() => invoke("sign_out")}
              className="text-destructive"
            >
              <LogOut />
              Log out
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </SidebarMenuItem>
    </SidebarMenu>
  );
}
