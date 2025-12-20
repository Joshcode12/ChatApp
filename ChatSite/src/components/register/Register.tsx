import './Register.css'
import { globalState } from '../../globalState';

function Register() {

    type UUID = string & { readonly brand: unique symbol };

    interface RegisterPayload {
        username: string;
        password: string;
        bio: string | null;
        profile_file_id: UUID | null;
    }

    interface LoginPayload {
        username: string;
        password: string;
    }

    async function RequestRegister() {
        const _username: string = (document.querySelector("#username-box") as HTMLInputElement).value
        const _password: string = (document.querySelector("#password-box") as HTMLInputElement).value
        const _bio: string | null = (document.querySelector("#bio-box") as HTMLInputElement).value.trim() || null;

        const data: RegisterPayload = {
            username: _username,
            password: _password,
            bio: _bio,
            profile_file_id: null
        }

        console.log(JSON.stringify(data))

        if (_username.length >= 1 && _password.length >= 1) {
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
        }
    }

    async function RequestLogin(){
        const _username: string = (document.querySelector("#username-box") as HTMLInputElement).value
        const _password: string = (document.querySelector("#password-box") as HTMLInputElement).value
        //const _bio: string | null = (document.querySelector("#bio-box") as HTMLInputElement).value.trim() || null;

        const data: LoginPayload = {
            username: _username,
            password: _password
        }

        console.log(JSON.stringify(data))

        if (_username.length >= 1 && _password.length >= 1) {
            const res: Response = await fetch(globalState.url + "chat/auth/login", {
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
        }
    }

    return (
        <div className='maxsize' id='register'>
            <input type="text" id="username-box" placeholder='Username'/>
            <input type="text" id="password-box" placeholder='Password'/>
            <input type="text" id="bio-box" placeholder='Bio'/>
            <div>
                <button onClick={RequestLogin} style={{backgroundColor: "#242"}}>Login</button>
                <button onClick={RequestRegister} style={{backgroundColor: "#442"}}>Register</button>
            </div>
        </div>
    )
}

export default Register