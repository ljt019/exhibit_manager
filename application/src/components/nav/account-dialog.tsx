import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { useGetUserProfile } from "@/hooks/data/queries/useGetProfileInfo";
import { Skeleton } from "@/components/ui/skeleton";

interface AccountDialogProps {
  isOpen: boolean;
  onClose: () => void;
}

export function AccountDialog({ isOpen, onClose }: AccountDialogProps) {
  const { data: userProfile, isError, isPending } = useGetUserProfile();

  if (isError) {
    return null;
  }

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Account Information</DialogTitle>
        </DialogHeader>
        <div className="grid gap-4 py-4">
          <div className="flex items-center gap-4">
            <Avatar className="h-20 w-20">
              {!isPending && userProfile?.picture ? (
                <AvatarImage src={userProfile.picture} alt={userProfile.name} />
              ) : (
                <AvatarFallback>
                  {isPending ? (
                    <Skeleton className="h-full w-full" />
                  ) : (
                    userProfile?.name?.charAt(0) || "?"
                  )}
                </AvatarFallback>
              )}
            </Avatar>
            <div>
              <h3 className="text-lg font-semibold">
                {isPending ? (
                  <Skeleton className="h-6 w-32" />
                ) : (
                  userProfile?.name
                )}
              </h3>
              <p className="text-sm text-muted-foreground">
                {isPending ? (
                  <Skeleton className="h-4 w-24" />
                ) : (
                  userProfile?.id
                )}
              </p>
            </div>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
