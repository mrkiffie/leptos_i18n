use crate::i18n::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    let i18n = provide_i18n_context();

    let on_switch = move |_| {
        let new_lang = match i18n.get_locale() {
            Locale::en => Locale::fr,
            Locale::fr => Locale::en,
        };
        i18n.set_locale(new_lang);
    };

    view! {
        <Router>
            <Routes>
                <I18nRoute trailing_slash=TrailingSlash::Drop>
                    <Route path="" view=Home trailing_slash=TrailingSlash::Drop/>
                    <Route path="counter" view=Counter trailing_slash=TrailingSlash::Drop/>
                </I18nRoute>
            </Routes>
            <button on:click=on_switch>{t!(i18n, click_to_change_lang)}</button>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <h1>{t!(i18n, hello_world)}</h1>
        <A href="counter">{t!(i18n, go_counter)}</A>
    }
}

#[component]
fn Counter() -> impl IntoView {
    let i18n = use_i18n();

    let (counter, set_counter) = create_signal(0);

    let inc = move |_| set_counter.update(|count| *count += 1);

    let count = move || counter.get();

    view! {
        <p>
        {t!{ i18n,
            click_count,
            count,
            <b> = |children| view!{ <b>{children}</b> },
        }}
        </p>
        <button on:click=inc>{t!(i18n, click_to_inc)}</button>
        <A href="..">{t!(i18n, go_home)}</A>
    }
}
