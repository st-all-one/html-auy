//! Auditoria do documento final: cada regra tem um par passa/falha e mensagem estável.
#![cfg(feature = "audit")]

use html::audit::{Finding, check};

#[derive(Clone, Copy)]
struct M;

impl html::Msg for M {
    fn as_str(self) -> &'static str {
        "Nome completo"
    }
}

const HEAD: &str = "<!DOCTYPE html><html lang=\"pt-BR\"><head><title>T</title></head><body>";

fn document(body: &str) -> String {
    format!("{HEAD}{body}</body></html>")
}

fn healthy() -> String {
    document("<h1>Título</h1><main><p>conteúdo</p></main>")
}

fn rules(html: &str) -> Vec<&'static str> {
    check(html).iter().map(|finding| finding.rule).collect()
}

#[test]
fn healthy_document_has_no_findings() {
    assert_eq!(check(&healthy()), Vec::<Finding>::new());
}

#[test]
fn main_unique() {
    let missing = document("<h1>T</h1>");
    assert_eq!(rules(&missing), ["main_unique"]);
    assert_eq!(
        check(&missing)
            .first()
            .map(|finding| finding.message.as_str()),
        Some("documento tem 0 elementos `<main>` (exige exatamente 1)")
    );

    let doubled = document("<h1>T</h1><main>a</main><main>b</main>");
    assert_eq!(rules(&doubled), ["main_unique"]);
}

#[test]
fn single_h1() {
    let doubled = document("<h1>A</h1><h1>B</h1><main>x</main>");
    assert!(rules(&doubled).contains(&"single_h1"));

    let missing = document("<main>x</main>");
    assert_eq!(rules(&missing), ["single_h1"]);
}

#[test]
fn heading_levels_no_skips() {
    let skipped = document("<h1>T</h1><main><h3>salto</h3></main>");
    assert_eq!(rules(&skipped), ["heading_levels_no_skips"]);
    assert_eq!(
        check(&skipped)
            .first()
            .map(|finding| finding.message.as_str()),
        Some("`h3` depois de `h1` pula o nível 2")
    );

    let in_order = document("<h1>T</h1><main><h2>a</h2><h3>b</h3></main>");
    assert_eq!(check(&in_order), Vec::<Finding>::new());
}

#[test]
fn ids_unique() {
    let repeated = document("<h1>T</h1><main><p id=\"a\">1</p><p id=\"a\">2</p></main>");
    assert_eq!(rules(&repeated), ["ids_unique"]);
}

#[test]
fn landmark_names_distinct() {
    let repeated = document(
        "<h1>T</h1><main><nav aria-label=\"Menu\">a</nav><nav aria-label=\"Menu\">b</nav></main>",
    );
    assert_eq!(rules(&repeated), ["landmark_names_distinct"]);
}

#[test]
fn landmark_names_required_when_repeated() {
    let repeated = document("<h1>T</h1><main><nav>a</nav><nav>b</nav></main>");
    assert_eq!(
        rules(&repeated),
        ["landmark_names_required", "landmark_names_required"]
    );
    assert_eq!(
        check(&repeated)
            .first()
            .map(|finding| finding.message.as_str()),
        Some("`nav` sem nome acessível: há 2 `nav` no documento")
    );

    let asides = document("<h1>T</h1><main><aside>a</aside><aside>b</aside></main>");
    assert_eq!(
        rules(&asides),
        ["landmark_names_required", "landmark_names_required"]
    );

    let single = document("<h1>T</h1><main><nav>a</nav><aside>b</aside></main>");
    assert_eq!(check(&single), Vec::<Finding>::new());

    let named = document(
        "<h1>T</h1><main><nav aria-label=\"Menu\">a</nav><nav aria-label=\"Rodapé\">b</nav></main>",
    );
    assert_eq!(check(&named), Vec::<Finding>::new());
}

#[test]
fn aria_refs_resolve() {
    let orphan = document("<h1>T</h1><main><div aria-labelledby=\"nope\">x</div></main>");
    assert_eq!(rules(&orphan), ["aria_refs_resolve"]);

    let resolved = document(
        "<h1>T</h1><main><p id=\"base\">rótulo</p><div aria-labelledby=\"base\">x</div></main>",
    );
    assert_eq!(check(&resolved), Vec::<Finding>::new());
}

#[test]
fn popover() {
    let orphan = document("<h1>T</h1><main><button popovertarget=\"alvo\">abrir</button></main>");
    assert_eq!(rules(&orphan), ["popover_target_resolves"]);

    let resolved = document(
        "<h1>T</h1><main><button popovertarget=\"alvo\">abrir</button>\
         <div id=\"alvo\" popover=\"\">painel</div></main>",
    );
    assert_eq!(check(&resolved), Vec::<Finding>::new());
}

#[test]
fn no_content_derived_identifiers() {
    let bad = document("<h1>T</h1><main><p id=\"1x\">a</p><input name=\"ok-1\"></main>");
    assert_eq!(rules(&bad), ["no_content_derived_identifiers"]);
}

#[test]
fn inline_script_without_src() {
    let inline = document("<h1>T</h1><main><script>alert(1)</script></main>");
    assert_eq!(rules(&inline), ["no_inline_script"]);

    let external = document("<h1>T</h1><main><script src=\"/a.js\"></script></main>");
    assert_eq!(check(&external), Vec::<Finding>::new());

    let json_ld =
        document("<h1>T</h1><main><script type=\"application/ld+json\">{\"a\":1}</script></main>");
    assert_eq!(check(&json_ld), Vec::<Finding>::new());
}

#[test]
fn style_and_event_handler() {
    let styled = document("<h1>T</h1><main><div style=\"color: red\">x</div></main>");
    assert_eq!(rules(&styled), ["no_style_attribute"]);

    let handler = document("<h1>T</h1><main><div onclick=\"go()\">x</div></main>");
    assert_eq!(rules(&handler), ["no_event_handler"]);
}

#[test]
fn findings_follow_document_order_and_are_deterministic() {
    let html =
        document("<h1>T</h1><main><div style=\"x\" onclick=\"y\"></div><h4>salto</h4></main>");
    let first = check(&html);
    let second = check(&html);
    assert_eq!(first, second, "auditoria tem de ser determinística");
    let order: Vec<&str> = first.iter().map(|finding| finding.rule).collect();
    assert_eq!(
        order,
        [
            "no_style_attribute",
            "no_event_handler",
            "heading_levels_no_skips"
        ]
    );
}

#[test]
fn field_markup_passes_its_own_audit() {
    let state = html::FieldState::new(html::AttrVal::new(""))
        .help(html::Text::new("Como consta no documento"))
        .error(html::Text::new("Informe o nome"));
    let field = html::Field::text(html::FieldName::new("sections[0][campo]"), M, &state);
    let page = document(&format!("<h1>T</h1><main>{}</main>", field.as_str()));
    assert_eq!(check(&page), Vec::<Finding>::new());
}

#[test]
fn duplicate_ids_are_reported_per_repetition() {
    let repeated =
        document("<h1>T</h1><main><p id=\"a\">1</p><p id=\"a\">2</p><p id=\"a\">3</p></main>");
    assert_eq!(
        rules(&repeated),
        ["ids_unique", "ids_unique"],
        "cada repetição depois da primeira é um finding"
    );
}
