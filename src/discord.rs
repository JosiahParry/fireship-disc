use leptos::*;


#[component]
pub fn DiscApp() -> impl IntoView {
    view! {
        <div class="flex">
        </div>
    }
}

#[component]
pub fn SideBar() -> impl IntoView {
    view! {
        // <div class="fixed top-0 left-0 h-screen w-16 m-0 flex flex-col  bg-gray-900 text-white shadow-lg">
        <div class="fixed top-0 left-0 h-screen w-16 m-0 flex flex-col  bg-gray-100 text-white shadow-lg">
            <SideBarIcon icon="fire".into()/>
            <Divider/>
            <SideBarIcon icon = "plus".into()/>
            <SideBarIcon icon = "bolt-lightning".into()/>
            <SideBarIcon icon = "poo".into()/>
            <Divider/>
            <SideBarIcon icon = "gear".into()/>
        </div>
    }
}

#[component]
pub fn SideBarIcon(
    #[prop(default = "file".into())]
    icon: String,
    #[prop(default = "tooltip 💡".into())]
    text: String
) -> impl IntoView {
    let fa_icon_class = format!("fa fa-{icon}");
    view! {
        <div class="sidebar-icon group">
            <i class={fa_icon_class}></i> 
            <span class="sidebar-tooltip group-hover:scale-100">
                {text}
            </span>
        </div>
    }
}

#[component]
pub fn Divider() -> impl IntoView {
    view! {
        <hr class="sidebar-hr"/>
    }
}

// channel section
#[component]
pub fn ChannelBlock() -> impl IntoView {
    view! {
        <div class="channel-block">
            <h5 class="channel-block-text">Channels</h5>
          </div>
    }
}

#[component]
pub fn TopicSelection(
    selection: String
) -> impl IntoView {
    view! {
        <div class="dropdown-selection">
            <i class = "fa fa-hashtag text-gray-400"/>
            <h5 class = "dropdown-selection-text"> {selection}</h5>
        </div>
    }
}

#[component]
pub fn ChevronIcon(
    #[prop(into)]
    expanded: Signal<bool>
) -> impl IntoView {
    
    let chev_dir = match expanded() {
        true => "chevron-down",
        false => "chevron-up"
    };

    let chev_class = format!("fa fa-{chev_dir} text-accent text-opacity-80 my-auto mr-1");

    view! {
        <i class={chev_class} />
    }
}

#[component]
pub fn Dropdown(
    #[prop(default = "Section Heading".into())]
    header: String,
    selections: Vec<String> // TODO figure this out lol
) -> impl IntoView {
    
    let (expand_state, set_expand) = create_signal(true);
    let current_state = move || expand_state();
    
    let selects = selections.into_iter().map(|selec| view! { <TopicSelection selection = selec/>} ).collect_view();

view! {
        <div class="dropdown">
            <div
                class = "dropdown-header"
                on:click = move |_| {
                    set_expand.set(!expand_state());
                }
            >
                <ChevronIcon expanded= Signal::derive(current_state) />
                <h5 class = if expand_state() { "dropdown-header-text-selected" } else { "dropdown-header-text" }>
                // <h5 class = "dropdown-header-text-selected">
                    {header}
                </h5>
                <i size = "12" class = "fa fa-plus text-accent text-opacity-80 my-auto ml-auto" />
            </div>
            {selects}
        </div>
    }
}


#[component]
pub fn ChannelBar() -> impl IntoView {
    view! {
        <div class = "channel-bar shadow-lg h-screen">
            <ChannelBlock/>
            <div class = "channel-container">
                <Dropdown header = "Topics".into() 
                    selections = vec![
                        "tailwind-css".into(),
                        "react".into()
                        ]
                />
                <Dropdown header = "Questions".into() 
                    selections = vec![
                        "jit-compilation".into(),
                        "purge-files".into(),
                        "dark-mode".into()
                        ]
                />
                <Dropdown header = "Random".into() 
                    selections = vec![
                        "variants".into(), "plugins".into()
                        ]
                />
            </div>
        </div>
    }
}

// nav bar
#[component]
pub fn Search() -> impl IntoView {
    view! {
        <div class = "search">
            <input class = "search-input" type = "text" placeholder = "Search..." />
            <i class = "fa fa-search text-secondary my-auto"/>
        </div>
    }
}

#[component]
pub fn TopNavigation() -> impl IntoView {
    view! {
        <div class = "top-navigation">
            <HashTagIcon/>
            <h5 class="title-text">"Tailwind CSS & Leptos"</h5>
            <Search />
            <BellIcon />
            <UserCircle />
        </div>
    }
}

#[component]
pub fn HashTagIcon() -> impl IntoView {
    view! {
        <i class = "fa fa-hashtag title-hashtag"/>
    }
}

#[component]
pub fn BellIcon() -> impl IntoView {
    view! {
        <i class = "fa fa-bell top-navigation-icon"/>
    }
}

#[component]
pub fn UserCircle() -> impl IntoView {
    view! {
        <i class = "fa fa-user top-navigation-icon"/>
    }
}
// TODO theme icon / dark mode toggle


// content
#[component]
pub fn PlusIcon() -> impl IntoView {
    view! {
        <i 
        class = "fa fa-circle-plus text-green-500 dark:shadow-lg mx-2 dark:text-primary"
        size = "22"
        />    
    }   
}

use rand::{thread_rng, Rng};

#[component]
pub fn Post(
    name: String,
    timestamp: String,
    text: String,
) -> impl IntoView {

    let mut rng = thread_rng();
    let n: u8 = rng.gen();

    // get a random avatar path
    let avatar_path = format!("https://api.dicebear.com/7.x/adventurer/svg?seed={n}");

    view! {
        <div class = "post">
            <div class = "avatar-wrapper">
                <img src = avatar_path class = "avatar"/>
            </div>

            <div class = "post-content">
                <p class = "post-owner">
                    {name}
                    <small class = "timestamp">{timestamp}</small>
                </p>
                <p class = "post-text"> {text} </p>
            </div>
        </div>
    }
}

#[component]
pub fn BottomBar() -> impl IntoView {
    view! {
        <div class = "bottom-bar">
            <PlusIcon/>
            <input type = "text" placeholder = "Enter message..." class = "bottom-bar-input" />
        </div>
    }
}


#[component]
pub fn ContentContainer() -> impl IntoView {
    let names = vec!["Ada", "Leon", "Jill", "Ellie", "Chris", "Claire", "Albert", "Rebecca", "H.U.N.K."];
    let timestamps = vec![
        "one week ago", "one week ago", "5 days ago", "4 days ago", "4 days ago", 
        "2 days ago", "22 hours ago", "3 hours ago", "Just now"
    ];

    let texts = vec![
        "Lorem ipsum dolor sit amet consectetur adipisicing elit. Lorem ipsum dolor sit
        amet consectetur adipisicing elit. Lorem ipsum dolor sit amet consectetur
        adipisicing elit. Lorem ipsum dolor sit amet consectetur adipisicing elit. Lorem
        ipsum dolor sit amet consectetur adipisicing elit.",

        "Lorem ipsum dolor.",

        "Lorem.",

        "Lorem ipsum dolor sit amet consectetur adipisicing elit.",

        "Lorem ipsum dolor sit amet consectetur adipisicing elit. Lorem ipsum dolor sit
        amet consectetur adipisicing elit. Lorem ipsum dolor sit amet consectetur
        adipisicing elit. Lorem ipsum dolor sit amet consectetur adipisicing elit. Lorem
        ipsum dolor sit amet consectetur adipisicing elit.
        
        Lorem ipsum dolor sit amet consectetur adipisicing elit. Lorem
        ipsum dolor sit amet consectetur adipisicing elit.
        ",

        "Lorem ipsum dolor sit amet consectetur adipisicing elit. Lorem ipsum dolor sit
        amet consectetur adipisicing elit. Lorem ipsum dolor sit amet consectetur
        adipisicing elit. Lorem ipsum dolor sit amet consectetur adipisicing elit.",

        "Lorem ipsum dolor sit amet consectetur adipisicing elit. ☺️",

        "Lorem ipsum dolor sit amet consectetur adipisicing elit. Lorem ipsum dolor sit
        amet consectetur adipisicing elit.",

        "Lorem ipsum dolor sit amet consectetur adipisicing elit. Lorem ipsum dolor sit
        amet consectetur adipisicing elit. Lorem ipsum dolor sit amet consectetur
        adipisicing elit. Lorem ipsum dolor sit amet consectetur adipisicing elit. Lorem
        ipsum dolor sit amet consectetur adipisicing elit."
    ];

    let all_posts = (0..9)
    .map(|i| {
        view! {
            <Post name = names[i].into() timestamp = timestamps[i].into() text = texts[i].into() />
        }
    }).collect_view();

    view! {
        <div class = "content-container">
            <TopNavigation/>
            <div class = "content-list">
                {all_posts}
            </div>
        
        </div>
    }
    

}
// const ChannelBar = () => {
//     return (
//       <div className='channel-bar shadow-lg'>
//         <ChannelBlock />
//         <div className='channel-container'>
//           <Dropdown header='Topics' selections={topics} />
//           <Dropdown header='Questions' selections={questions} />
//           <Dropdown header='Random' selections={random} />
//         </div>
//       </div>
//     );
//   };
  

