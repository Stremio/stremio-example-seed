#![allow(clippy::needless_pass_by_value)]

#[macro_use]
extern crate seed;
use env_web::Env;
use seed::{prelude::*, App, *};
use stremio_core::state_types::{Action, ActionLoad, CatalogFiltered, Ctx, Loadable, Msg, Update, TypeEntry, CatalogEntry};
use stremio_core::types::MetaPreview;
use stremio_core::types::addons::{ResourceRequest, ResourceRef};
use stremio_derive::Model;

// ------ ------
//     Model
// ------ ------

#[derive(Model, Default)]
struct Model {
    ctx: Ctx<Env>,
    catalog: CatalogFiltered<MetaPreview>,
}

// ------ ------
//     Init
// ------ ------

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Init<Model> {
    orders.send_msg(default_load());
    Init::default()
}

// ------ ------
//    Update
// ------ ------

fn default_load() -> Msg {
    let req = ResourceRequest {
        base: "https://v3-cinemeta.strem.io/manifest.json".to_owned(),
        path: ResourceRef::without_extra("catalog", "movie", "top"),
    };
    Msg::Action(Action::Load(ActionLoad::CatalogFiltered(req)))
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let fx = model.update(&msg);
    if !fx.has_changed {
        orders.skip();
    }
//    for cmd in fx.effects {
//        orders.perform_cmd(cmd);
//    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
//    log!("TYPES:", model.catalog.types);
//    log!("CATALOGS:", model.catalog.catalogs);

    div![
        view_type_selector(&model.catalog.types),
        view_catalog_selector(&model.catalog.catalogs, model.catalog.selected.as_ref()).unwrap_or_else(|| empty![])
    ]


//    let groups = model.catalog.groups.iter().map(|group| {
//        let el = match &group.content {
//            Loadable::Err(catalog_error) => h3![format!("{:#?}", catalog_error)],
//            Loadable::Loading => h3!["Loading"],
//            Loadable::Ready(meta_previews) if meta_previews.is_empty() => div![],
//            Loadable::Ready(meta_previews) => div![
//                class!["meta-items-container"],
//                meta_previews.iter().take(7).map(view_meta_preview)
//            ],
//        };
//        div![class!["board-row"], class!["addon-catalog-row"], el]
//    });
//
//    div![
//        class!["board-container"],
//        div![class!["board-content"], groups]
//    ]
}

fn view_type_selector(type_entries: &[TypeEntry]) -> Node<Msg> {
    select![
        type_entries.iter().map(|type_entry| {
            let req = type_entry.load.clone();
            option![
                attrs!{
                    At::Selected => type_entry.is_selected.as_at_value(),
                },
                raw_ev(Ev::Click, |_| Msg::Action(Action::Load(ActionLoad::CatalogFiltered(req)))),
                type_entry.type_name
            ]
        })
    ]
}

fn view_catalog_selector(catalog_entries: &[CatalogEntry], selected_req: Option<&ResourceRequest>) -> Option<Node<Msg>> {
    selected_req.map(|selected_req| {
        select![
            catalog_entries
                .iter()
                .filter(|catalog_entry| &catalog_entry.load.path.type_name == &selected_req.path.type_name)
                .map(|catalog_entry| {
                    let req = catalog_entry.load.clone();
                    option![
                        attrs!{
                            At::Selected => catalog_entry.is_selected.as_at_value(),
                        },
                        raw_ev(Ev::Click, |_| Msg::Action(Action::Load(ActionLoad::CatalogFiltered(req)))),
                        catalog_entry.name,
                    ]
                })
        ]
    })
}

fn view_meta_preview(meta_preview: &MetaPreview) -> Node<Msg> {
    let default_poster = "https://www.stremio.com/images/add-on-money.png".to_owned();
    let poster = meta_preview.poster.as_ref().unwrap_or(&default_poster);
    let poster_shape = meta_preview.poster_shape.to_str();

    div![
        attrs! {
            At::Class => format!("meta-item meta-item-container poster-shape-{}", poster_shape);
            At::Title => meta_preview.name
        },
        div![
            class!["poster-image-container"],
            div![
                class!["poster-image-layer"],
                div![
                    class!["poster-image"],
                    style! { "background-image" => format!("url({})", poster) },
                    raw_ev(Ev::Click, |_| default_load()) //raw_ev(Ev::Click, |_| Msg::Action(Action::UserOp(ActionUser::Login{ email, password })))
                ]
            ]
        ],
        div![
            class!["title-bar-container"],
            div![class!["title"], meta_preview.name]
        ],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::build(init, update, view).build_and_start();
}
