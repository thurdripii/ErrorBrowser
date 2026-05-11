use wry::{WebView, WebViewBuilder};
use tao::window::Window;
use std::rc::Rc;
use crate::adblocker::ErrorAdblocker;

pub fn create_error_webview(
    window: &Window, 
    toolbar_height: i32, 
    adblocker: Rc<ErrorAdblocker>
) -> wry::Result<WebView> {
    
    let size = window.inner_size();

    WebViewBuilder::new(window)
        .with_bounds(wry::Rect {
            x: 0,
            y: toolbar_height,
            width: size.width,
            height: size.height - toolbar_height as u32,
        })
        // FLAGS CRÍTICAS PARA O ATOM D2500 (Sem Driver de Vídeo)
        .with_additional_browser_args(
            "--disable-gpu \
             --disable-software-rasterizer \
             --disable-dev-shm-usage \
             --no-sandbox \
             --disable-features=IsolateOrigins,site-per-process"
        )
        // Lógica de Bloqueio de Anúncios
        .with_navigation_handler(move |url| {
            // Opcional: Bloquear navegação para sites maliciosos inteiros aqui
            true 
        })
        .with_url("https://www.google.com")?
        .build()
}
