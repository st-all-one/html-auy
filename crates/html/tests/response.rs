//! Contrato público de resposta parcial e classes de request.

use html::{Id, MainSwap, Partial, RegionUpdate, RequestClass, Swap, Sync, Text, html};

#[test]
fn partial_preserves_main_and_update_order() {
    let partial = Partial::new(html! { p { (Text::new("principal")) } })
        .update(
            Id::new("cart"),
            Swap::Inner,
            html! { span { (Text::new("3")) } },
        )
        .update(
            Id::new("status"),
            Swap::Outer,
            html! { p { (Text::new("ok")) } },
        )
        .focus(Id::new("status"));
    assert!(matches!(partial.main(), MainSwap::Replace(_)));
    assert_eq!(partial.updates().len(), 2);
    assert_eq!(partial.updates()[0].target().as_str(), "cart");
    assert_eq!(partial.updates()[0].swap(), Swap::Inner);
    assert_eq!(partial.updates()[1].target().as_str(), "status");
    assert_eq!(partial.focus_target().map(Id::as_str), Some("status"));
}

#[test]
fn keep_main_and_selection_contracts_are_explicit() {
    let target = Id::new("cart");
    let partial = Partial::keep_main()
        .select(target.clone())
        .reselect(Id::new("content"))
        .select_oob(vec![Id::new("count")]);
    assert!(partial.main().keeps());
    assert_eq!(partial.select_target().map(Id::as_str), Some("cart"));
    assert_eq!(partial.reselect_target().map(Id::as_str), Some("content"));
    assert_eq!(partial.select_oob_targets().len(), 1);
}

#[test]
fn region_update_keeps_swap_and_markup() {
    let update = RegionUpdate::new(
        Id::new("x"),
        Swap::BeforeEnd,
        html! { ul { li { (Text::new("novo")) } } },
    );
    assert_eq!(update.target().as_str(), "x");
    assert_eq!(update.swap(), Swap::BeforeEnd);
    assert!(update.markup().as_str().contains("<li>novo</li>"));
}

#[test]
fn request_classes_pin_vary() {
    assert_eq!(RequestClass::Full.vary(), "");
    assert_eq!(RequestClass::Partial.vary(), "HX-Request");
    assert_eq!(RequestClass::Targeted.vary(), "HX-Request, HX-Target");
    assert_eq!(
        RequestClass::HistoryRestore.vary(),
        "HX-Request, HX-History-Restore-Request"
    );
    assert_eq!(RequestClass::Boosted.vary(), "HX-Request, HX-Boosted");
}

#[test]
fn sync_has_no_queue() {
    assert_eq!(Sync::Drop.as_str(), "drop");
    assert_eq!(Sync::Replace.as_str(), "replace");
    assert_eq!(Sync::Abort.as_str(), "abort");
}
