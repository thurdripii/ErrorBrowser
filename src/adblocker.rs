use adblock::engine::Engine;
use adblock::lists::FilterSet;
use std::fs::File;
use std::io::BufReader;

pub struct ErrorAdblocker {
    pub engine: Engine,
}

impl ErrorAdblocker {
    pub fn new() -> Self {
        let mut filter_set = FilterSet::new();

        // Tenta carregar a lista de filtros da pasta assets
        // Se o arquivo não existir, o navegador ainda abre, mas sem bloquear
        if let Ok(file) = File::open("assets/adblock_rules/easylist.txt") {
            let reader = BufReader::new(file);
            filter_set.add_filters_from_reader(reader);
        }

        // true = habilita suporte a cosméticos (esconder elementos da página)
        let engine = Engine::from_filter_set(filter_set, true);
        
        Self { engine }
    }

    pub fn should_block(&self, url: &str, source_url: &str) -> bool {
        // Verifica se a URL deve ser bloqueada
        // "resource_type" pode ser imagem, script, etc. Aqui usamos o genérico.
        let result = self.engine.check_network_urls(url, source_url, "");
        result.matched
    }
}
