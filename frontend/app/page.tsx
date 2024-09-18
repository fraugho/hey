"use client"
import Image from "next/image";
import React, { useState, useEffect, useCallback } from 'react';
import { io, Socket } from 'socket.io-client';
import { Textarea } from "@/components/ui/textarea"

export default function Home() {

    interface Message{
        content: string;
        id: number;
    }
    const [socket, setSocket] = useState<Socket | null>(null);
    const [messages, setMessages] = useState<string[]>([]);
    const [ClientMessage, setClientMessage] = useState<Message>({content: "", id: 1});

    useEffect(() => {
        const newSocket = io('http://127.0.0.1:8080');

        newSocket.on('connect', () => {
            console.log("Connected to SocketIO server");
        });

        newSocket.on('disconnect', () => {
            console.log("Disconnected from SocketIO server");
        });

        newSocket.on('message', (data) => {
            setMessages((prevMessages) => [...prevMessages, data]);
        });

        setSocket(newSocket);

        return () => {
            newSocket.disconnect();
        };
    }, []);

    const sendMessage = useCallback(() =>{
        if (socket && ClientMessage){
            socket.send(ClientMessage);
            setClientMessage({content: "", id: 1},);
        }
    }, [socket, ClientMessage]);

    return (
        <div className="flex h-screen w-full">
            <div className="flex w-1/4 bg-red-600">
            </div>
            <div className="flex flex-col w-3/4 bg-blue-500">
                <div className="h-3/4 bg-green-700">
                    <h2 className="text-white text-xl mb-4">Messages</h2>
                    {messages.map((msg, index) => (
                        <div key={index} className="text-white mb-2">{msg}</div>
                    ))}
                </div>
                <div className="flex h-1/4 bg-cyan-800">
                    <Textarea
                        className="w-11/12" 
                        placeholder="Type Message"
                        value={ClientMessage.content}
                        onChange={(e) => setClientMessage(prevMessage => ({ ...prevMessage, content: e.target.value }))}
                    />
                    <button 
                        className="w-1/12 bg-black"
                        onClick={sendMessage}>
                        Send
                    </button>
                </div>
            </div>
        </div>
    );
}
