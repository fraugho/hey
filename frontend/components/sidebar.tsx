"use client"
import { Button } from "@/components/ui/button"
import { ScrollArea } from "@/components/ui/scroll-area"
import { Hash, Plus } from "lucide-react"

interface SidebarProps {
    selectedChannel: string
    onSelectChannel: (channel: string) => void
}

const channels = ['general', 'random', 'introductions', 'announcements']

export function Sidebar({ selectedChannel, onSelectChannel }: SidebarProps) {
    return (
        <div className="w-64 bg-secondary p-4 flex flex-col">
            <h2 className="text-xl font-bold mb-4">Channels</h2>
            <ScrollArea className="flex-grow">
                {channels.map((channel) => (
                    <Button
                        key={channel}
                        variant={channel === selectedChannel ? "secondary" : "ghost"}
                        className="w-full justify-start mb-1"
                        onClick={() => onSelectChannel(channel)}
                    >
                        <Hash className="mr-2 h-4 w-4" />
                        {channel}
                    </Button>
                ))}
            </ScrollArea>
            <Button className="mt-4">
                <Plus className="mr-2 h-4 w-4" />
                Add Channel
            </Button>
        </div>
    )
}
