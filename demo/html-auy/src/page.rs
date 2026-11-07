//! Portal EXAMPLE.html implementado com `html!` + o contrato de valores do crate.

use html::{
    Alt, AttrVal, Class, Fragment, HeightPx, Href, Label, Markup, Src, Text, WidthPx, html,
};

const HEAD: &str = include_str!("../assets/head.html");

use crate::content::{CONTROLS, EXEC_QUICK, GAZETTE, LEG_QUICK, Msg, NAV, SERVICES};

fn hash() -> Href<'static> {
    Href::fragment(Fragment::new("#").expect("fragmento válido"))
}

fn anchor(fragment: &'static str) -> Href<'static> {
    Href::fragment(Fragment::new(fragment).expect("fragmento válido"))
}

fn header_controls() -> Markup {
    html! {
        div class="flex items-center gap-space-xs" {
            for control in CONTROLS {
                button class=(Class::new(control.class)) title=(AttrVal::new(control.title)) type="button" {
                    @if !control.icon.is_empty() {
                        span class="material-symbols-outlined text-[14px]" {
                            (Text::new(control.icon))
                        }
                    }
                    span { (Text::new(control.label)) }
                }
            }
        }
    }
}

fn service_grid() -> Markup {
    html! {
        div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-space-md" {
            for service in SERVICES {
                a class="group p-space-lg bg-surface-container-lowest rounded shadow-sm hover:shadow-md transition-all flex flex-col justify-between" href=(hash()) {
                    div {
                        div class="flex items-center justify-between mb-3" {
                            div class=(Class::new(if service.alternate {
                                "bg-surface-container flex group-hover:bg-secondary group-hover:text-on-secondary h-12 items-center justify-center rounded text-secondary transition-colors w-12"
                            } else {
                                "bg-surface-container flex group-hover:bg-primary group-hover:text-on-primary h-12 items-center justify-center rounded text-primary transition-colors w-12"
                            })) {
                                span class="material-symbols-outlined text-[26px]" {
                                    (Text::new(service.icon))
                                }
                            }
                            span class=(Class::new(if service.alternate {
                                "bg-surface-container-high font-caption font-semibold px-2 py-0.5 rounded text-caption text-primary tracking-wider uppercase"
                            } else {
                                "bg-secondary-fixed font-caption font-semibold px-2 py-0.5 rounded text-caption text-secondary tracking-wider uppercase"
                            })) {
                                (Text::new(service.badge))
                            }
                        }
                        h3 class=(Class::new(if service.alternate {
                            "font-bold font-title-md group-hover:text-secondary mb-1 text-on-surface text-title-md"
                        } else {
                            "font-bold font-title-md group-hover:text-primary mb-1 text-on-surface text-title-md"
                        })) {
                            (Text::new(service.title))
                        }
                        p class="font-body-sm text-body-sm text-on-surface-variant" {
                            (Text::new(service.body))
                        }
                    }
                    div class="mt-4 pt-3 flex items-center justify-between text-caption font-caption text-on-surface-variant" {
                        span { (Text::new(service.footer)) }
                        span class="material-symbols-outlined text-[18px] text-secondary group-hover:translate-x-1 transition-transform" {
                            (Text::new("arrow_forward"))
                        }
                    }
                }
            }
        }
    }
}

fn header() -> Markup {
    html! {
    header class="fixed top-0 w-full z-50 bg-surface-container-lowest shadow-[0_1px_8px_rgba(0,0,0,0.04)]" {
        div class="bg-primary text-on-primary border-b border-outline-variant/20" {
            div class="max-w-[1440px] mx-auto px-6 lg:px-12 h-9 flex items-center justify-between font-caption text-caption" {
                div class="flex items-center gap-space-lg" {
                    span class="flex items-center gap-space-xs" {
                        span class="w-3.5 h-2.5 bg-secondary inline-block rounded-none border border-outline-variant/30" {
                        }
                        span class="font-label-sm text-label-sm tracking-wider uppercase opacity-90" {
                            (Text::new("República Federativa do Brasil"))
                        }
                    }
                    div class="hidden md:flex items-center gap-space-md opacity-80" {
                        a class="hover:text-surface transition-colors" href=(anchor("#conteudo")) {
                            (Text::new("Ir para o Conteúdo [1]"))
                        }
                        a class="hover:text-surface transition-colors" href=(anchor("#menu")) {
                            (Text::new("Ir para o Menu [2]"))
                        }
                        a class="hover:text-surface transition-colors" href=(anchor("#busca")) {
                            (Text::new("Ir para a Busca [3]"))
                        }
                    }
                }
                div class="flex items-center gap-space-md" {
                    (header_controls())
                    div class="h-3 w-px bg-outline-variant/30" {
                    }
                    a class="font-label-sm text-label-sm hover:underline flex items-center gap-1" data_path="ouvidoria-e-sic" href=(hash()) {
                        span class="material-symbols-outlined text-[14px]" {
                            (Text::new("info"))
                        }
                        span {
                            (Text::new("LAI nº 12.527"))
                        }
                    }
                }
            }
        }
        div class="bg-surface-container-lowest border-b border-outline-variant/30" {
            div class="max-w-[1440px] mx-auto px-6 lg:px-12 h-20 flex items-center justify-between gap-space-md" {
                div class="flex items-center gap-space-md" {
                    div class="w-12 h-12 rounded bg-surface-container flex items-center justify-center border border-outline-variant/40" {
                        span class="material-symbols-outlined text-primary text-[28px]" {
                            (Text::new("account_balance"))
                        }
                    }
                    div {
                        div class="font-label-sm text-label-sm uppercase tracking-widest text-secondary font-bold" {
                            (Text::new("Portal Cívico Unificado"))
                        }
                        div class="font-headline-sm text-headline-sm font-headline-sm text-on-surface leading-tight" {
                            (Text::new("Governo Municipal"))
                        }
                    }
                }
                div class="hidden xl:flex items-center bg-surface-container-low p-1 rounded border border-outline-variant/30" {
                    a class="flex items-center gap-2 px-space-md py-1.5 rounded font-label-md text-label-md font-semibold bg-surface-container-lowest text-primary shadow-[0_1px_4px_rgba(0,0,0,0.05)] border border-outline-variant/40" data_path="prefeitura-secretarias" href=(hash()) {
                        span class="material-symbols-outlined text-[18px] text-primary" {
                            (Text::new("domain"))
                        }
                        span {
                            (Text::new("Poder Executivo "))
                            span class="font-caption text-caption opacity-75 font-normal" {
                                (Text::new("(Prefeitura)"))
                            }
                        }
                    }
                    a class="flex items-center gap-2 px-space-md py-1.5 rounded font-label-md text-label-md text-on-surface-variant hover:text-on-surface hover:bg-surface-container-highest transition-colors" data_path="camara-vereadores" href=(hash()) {
                        span class="material-symbols-outlined text-[18px] text-secondary" {
                            (Text::new("gavel"))
                        }
                        span {
                            (Text::new("Poder Legislativo "))
                            span class="font-caption text-caption opacity-75 font-normal" {
                                (Text::new("(Câmara)"))
                            }
                        }
                    }
                }
                div class="flex-1 max-w-md hidden lg:block" id="busca" {
                    div class="relative flex items-center" {
                        input class="w-full bg-surface-container-low border border-outline-variant/40 rounded h-11 pl-10 pr-4 font-body-sm text-body-sm text-on-surface placeholder:text-on-surface-variant/70 focus:outline-none focus:border-primary focus:bg-surface-container-lowest transition-all" placeholder=(AttrVal::new("Buscar leis, processos, serviços, diários oficiais...")) type="text" name="q" aria_label=(Label::text(Msg::Search));
                        span class="material-symbols-outlined absolute left-3 text-on-surface-variant text-[20px]" {
                            (Text::new("search"))
                        }
                    }
                }
                div class="flex items-center gap-space-md" {
                    a class="hidden sm:flex items-center gap-1.5 px-space-md h-10 rounded border border-secondary text-secondary hover:bg-secondary-fixed hover:text-on-secondary-fixed font-label-md text-label-md font-medium transition-colors" data_path="portal-da-transparencia" href=(hash()) {
                        span class="material-symbols-outlined text-[18px]" {
                            (Text::new("policy"))
                        }
                        span {
                            (Text::new("Transparência"))
                        }
                    }
                    div class="w-8 h-8 rounded-full bg-primary flex items-center justify-center" {
                        span class="material-symbols-outlined text-on-primary text-[18px]" {
                            (Text::new("person"))
                        }
                    }
                }
            }
        }
        div class="bg-surface-container-low border-b border-outline-variant/30" id="menu" {
            div class="max-w-[1440px] mx-auto px-6 lg:px-12" {
                nav class="flex items-center gap-space-lg overflow-x-auto h-12 font-label-md text-label-md scrollbar-none" data_active_classes="text-primary border-b-2 border-primary font-bold" {
                    for (path, label) in NAV {
                        a class="text-on-surface-variant hover:text-on-surface py-3 inline-block whitespace-nowrap transition-colors" data_path=(AttrVal::new(path)) href=(hash()) {
                            (Text::new(label))
                        }
                    }
                }
            }
        }
    }
        }
}

fn main_content() -> Markup {
    html! {
    div class="flex flex-col w-full" {
        section class="relative w-full bg-surface-container-low px-6 lg:px-12 py-10 shadow-sm overflow-hidden" aria_label=(Label::text(Msg::Hero)) {
            div class="absolute -right-24 -top-24 w-96 h-96 rounded-full bg-secondary-fixed/40 blur-3xl pointer-events-none" {
            }
            div class="absolute -left-20 bottom-0 w-80 h-80 rounded-full bg-surface-container-high/60 blur-2xl pointer-events-none" {
            }
            div class="max-w-[1280px] mx-auto relative z-10" {
                div class="flex flex-wrap items-center gap-space-sm mb-4" {
                    span class="inline-flex items-center gap-1.5 px-3 py-1 rounded bg-surface-container-lowest text-primary text-label-sm font-label-sm uppercase tracking-widest shadow-sm" {
                        span class="w-2 h-2 rounded-full bg-secondary" {
                        }
                        (Text::new(" Ambiente Cívico Integrado "))
                    }
                    span class="text-caption font-caption text-on-surface-variant flex items-center gap-1" {
                        span class="material-symbols-outlined text-[14px] text-secondary" {
                            (Text::new("verified"))
                        }
                        (Text::new(" Emissão em tempo real com certificação ICP-Brasil "))
                    }
                }
                div class="grid grid-cols-1 lg:grid-cols-12 gap-space-lg items-end" {
                    div class="lg:col-span-8" {
                        h1 class="font-headline-xl text-headline-xl text-on-surface leading-tight tracking-tight mb-space-sm" {
                            (Text::new("Transparência, Cidadania e Gestão Democrática Municipal"))
                        }
                        p class="font-body-lg text-body-lg text-on-surface-variant max-w-3xl" {
                            (Text::new(" Acesso unificado e irrestrito aos atos oficiais, tramitações e deliberações conjuntas do "))
                            strong class="text-on-surface font-semibold" {
                                (Text::new("Poder Executivo (Prefeitura)"))
                            }
                            (Text::new(" e do "))
                            strong class="text-on-surface font-semibold" {
                                (Text::new("Poder Legislativo (Câmara de Vereadores)"))
                            }
                            (Text::new(" em consonância com a Lei de Acesso à Informação. "))
                        }
                    }
                    div class="lg:col-span-4 flex lg:justify-end" {
                        div class="bg-surface-container-lowest p-space-md rounded shadow-sm text-left w-full lg:w-auto min-w-[260px]" {
                            div class="text-caption font-caption text-on-surface-variant uppercase tracking-wider mb-1" {
                                (Text::new("Status Operacional dos Poderes"))
                            }
                            div class="flex items-center justify-between text-body-sm font-body-sm text-on-surface py-1" {
                                span class="flex items-center gap-2" {
                                    span class="w-2 h-2 rounded-full bg-secondary" {
                                    }
                                    (Text::new(" Paço Municipal "))
                                }
                                span class="font-label-sm text-label-sm text-secondary font-medium" {
                                    (Text::new("Expediente Normal"))
                                }
                            }
                            div class="flex items-center justify-between text-body-sm font-body-sm text-on-surface py-1" {
                                span class="flex items-center gap-2" {
                                    span class="w-2 h-2 rounded-full bg-secondary" {
                                    }
                                    (Text::new(" Plenário da Câmara "))
                                }
                                span class="font-label-sm text-label-sm text-secondary font-medium" {
                                    (Text::new("Pauta Publicada"))
                                }
                            }
                        }
                    }
                }
                div class="mt-8 bg-surface-container-lowest p-space-md lg:p-space-lg rounded shadow-md" {
                    label class="block font-label-md text-label-md text-on-surface font-semibold mb-2" for="input-civic-search" {
                        (Text::new(" Pesquisa Geral Integrada nos Dois Poderes "))
                    }
                    div class="relative flex items-center" {
                        span class="material-symbols-outlined absolute left-4 text-on-surface-variant text-[24px]" {
                            (Text::new("search"))
                        }
                        input class="w-full bg-surface-container-low rounded h-14 pl-12 pr-32 font-body-md text-body-md text-on-surface placeholder:text-on-surface-variant/70 focus:outline-none focus:bg-surface-container-lowest shadow-inner transition-all" id="input-civic-search" placeholder=(AttrVal::new("Buscar leis, decretos, processos, vereadores, obras, certidões ou licitações...")) type="text" name="q";
                        button class="absolute right-2 px-6 h-10 rounded bg-primary text-on-primary font-label-md text-label-md font-semibold hover:bg-primary-container transition-colors flex items-center gap-2" type="button" {
                            span {
                                (Text::new("Consultar"))
                            }
                            span class="material-symbols-outlined text-[18px]" {
                                (Text::new("arrow_forward"))
                            }
                        }
                    }
                    div class="mt-4 flex flex-wrap items-center gap-2" {
                        span class="text-caption font-caption text-on-surface-variant uppercase tracking-wider mr-2" {
                            (Text::new("Consultas Rápidas:"))
                        }
                        a class="px-3 py-1 rounded bg-surface-container text-primary hover:bg-secondary-fixed hover:text-on-secondary-fixed text-label-sm font-label-sm font-medium transition-colors" href=(hash()) {
                            (Text::new(" IPTU 2026 "))
                        }
                        a class="px-3 py-1 rounded bg-surface-container text-primary hover:bg-secondary-fixed hover:text-on-secondary-fixed text-label-sm font-label-sm font-medium transition-colors" href=(hash()) {
                            (Text::new(" Diário Oficial de Hoje "))
                        }
                        a class="px-3 py-1 rounded bg-surface-container text-primary hover:bg-secondary-fixed hover:text-on-secondary-fixed text-label-sm font-label-sm font-medium transition-colors" href=(hash()) {
                            (Text::new(" Sessões da Câmara "))
                        }
                        a class="px-3 py-1 rounded bg-surface-container text-primary hover:bg-secondary-fixed hover:text-on-secondary-fixed text-label-sm font-label-sm font-medium transition-colors" href=(hash()) {
                            (Text::new(" Pauta de Votação "))
                        }
                        a class="px-3 py-1 rounded bg-surface-container text-primary hover:bg-secondary-fixed hover:text-on-secondary-fixed text-label-sm font-label-sm font-medium transition-colors" href=(hash()) {
                            (Text::new(" Concursos & Editais "))
                        }
                        a class="px-3 py-1 rounded bg-surface-container text-primary hover:bg-secondary-fixed hover:text-on-secondary-fixed text-label-sm font-label-sm font-medium transition-colors" href=(hash()) {
                            (Text::new(" Carta de Serviços SUS "))
                        }
                    }
                }
            }
        }
        section class="max-w-[1280px] mx-auto w-full px-6 lg:px-12 py-12" aria_label=(Label::text(Msg::Powers)) {
            div class="flex flex-col md:flex-row md:items-end justify-between mb-8 gap-4" {
                div {
                    div class="font-label-sm text-label-sm text-secondary uppercase tracking-widest font-bold" {
                        (Text::new("Pilares da República Municipal"))
                    }
                    h2 class="font-headline-lg text-headline-lg text-on-surface" {
                        (Text::new("Poderes Constituídos do Município"))
                    }
                }
                p class="font-body-sm text-body-sm text-on-surface-variant max-w-md" {
                    (Text::new(" Separação harmônica com canais diretos para acompanhamento orçamentário, tramitação de projetos e agendas dos mandatários. "))
                }
            }
            div class="grid grid-cols-1 lg:grid-cols-2 gap-space-lg" {
                article class="bg-surface-container-lowest rounded shadow-md overflow-hidden flex flex-col justify-between group" {
                    div {
                        div class="relative h-56 w-full overflow-hidden bg-primary-container" {
                            img class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500 opacity-90" data_alt=(AttrVal::new("Fachada neoclássica sóbria do Paço Municipal com bandeira oficial hasteada em um mastro de bronze sob luz natural e jardins cívicos bem cuidados.")) src=(Src::parse("https://lh3.googleusercontent.com/aida-public/AB6AXuCwsZDqi89M8FqrRuKKYOaevhA0bfJVndyu2aqAS6ZD1IfMr9_7tpkQkX6WgWIScsPX5gAuaC83HehrTrqqS1FB2SMDALreGsG1oW9cBEhH1malTaumU7mODgW5QomhMAjQ3TcIT8-EARYEOT06QUQN79NxKZd_UtU7IOqOwzienS8QddKAmtHihxgSX1ARrwY7cSmHEyI1sMr1ZH38-FCELuG-CJ1HmKIiK9cthw8").expect("URL de imagem válida")) alt=(Alt::new("Fachada neoclássica sóbria do Paço Municipal com bandeira oficial hasteada em um mastro de bronze sob luz natural e jardins cívicos bem cuidados.")) width=(WidthPx::new(1200)) height=(HeightPx::new(630));
                            div class="absolute inset-0 bg-gradient-to-t from-primary via-primary/40 to-transparent" {
                            }
                            div class="absolute bottom-4 left-6 right-6 flex items-center justify-between text-on-primary" {
                                div class="flex items-center gap-3" {
                                    div class="w-10 h-10 rounded bg-surface-container-lowest text-primary flex items-center justify-center shadow" {
                                        span class="material-symbols-outlined text-[24px]" {
                                            (Text::new("domain"))
                                        }
                                    }
                                    div {
                                        span class="font-label-sm text-label-sm uppercase tracking-wider text-surface-variant" {
                                            (Text::new("Poder Executivo"))
                                        }
                                        h3 class="font-headline-sm text-headline-sm font-bold text-on-primary" {
                                            (Text::new("Prefeitura Municipal"))
                                        }
                                    }
                                }
                                span class="px-2.5 py-1 rounded bg-secondary-fixed text-on-secondary-fixed font-label-sm text-label-sm font-semibold" {
                                    (Text::new(" Paço da Cidade "))
                                }
                            }
                        }
                        div class="p-space-lg space-y-space-md" {
                            p class="font-body-md text-body-md text-on-surface-variant" {
                                (Text::new(" Administração direta e indireta responsável pela execução de políticas públicas, infraestrutura urbana, saúde, educação infantil e zeladoria. "))
                            }
                            div class="grid grid-cols-2 sm:grid-cols-4 gap-2 pt-2" {
                                for (icon, label) in EXEC_QUICK {
                                    a class="p-3 bg-surface-container-low rounded text-center hover:bg-surface-container-high transition-colors" href=(hash()) {
                                        span class="material-symbols-outlined text-primary text-[22px] mb-1" { (Text::new(icon)) }
                                        span class="block font-label-sm text-label-sm text-on-surface font-semibold" { (Text::new(label)) }
                                    }
                                }
                            }
                            div class="space-y-2 pt-2" {
                                div class="flex items-center justify-between text-body-sm font-body-sm py-2 px-3 bg-surface-container-low rounded" {
                                    span class="flex items-center gap-2 text-on-surface font-medium" {
                                        span class="material-symbols-outlined text-[18px] text-secondary" {
                                            (Text::new("map"))
                                        }
                                        (Text::new(" Obras em Andamento no Município "))
                                    }
                                    span class="font-label-sm text-label-sm font-bold text-primary" {
                                        (Text::new("34 ativas"))
                                    }
                                }
                                div class="flex items-center justify-between text-body-sm font-body-sm py-2 px-3 bg-surface-container-low rounded" {
                                    span class="flex items-center gap-2 text-on-surface font-medium" {
                                        span class="material-symbols-outlined text-[18px] text-secondary" {
                                            (Text::new("assignment_turned_in"))
                                        }
                                        (Text::new(" Carta Geral de Serviços ao Cidadão "))
                                    }
                                    span class="font-label-sm text-label-sm font-bold text-primary" {
                                        (Text::new("142 itens"))
                                    }
                                }
                            }
                        }
                    }
                    div class="p-space-lg pt-0 flex flex-wrap gap-space-sm items-center justify-between" {
                        a class="inline-flex items-center gap-2 px-space-md py-2.5 rounded bg-primary text-on-primary font-label-md text-label-md font-semibold hover:bg-primary-container transition-colors" href=(hash()) {
                            span {
                                (Text::new("Acessar Gabinete & Secretarias"))
                            }
                            span class="material-symbols-outlined text-[16px]" {
                                (Text::new("chevron_right"))
                            }
                        }
                        a class="inline-flex items-center gap-1.5 text-label-md font-label-md text-secondary hover:underline font-semibold" href=(hash()) {
                            span class="material-symbols-outlined text-[18px]" {
                                (Text::new("calendar_month"))
                            }
                            (Text::new(" Agendamento no Paço "))
                        }
                    }
                }
                article class="bg-surface-container-lowest rounded shadow-md overflow-hidden flex flex-col justify-between group" {
                    div {
                        div class="relative h-56 w-full overflow-hidden bg-primary" {
                            img class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500 opacity-90" data_alt=(AttrVal::new("Plenário solene da Câmara Municipal de Vereadores com bancadas de madeira nobre, microfones individuais e painel eletrônico de votação em sessão oficial.")) src=(Src::parse("https://lh3.googleusercontent.com/aida-public/AB6AXuDt3djx3rBGkF14Bo_1wftqD-Js-hx8DUxQkoNUj1abCenU-oi_1TGbtTPoDW6w3ZGy4-x4HMgImblyLbsgSusE8UEJdSIayg_9ugL-GIVrQSeW0ZwZ8KubF-ivHYamJOH7tOMbnE6oni00qCXRsmUiXxZZamGzMIa8l1oitcPTB1iu4i_K1S64n0Lg-LYVPzKoqhWaQiylyRPrrVD3Sse0-3w3Vr53WDStDapktv4").expect("URL de imagem válida")) alt=(Alt::new("Plenário solene da Câmara Municipal de Vereadores com bancadas de madeira nobre, microfones individuais e painel eletrônico de votação em sessão oficial.")) width=(WidthPx::new(1200)) height=(HeightPx::new(630));
                            div class="absolute inset-0 bg-gradient-to-t from-primary via-primary/50 to-transparent" {
                            }
                            div class="absolute bottom-4 left-6 right-6 flex items-center justify-between text-on-primary" {
                                div class="flex items-center gap-3" {
                                    div class="w-10 h-10 rounded bg-surface-container-lowest text-secondary flex items-center justify-center shadow" {
                                        span class="material-symbols-outlined text-[24px]" {
                                            (Text::new("gavel"))
                                        }
                                    }
                                    div {
                                        span class="font-label-sm text-label-sm uppercase tracking-wider text-surface-variant" {
                                            (Text::new("Poder Legislativo"))
                                        }
                                        h3 class="font-headline-sm text-headline-sm font-bold text-on-primary" {
                                            (Text::new("Câmara Municipal"))
                                        }
                                    }
                                }
                                span class="px-2.5 py-1 rounded bg-secondary-fixed text-on-secondary-fixed font-label-sm text-label-sm font-semibold" {
                                    (Text::new(" 15 Vereadores "))
                                }
                            }
                        }
                        div class="p-space-lg space-y-space-md" {
                            p class="font-body-md text-body-md text-on-surface-variant" {
                                (Text::new(" Órgão representativo popular incumbido de legislar, fiscalizar as contas do Executivo e debater as demandas da sociedade civil organizada. "))
                            }
                            div class="grid grid-cols-2 sm:grid-cols-4 gap-2 pt-2" {
                                for (icon, label) in LEG_QUICK {
                                    a class="p-3 bg-surface-container-low rounded text-center hover:bg-surface-container-high transition-colors" href=(hash()) {
                                        span class="material-symbols-outlined text-secondary text-[22px] mb-1" { (Text::new(icon)) }
                                        span class="block font-label-sm text-label-sm text-on-surface font-semibold" { (Text::new(label)) }
                                    }
                                }
                            }
                            div class="space-y-2 pt-2" {
                                div class="flex items-center justify-between text-body-sm font-body-sm py-2 px-3 bg-surface-container-low rounded" {
                                    span class="flex items-center gap-2 text-on-surface font-medium" {
                                        span class="material-symbols-outlined text-[18px] text-secondary" {
                                            (Text::new("mic"))
                                        }
                                        (Text::new(" Transmissão ao Vivo do Plenário "))
                                    }
                                    span class="font-label-sm text-label-sm font-semibold text-secondary flex items-center gap-1" {
                                        span class="w-2 h-2 rounded-full bg-secondary" {
                                        }
                                        (Text::new(" Canal Aberto HD "))
                                    }
                                }
                                div class="flex items-center justify-between text-body-sm font-body-sm py-2 px-3 bg-surface-container-low rounded" {
                                    span class="flex items-center gap-2 text-on-surface font-medium" {
                                        span class="material-symbols-outlined text-[18px] text-secondary" {
                                            (Text::new("query_stats"))
                                        }
                                        (Text::new(" Cota Parlamentar e Verbas de Gabinete "))
                                    }
                                    span class="font-label-sm text-label-sm font-bold text-primary" {
                                        (Text::new("100% Pública"))
                                    }
                                }
                            }
                        }
                    }
                    div class="p-space-lg pt-0 flex flex-wrap gap-space-sm items-center justify-between" {
                        a class="inline-flex items-center gap-2 px-space-md py-2.5 rounded bg-secondary text-on-secondary font-label-md text-label-md font-semibold hover:bg-on-secondary-fixed-variant transition-colors" href=(hash()) {
                            span {
                                (Text::new("Conhecer os 15 Vereadores"))
                            }
                            span class="material-symbols-outlined text-[16px]" {
                                (Text::new("chevron_right"))
                            }
                        }
                        a class="inline-flex items-center gap-1.5 text-label-md font-label-md text-primary hover:underline font-semibold" href=(hash()) {
                            span class="material-symbols-outlined text-[18px]" {
                                (Text::new("how_to_vote"))
                            }
                            (Text::new(" Pauta de Votações "))
                        }
                    }
                }
            }
        }
        section class="w-full bg-surface-container py-14" aria_label=(Label::text(Msg::Budget)) {
            div class="max-w-[1280px] mx-auto px-6 lg:px-12" {
                div class="flex flex-col lg:flex-row lg:items-end justify-between mb-8 gap-4" {
                    div {
                        div class="inline-flex items-center gap-2 px-2.5 py-0.5 rounded bg-surface-container-lowest text-primary text-label-sm font-label-sm font-bold uppercase tracking-wider mb-2" {
                            span class="material-symbols-outlined text-[16px] text-secondary" {
                                (Text::new("verified_user"))
                            }
                            (Text::new(" Transparência Fiscal em Tempo Real • Exercício 2025 "))
                        }
                        h2 class="font-headline-lg text-headline-lg text-on-surface" {
                            (Text::new("Execução Orçamentária e Balanço Constitucional"))
                        }
                    }
                    div class="flex flex-wrap items-center gap-2" {
                        button class="inline-flex items-center gap-1.5 px-3 py-2 bg-surface-container-lowest rounded text-primary hover:bg-surface-container-high text-label-sm font-label-sm font-semibold shadow-sm transition-colors" type="button" {
                            span class="material-symbols-outlined text-[16px]" {
                                (Text::new("picture_as_pdf"))
                            }
                            (Text::new(" RGF / RREO (PDF) "))
                        }
                        button class="inline-flex items-center gap-1.5 px-3 py-2 bg-surface-container-lowest rounded text-primary hover:bg-surface-container-high text-label-sm font-label-sm font-semibold shadow-sm transition-colors" type="button" {
                            span class="material-symbols-outlined text-[16px]" {
                                (Text::new("dataset"))
                            }
                            (Text::new(" Dados Abertos (CSV/API) "))
                        }
                        span class="inline-flex items-center gap-1 px-3 py-2 bg-secondary-fixed text-on-secondary-fixed rounded text-label-sm font-label-sm font-semibold" {
                            span class="material-symbols-outlined text-[16px]" {
                                (Text::new("check_circle"))
                            }
                            (Text::new(" TCE Homologado "))
                        }
                    }
                }
                div class="grid grid-cols-1 md:grid-cols-3 gap-space-md mb-6" {
                    div class="bg-surface-container-lowest p-space-lg rounded shadow-sm" {
                        div class="flex items-center justify-between text-on-surface-variant mb-2" {
                            span class="font-label-sm text-label-sm uppercase tracking-wider font-semibold" {
                                (Text::new("Orçamento Fixado (LOA)"))
                            }
                            span class="material-symbols-outlined text-primary text-[20px]" {
                                (Text::new("account_balance"))
                            }
                        }
                        div class="font-headline-md text-headline-md text-primary font-bold tracking-tight" {
                            (Text::new(" R$ 428.500.000,00 "))
                        }
                        p class="font-caption text-caption text-on-surface-variant mt-2" {
                            (Text::new(" Lei Orçamentária Anual nº 1.842 devidamente sancionada. "))
                        }
                    }
                    div class="bg-surface-container-lowest p-space-lg rounded shadow-sm" {
                        div class="flex items-center justify-between text-on-surface-variant mb-2" {
                            span class="font-label-sm text-label-sm uppercase tracking-wider font-semibold" {
                                (Text::new("Receita Arrecadada"))
                            }
                            span class="font-label-sm text-label-sm px-2 py-0.5 rounded bg-secondary-fixed text-on-secondary-fixed font-bold" {
                                (Text::new("26,2%"))
                            }
                        }
                        div class="font-headline-md text-headline-md text-secondary font-bold tracking-tight" {
                            (Text::new(" R$ 112.430.820,00 "))
                        }
                        div class="w-full bg-surface-container-low h-2 rounded mt-3 overflow-hidden" {
                            div class="bg-secondary h-full rounded w-26-2" {
                            }
                        }
                        div class="flex justify-between items-center text-caption font-caption text-on-surface-variant mt-2" {
                            span {
                                (Text::new("IPTU, ISS, FPM, ICMS"))
                            }
                            span class="text-on-surface font-medium" {
                                (Text::new("Meta 1º Quadrimestre Atingida"))
                            }
                        }
                    }
                    div class="bg-surface-container-lowest p-space-lg rounded shadow-sm" {
                        div class="flex items-center justify-between text-on-surface-variant mb-2" {
                            span class="font-label-sm text-label-sm uppercase tracking-wider font-semibold" {
                                (Text::new("Despesas Liquidadas"))
                            }
                            span class="font-label-sm text-label-sm px-2 py-0.5 rounded bg-surface-container-high text-primary font-bold" {
                                (Text::new("22,9%"))
                            }
                        }
                        div class="font-headline-md text-headline-md text-on-surface font-bold tracking-tight" {
                            (Text::new(" R$ 98.120.450,00 "))
                        }
                        div class="w-full bg-surface-container-low h-2 rounded mt-3 overflow-hidden" {
                            div class="bg-primary-container h-full rounded w-22-9" {
                            }
                        }
                        div class="flex justify-between items-center text-caption font-caption text-on-surface-variant mt-2" {
                            span {
                                (Text::new("Investimentos, Custeio e Folha"))
                            }
                            span class="text-on-surface font-medium" {
                                (Text::new("Superávit Primário Corrente"))
                            }
                        }
                    }
                }
                div class="bg-surface-container-lowest p-space-lg rounded shadow-sm" {
                    div class="flex flex-col md:flex-row md:items-center justify-between mb-4" {
                        h3 class="font-title-md text-title-md text-on-surface font-bold" {
                            (Text::new(" Limites e Aplicações Constitucionais Obrigatórias "))
                        }
                        span class="text-caption font-caption text-on-surface-variant" {
                            (Text::new(" Atualizado com base nos lançamentos contábeis das últimas 24h "))
                        }
                    }
                    div class="grid grid-cols-1 md:grid-cols-2 gap-space-lg" {
                        div class="p-space-md bg-surface-container-low rounded" {
                            div class="flex items-center justify-between mb-2" {
                                div class="flex items-center gap-2" {
                                    span class="material-symbols-outlined text-secondary text-[22px]" {
                                        (Text::new("health_and_safety"))
                                    }
                                    span class="font-label-md text-label-md font-bold text-on-surface" {
                                        (Text::new("Saúde Pública (SUS Municipal)"))
                                    }
                                }
                                span class="font-title-md text-title-md font-bold text-secondary" {
                                    (Text::new("21,4%"))
                                }
                            }
                            div class="w-full bg-surface-container h-2.5 rounded overflow-hidden relative" {
                                div class="bg-secondary h-full rounded w-21-4" {
                                }
                                div class="absolute top-0 bottom-0 w-0.5 bg-error z-10 left-15" title="Mínimo Constitucional: 15%" { }
                            }
                            div class="flex items-center justify-between text-caption font-caption text-on-surface-variant mt-2" {
                                span {
                                    (Text::new("Mínimo Constitucional: "))
                                    strong {
                                        (Text::new("15,0%"))
                                    }
                                }
                                span class="text-secondary font-semibold" {
                                    (Text::new("+6,4% acima do limite"))
                                }
                            }
                        }
                        div class="p-space-md bg-surface-container-low rounded" {
                            div class="flex items-center justify-between mb-2" {
                                div class="flex items-center gap-2" {
                                    span class="material-symbols-outlined text-primary text-[22px]" {
                                        (Text::new("school"))
                                    }
                                    span class="font-label-md text-label-md font-bold text-on-surface" {
                                        (Text::new("Educação Básica & FUNDEB"))
                                    }
                                }
                                span class="font-title-md text-title-md font-bold text-primary" {
                                    (Text::new("27,8%"))
                                }
                            }
                            div class="w-full bg-surface-container h-2.5 rounded overflow-hidden relative" {
                                div class="bg-primary h-full rounded w-27-8" {
                                }
                                div class="absolute top-0 bottom-0 w-0.5 bg-error z-10 left-25" title="Mínimo Constitucional: 25%" { }
                            }
                            div class="flex items-center justify-between text-caption font-caption text-on-surface-variant mt-2" {
                                span {
                                    (Text::new("Mínimo Constitucional: "))
                                    strong {
                                        (Text::new("25,0%"))
                                    }
                                }
                                span class="text-primary font-semibold" {
                                    (Text::new("+2,8% acima do limite"))
                                }
                            }
                        }
                    }
                }
            }
        }
        section class="max-w-[1280px] mx-auto w-full px-6 lg:px-12 py-14" aria_label=(Label::text(Msg::Services)) {
            div class="flex flex-col md:flex-row md:items-end justify-between mb-8 gap-4" {
                div {
                    div class="font-label-sm text-label-sm text-secondary uppercase tracking-widest font-bold" {
                        (Text::new("Atendimento Descentralizado"))
                    }
                    h2 class="font-headline-lg text-headline-lg text-on-surface" {
                        (Text::new("Serviços Essenciais ao Cidadão e Contribuinte"))
                    }
                }
                a class="font-label-md text-label-md text-secondary hover:underline font-semibold flex items-center gap-1" href=(hash()) {
                    span {
                        (Text::new("Ver Guia Completo de 142 Serviços"))
                    }
                    span class="material-symbols-outlined text-[16px]" {
                        (Text::new("arrow_forward"))
                    }
                }
            }
            (service_grid())
        }
        section class="w-full bg-surface-container-low py-14" aria_label=(Label::text(Msg::Gazette)) {
            div class="max-w-[1280px] mx-auto px-6 lg:px-12" {
                div class="grid grid-cols-1 lg:grid-cols-12 gap-space-lg" {
                    div class="lg:col-span-4 bg-surface-container-lowest p-space-lg rounded shadow-md flex flex-col justify-between" {
                        div {
                            div class="flex items-center justify-between mb-4" {
                                span class="px-2.5 py-1 rounded bg-tertiary-container text-on-tertiary font-label-sm text-label-sm font-semibold" {
                                    (Text::new(" Diário Oficial do Município "))
                                }
                                span class="text-caption font-caption text-on-surface-variant" {
                                    (Text::new("Hoje"))
                                }
                            }
                            div class="text-label-sm font-label-sm text-on-surface-variant uppercase tracking-wider" {
                                (Text::new("Edição Ordinária"))
                            }
                            div class="font-headline-md text-headline-md font-bold text-primary mt-1 mb-2" {
                                (Text::new(" Edição nº 3.418 "))
                            }
                            p class="font-body-sm text-body-sm text-on-surface-variant mb-4" {
                                (Text::new(" Publicação oficial de decretos, leis sancionadas, portarias de recursos humanos e homologações de licitações. "))
                            }
                            div class="p-3 bg-surface-container-low rounded space-y-1 mb-6" {
                                div class="flex items-center gap-1.5 text-secondary text-label-sm font-label-sm font-semibold" {
                                    span class="material-symbols-outlined text-[18px]" {
                                        (Text::new("lock"))
                                    }
                                    (Text::new(" ICP-Brasil Padrão A1/A3 "))
                                }
                                div class="font-caption text-caption text-on-surface-variant font-mono truncate" {
                                    (Text::new(" SHA-256: 4e9f82d1c6a0b...89ef02b "))
                                }
                            }
                        }
                        div class="space-y-2" {
                            a class="w-full inline-flex items-center justify-center gap-2 px-4 py-2.5 rounded bg-primary text-on-primary font-label-md text-label-md font-semibold hover:bg-primary-container transition-colors" href=(hash()) {
                                span class="material-symbols-outlined text-[18px]" {
                                    (Text::new("download"))
                                }
                                span {
                                    (Text::new("Baixar Diário Oficial (PDF)"))
                                }
                            }
                            a class="w-full inline-flex items-center justify-center gap-2 px-4 py-2 rounded bg-surface-container text-primary font-label-md text-label-md font-medium hover:bg-surface-container-high transition-colors" href=(hash()) {
                                span {
                                    (Text::new("Acervo Histórico de Edições"))
                                }
                            }
                        }
                    }
                    div class="lg:col-span-8 flex flex-col justify-between" {
                        div {
                            div class="flex items-center justify-between mb-4" {
                                div {
                                    div class="font-label-sm text-label-sm text-secondary uppercase tracking-widest font-bold" {
                                        (Text::new("Atos Normativos Recentes"))
                                    }
                                    h3 class="font-headline-sm text-headline-sm text-on-surface font-bold" {
                                        (Text::new("Últimas Publicações dos Dois Poderes"))
                                    }
                                }
                                a class="font-label-md text-label-md text-secondary hover:underline font-semibold hidden sm:inline-flex items-center gap-1" href=(hash()) {
                                    span {
                                        (Text::new("Ver Todos os Atos"))
                                    }
                                    span class="material-symbols-outlined text-[16px]" {
                                        (Text::new("arrow_forward"))
                                    }
                                }
                            }
                            div class="space-y-3" {
                                for act in GAZETTE {
                                    article class="p-space-md bg-surface-container-lowest rounded shadow-sm hover:shadow-md transition-shadow" {
                                        div class="flex flex-wrap items-center justify-between gap-2 mb-1" {
                                            div class="flex items-center gap-2" {
                                                span class=(Class::new(act.badge_class)) {
                                                    (Text::new(act.badge))
                                                }
                                                span class="text-caption font-caption text-on-surface-variant" {
                                                    (Text::new(act.org))
                                                }
                                            }
                                            span class="text-caption font-caption text-on-surface-variant" {
                                                (Text::new(act.when))
                                            }
                                        }
                                        h4 class="font-title-sm text-title-sm font-semibold text-on-surface" {
                                            (Text::new(act.body))
                                        }
                                        div class="mt-2 flex items-center justify-between text-caption font-caption text-on-surface-variant" {
                                            span {
                                                (Text::new(act.meta))
                                            }
                                            a class="text-secondary font-semibold hover:underline flex items-center gap-1" href=(hash()) {
                                                (Text::new(act.cta))
                                                span class="material-symbols-outlined text-[14px]" {
                                                    (Text::new("arrow_forward"))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        section class="max-w-[1280px] mx-auto w-full px-6 lg:px-12 py-14" aria_label=(Label::text(Msg::Participation)) {
            div class="grid grid-cols-1 lg:grid-cols-12 gap-space-lg" {
                div class="lg:col-span-7 bg-surface-container-lowest p-space-lg rounded shadow-sm flex flex-col justify-between" {
                    div {
                        div class="flex items-center justify-between mb-4" {
                            span class="px-3 py-1 rounded bg-secondary-fixed text-on-secondary-fixed font-label-sm text-label-sm font-bold flex items-center gap-1.5" {
                                span class="w-2 h-2 rounded-full bg-secondary" {
                                }
                                (Text::new(" Plenário em Pauta "))
                            }
                            span class="text-caption font-caption text-on-surface-variant" {
                                (Text::new("Câmara Municipal"))
                            }
                        }
                        h3 class="font-headline-sm text-headline-sm text-on-surface font-bold mb-2" {
                            (Text::new(" Próxima Sessão Ordinária: Terça-feira, às 09:00h "))
                        }
                        p class="font-body-md text-body-md text-on-surface-variant mb-6" {
                            (Text::new(" Pauta prioritária para votação em 2º turno: Projeto de Lei Complementar do "))
                            strong class="text-on-surface" {
                                (Text::new("Novo Plano Diretor Municipal"))
                            }
                            (Text::new(" e diretrizes de zoneamento sustentável para a próxima década. "))
                        }
                        div class="p-space-md bg-surface-container-low rounded space-y-3 mb-6" {
                            div class="flex items-start gap-3" {
                                span class="material-symbols-outlined text-secondary text-[24px] mt-0.5" {
                                    (Text::new("event_upcoming"))
                                }
                                div {
                                    div class="font-label-md text-label-md font-semibold text-on-surface" {
                                        (Text::new("Ordem do Dia Convocada"))
                                    }
                                    p class="font-body-sm text-body-sm text-on-surface-variant" {
                                        (Text::new(" 1. Leitura de Expedientes do Poder Executivo • 2. Votação de 4 Moções Cívicas • 3. Debate Aberto sobre Habitação de Interesse Social. "))
                                    }
                                }
                            }
                        }
                    }
                    div class="flex flex-wrap items-center gap-space-sm" {
                        a class="inline-flex items-center gap-2 px-space-md py-2.5 rounded bg-secondary text-on-secondary font-label-md text-label-md font-semibold hover:bg-on-secondary-fixed-variant transition-colors" href=(hash()) {
                            span class="material-symbols-outlined text-[18px]" {
                                (Text::new("live_tv"))
                            }
                            span {
                                (Text::new("Acompanhar Transmissão"))
                            }
                        }
                        a class="inline-flex items-center gap-1.5 px-space-md py-2.5 rounded bg-surface-container text-primary font-label-md text-label-md font-medium hover:bg-surface-container-high transition-colors" href=(hash()) {
                            span class="material-symbols-outlined text-[18px]" {
                                (Text::new("feed"))
                            }
                            span {
                                (Text::new("Baixar Ordem do Dia Completa"))
                            }
                        }
                    }
                }
                div class="lg:col-span-5 bg-surface-container p-space-lg rounded shadow-sm flex flex-col justify-between" {
                    div {
                        div class="font-label-sm text-label-sm text-secondary uppercase tracking-widest font-bold mb-1" {
                            (Text::new("Controle Social Ativo"))
                        }
                        h3 class="font-headline-sm text-headline-sm text-on-surface font-bold mb-3" {
                            (Text::new(" Ouvidoria & Acesso à Informação "))
                        }
                        p class="font-body-sm text-body-sm text-on-surface-variant mb-6" {
                            (Text::new(" O cidadão dispõe de canais auditados para peticionar aos dois poderes, solicitar dados sob a Lei nº 12.527/2011 e propor iniciativas de lei. "))
                        }
                        div class="space-y-3 mb-6" {
                            div class="p-3 bg-surface-container-lowest rounded shadow-sm flex items-center justify-between" {
                                div class="flex items-center gap-2.5" {
                                    span class="material-symbols-outlined text-secondary text-[22px]" {
                                        (Text::new("record_voice_over"))
                                    }
                                    div {
                                        div class="font-label-sm text-label-sm font-bold text-on-surface" {
                                            (Text::new("Tribuna Popular & Audiências"))
                                        }
                                        div class="font-caption text-caption text-on-surface-variant" {
                                            (Text::new("Inscrição para fala em plenário"))
                                        }
                                    }
                                }
                                span class="material-symbols-outlined text-on-surface-variant text-[20px]" {
                                    (Text::new("chevron_right"))
                                }
                            }
                            div class="p-3 bg-surface-container-lowest rounded shadow-sm flex items-center justify-between" {
                                div class="flex items-center gap-2.5" {
                                    span class="material-symbols-outlined text-primary text-[22px]" {
                                        (Text::new("info"))
                                    }
                                    div {
                                        div class="font-label-sm text-label-sm font-bold text-on-surface" {
                                            (Text::new("e-SIC Municipal Unificado"))
                                        }
                                        div class="font-caption text-caption text-on-surface-variant" {
                                            (Text::new("Protocolar Pedido de Informação"))
                                        }
                                    }
                                }
                                span class="material-symbols-outlined text-on-surface-variant text-[20px]" {
                                    (Text::new("chevron_right"))
                                }
                            }
                        }
                    }
                    div class="p-space-md bg-surface-container-lowest rounded shadow-sm" {
                        div class="flex items-center justify-between" {
                            span class="text-caption font-caption text-on-surface-variant" {
                                (Text::new("Tempo Médio de Resposta e-SIC:"))
                            }
                            span class="font-label-md text-label-md font-bold text-secondary" {
                                (Text::new("3,2 dias úteis"))
                            }
                        }
                        div class="text-caption font-caption text-on-surface-variant/80 mt-1" {
                            (Text::new(" Meta legal LRF: até 20 dias • Índice de Atendimento: 99,4% "))
                        }
                    }
                }
            }
        }
    }
        }
}

fn footer() -> Markup {
    html! {
    footer class="w-full bg-surface-container text-on-surface border-t border-outline-variant/30 mt-space-xl" {
        div class="max-w-[1440px] mx-auto px-6 lg:px-12 py-12" {
            div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-space-xl" {
                div class="space-y-space-sm" {
                    div class="flex items-center gap-2 mb-space-sm" {
                        span class="material-symbols-outlined text-primary text-[28px]" {
                            (Text::new("domain"))
                        }
                        div class="font-title-sm text-title-sm text-primary font-bold uppercase tracking-wider" {
                            (Text::new("Poder Executivo"))
                        }
                    }
                    div class="font-label-md text-label-md font-semibold text-on-surface" {
                        (Text::new("Prefeitura Municipal - Paço Municipal"))
                    }
                    p class="font-body-sm text-body-sm text-on-surface-variant" {
                        (Text::new("Praça dos Três Poderes, nº 100 - Centro"))
                        br;
                        (Text::new("CEP: 00000-000 • Atendimento: 08h às 17h"))
                        br;
                        (Text::new("Telefone Central: (00) 3000-0000"))
                        br;
                        (Text::new("E-mail: contato@municipio.gov.br"))
                    }
                }
                div class="space-y-space-sm" {
                    div class="flex items-center gap-2 mb-space-sm" {
                        span class="material-symbols-outlined text-secondary text-[28px]" {
                            (Text::new("gavel"))
                        }
                        div class="font-title-sm text-title-sm text-secondary font-bold uppercase tracking-wider" {
                            (Text::new("Poder Legislativo"))
                        }
                    }
                    div class="font-label-md text-label-md font-semibold text-on-surface" {
                        (Text::new("Câmara Municipal de Vereadores"))
                    }
                    p class="font-body-sm text-body-sm text-on-surface-variant" {
                        (Text::new("Palácio Legislativo Municipal, nº 250 - Centro"))
                        br;
                        (Text::new("CEP: 00000-000 • Sessões Ordinárias: Terças 18h"))
                        br;
                        (Text::new("Telefone: (00) 3000-1111"))
                        br;
                        (Text::new("E-mail: legislativo@camara.gov.br"))
                    }
                }
                div class="space-y-space-sm" {
                    div class="font-title-sm text-title-sm text-primary font-bold mb-space-sm uppercase tracking-wider" {
                        (Text::new("Transparência & Controle"))
                    }
                    ul class="space-y-space-xs font-body-sm text-body-sm text-on-surface-variant" {
                        li {
                            a class="hover:text-primary hover:underline transition-colors" data_path="portal-da-transparencia" href=(hash()) {
                                (Text::new("Execução Orçamentária e Balancetes"))
                            }
                        }
                        li {
                            a class="hover:text-primary hover:underline transition-colors" data_path="portal-da-transparencia" href=(hash()) {
                                (Text::new("Folha de Pagamento & Remunerações"))
                            }
                        }
                        li {
                            a class="hover:text-primary hover:underline transition-colors" data_path="portal-da-transparencia" href=(hash()) {
                                (Text::new("Licitações, Contratos e Editais"))
                            }
                        }
                        li {
                            a class="hover:text-primary hover:underline transition-colors" data_path="ouvidoria-e-sic" href=(hash()) {
                                (Text::new("e-SIC (Acesso à Informação LAI)"))
                            }
                        }
                        li {
                            a class="hover:text-primary hover:underline transition-colors" data_path="portal-da-transparencia" href=(hash()) {
                                (Text::new("Portal de Dados Abertos Municipais"))
                            }
                        }
                    }
                }
                div class="space-y-space-sm" {
                    div class="font-title-sm text-title-sm text-primary font-bold mb-space-sm uppercase tracking-wider" {
                        (Text::new("Conformidade Cívica"))
                    }
                    div class="p-space-md bg-surface-container-lowest rounded border border-outline-variant/40 space-y-space-xs" {
                        div class="flex items-center gap-2 text-secondary" {
                            span class="material-symbols-outlined text-[20px]" {
                                (Text::new("verified_user"))
                            }
                            span class="font-label-sm text-label-sm font-bold" {
                                (Text::new("Conformidade LAI nº 12.527/2011"))
                            }
                        }
                        p class="font-caption text-caption text-on-surface-variant" {
                            (Text::new("Portal estruturado em conformidade com as diretrizes e-MAG, LRF nº 101/2000 e LGPD nº 13.709/2018."))
                        }
                    }
                    div class="text-on-surface-variant font-caption text-caption pt-space-xs flex flex-wrap gap-x-space-md gap-y-space-xs" {
                        span {
                            (Text::new("Selo Ouro Transparência Pública"))
                        }
                        span {
                            (Text::new("Atende e-MAG 3.1 AAA"))
                        }
                    }
                }
            }
            div class="mt-12 pt-space-md border-t border-outline-variant/30 flex flex-col md:flex-row items-center justify-between gap-space-md font-caption text-caption text-on-surface-variant" {
                div {
                    (Text::new("© 2025 Governo Municipal Integrado. Todos os direitos reservados. Informação pública e irrestrita."))
                }
                div class="flex items-center gap-space-lg" {
                    a class="hover:underline" data_path="ouvidoria-e-sic" href=(hash()) {
                        (Text::new("Política de Privacidade e LGPD"))
                    }
                    a class="hover:underline" data_path="legislacao-atos" href=(hash()) {
                        (Text::new("Termos de Uso"))
                    }
                    a class="hover:underline" data_path="ouvidoria-e-sic" href=(hash()) {
                        (Text::new("Mapa do Site"))
                    }
                }
            }
        }
    }
        }
}

/// Documento completo. O `Page` do crate tem um `<h1>` próprio; o EXAMPLE tem o `<h1>` no hero,
/// então o shell é montado pelo consumidor e o conteúdo continua validado por `html!`.
pub fn page() -> String {
    let mut out = String::with_capacity(64_000);
    out.push_str("<!DOCTYPE html><html lang=\"pt-BR\">");
    out.push_str(HEAD);
    out.push_str("<body class=\"bg-background font-body-md text-on-surface antialiased selection:bg-secondary-fixed selection:text-on-secondary-fixed\">");
    out.push_str(header().as_str());
    out.push_str("<main class=\"w-full pt-41 bg-background\" id=\"conteudo\">");
    out.push_str(main_content().as_str());
    out.push_str("</main>");
    out.push_str(footer().as_str());
    out.push_str("</body></html>");
    out
}

#[cfg(test)]
mod tests {
    use super::page;

    #[test]
    fn page_has_the_document_invariants() {
        let s = page();
        assert_eq!(s.matches("<h1").count(), 1, "um único h1");
        assert_eq!(s.matches("<main").count(), 1, "um único main");
        assert_eq!(s.matches("<a ").count(), 56, "todos os links do exemplo");
        assert_eq!(s.matches("<button ").count(), 8, "controles do header");
        assert!(!s.contains("href=\"/#"), "fragmentos não viram /#");
        assert!(!s.contains("style="), "sem style inline");
        assert!(!s.contains("onclick="), "sem handler inline");
    }

    #[test]
    fn service_cards_and_controls_are_present() {
        let s = page();
        for text in [
            "IPTU &amp; Certidão de Débitos",
            "Consultas &amp; Farmácia Básica",
            "Matrícula Escolar &amp; Transporte",
            "Alvarás, Habite-se &amp; Obras",
            "Iluminação e Manutenção de Vias",
            "Protocolo Geral Sem Papel",
            "Acessibilidade e-MAG",
            "Alto Contraste",
            "VLibras",
        ] {
            assert!(s.contains(text), "faltou {text}");
        }
    }

    #[test]
    fn service_cards_keep_alternating_tone() {
        let s = page();
        assert!(s.contains("group-hover:bg-primary"));
        assert!(s.contains("group-hover:bg-secondary"));
        assert!(s.contains("font-bold font-label-sm"), "controles compactos");
    }

    #[test]
    fn fragment_links_are_exact() {
        let s = page();
        for href in ["#conteudo", "#menu", "#busca", "#"] {
            assert!(
                s.contains(&format!("href=\"{href}\"")),
                "faltou href={href}"
            );
        }
    }
}
