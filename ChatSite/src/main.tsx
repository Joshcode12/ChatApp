/*
    how to run the frontend site

    open the ChatSite folder in vscode
    open vscode's terminal
    make sure u got dependencies by typing npm install
    run the site locally by typing npm run dev
    ctrl + c to stop running
*/

import React from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter, Routes, Route } from 'react-router-dom'
import './index.css'
import App from './App.tsx'
import Chat from './components/chat/Chat.tsx'
import Home from './components/home/Home.tsx'
import Register from './components/register/Register.tsx'

ReactDOM.createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <BrowserRouter>
            <Routes>
                <Route path="/" element={<App />}>
                    <Route index element={<Home />} />
                    <Route path="chat/*" element={<Chat />} />
                    <Route path="register/*" element={<Register />} />
                </Route>
            </Routes>
        </BrowserRouter>
    </React.StrictMode>
)
