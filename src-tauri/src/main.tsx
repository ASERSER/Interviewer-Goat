import React from 'react'
import ReactDOM from 'react-dom/client'
import { HUD } from './HUD'
import './style.css'

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <HUD />
  </React.StrictMode>,
)
