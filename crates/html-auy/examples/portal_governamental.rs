//! Página completa de um portal governamental, renderizada com a API pública.
//!
//! O locale é resolvido pelo consumidor; este exemplo aceita `pt-BR`, `en`, `es` e `fr` como
//! primeiro argumento. O crate recebe o `Lang` e as mensagens já resolvidas.
//!
//! ```text
//! cargo run -p html-auy --example portal_governamental -- pt-BR
//! cargo run -p html-auy --example portal_governamental -- en
//! ```

#![expect(
    clippy::expect_used,
    reason = "exemplo autocontido: idioma e datas são constantes válidas; o panic é diagnóstico"
)]
use std::io::{self, Write};

use html_auy::Alternate;
use html_auy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Locale {
    PtBr,
    En,
    Es,
    Fr,
}

impl Locale {
    const ALL: [Self; 4] = [Self::PtBr, Self::En, Self::Es, Self::Fr];

    const fn tag(self) -> &'static str {
        match self {
            Self::PtBr => "pt-BR",
            Self::En => "en",
            Self::Es => "es",
            Self::Fr => "fr",
        }
    }

    fn from_arg(value: &str) -> Option<Self> {
        match value {
            "pt" | "pt-BR" | "pt-br" => Some(Self::PtBr),
            "en" => Some(Self::En),
            "es" => Some(Self::Es),
            "fr" => Some(Self::Fr),
            _ => None,
        }
    }

    fn lang(self) -> Lang {
        Lang::parse(self.tag()).expect("etiqueta BCP-47 fixa")
    }

    const fn dir(self) -> Dir {
        // Todos os locales atuais são LTR; um locale RTL entraria aqui.
        match self {
            Self::PtBr | Self::En | Self::Es | Self::Fr => Dir::Ltr,
        }
    }

    const fn canonical(self) -> &'static str {
        match self {
            Self::PtBr => "https://prefeitura.exemplo.gov.br/",
            Self::En => "https://prefeitura.exemplo.gov.br/en/",
            Self::Es => "https://prefeitura.exemplo.gov.br/es/",
            Self::Fr => "https://prefeitura.exemplo.gov.br/fr/",
        }
    }

    const fn path(self) -> &'static str {
        match self {
            Self::PtBr => "/",
            Self::En => "/en/",
            Self::Es => "/es/",
            Self::Fr => "/fr/",
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::PtBr => "Português",
            Self::En => "English",
            Self::Es => "Español",
            Self::Fr => "Français",
        }
    }
}

#[derive(Clone, Copy)]
enum Key {
    Skip,
    Title,
    Description,
    GovernmentName,
    MainNav,
    FooterNav,
    LanguageNav,
    Home,
    Services,
    News,
    Transparency,
    Contact,
    NoticeLabel,
    NoticeStrong,
    NoticeBody,
    ServicesHeading,
    Health,
    HealthDescription,
    Education,
    EducationDescription,
    Transport,
    TransportDescription,
    Environment,
    EnvironmentDescription,
    NewsHeading,
    NewsDateOne,
    NewsOneTitle,
    NewsOneBody,
    NewsDateTwo,
    NewsTwoTitle,
    NewsTwoBody,
    Address,
    Hours,
    Accessibility,
    Privacy,
    Sitemap,
    HeroEyebrow,
    HeroTitle,
    HeroBody,
    HeroButton,
}

/// Mensagem resolvida para um locale: o crate continua sem saber o que é locale.
#[derive(Clone, Copy)]
struct Message {
    locale: Locale,
    key: Key,
}

impl Msg for Message {
    #[expect(
        clippy::too_many_lines,
        reason = "catálogo de quatro locales em um match exaustivo; dividir esconderia a tabela"
    )]
    fn as_str(self) -> &'static str {
        match self.locale {
            Locale::PtBr => match self.key {
                Key::Skip => "Pular para o conteúdo",
                Key::Title => "Portal da Prefeitura de Exemplo",
                Key::Description => {
                    "Serviços, notícias e canais de atendimento da Prefeitura de Exemplo."
                }
                Key::GovernmentName => "Prefeitura de Exemplo",
                Key::MainNav => "Navegação principal",
                Key::FooterNav => "Navegação do rodapé",
                Key::LanguageNav => "Idioma",
                Key::Home => "Início",
                Key::Services => "Serviços",
                Key::News => "Notícias",
                Key::Transparency => "Transparência",
                Key::Contact => "Contato",
                Key::NoticeLabel => "Aviso de atendimento",
                Key::NoticeStrong => "Atendimento presencial",
                Key::NoticeBody => ": de segunda a sexta, das 8h às 17h, no Paço Municipal.",
                Key::ServicesHeading => "Serviços para o cidadão",
                Key::Health => "Saúde",
                Key::HealthDescription => "Agende consultas, veja unidades e acompanhe campanhas.",
                Key::Education => "Educação",
                Key::EducationDescription => {
                    "Matrículas, escolas, calendário e transporte escolar."
                }
                Key::Transport => "Transporte",
                Key::TransportDescription => "Linhas, horários e cartão de mobilidade.",
                Key::Environment => "Meio ambiente",
                Key::EnvironmentDescription => "Coleta seletiva, denúncias e licenciamento.",
                Key::NewsHeading => "Últimas notícias",
                Key::NewsDateOne => "5 de novembro de 2026",
                Key::NewsOneTitle => "Prefeitura entrega nova frota de ônibus",
                Key::NewsOneBody => {
                    "Veículos começam a operar nas linhas municipais na próxima semana."
                }
                Key::NewsDateTwo => "4 de novembro de 2026",
                Key::NewsTwoTitle => "Feira de saúde atende neste sábado",
                Key::NewsTwoBody => "Atendimentos gratuitos, vacinação e orientação nutricional.",
                Key::Address => "Praça da República, 100 — Centro",
                Key::Hours => "Atendimento: segunda a sexta, 8h às 17h",
                Key::Accessibility => "Acessibilidade",
                Key::Privacy => "Privacidade",
                Key::Sitemap => "Mapa do site",
                Key::HeroEyebrow => "Portal do cidadão",
                Key::HeroTitle => "Serviços públicos, notícias e participação",
                Key::HeroBody => {
                    "Encontre o serviço que você precisa, acompanhe as ações da prefeitura e fale com a administração municipal."
                }
                Key::HeroButton => "Encontrar um serviço",
            },
            Locale::En => match self.key {
                Key::Skip => "Skip to content",
                Key::Title => "City Hall of Example Portal",
                Key::Description => {
                    "Services, news, and contact channels from the City Hall of Example."
                }
                Key::GovernmentName => "City Hall of Example",
                Key::MainNav => "Main navigation",
                Key::FooterNav => "Footer navigation",
                Key::LanguageNav => "Language",
                Key::Home => "Home",
                Key::Services => "Services",
                Key::News => "News",
                Key::Transparency => "Transparency",
                Key::Contact => "Contact",
                Key::NoticeLabel => "Service notice",
                Key::NoticeStrong => "In-person service",
                Key::NoticeBody => ": Monday to Friday, 8 a.m. to 5 p.m., at City Hall.",
                Key::ServicesHeading => "Services for residents",
                Key::Health => "Health",
                Key::HealthDescription => {
                    "Book appointments, find clinics, and follow health campaigns."
                }
                Key::Education => "Education",
                Key::EducationDescription => "Enrollment, schools, calendar, and school transport.",
                Key::Transport => "Transport",
                Key::TransportDescription => "Routes, schedules, and the mobility card.",
                Key::Environment => "Environment",
                Key::EnvironmentDescription => "Recycling, reports, and environmental licensing.",
                Key::NewsHeading => "Latest news",
                Key::NewsDateOne => "November 5, 2026",
                Key::NewsOneTitle => "City Hall delivers a new bus fleet",
                Key::NewsOneBody => {
                    "The vehicles will begin operating on municipal routes next week."
                }
                Key::NewsDateTwo => "November 4, 2026",
                Key::NewsTwoTitle => "Health fair takes place this Saturday",
                Key::NewsTwoBody => "Free consultations, vaccinations, and nutritional guidance.",
                Key::Address => "100 República Square — Downtown",
                Key::Hours => "Hours: Monday to Friday, 8 a.m. to 5 p.m.",
                Key::Accessibility => "Accessibility",
                Key::Privacy => "Privacy",
                Key::Sitemap => "Sitemap",
                Key::HeroEyebrow => "Citizen portal",
                Key::HeroTitle => "Public services, news, and participation",
                Key::HeroBody => {
                    "Find the service you need, follow the city government's work, and contact the municipal administration."
                }
                Key::HeroButton => "Find a service",
            },
            Locale::Es => match self.key {
                Key::Skip => "Saltar al contenido",
                Key::Title => "Portal del Ayuntamiento de Ejemplo",
                Key::Description => {
                    "Servicios, noticias y canales de atención del Ayuntamiento de Ejemplo."
                }
                Key::GovernmentName => "Ayuntamiento de Ejemplo",
                Key::MainNav => "Navegación principal",
                Key::FooterNav => "Navegación del pie de página",
                Key::LanguageNav => "Idioma",
                Key::Home => "Inicio",
                Key::Services => "Servicios",
                Key::News => "Noticias",
                Key::Transparency => "Transparencia",
                Key::Contact => "Contacto",
                Key::NoticeLabel => "Aviso de atención",
                Key::NoticeStrong => "Atención presencial",
                Key::NoticeBody => {
                    ": de lunes a viernes, de 8:00 a 17:00, en el Palacio Municipal."
                }
                Key::ServicesHeading => "Servicios para la ciudadanía",
                Key::Health => "Salud",
                Key::HealthDescription => "Programe citas, consulte centros y siga las campañas.",
                Key::Education => "Educación",
                Key::EducationDescription => {
                    "Matrículas, escuelas, calendario y transporte escolar."
                }
                Key::Transport => "Transporte",
                Key::TransportDescription => "Líneas, horarios y tarjeta de movilidad.",
                Key::Environment => "Medio ambiente",
                Key::EnvironmentDescription => {
                    "Recogida selectiva, denuncias y licencias ambientales."
                }
                Key::NewsHeading => "Últimas noticias",
                Key::NewsDateOne => "5 de noviembre de 2026",
                Key::NewsOneTitle => "El Ayuntamiento entrega una nueva flota de autobuses",
                Key::NewsOneBody => {
                    "Los vehículos comenzarán a operar en las líneas municipales la próxima semana."
                }
                Key::NewsDateTwo => "4 de noviembre de 2026",
                Key::NewsTwoTitle => "La feria de salud atiende este sábado",
                Key::NewsTwoBody => "Atenciones gratuitas, vacunación y orientación nutricional.",
                Key::Address => "Plaza de la República, 100 — Centro",
                Key::Hours => "Horario: de lunes a viernes, de 8:00 a 17:00",
                Key::Accessibility => "Accesibilidad",
                Key::Privacy => "Privacidad",
                Key::Sitemap => "Mapa del sitio",
                Key::HeroEyebrow => "Portal de la ciudadanía",
                Key::HeroTitle => "Servicios públicos, noticias y participación",
                Key::HeroBody => {
                    "Encuentre el servicio que necesita, siga las acciones del ayuntamiento y hable con la administración municipal."
                }
                Key::HeroButton => "Encontrar un servicio",
            },
            Locale::Fr => match self.key {
                Key::Skip => "Aller au contenu",
                Key::Title => "Portail de la Mairie d'Exemple",
                Key::Description => {
                    "Services, actualités et canaux de contact de la Mairie d'Exemple."
                }
                Key::GovernmentName => "Mairie d'Exemple",
                Key::MainNav => "Navigation principale",
                Key::FooterNav => "Navigation du pied de page",
                Key::LanguageNav => "Langue",
                Key::Home => "Accueil",
                Key::Services => "Services",
                Key::News => "Actualités",
                Key::Transparency => "Transparence",
                Key::Contact => "Contact",
                Key::NoticeLabel => "Avis d'accueil",
                Key::NoticeStrong => "Accueil en personne",
                Key::NoticeBody => " : du lundi au vendredi, de 8 h à 17 h, à l'hôtel de ville.",
                Key::ServicesHeading => "Services aux habitants",
                Key::Health => "Santé",
                Key::HealthDescription => {
                    "Prenez rendez-vous, trouvez des centres et suivez les campagnes."
                }
                Key::Education => "Éducation",
                Key::EducationDescription => {
                    "Inscriptions, écoles, calendrier et transport scolaire."
                }
                Key::Transport => "Transports",
                Key::TransportDescription => "Lignes, horaires et carte de mobilité.",
                Key::Environment => "Environnement",
                Key::EnvironmentDescription => {
                    "Collecte sélective, signalements et autorisations environnementales."
                }
                Key::NewsHeading => "Dernières actualités",
                Key::NewsDateOne => "5 novembre 2026",
                Key::NewsOneTitle => "La mairie livre une nouvelle flotte de bus",
                Key::NewsOneBody => {
                    "Les véhicules commenceront à circuler sur les lignes municipales la semaine prochaine."
                }
                Key::NewsDateTwo => "4 novembre 2026",
                Key::NewsTwoTitle => "La foire de la santé a lieu ce samedi",
                Key::NewsTwoBody => {
                    "Consultations gratuites, vaccination et conseils nutritionnels."
                }
                Key::Address => "100, place de la République — Centre-ville",
                Key::Hours => "Horaires : du lundi au vendredi, de 8 h à 17 h",
                Key::Accessibility => "Accessibilité",
                Key::Privacy => "Confidentialité",
                Key::Sitemap => "Plan du site",
                Key::HeroEyebrow => "Portail citoyen",
                Key::HeroTitle => "Services publics, actualités et participation",
                Key::HeroBody => {
                    "Trouvez le service dont vous avez besoin, suivez les actions de la mairie et contactez l'administration municipale."
                }
                Key::HeroButton => "Trouver un service",
            },
        }
    }
}

impl Message {
    fn text(locale: Locale, key: Key) -> Text<'static> {
        Text::msg(Self { locale, key })
    }
}

const fn msg(locale: Locale, key: Key) -> Message {
    Message { locale, key }
}

struct GovernmentOrganization;

impl JsonGraph for GovernmentOrganization {
    fn write_json(&self, out: &mut String) {
        out.push_str(
            r#"{"@context":"https://schema.org","@type":"GovernmentOrganization","name":"Prefeitura de Exemplo","url":"https://prefeitura.exemplo.gov.br/"}"#,
        );
    }
}

fn assets() -> Assets {
    [
        Asset::style("/assets/portal.css"),
        Asset::script("/assets/portal.js"),
    ]
    .into_iter()
    .collect()
}

fn alternates() -> Vec<Alternate<'static>> {
    let mut alternates: Vec<Alternate<'static>> = Locale::ALL
        .iter()
        .map(|candidate| {
            Alternate::new(
                candidate.lang(),
                ExternalUrl::parse(candidate.canonical()).expect("URL canônica"),
            )
        })
        .collect();
    alternates.push(Alternate::x_default(
        ExternalUrl::parse(Locale::PtBr.canonical()).expect("URL canônica"),
    ));
    alternates
}

fn language_switch(locale: Locale) -> Markup {
    html! {
        nav class=(Class::new("language-switch"))
            aria_label=(Label::text(msg(locale, Key::LanguageNav))) {
            ul {
                for candidate in Locale::ALL {
                    li {
                        @if candidate == locale {
                            a href=(Href::internal(LUrl::new(candidate.path()).expect("rota de locale")))
                                lang=(candidate.lang())
                                hreflang=(candidate.lang())
                                rel="alternate"
                                class=(Class::new("lang-link is-current"))
                                aria_current="page" {
                                Text::new(candidate.label())
                            }
                        } @else {
                            a href=(Href::internal(LUrl::new(candidate.path()).expect("rota de locale")))
                                lang=(candidate.lang())
                                hreflang=(candidate.lang())
                                rel="alternate"
                                class=(Class::new("lang-link")) {
                                Text::new(candidate.label())
                            }
                        }
                    }
                }
            }
        }
    }
}

fn header(locale: Locale) -> Markup {
    html! {
        header class=(Class::new("site-header")) {
            div class=(Class::new("wrap header-inner")) {
                a class=(Class::new("brand")) href="/" {
                    span class=(Class::new("brand-mark")) aria_hidden="true" {
                        Text::new("P")
                    }
                    Message::text(locale, Key::GovernmentName)
                }
                nav aria_label=(Label::text(msg(locale, Key::MainNav))) {
                    ul class=(Class::new("main-nav")) {
                        li {
                            a href="/" aria_current="page" {
                                Message::text(locale, Key::Home)
                            }
                        }
                        li { a href="/servicos" { Message::text(locale, Key::Services) } }
                        li { a href="/noticias" { Message::text(locale, Key::News) } }
                        li {
                            a href="/transparencia" {
                                Message::text(locale, Key::Transparency)
                            }
                        }
                        li { a href="/contato" { Message::text(locale, Key::Contact) } }
                    }
                }
                (language_switch(locale))
            }
        }
    }
}

fn notice(locale: Locale) -> Markup {
    html! {
        aside class=(Class::new("notice")) aria_label=(Label::text(msg(locale, Key::NoticeLabel))) {
            p {
                strong { Message::text(locale, Key::NoticeStrong) }
                Message::text(locale, Key::NoticeBody)
            }
        }
    }
}

fn services(locale: Locale) -> Markup {
    html! {
        section class=(Class::new("section")) {
            div class=(Class::new("wrap")) {
                h2 { Message::text(locale, Key::ServicesHeading) }
                ul class=(Class::new("card-grid")) {
                    li {
                        a class=(Class::new("card")) href="/servicos/saude" {
                            h3 { Message::text(locale, Key::Health) }
                            p { Message::text(locale, Key::HealthDescription) }
                        }
                    }
                    li {
                        a class=(Class::new("card")) href="/servicos/educacao" {
                            h3 { Message::text(locale, Key::Education) }
                            p { Message::text(locale, Key::EducationDescription) }
                        }
                    }
                    li {
                        a class=(Class::new("card")) href="/servicos/transporte" {
                            h3 { Message::text(locale, Key::Transport) }
                            p { Message::text(locale, Key::TransportDescription) }
                        }
                    }
                    li {
                        a class=(Class::new("card")) href="/servicos/meio-ambiente" {
                            h3 { Message::text(locale, Key::Environment) }
                            p { Message::text(locale, Key::EnvironmentDescription) }
                        }
                    }
                }
            }
        }
    }
}

fn news(locale: Locale) -> Markup {
    let first = Date::new(2026, 11, 5).expect("data fixa");
    let second = Date::new(2026, 11, 4).expect("data fixa");
    html! {
        section class=(Class::new("section section-alt")) {
            div class=(Class::new("wrap")) {
                h2 { Message::text(locale, Key::NewsHeading) }
                ul class=(Class::new("news-list")) {
                    li {
                        article {
                            p class=(Class::new("date")) {
                                time datetime=(first) {
                                    Message::text(locale, Key::NewsDateOne)
                                }
                            }
                            h3 {
                                a href="/noticias/nova-frota" {
                                    Message::text(locale, Key::NewsOneTitle)
                                }
                            }
                            p { Message::text(locale, Key::NewsOneBody) }
                        }
                    }
                    li {
                        article {
                            p class=(Class::new("date")) {
                                time datetime=(second) {
                                    Message::text(locale, Key::NewsDateTwo)
                                }
                            }
                            h3 {
                                a href="/noticias/feira-saude" {
                                    Message::text(locale, Key::NewsTwoTitle)
                                }
                            }
                            p { Message::text(locale, Key::NewsTwoBody) }
                        }
                    }
                }
            }
        }
    }
}

fn footer(locale: Locale) -> Markup {
    html! {
        footer class=(Class::new("site-footer")) {
            div class=(Class::new("wrap footer-grid")) {
                div {
                    h2 { Message::text(locale, Key::GovernmentName) }
                    p { Message::text(locale, Key::Address) }
                    p { Message::text(locale, Key::Hours) }
                }
                nav aria_label=(Label::text(msg(locale, Key::FooterNav))) {
                    ul {
                        li { a href="/acessibilidade" { Message::text(locale, Key::Accessibility) } }
                        li { a href="/privacidade" { Message::text(locale, Key::Privacy) } }
                        li { a href="/mapa-do-site" { Message::text(locale, Key::Sitemap) } }
                    }
                }
            }
        }
    }
}

fn page(locale: Locale) -> Page {
    let regions = Regions::new()
        .header(header(locale))
        .before_main(notice(locale))
        .footer(footer(locale));

    let data = ShellData::new(
        Message::text(locale, Key::Title),
        Message::text(locale, Key::GovernmentName),
        locale.lang(),
    )
    .description(Message::text(locale, Key::Description))
    .canonical(ExternalUrl::parse(locale.canonical()).expect("URL canônica"))
    .dir(locale.dir())
    .alternates(alternates())
    .skip(Message::text(locale, Key::Skip))
    .assets(assets())
    .schemas(vec![JsonLd::graph(&GovernmentOrganization)])
    .regions(regions);

    let main = html! {
        section class=(Class::new("hero")) {
            div class=(Class::new("wrap")) {
                p class=(Class::new("eyebrow")) { Message::text(locale, Key::HeroEyebrow) }
                h2 { Message::text(locale, Key::HeroTitle) }
                p { Message::text(locale, Key::HeroBody) }
                a class=(Class::new("button")) href="/servicos" {
                    Message::text(locale, Key::HeroButton)
                }
            }
        }
        (Markup::concat([services(locale), news(locale)]))
    };

    Page::new(&data, View::new(main))
}

fn main() -> io::Result<()> {
    let locale = match std::env::args().nth(1) {
        None => Locale::PtBr,
        Some(value) => Locale::from_arg(&value).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "locale inválido; use pt-BR, en, es ou fr",
            )
        })?,
    };
    let page = page(locale);
    let mut out = io::stdout().lock();
    out.write_all(page.markup().as_str().as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Locale, page};

    #[test]
    fn locales_change_lang_title_and_alternates() {
        for (locale, lang, title, services) in [
            (
                Locale::PtBr,
                "pt-BR",
                "Portal da Prefeitura de Exemplo",
                "Serviços para o cidadão",
            ),
            (
                Locale::En,
                "en",
                "City Hall of Example Portal",
                "Services for residents",
            ),
            (
                Locale::Es,
                "es",
                "Portal del Ayuntamiento de Ejemplo",
                "Servicios para la ciudadanía",
            ),
            (
                Locale::Fr,
                "fr",
                "Portail de la Mairie d&#x27;Exemple",
                "Services aux habitants",
            ),
        ] {
            let page = page(locale);
            let markup = page.markup().as_str();
            assert!(
                markup.contains(&format!("<html lang=\"{lang}\" dir=\"ltr\">")),
                "{lang}"
            );
            assert!(
                markup.contains(&format!("<title>{title}</title>")),
                "{lang}"
            );
            assert!(
                markup.contains(&format!(">{services}</h2>")),
                "catálogo de {lang}"
            );
            assert!(
                markup.contains(&format!(
                    "<link rel=\"canonical\" href=\"{}\">",
                    locale.canonical()
                )),
                "canonical de {lang}"
            );
            assert!(markup.contains("hreflang=\"fr\""), "{lang}");
            assert!(markup.contains("hreflang=\"x-default\""), "{lang}");
            assert!(markup.contains("class=\"language-switch\""), "{lang}");
            for candidate in Locale::ALL {
                assert!(
                    markup.contains(&format!("href=\"{}\"", candidate.path())),
                    "link {} em {lang}",
                    candidate.path()
                );
            }
            assert!(
                markup.contains(&format!("aria-current=\"page\" lang=\"{lang}\"")),
                "locale atual {lang}"
            );
        }
    }
}
