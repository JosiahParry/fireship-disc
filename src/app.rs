use leptos::*;
use leptos_meta::*;
// use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <script src="https://kit.fontawesome.com/fa0a7fa4d0.js" crossorigin="anonymous"></script>
        // sets the document title
        <Title text="Welcome to Leptos"/>
        // content for this welcome page
        // <HomePage/>
        <div class="flex">
            <crate::discord::SideBar />
            <crate::discord::ChannelBar />
            <crate::discord::ContentContainer />
            <crate::discord::BottomBar/>
            // <ContentContainer />
          </div>
        // <Router>
        //     <main>
        //         <Routes>
        //             <Route path="" view=HomePage/>
        //             <Route path="/*any" view=NotFound/>
        //         </Routes>
        //     </main>
        // </Router>
    }
}
