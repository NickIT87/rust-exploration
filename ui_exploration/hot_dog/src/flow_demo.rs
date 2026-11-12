use dioxus::prelude::*;
use dioxus_flow::prelude::*;

#[component]
pub(crate) fn App() -> Element {
    let nodes = use_signal(|| {
        vec![
            Node::new("order", "Заказ", (40.0, 100.0)).node_type("input"),
            Node::new("prepare", "Приготовить хот-дог", (300.0, 100.0)),
            Node::new("serve", "Выдать заказ", (560.0, 100.0)).node_type("output"),
        ]
    });
    let edges = use_signal(|| {
        vec![
            Edge::new("order", "prepare").animated(true),
            Edge::new("prepare", "serve").animated(true),
        ]
    });

    rsx! {
        main { style: "min-height: 98vh; padding: 32px; box-sizing: border-box; background: #111827; color: #f9fafb; font-family: sans-serif;",
            h1 { "🌭 Hot-dog flow" }
            p {
                "Демонстрация интерактивного графа на dioxus-flow: перетаскивайте узлы, двигайте холст и меняйте масштаб."
            }
            div { style: "height: 560px; width: 100%; overflow: hidden; border: 1px solid #374151; border-radius: 16px;",
                Flow { nodes, edges, fit_view: true,
                    Background {}
                    Controls {}
                    MiniMap {}
                }
            }
        }
    }
}
