mod engine;
mod ui;
mod adblocker;

use tao::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;
use std::rc::Rc;

fn main() -> wry::Result<()> {
    // 1. Criar o loop de eventos (necessário para janelas no Windows)
    let event_loop = EventLoop::new();
    
    // 2. Configurar a janela principal
    let window = WindowBuilder::new()
        .with_title("Error Browser")
        .with_inner_size(tao::dpi::LogicalSize::new(1024.0, 768.0))
        .build(&event_loop)
        .unwrap();

    // Envolver a janela num Smart Pointer (Rc) para usar dentro do WebView
    let window = Rc::new(window);

    // 3. Inicializar o bloqueador de anúncios (o "segurança" do navegador)
    let adblocker = adblocker::ErrorAdblocker::new();

    // 4. Altura da barra de ferramentas (ajustado para o estilo Chrome 23)
    let toolbar_height = 75;

    // 5. Criar o motor WebView com lógica de botões (IPC)
    let webview = WebViewBuilder::new(&window)
        .with_bounds(wry::Rect {
            x: 0,
            y: toolbar_height,
            width: window.inner_size().width,
            height: window.inner_size().height - toolbar_height as u32,
        })
        // Flags para o Atom D2500 rodar sem driver de vídeo
        .with_additional_browser_args("--disable-gpu --disable-software-rasterizer")
        .with_url("https://www.google.com")?
        // --- Lógica para os botões do ui.rs funcionarem ---
        .with_ipc_handler(move |msg| {
            match msg.as_str() {
                "back" => { println!("Voltando..."); }, // Adicionar lógica de histórico aqui
                "forward" => { println!("Avançando..."); },
                "reload" => { println!("Atualizando..."); },
                _ => {
                    if msg.starts_with("goto:") {
                        let url = msg.replace("goto:", "");
                        println!("Navegando para: {}", url);
                    }
                }
            }
        })
        .build()?;

    // 6. Injetar a interface visual do ui.rs
    // Usamos um pequeno atraso ou injetamos via script para garantir que apareça no topo
    webview.evaluate_script(&format!(
        r#"
        (function() {{
            let ui = document.createElement('div');
            ui.innerHTML = `{}`;
            document.body.prepend(ui);
        }})();
        "#,
        ui::get_ui_html()
    ))?;

    // 7. O Loop que mantém o navegador aberto
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            // Se o usuário clicar no "X" vermelho da janela
            tao::event::Event::WindowEvent {
                event: tao::event::WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            // Redimensionar o motor do site se o usuário esticar a janela
            tao::event::Event::WindowEvent {
                event: tao::event::WindowEvent::Resized(size),
                ..
            } => {
                let _ = webview.set_bounds(wry::Rect {
                    x: 0,
                    y: toolbar_height,
                    width: size.width,
                    height: size.height - toolbar_height as u32,
                });
            }
            _ => (),
        }
    });
}
