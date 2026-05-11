fn main() {
    // Só executa isso se estivermos compilando para Windows
    #[cfg(windows)]
    {
        // Compila o arquivo de recurso que aponta para o seu ícone
        embed_resource::compile("assets/icon.rc");
    }
}
