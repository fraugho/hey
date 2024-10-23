import { ScrollArea } from "@/components/ui/scroll-area"
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"

const users = [
  { name: 'Alice', status: 'online' },
  { name: 'Bob', status: 'idle' },
  { name: 'Charlie', status: 'offline' },
  { name: 'David', status: 'online' },
];

export function UserList() {
  return (
    <div className="w-64 bg-secondary p-4">
      <h2 className="text-xl font-bold mb-4">Online - 4</h2>
      <ScrollArea className="h-[calc(100vh-8rem)]">
        {users.map((user) => (
          <div key={user.name} className="flex items-center mb-4">
            <Avatar className="h-8 w-8 mr-2">
              <AvatarImage src={`https://api.dicebear.com/6.x/initials/svg?seed=${user.name}`} />
              <AvatarFallback>{user.name[0]}</AvatarFallback>
            </Avatar>
            <div>
              <div className="font-medium">{user.name}</div>
              <div className="text-xs text-muted-foreground">{user.status}</div>
            </div>
          </div>
        ))}
      </ScrollArea>
    </div>
  )
}
