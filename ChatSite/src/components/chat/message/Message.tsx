import './Message.css'

export type MessageData = {
    user: string;
    text: string;
};

function Message({ user, text }: MessageData) {
    return (
        <div className='chat-message'>
            <span>{user}:</span>
            <p>{text}</p>
        </div>
    )
}

export default Message