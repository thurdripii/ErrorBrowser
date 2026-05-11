pub fn get_ui_html() -> &'static str {
    r#"
    <div id="error-ui">
        <div class="tab-bar">
            <div class="tab active">
                <img src="https://www.google.com/favicon.ico" class="favicon">
                <span>Google</span>
                <div class="close-tab" onclick="window.ipc.postMessage('close-tab')">×</div>
            </div>
            <div class="new-tab-btn">+</div>
        </div>

        <div class="toolbar">
            <div class="nav-buttons">
                <button class="btn" onclick="window.ipc.postMessage('back')" title="Voltar">
                    <svg width="16" height="16" viewBox="0 0 24 24"><path fill="#5f6368" d="M20 11H7.83l5.59-5.59L12 4l-8 8 8 8 1.41-1.41L7.83 13H20v-2z"/></svg>
                </button>
                <button class="btn" onclick="window.ipc.postMessage('forward')" title="Avançar">
                    <svg width="16" height="16" viewBox="0 0 24 24" style="transform: rotate(180deg)"><path fill="#5f6368" d="M20 11H7.83l5.59-5.59L12 4l-8 8 8 8 1.41-1.41L7.83 13H20v-2z"/></svg>
                </button>
                <button class="btn" onclick="window.ipc.postMessage('reload')" title="Recarregar">
                    <svg width="16" height="16" viewBox="0 0 24 24"><path fill="#5f6368" d="M17.65 6.35A7.958 7.958 0 0 0 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/></svg>
                </button>
            </div>

            <input type="text" id="url-input" class="omnibox" 
                   placeholder="Pesquise ou digite uma URL" 
                   onkeydown="if(event.key==='Enter') window.ipc.postMessage('goto:'+this.value)" />

            <div class="star" onclick="window.ipc.postMessage('favorite')">★</div>
            <div class="menu-btn">⋮</div>
        </div>
    </div>

    <style>
        :root {
            --bg-color: #dee1e6;
            --toolbar-color: #f1f3f4;
            --tab-active: #f1f3f4;
            --tab-inactive: #bac0c7;
        }

        body { margin: 0; font-family: 'Segoe UI', Tahoma, sans-serif; overflow: hidden; }

        #error-ui {
            background-color: var(--bg-color);
            padding-top: 4px;
            border-bottom: 1px solid #888;
            user-select: none;
        }

        /* Abas Estilo Chrome 23 */
        .tab-bar { display: flex; align-items: flex-end; padding-left: 8px; height: 32px; }
        .tab {
            background: var(--tab-inactive);
            padding: 6px 12px;
            border-radius: 8px 8px 0 0;
            font-size: 12px;
            display: flex;
            align-items: center;
            gap: 8px;
            width: 180px;
            border: 1px solid #999;
            border-bottom: none;
            color: #444;
        }
        .tab.active {
            background: var(--tab-active);
            z-index: 2;
            border-color: #888;
            color: #000;
        }
        .favicon { width: 16px; height: 16px; }
        .close-tab { margin-left: auto; cursor: pointer; font-size: 16px; color: #777; }
        .close-tab:hover { color: #d32f2f; }
        .new-tab-btn { padding: 0 10px; cursor: pointer; font-size: 20px; color: #555; }

        /* Barra de Ferramentas */
        .toolbar {
            background-color: var(--toolbar-color);
            height: 40px;
            display: flex;
            align-items: center;
            padding: 0 10px;
            gap: 10px;
            border-top: 1px solid #fff;
        }

        .nav-buttons { display: flex; gap: 4px; }
        .btn {
            background: none; border: none; padding: 6px;
            cursor: pointer; display: flex; align-items: center; justify-content: center;
        }
        .btn:hover { background: #e0e0e0; border-radius: 50%; }

        /* Omnibox (Barra de busca) */
        .omnibox {
            flex-grow: 1;
            height: 28px;
            border-radius: 14px;
            border: 1px solid #ccc;
            padding: 0 15px;
            font-size: 13px;
            background: white;
            outline: none;
        }
        .omnibox:focus { border-color: #4d90fe; box-shadow: 0 0 3px rgba(77,144,254,0.5); }

        .star { color: #5f6368; font-size: 18px; cursor: pointer; margin-right: 5px; }
        .star:hover { color: #f4b400; }
        .menu-btn { font-size: 20px; color: #5f6368; cursor: pointer; padding: 0 5px; }
    </style>
    "#
}
