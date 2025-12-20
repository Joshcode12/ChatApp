import './Home.css'
import { globalState } from '../../globalState';

function Home() {

    type UUID = string & { readonly brand: unique symbol };

    interface FindUserPayload {
        username: string;
    }

    async function FindUser() {
        const _username: string = (document.querySelector("#username-box") as HTMLInputElement).value

        const data: FindUserPayload = {
            username: _username
        }

        console.log(JSON.stringify(data))

        if (_username.length >= 1) {
            // idk the url yet so commented out
            /*
            const res: Response = await fetch(globalState.url + "chat/auth/register", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json", // sending JSON
                },
                body: JSON.stringify(data)
            })
            console.log(res);
            if(res.status == 200){ // OK
                console.log("Good")
            } else {
                console.log("Bad")
            }
            */
        }
    }

    return (
        <div className='maxsize' id='finduser'>
            <input type="text" id="username-box" placeholder='Username'/>
            <div>
                <button onClick={FindUser} style={{backgroundColor: "#442"}}>Find</button>
            </div>
        </div>
    )
}

export default Home