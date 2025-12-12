import { useState } from 'react'
import './App.css'
import Sidebar from './components/sidebar/Sidebar'
import { Outlet } from 'react-router-dom'

function App() {

    // these files can only return a single element
    // so if you want to return multiple you need to do <> </> for it to count as one
    return (
        <div className='app-grid'>
            <Sidebar />
            <Outlet />
        </div>
    )
}

export default App
