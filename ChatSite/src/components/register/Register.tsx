import './Register.css'

function Register() {

    type UUID = string & { readonly brand: unique symbol };

    interface RegisterPayload {
        username: string;
        password: string;
        bio: string | null;
        profile_file_id: UUID | null;
    }

    function RequestRegister() {
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
            fetch("http://127.0.0.1:3000/chat/users/register", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json", // sending JSON
                },
                body: JSON.stringify(data)
            })
        }
    }

    return (
        <div className='maxsize' id='register'>
            <input type="text" id="username-box" placeholder='Username'/>
            <input type="text" id="password-box" placeholder='Password'/>
            <input type="text" id="bio-box" placeholder='Bio'/>
            <button onClick={RequestRegister}>Confirm</button>
        </div>
    )
}

export default Register