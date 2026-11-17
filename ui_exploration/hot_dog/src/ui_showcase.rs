use dioxus::prelude::*;

#[component]
pub(crate) fn App() -> Element {
    let mut quantity = use_signal(|| 2);
    let mut selected = use_signal(|| "Классический".to_string());
    let mut vegetarian = use_signal(|| false);
    let mut modal_open = use_signal(|| false);
    let mut toast = use_signal(|| false);

    rsx! {
        document::Stylesheet { href: asset!("/assets/showcase.css") }
        div { class: "showcase-shell",
            aside { class: "sidebar",
                a { class: "brand", href: "#top", span { class: "brand-icon", "🌭" } "hotdog" }
                p { class: "side-label", "БИБЛИОТЕКА" }
                nav { class: "side-nav",
                    a { class: "active", href: "#overview", span { "◫" } "Обзор" }
                    a { href: "#buttons", span { "▣" } "Кнопки" }
                    a { href: "#forms", span { "☷" } "Формы" }
                    a { href: "#feedback", span { "◉" } "Обратная связь" }
                    a { href: "#data", span { "▦" } "Данные" }
                }
                div { class: "sidebar-bottom", div { class: "avatar", "Н" } div { strong { "Никита" } small { "Дизайн-система" } } }
            }
            main { class: "main-content", id: "top",
                header { class: "topbar", div { class: "breadcrumbs", "Библиотека  /  " b { "Компоненты" } } button { class: "icon-button", "⌕" } }
                section { class: "page-heading", id: "overview",
                    div { span { class: "eyebrow", "HOTDOG UI · V 1.0" } h1 { "Элементы интерфейса" } p { "Основные компоненты для сборки удобных и понятных интерфейсов." } }
                    button { class: "button button-primary", onclick: move |_| toast.set(true), span { "＋" } " Создать заказ" }
                }
                section { class: "stats-grid",
                    StatCard { label: "Заказов сегодня", value: "128", delta: "+12,8%", icon: "↗", color: "orange" }
                    StatCard { label: "Средний чек", value: "₽ 640", delta: "+4,2%", icon: "₽", color: "blue" }
                    StatCard { label: "Выполнено", value: "94%", delta: "+2,1%", icon: "✓", color: "green" }
                    StatCard { label: "Время ожидания", value: "12 мин", delta: "−1,4%", icon: "◷", color: "purple" }
                }
                div { class: "content-grid",
                    div { class: "column",
                        section { class: "panel", id: "buttons",
                            PanelHeading { title: "Кнопки", meta: "Действия и состояния" }
                            p { class: "example-label", "Варианты" }
                            div { class: "component-row",
                                button { class: "button button-primary", "Основная" }
                                button { class: "button button-secondary", "Вторичная" }
                                button { class: "button button-quiet", "Текстовая" }
                                button { class: "button button-danger", "Удалить" }
                            }
                            p { class: "example-label", "Размеры и иконки" }
                            div { class: "component-row align-center",
                                button { class: "button button-primary button-small", "Маленькая" }
                                button { class: "button button-primary", span { "＋" } " Добавить позицию" }
                                button { class: "button button-secondary button-square", "⋯" }
                                button { class: "button button-primary", disabled: true, "Недоступна" }
                            }
                        }
                        section { class: "panel", id: "forms",
                            PanelHeading { title: "Поля формы", meta: "Ввод и выбор" }
                            div { class: "form-grid",
                                label { class: "field", "Название заказа" input { placeholder: "Например, обед для команды" } }
                                label { class: "field", "Тип хот-дога" select { value: "Классический", onchange: move |e| selected.set(e.value()), option { "Классический" } option { "Чикаго" } option { "Вегетарианский" } } }
                                label { class: "field full-field", "Комментарий к заказу" textarea { rows: "3", placeholder: "Без лука, соус отдельно…" } }
                            }
                            div { class: "control-row",
                                div { class: "stepper", button { aria_label: "Уменьшить количество", onclick: move |_| quantity.with_mut(|n| *n = (*n - 1).max(1)), "−" } span { "{quantity}" } button { aria_label: "Увеличить количество", onclick: move |_| quantity.with_mut(|n| *n += 1), "＋" } }
                                label { class: "check-control", input { r#type: "checkbox", checked: vegetarian(), onchange: move |e| vegetarian.set(e.checked()) } span { class: "custom-check" } "Без мяса" }
                                label { class: "switch-control", input { r#type: "checkbox" } span { class: "switch" } "Срочный заказ" }
                            }
                            p { class: "selection-hint", "Выбрано: {selected} · {quantity} шт." }
                        }
                        section { class: "panel", id: "feedback",
                            PanelHeading { title: "Обратная связь", meta: "Статусы и уведомления" }
                            div { class: "component-row", span { class: "badge badge-green", "● Готов" } span { class: "badge badge-orange", "● В работе" } span { class: "badge badge-red", "● Отменён" } span { class: "badge badge-gray", "Черновик" } }
                            div { class: "alert alert-info", span { "ⓘ" } div { strong { "Информация" } p { "Заказ можно изменить до начала приготовления." } } }
                            div { class: "feedback-actions", button { class: "button button-secondary", onclick: move |_| modal_open.set(true), "Показать диалог" } button { class: "button button-quiet", onclick: move |_| toast.set(true), "Показать уведомление" } }
                        }
                    }
                    div { class: "column",
                        section { class: "panel", id: "data",
                            PanelHeading { title: "Карточка заказа", meta: "Пример компонента" }
                            article { class: "order-card",
                                div { class: "order-card-top", div { class: "food-icon", "🌭" } div { class: "order-title", strong { "Комбо «Классика»" } small { "Заказ #HD-2048 · 12:42" } } span { class: "badge badge-orange", "Готовится" } }
                                div { class: "order-details", div { span { "Состав" } b { "Хот-дог, картофель фри" } } div { span { "Количество" } b { "2 шт." } } div { span { "К оплате" } b { "₽ 890" } } }
                                div { class: "order-card-footer", span { class: "avatar avatar-small", "М" } span { "Мария Соколова" } button { class: "button button-primary button-small", "Открыть заказ" } }
                            }
                            p { class: "example-label table-label", "Последние заказы" }
                            div { class: "table-wrap", table {
                                thead { tr { th { "ЗАКАЗ" } th { "КЛИЕНТ" } th { "СТАТУС" } th { "СУММА" } th { "" } } }
                                tbody {
                                    OrderRow { code: "#HD-2048", customer: "Мария Соколова", status: "Готовится", status_class: "badge-orange", amount: "₽ 890" }
                                    OrderRow { code: "#HD-2047", customer: "Илья Петров", status: "Готов", status_class: "badge-green", amount: "₽ 540" }
                                    OrderRow { code: "#HD-2046", customer: "Анна Волкова", status: "Отменён", status_class: "badge-red", amount: "₽ 1 280" }
                                }
                            } }
                        }
                        section { class: "panel palette-panel",
                            PanelHeading { title: "Цвета и шкалы", meta: "Токены интерфейса" }
                            div { class: "swatches", div { span { class: "swatch swatch-orange" } small { "Акцент" } code { "#F97316" } } div { span { class: "swatch swatch-ink" } small { "Текст" } code { "#1F2937" } } div { span { class: "swatch swatch-green" } small { "Успех" } code { "#16A34A" } } div { span { class: "swatch swatch-red" } small { "Ошибка" } code { "#DC2626" } } }
                            p { class: "example-label progress-label", "Прогресс заказа" }
                            div { class: "progress-meta", span { "Приготовление" } b { "68%" } }
                            div { class: "progress-track", div { class: "progress-fill" } }
                            div { class: "pagination", span { "Показано 1–3 из 24" } div { button { "‹" } button { class: "current-page", "1" } button { "2" } button { "3" } button { "›" } } }
                        }
                    }
                }
                footer { class: "page-footer", "Hotdog UI · Компоненты для быстрого старта" span { "Сделано с заботой  ✳" } }
            }
            if modal_open() { div { class: "modal-backdrop", onclick: move |_| modal_open.set(false), div { class: "modal", onclick: move |e| e.stop_propagation(), span { class: "modal-icon", "✓" } h2 { "Подтвердить действие?" } p { "Изменения будут сохранены в заказе." } div { class: "modal-actions", button { class: "button button-quiet", onclick: move |_| modal_open.set(false), "Отмена" } button { class: "button button-primary", onclick: move |_| modal_open.set(false), "Подтвердить" } } } } }
            if toast() { div { class: "toast", span { "✓" } div { strong { "Готово" } small { "Действие выполнено успешно." } } button { onclick: move |_| toast.set(false), "×" } } }
        }
    }
}

#[component]
fn StatCard(label: &'static str, value: &'static str, delta: &'static str, icon: &'static str, color: &'static str) -> Element {
    rsx! { article { class: "stat-card", div { class: "stat-top", span { "{label}" } span { class: "stat-icon {color}", "{icon}" } } div { class: "stat-value", "{value}" } div { class: "stat-foot", b { "{delta}" } " к прошлой неделе" } } }
}

#[component]
fn PanelHeading(title: &'static str, meta: &'static str) -> Element {
    rsx! { div { class: "panel-heading", div { h2 { "{title}" } p { "{meta}" } } button { class: "more-button", "···" } } }
}

#[component]
fn OrderRow(code: &'static str, customer: &'static str, status: &'static str, status_class: &'static str, amount: &'static str) -> Element {
    rsx! { tr { td { b { "{code}" } } td { "{customer}" } td { span { class: "badge {status_class}", "{status}" } } td { b { "{amount}" } } td { button { class: "row-more", "···" } } } }
}
