//! Página completa de um portal governamental, renderizada com a API pública.
//!
//! Execute com:
//!
//! ```text
//! cargo run -p html-auy --example portal_governamental
//! ```

#![expect(
    clippy::expect_used,
    reason = "exemplo autocontido: rotas, idioma e offset são constantes válidas; o panic é diagnóstico"
)]

use std::io::{self, Write};

use html_auy::{
    Asset, Assets, Class, ExternalUrl, Href, JsonGraph, JsonLd, LUrl, Label, Lang, Msg, Offset,
    Page, Regions, ShellData, Text, Timestamp, View, html,
};

#[derive(Clone, Copy)]
enum PortalMsg {
    Skip,
    Title,
    Description,
    Heading,
    MainNav,
    FooterNav,
    Home,
    Services,
    News,
    Transparency,
    Contact,
    NoticeLabel,
}

impl Msg for PortalMsg {
    fn as_str(self) -> &'static str {
        match self {
            Self::Skip => "Pular para o conteúdo",
            Self::Title => "Portal da Prefeitura de Exemplo",
            Self::Description => {
                "Serviços, notícias e canais de atendimento da Prefeitura de Exemplo."
            }
            Self::Heading => "Prefeitura de Exemplo",
            Self::MainNav => "Navegação principal",
            Self::FooterNav => "Navegação do rodapé",
            Self::Home => "Início",
            Self::Services => "Serviços",
            Self::News => "Notícias",
            Self::Transparency => "Transparência",
            Self::Contact => "Contato",
            Self::NoticeLabel => "Aviso de atendimento",
        }
    }
}

struct GovernmentOrganization;

impl JsonGraph for GovernmentOrganization {
    fn write_json(&self, out: &mut String) {
        out.push_str(
            r#"{"@context":"https://schema.org","@type":"GovernmentOrganization","name":"Prefeitura de Exemplo","url":"https://prefeitura.exemplo.gov.br/"}"#,
        );
    }
}

fn lang() -> Lang {
    Lang::parse("pt-BR").expect("etiqueta BCP-47 fixa")
}

fn path(value: &'static str) -> LUrl<'static> {
    LUrl::new(value).expect("rota interna fixa")
}

fn href(value: &'static str) -> Href<'static> {
    Href::internal(path(value))
}

fn assets() -> Assets {
    [
        Asset::style("/assets/portal.css"),
        Asset::script("/assets/portal.js"),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn header() -> html_auy::Markup {
    html! {
        header class=(Class::new("site-header")) {
            div class=(Class::new("wrap header-inner")) {
                a class=(Class::new("brand")) href=(href("/")) {
                    span class=(Class::new("brand-mark")) aria_hidden="true" {
                        Text::new("P")
                    }
                    Text::new("Prefeitura de Exemplo")
                }
                nav aria_label=(Label::text(PortalMsg::MainNav)) {
                    ul class=(Class::new("main-nav")) {
                        li {
                            a href=(href("/")) aria_current="page" {
                                Text::msg(PortalMsg::Home)
                            }
                        }
                        li { a href=(href("/servicos")) { Text::msg(PortalMsg::Services) } }
                        li { a href=(href("/noticias")) { Text::msg(PortalMsg::News) } }
                        li {
                            a href=(href("/transparencia")) {
                                Text::msg(PortalMsg::Transparency)
                            }
                        }
                        li { a href=(href("/contato")) { Text::msg(PortalMsg::Contact) } }
                    }
                }
            }
        }
    }
}

fn notice() -> html_auy::Markup {
    html! {
        aside class=(Class::new("notice")) aria_label=(Label::text(PortalMsg::NoticeLabel)) {
            p {
                strong { Text::new("Atendimento presencial") }
                Text::new(": de segunda a sexta, das 8h às 17h, no Paço Municipal.")
            }
        }
    }
}

fn services() -> html_auy::Markup {
    html! {
        section class=(Class::new("section")) {
            div class=(Class::new("wrap")) {
                h2 { Text::new("Serviços para o cidadão") }
                ul class=(Class::new("card-grid")) {
                    li {
                        a class=(Class::new("card")) href=(href("/servicos/saude")) {
                            h3 { Text::new("Saúde") }
                            p { Text::new("Agende consultas, veja unidades e acompanhe campanhas.") }
                        }
                    }
                    li {
                        a class=(Class::new("card")) href=(href("/servicos/educacao")) {
                            h3 { Text::new("Educação") }
                            p { Text::new("Matrículas, escolas, calendário e transporte escolar.") }
                        }
                    }
                    li {
                        a class=(Class::new("card")) href=(href("/servicos/transporte")) {
                            h3 { Text::new("Transporte") }
                            p { Text::new("Linhas, horários e cartão de mobilidade.") }
                        }
                    }
                    li {
                        a class=(Class::new("card")) href=(href("/servicos/meio-ambiente")) {
                            h3 { Text::new("Meio ambiente") }
                            p { Text::new("Coleta seletiva, denúncias e licenciamento.") }
                        }
                    }
                }
            }
        }
    }
}

fn news() -> html_auy::Markup {
    let offset = Offset::from_hms(-3, 0).expect("offset fixo de Brasília");
    html! {
        section class=(Class::new("section section-alt")) {
            div class=(Class::new("wrap")) {
                h2 { Text::new("Últimas notícias") }
                ul class=(Class::new("news-list")) {
                    li {
                        article {
                            p class=(Class::new("date")) {
                                time datetime=(Timestamp::new(1_793_880_000, offset)) {
                                    Text::new("5 de novembro de 2026")
                                }
                            }
                            h3 {
                                a href=(href("/noticias/nova-frota")) {
                                    Text::new("Prefeitura entrega nova frota de ônibus")
                                }
                            }
                            p { Text::new("Veículos começam a operar nas linhas municipais na próxima semana.") }
                        }
                    }
                    li {
                        article {
                            p class=(Class::new("date")) {
                                time datetime=(Timestamp::new(1_793_813_400, offset)) {
                                    Text::new("4 de novembro de 2026")
                                }
                            }
                            h3 {
                                a href=(href("/noticias/feira-saude")) {
                                    Text::new("Feira de saúde atende neste sábado")
                                }
                            }
                            p { Text::new("Atendimentos gratuitos, vacinação e orientação nutricional.") }
                        }
                    }
                }
            }
        }
    }
}

fn footer() -> html_auy::Markup {
    html! {
        footer class=(Class::new("site-footer")) {
            div class=(Class::new("wrap footer-grid")) {
                div {
                    h2 { Text::new("Prefeitura de Exemplo") }
                    p { Text::new("Praça da República, 100 — Centro") }
                    p { Text::new("Atendimento: segunda a sexta, 8h às 17h") }
                }
                nav aria_label=(Label::text(PortalMsg::FooterNav)) {
                    ul {
                        li { a href=(href("/acessibilidade")) { Text::new("Acessibilidade") } }
                        li { a href=(href("/privacidade")) { Text::new("Privacidade") } }
                        li { a href=(href("/mapa-do-site")) { Text::new("Mapa do site") } }
                    }
                }
            }
        }
    }
}

fn page() -> Page {
    let mut regions = Regions::new();
    regions.header = Some(header());
    regions.before_main = Some(notice());
    regions.footer = Some(footer());

    let data = ShellData::new(
        Text::msg(PortalMsg::Title),
        Text::msg(PortalMsg::Heading),
        lang(),
    )
    .description(Text::msg(PortalMsg::Description))
    .canonical(ExternalUrl::parse("https://prefeitura.exemplo.gov.br/").expect("URL canônica"))
    .skip(Text::msg(PortalMsg::Skip))
    .assets(assets())
    .schemas(vec![JsonLd::graph(&GovernmentOrganization)])
    .regions(regions);

    let main = html! {
        section class=(Class::new("hero")) {
            div class=(Class::new("wrap")) {
                p class=(Class::new("eyebrow")) { Text::new("Portal do cidadão") }
                h2 { Text::new("Serviços públicos, notícias e participação") }
                p {
                    Text::new("Encontre o serviço que você precisa, acompanhe as ações da prefeitura e fale com a administração municipal.")
                }
                a class=(Class::new("button")) href=(href("/servicos")) {
                    Text::new("Encontrar um serviço")
                }
            }
        }
        for part in [services(), news()] {
            (part)
        }
    };

    Page::new(&data, View::new(main))
}

fn main() -> io::Result<()> {
    let page = page();
    let mut out = io::stdout().lock();
    out.write_all(page.markup().as_str().as_bytes())?;
    Ok(())
}
