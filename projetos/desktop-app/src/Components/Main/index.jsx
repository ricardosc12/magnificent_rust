import style from './style.module.css'
import { appWindow } from "@tauri-apps/api/window";
import './css.css'
import { createEffect, createSignal } from 'solid-js';

export default function Main(){

    const [focused_, setFocused] = createSignal(false)
    var listen;

    async function unfocus(){
        listen = await appWindow.onFocusChanged(async({ payload: focused }) => {
            if(focused_()===true) {
                await appWindow.setCursorGrab(true)
                await appWindow.setFocus()
                await appWindow.maximize()
            }
            // await appWindow.se
            document.getElementById("main").innerHTML = String(focused)
        });
    }

    createEffect(()=>{      
        document.addEventListener("keydown",async(e)=>{
            if(String(e.code) === "ArrowDown") {
                setFocused(false)
                await appWindow.minimize()
                await appWindow.setCursorGrab(false)
            }
            if(String(e.code) === "ArrowUp") {
                setFocused(true)
            }
        })  
        unfocus()
    })

    return <div data-tauri-drag-region id="main" className={style.main} tabIndex="-1">TESTEs</div>
}