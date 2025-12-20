import { useEffect, useRef, useState } from 'react';
import './Chat.css'
import Message from './message/Message';
import type { MessageData } from './message/Message';

function Chat() {
    
    const [Messages, setMessages] = useState<MessageData[]>([
        {user: "bruh", text: "Hello"},
        {user: "bruh", text: "Hello"},
        {user: "bruh", text: "Hello"},
        {user: "bruh", text: "Hello"}
    ]);

    function SendMessage(){
        var messageListDiv: HTMLDivElement = document.querySelector(".message-list") as HTMLDivElement;

        var textarea: HTMLTextAreaElement = document.querySelector("#text-input") as HTMLTextAreaElement;
        var messageSent: string = textarea.value;

        if(messageSent.length >= 1){
            setMessages([...Messages, {user:"test", text: messageSent}]);
            textarea.value = "";

            messageListDiv.scrollTop = messageListDiv.scrollHeight;
        }
    }

    return (
        <div className='chat-grid'>
            <header className='maxsize'>
                Display user info here
            </header>
            <section className='maxsize message-list'>
                {Messages.map((message, index) => (
                    <Message {...message}/>
                ))}
            </section>
            <footer className='maxsize'>
                <textarea
                id='text-input'
                placeholder='Message'
                onKeyDown={(e) => {
                    if (e.key === "Enter" && !e.shiftKey) {
                        e.preventDefault();
                        SendMessage();
                    }
                }}
                />
            </footer>
        </div>
    )
}

export default Chat