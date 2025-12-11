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
import { BrowserRouter } from 'react-router-dom'
import './index.css'
import App from './App.tsx'

ReactDOM.createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <BrowserRouter>
            <App />
        </BrowserRouter>
    </React.StrictMode>
)
