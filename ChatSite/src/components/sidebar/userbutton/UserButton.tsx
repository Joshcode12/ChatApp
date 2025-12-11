interface UserDisplayData {
    name: string
}

function UserButton({ name } : UserDisplayData){

    return (
        <button>
            <h1>{name}</h1>
        </button>
    )

}