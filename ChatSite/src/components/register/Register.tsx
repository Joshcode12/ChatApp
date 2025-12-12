import './Register.css'

function Register(){

    interface RegisterPayload {
        username: string,
        password: string,
        bio: string,
        profile_file_id: string
    }

    function RequestRegister(){
        var _username: string = (document.querySelector("#username-box") as HTMLInputElement).value
        var _password: string = (document.querySelector("#password-box") as HTMLInputElement).value
        var _bio: string = (document.querySelector("#bio-box") as HTMLInputElement).value

        var data: RegisterPayload = {
            username: _username,
            password: _password,
            bio: _bio,
            profile_file_id: ""//crypto.randomUUID()
        }

        console.log(JSON.stringify(data))

        if(_username.length >= 1 && _password.length >= 1){
            fetch("https://melania-coetaneous-annemarie.ngrok-free.dev/chat/users/register", {
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