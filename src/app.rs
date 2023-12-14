use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/rr.css"/>

        // sets the document title
        <Title text="Welcome to Ribbit Ribbit"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    view! {
        <div class="text-4xl bg-gradient-to-tl from-green-800 to-green-500 text-yellow-100 font-mono flex flex-row min-h-screen">
            Ribbit Ribbit

            <iframe
                title="like button"
                src="https://like-button-boko8kc8.fermyon.app/static/likebutton.html?key=test2"
            ></iframe>
        </div>
    }
}
