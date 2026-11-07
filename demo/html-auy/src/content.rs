//! Dados do portal: conteúdo estático separado da marcação.

/// Cartão de serviço: os dados ficam aqui; a marcação vem do `html!`.
#[derive(Clone, Copy)]
pub(crate) struct Service {
    pub(crate) icon: &'static str,
    pub(crate) badge: &'static str,
    pub(crate) title: &'static str,
    pub(crate) body: &'static str,
    pub(crate) footer: &'static str,
    /// Alterna o tom do cartão (secundário nos índices ímpares), como no desenho.
    pub(crate) alternate: bool,
}

pub(crate) const SERVICES: &[Service] = &[
    Service {
        icon: "receipt_long",
        badge: "Tributos",
        title: "IPTU & Certidão de Débitos",
        body: "Emissão de 2ª via da cota única ou parcelas, certidão negativa de tributos imobiliários e mobiliários.",
        footer: "100% Digital • Via CPF/CNPJ",
        alternate: false,
    },
    Service {
        icon: "medical_services",
        badge: "SUS Municipal",
        title: "Consultas & Farmácia Básica",
        body: "Agendamento nas Unidades Básicas de Saúde (UBS), estoque em tempo real de remédios e campanhas de vacinação.",
        footer: "Cartão SUS • Teleagendamento",
        alternate: true,
    },
    Service {
        icon: "local_library",
        badge: "Educação",
        title: "Matrícula Escolar & Transporte",
        body: "Cadastro para creches e escolas municipais, solicitação e passe eletrônico do transporte rural e universitário.",
        footer: "Rede Municipal • Matrícula Fácil",
        alternate: false,
    },
    Service {
        icon: "apartment",
        badge: "Urbanismo",
        title: "Alvarás, Habite-se & Obras",
        body: "Licenciamento expresso para reformas e construções, expedição de alvarás de funcionamento comercial e Habite-se.",
        footer: "Aprovação Digital • Plantas BIM",
        alternate: true,
    },
    Service {
        icon: "lightbulb",
        badge: "Zeladoria Cívica",
        title: "Iluminação e Manutenção de Vias",
        body: "Abertura de chamados para troca de lâmpadas de LED, tapa-buracos, poda preventiva de árvores e desobstrução de bueiros.",
        footer: "Prazo Máximo: 72 horas úteis",
        alternate: false,
    },
    Service {
        icon: "history_edu",
        badge: "Processo Eletrônico",
        title: "Protocolo Geral Sem Papel",
        body: "Abertura, juntada de documentos e acompanhamento de qualquer processo administrativo com autenticação Gov.br.",
        footer: "Rastreamento em Tempo Real",
        alternate: true,
    },
];

/// Controle de acessibilidade do cabeçalho.
#[derive(Clone, Copy)]
pub(crate) struct Control {
    pub(crate) title: &'static str,
    pub(crate) icon: &'static str,
    pub(crate) label: &'static str,
    pub(crate) class: &'static str,
}

pub(crate) const CONTROLS: &[Control] = &[
    Control {
        title: "Acessibilidade e-MAG",
        icon: "accessibility_new",
        label: "Acessibilidade",
        class: "flex items-center gap-1 hover:bg-primary-container px-space-xs py-0.5 rounded transition-colors",
    },
    Control {
        title: "Alto Contraste",
        icon: "contrast",
        label: "Contraste",
        class: "flex items-center gap-1 hover:bg-primary-container px-space-xs py-0.5 rounded transition-colors",
    },
    Control {
        title: "Aumentar Fonte",
        icon: "",
        label: "A+",
        class: "font-bold font-label-sm hover:bg-primary-container px-space-xs py-0.5 rounded text-label-sm",
    },
    Control {
        title: "Diminuir Fonte",
        icon: "",
        label: "A-",
        class: "font-bold font-label-sm hover:bg-primary-container px-space-xs py-0.5 rounded text-label-sm",
    },
    Control {
        title: "VLibras",
        icon: "sign_language",
        label: "VLibras",
        class: "flex font-label-sm font-semibold gap-1 hover:bg-primary-container items-center px-space-xs py-0.5 rounded text-label-sm transition-colors",
    },
];

/// Atalhos dos cartões de Poderes.
pub(crate) const EXEC_QUICK: &[(&str, &str)] = &[
    ("local_hospital", "Saúde"),
    ("school", "Educação"),
    ("construction", "Obras"),
    ("account_balance_wallet", "Finanças"),
];

pub(crate) const LEG_QUICK: &[(&str, &str)] = &[
    ("badge", "Bancadas"),
    ("live_tv", "TV Câmara"),
    ("description", "Projetos de Lei"),
    ("finance_mode", "Cota Gabinete"),
];

/// Navegação principal (path de `data-*` + rótulo).
pub(crate) const NAV: &[(&str, &str)] = &[
    ("inicio", "Início"),
    ("prefeitura-secretarias", "Prefeitura & Secretarias"),
    ("camara-vereadores", "Câmara & Vereadores"),
    ("portal-da-transparencia", "Portal da Transparência"),
    ("servicos-ao-cidadao", "Serviços ao Cidadão"),
    ("diario-oficial", "Diário Oficial"),
    ("legislacao-atos", "Legislação & Atos"),
    ("ouvidoria-e-sic", "Ouvidoria / e-SIC"),
];

/// Atos do Diário Oficial (três cartões).
pub(crate) struct Act {
    pub(crate) badge: &'static str,
    pub(crate) badge_class: &'static str,
    pub(crate) org: &'static str,
    pub(crate) when: &'static str,
    pub(crate) body: &'static str,
    pub(crate) meta: &'static str,
    pub(crate) cta: &'static str,
}

pub(crate) const GAZETTE: &[Act] = &[
    Act {
        badge: "Decreto nº 4.102/2025",
        badge_class: "px-2 py-0.5 rounded bg-surface-container-high text-primary font-label-sm text-label-sm font-bold",
        org: "Poder Executivo",
        when: "Publicado hoje às 08:30",
        body: "Regulamenta o programa de incentivo à transição energética e instalação de painéis fotovoltaicos em prédios municipais.",
        meta: "Signatário: Gabinete do Prefeito Municipal",
        cta: "Ler Inteiro Teor",
    },
    Act {
        badge: "Lei Ordinária nº 2.894/2025",
        badge_class: "px-2 py-0.5 rounded bg-secondary-fixed text-on-secondary-fixed font-label-sm text-label-sm font-bold",
        org: "Poder Legislativo",
        when: "Publicado hoje às 08:15",
        body: "Institui a Política Municipal de Dados Abertos e estabelece parâmetros rigorosos de transparência ativa na saúde infantil.",
        meta: "Autoria: Mesa Diretora da Câmara de Vereadores (Aprovação Unânime)",
        cta: "Ler Inteiro Teor",
    },
    Act {
        badge: "Concorrência Eletrônica nº 012/2025",
        badge_class: "px-2 py-0.5 rounded bg-surface-container text-tertiary-container font-label-sm text-label-sm font-bold",
        org: "Secretaria de Obras",
        when: "Abertura: 15/04/2025",
        body: "Contratação de empresa especializada em engenharia para drenagem e pavimentação asfáltica do Anel Viário Norte.",
        meta: "Valor Estimado: R$ 8.450.000,00 • Lei nº 14.133/21",
        cta: "Acessar Edital e Anexos",
    },
];

/// Mensagens usadas em nomes acessíveis.
#[derive(Clone, Copy)]
pub(crate) enum Msg {
    Search,
    Hero,
    Powers,
    Budget,
    Services,
    Gazette,
    Participation,
}

impl html::Msg for Msg {
    fn as_str(self) -> &'static str {
        match self {
            Self::Search => "Buscar no portal",
            Self::Hero => "Destaques do portal",
            Self::Powers => "Poderes constituídos",
            Self::Budget => "Execução orçamentária",
            Self::Services => "Serviços ao cidadão",
            Self::Gazette => "Diário oficial e atos normativos",
            Self::Participation => "Participação cidadã",
        }
    }
}
