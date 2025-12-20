import { useState } from "react"
import { Link, useNavigate } from "react-router-dom"
import './Sidebar.css'

function Sidebar(){

    /*
        Users is an array
        it will auto update the UI when Users is modified
        but you dont modify Users directly

        if you want to modify users you need to run setUsers([])
        and provide it with an array
    */
    const [Users, setUsers] = useState(["test1", "test2", "test3", "dfshdsfhdsfhjkdslhdfs"]);

    const navigate = useNavigate();

    return (
        <aside className="sidebar">
            <header className="pad20">
                <button id="find" onClick={
                    () => navigate("/")
                }>Find User</button>
            </header>
            <hr />
            <div className="pad20h">
                <h2>Direct Messages</h2>
                {Users.map((user, index) => (
                    <>
                    <button onClick={
                        () => navigate(`/chat/${user}`)
                    }>{user}</button>
                    <br />
                    </>
                ))}
            </div>
            <footer className="pad20">
                <button onClick={
                    () => navigate('/register')
                }>Register</button>
            </footer>
        </aside>
    )

}

export default Sidebar