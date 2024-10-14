"use client"

import { useState } from 'react';
import { ScrollArea } from "@/components/ui/scroll-area"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Send } from "lucide-react"

export function ChatArea({ channel }) {
  const [messages, setMessages] = useState([]);
  const [newMessage, setNewMessage] = useState('');

  const handleSendMessage = () => {
    if (newMessage.trim()) {
      setMessages([...messages, { text: newMessage, sender: 'You' }]);
      setNewMessage('');
    }
  };

  return (
    <div className="flex-1 flex flex-col">
      <div className="bg-background p-4 border-b">
        <h2 className="text-xl font-bold">#{channel}</h2>
      </div>
      <ScrollArea className="flex-1 p-4">
        {messages.map((message, index) => (
          <div key={index} className="mb-4">
            <span className="font-bold">{message.sender}: </span>
            <span>{message.text}</span>
          </div>
        ))}
      </ScrollArea>
      <div className="p-4 flex">
        <Input
          className="flex-1 mr-2"
          placeholder={`Message #${channel}`}
          value={newMessage}
          onChange={(e) => setNewMessage(e.target.value)}
          onKeyPress={(e) => e.key === 'Enter' && handleSendMessage()}
        />
        <Button onClick={handleSendMessage}>
          <Send className="h-4 w-4" />
        </Button>
      </div>
    </div>
  )
}
