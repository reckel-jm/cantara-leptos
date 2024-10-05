use html::button;
use leptos::*;
use web_sys::HtmlElement;


#[derive(Clone)]
enum ActivePage {
    SongSelection,
    Presentation,
    Settings
}

impl ActivePage {
    pub fn get_title(&self) -> String {
        match self {
            ActivePage::SongSelection => "Song Selection".to_string(),
            ActivePage::Presentation => "Presentation".to_string(),
            ActivePage::Settings => "Settings".to_string() 
        }
    }
}

#[derive(Debug, Clone)]
struct SongFile {
    pub name: String,
    pub author: String,
    pub tags: Vec<String>
}

fn main() {
    console_error_panic_hook::set_once();

    let song_amazing_grace = SongFile {
        name: "Amazing Grace".to_string(),
        author: "John Newton".to_string(),
        tags: vec!["Classical".to_string()],
    };

    let how_can_it_be = SongFile {
        name: "How can it be".to_string(),
        author: "Charles Wesley".to_string(),
        tags: vec!["Worship".to_string()],
    };

    let song_repo = vec![song_amazing_grace, how_can_it_be];

    let (song_repo_reader, song_repo_writer) = create_signal(song_repo);

    mount_to_body(move || view!(<p><App song_repo_reader=song_repo_reader /></p>));
}

#[component]
fn App(song_repo_reader: ReadSignal<Vec<SongFile>>) -> impl IntoView {

    let default_page = ActivePage::SongSelection;
    let (active_page, set_active_page) = create_signal(default_page);
    let (view_header, set_view_header) = create_signal(true);

    view! {
        <Header display=view_header />
        <Menubar active_page=active_page set_active_page=set_active_page />

        <div class="w3-container w3-theme" style="margin-left:38.5px;min-height:100%">
            { move || {
                match active_page.get() {
                    ActivePage::SongSelection => view! { <div class="w3-animate-opacity"><SongSelection song_repo_reader=song_repo_reader /></div> }.into_view(),
                    ActivePage::Settings => view! { <div class="w3-animate-opacity"><SettingsPage /></div> }.into_view(),
                    _ => view! { <SongSelection song_repo_reader=song_repo_reader /> }.into_view()
                }
            } }
        </div>
    }
}

#[component]
fn SongSelection(song_repo_reader: ReadSignal<Vec<SongFile>>) -> impl IntoView {

    view! {
        <ul class="w3-ul w3-card-4 w3-hoverable" style="width:50%">
            <For
                each=move || song_repo_reader.get()
                key=|(file)| file.name.clone()
                let:child
            >
                <li>{ child.name }</li>
            </For>
        </ul>
    }
}

fn SettingsPage() -> impl IntoView {
    view! {
        <h2>Settings</h2>
    }
}

#[component]
fn CountButton(title: ReadSignal<String>) -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button class="w3-btn"
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
        { move || title.get() }
        {move || count.get() }
        </button>
    }
}

#[component]
fn ShowHeaderButton(get_show_header: ReadSignal<bool>, set_show_header: WriteSignal<bool>) -> impl IntoView {
    view! {
        <button class="w3-button"
            on:click=move |_| {
                set_show_header.set(!get_show_header.get());
            }
        >
        {move || {
                if get_show_header.get() {
                    "Header shown"
                } else {
                    "Header not shown"
                }
            } 
        }
        </button>
    }
}

#[component]
fn Header(display: ReadSignal<bool>) -> impl IntoView {
    view! {
        <div class="w3-row w3-padding w3-theme-d2 w3-large">
            <div class="w3-quarter">
                <div class="w3-bar w3-cell-middle" style="margin:auto">
                    Cantara
                </div>
            </div>

            <div class="w3-half">
                <input type="text" class="w3-amber w3-border-0 w3-padding" style="width:100%" />
            </div>

            <div class="w3-quarter">
                <div class="w3-bar w3-large">
                    <a href="#" class="w3-bar-item w3-button w3-left"><i class="fa fa-search"></i></a>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Menubar(active_page: ReadSignal<ActivePage>, set_active_page: WriteSignal<ActivePage>) -> impl IntoView {
    view!{
        <div class="w3-sidebar w3-bar-block w3-theme-d1" style="width:auto">
            <button class="w3-button"
                on:click = move |_| {
                    set_active_page.set(ActivePage::SongSelection);
                }
            >
                <i class="fa fa-bars"></i>
            </button><br/>
            <button class="w3-button"
                on:click = move |_| {
                    set_active_page.set(ActivePage::Presentation);
                }
            >
                <i class="fa fa-laptop"></i>
            </button><br/>
            <button class="w3-button"
                on:click = move |_| {
                    set_active_page.set(ActivePage::Settings);
                }
            >
                <i class="fa fa-wrench"></i>
            </button>
        </div>
    }
}