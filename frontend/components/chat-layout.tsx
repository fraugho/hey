"use client"

import { useState } from 'react';
import { Sidebar } from '@/components/sidebar';
import { ChatArea } from '@/components/chat-area';
import { UserList } from '@/components/user-list';

export default function ChatLayout() {
  const [selectedChannel, setSelectedChannel] = useState('general');

  return (
    <div className="flex h-screen bg-background text-foreground">
      <Sidebar selectedChannel={selectedChannel} onSelectChannel={setSelectedChannel} />
      <ChatArea channel={selectedChannel} />
      <UserList />
    </div>
  );
}
