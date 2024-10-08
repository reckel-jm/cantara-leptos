use html::button;
use leptos::*;
use web_sys::HtmlElement;
use leptos_hotkeys::use_hotkeys;

#[derive(Clone, PartialEq)]
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
    pub tags: Vec<String>,
    pub filepath: String,
}

fn main() {
    console_error_panic_hook::set_once();

    let song_amazing_grace = SongFile {
        name: "Amazing Grace".to_string(),
        author: "John Newton".to_string(),
        tags: vec!["Classical".to_string()],
        filepath: "".to_string()
    };

    let song_how_can_it_be = SongFile {
        name: "How can it be".to_string(),
        author: "Charles Wesley".to_string(),
        tags: vec!["Worship".to_string(), "Oldschool".to_string()],
        filepath: "".to_string()
    };

    let song_was_man_mit_geld = SongFile {
        name: "Was man mit Geld nicht kaufen kann".to_string(),
        author: "Jan Martin Reckel".to_string(),
        tags: vec!["Neue Lieder".to_string()],
        filepath: "".to_string()
    };

    let song_repo = vec![song_amazing_grace, song_how_can_it_be, song_was_man_mit_geld];

    let (song_repo_reader, song_repo_writer) = create_signal(song_repo);

    mount_to_body(move || view!(<App song_repo_reader=song_repo_reader />));
}

#[component]
fn App(song_repo_reader: ReadSignal<Vec<SongFile>>) -> impl IntoView {

    let default_page = ActivePage::SongSelection;
    let (active_page, set_active_page) = create_signal(default_page);
    
    let (show_header, set_show_header) = create_signal(true);
    let song_selection: Vec<SongFile> = vec![];
    let (song_selection, set_song_selection) = create_signal(song_selection);
    
    /*use_hotkeys!(("keys") => move |_| {
        dbg!("Pressed an f");
    });*/
    
    view! {
        <Show when=move || active_page.get() != ActivePage::Presentation><Header /></Show>
        
        <Menubar active_page=active_page set_active_page=set_active_page />

        <div class="w3-container w3-theme" style="margin-left:38.5px;height:100%">
            <Show when=move || active_page.get() == ActivePage::SongSelection>
                <div class="w3-animate-opacity fill-full"><SongSelectionPage song_repo_reader=song_repo_reader song_selection_reader=song_selection song_selection_writer=set_song_selection /></div>
            </Show>
            <Show when=move || active_page.get() == ActivePage::Settings>
                <div class="w3-animate-opacity fill-full"><SettingsPage /></div>
            </Show>
        </div>
    }
}

/// This Component renders a page where the Song Selection inside cantara takes place
#[component]
fn SongSelectionPage(
    /// A read signal which contains all SongFiles of the song repository
    song_repo_reader: ReadSignal<Vec<SongFile>>,
    /// The ReadSignal to the SongSelection vector where the selected SongFiles are saved
    song_selection_reader: ReadSignal<Vec<SongFile>>,
    /// The WriteSignal to the SongSelection vector where the selected SongFiles are saved, they will be modified inside this page.
    song_selection_writer: WriteSignal<Vec<SongFile>>   
    ) -> impl IntoView {
        
    let select_song = move |i: usize| {
        dbg!(i);
        let song_files = song_repo_reader.get();
        let song_file = song_files.get(i).unwrap();
        song_selection_writer.update(|x| x.push(song_file.clone()))
    };

    view! {
        <div class="row"> 
            <div class="w3-hoverable w3-mobile column selection-scroll-area">
                <For
                    each=move || song_repo_reader.get().into_iter().enumerate()
                    key=|(_, file)| file.name.clone()
                    let:child
                >
                    <button class="w3-button" style="width:100%"
                        on:click=move |_| {
                            select_song(child.0);
                            log::info!("Printed {}", child.0);
                        }
                        on:contextmenu=move |_| {
                            log::info!("Right click issued");
                        }
                    >
                        <strong>{ child.1.name }</strong><br/>
                        { child.1.author }<br />
                        <SongFileTagsBadges song_file_tags=create_signal(child.1.tags).0 />
                    </button>
                </For> 
            </div>
            <SongSelectionBox song_selection_reader=song_selection_reader song_selection_writer=song_selection_writer/>
        </div>
        <SongSelectionFooter />
    }
}

#[component]
fn SongFileTagsBadges(song_file_tags: ReadSignal<Vec<String>>) -> impl IntoView {
    view! {
        <For
            each=move || song_file_tags.get()
            key=|entry| entry.clone()
            let:entry
        >
            <span class="w3-tag wd-round-small w3-red songtag">{entry}</span>
        </For>
    }
}

/// This is the footer of the song selection page (where some buttons are located)
#[component]
fn SongSelectionFooter() -> impl IntoView {
    view! {
        <div class="footer">
            <p>Presentation buttons come here</p>
        </div>
    }
}

#[component]
fn SongSelectionBox(
    /// The ReadSignal to the SongSelection vector where the selected SongFiles are saved
    song_selection_reader: ReadSignal<Vec<SongFile>>,
    /// The WriteSignal to the SongSelection vector where the selected SongFiles are saved, they will be modified inside this page.
    song_selection_writer: WriteSignal<Vec<SongFile>>
) -> impl IntoView {
    view! {
        <div class="w3-hoverable w3-mobile column selection-scroll-area">
            <Show when=move || {song_selection_reader.get().is_empty()}>
                <p>Please select a song first</p>
            </Show>
            <Show when=move || {!song_selection_reader.get().is_empty()}>
                <For
                    each=move || song_selection_reader.get().into_iter().enumerate()
                    key=|(i, entry)| (i.clone(), entry.name.clone(), entry.filepath.clone())
                    let:child
                >
                    <button class="w3-button w3-animate-bottom" style="width:100%">
                        <strong>{ child.1.name }</strong><br/>
                        { child.1.author }<br />
                        <SongFileTagsBadges song_file_tags=create_signal(child.1.tags).0 />
                        <SongSelectionDeleteEntryButton 
                            index={child.0}
                            song_selection_writer = song_selection_writer
                        /> 
                    </button>
                </For> 
            </Show>
        </div>
    }
}

#[component]
fn SongSelectionDeleteEntryButton(
    index: usize,
    song_selection_writer: WriteSignal<Vec<SongFile>>
) -> impl IntoView {
    let delete_entry = move || {
        song_selection_writer.update(
            |songselection| {
                let _ = songselection.remove(index);
            }
        )
    };
    
    view ! {
        <button 
            class="w3-circle" 
            on:click=move |_| { delete_entry(); }
        >{index.to_string()}</button>
    }
}


#[component]
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
fn Header() -> impl IntoView {
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
