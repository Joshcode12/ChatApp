import './Chat.css'

function Chat() {

    function hfkjas(){
        
    }

    return (
        <div className='chat-grid'>
            <header className='maxsize'>
                Display user info here
            </header>
            <section className='maxsize'>
                View chat here
            </section>
            <footer className='maxsize'>
                <input type="text" name="" id="" placeholder='Message'
                onKeyDown={(e) => {
                    if (e.key === "Enter") {
                        
                    }
                }}
                />
            </footer>
        </div>
    )
}

export default Chat