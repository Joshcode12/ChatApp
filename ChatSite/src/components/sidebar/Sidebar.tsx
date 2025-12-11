import { useState } from "react"
import './Sidebar.css'

function Sidebar(){

    /*
        Users is an array
        it will auto update the UI when Users is modified
        but you dont modify Users directly

        if you want to modify users you need to run setUsers([])
        and provide it with an array
    */
    const [Users, setUsers] = useState(["test1", "test2", "test3"]);

    return (
        <aside>
            <button id="find">Find User</button>
            <hr />
            <h2>Direct Messages</h2>
            {Users.map((user, index) => (
                <>
                <button>{user}</button>
                <br />
                </>
            ))}
        </aside>
    )

}

export default Sidebar