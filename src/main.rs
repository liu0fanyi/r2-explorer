mod models;
use models::{DufsItem, PathType};
use leptos::prelude::*;
use leptos::task::spawn_local;
use gloo_net::http::Request;

#[component]
fn App() -> impl IntoView {
    let (current_path, set_current_path) = signal(String::new());
    let (items, set_items) = signal(Vec::<DufsItem>::new());
    let (loading, set_loading) = signal(false);

    let fetch_items = move |path: String| {
        set_loading.set(true);
        spawn_local(async move {
            let url = format!("/api/list?prefix={}", js_sys::encode_uri_component(&path));
            if let Ok(resp) = Request::get(&url).send().await {
                if let Ok(data) = resp.json::<Vec<DufsItem>>().await {
                    set_items.set(data);
                }
            }
            set_loading.set(false);
        });
    };

    // Initial fetch
    Effect::new({
        let fetch_items = fetch_items.clone();
        move |_| {
            fetch_items(current_path.get());
        }
    });

    let navigate = move |name: String, is_dir: bool| {
        if is_dir {
            let mut new_path = current_path.get_untracked();
            if !new_path.is_empty() && !new_path.ends_with('/') {
                new_path.push('/');
            }
            new_path.push_str(&name);
            if !new_path.ends_with('/') {
                new_path.push('/');
            }
            set_current_path.set(new_path);
        }
    };

    let go_back = move || {
        let mut path = current_path.get_untracked();
        if path.ends_with('/') {
            path.pop();
        }
        if let Some(pos) = path.rfind('/') {
            let new_path = path[..=pos].to_string();
            set_current_path.set(new_path);
        } else {
            set_current_path.set(String::new());
        }
    };

    view! {
        <div class="min-h-screen bg-slate-50 text-slate-900 font-sans selection:bg-blue-100">
            // Header
            <header class="sticky top-0 z-30 bg-white/80 backdrop-blur-md border-b border-slate-200 px-6 py-4 flex items-center justify-between">
                <div class="flex items-center gap-4">
                    <div class="bg-blue-600 p-2 rounded-xl shadow-lg shadow-blue-200">
                        <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 10-9.78 2.096A4.001 4.001 0 003 15z" />
                        </svg>
                    </div>
                    <div>
                        <h1 class="text-lg font-black tracking-tight text-slate-800">"R2 Explorer"</h1>
                        <p class="text-[10px] uppercase tracking-widest text-slate-400 font-bold">"Cloudflare Storage" </p>
                    </div>
                </div>
                
                <div class="flex items-center gap-2">
                    <div class="h-8 w-8 rounded-full bg-slate-100 flex items-center justify-center text-slate-400">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" /></svg>
                    </div>
                </div>
            </header>

            <main class="max-w-7xl mx-auto p-6">
                // Breadcrumbs
                <nav class="mb-8 flex items-center gap-2 text-sm font-medium">
                    <button 
                        on:click=move |_| set_current_path.set(String::new())
                        class="text-slate-400 hover:text-blue-600 transition-colors"
                    >
                        "Root"
                    </button>
                    {move || {
                        let path = current_path.get();
                        path.split('/')
                            .filter(|s| !s.is_empty())
                            .map(|s| view! {
                                <span class="text-slate-300">"/"</span>
                                <span class="text-slate-600">{s.to_string()}</span>
                            })
                            .collect_view()
                    }}
                </nav>

                // Grid View
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
                    // Back button if not at root
                    <Show when=move || !current_path.get().is_empty()>
                        <div 
                            on:click=move |_| go_back()
                            class="group bg-white p-4 rounded-2xl border border-slate-200 hover:border-blue-300 hover:shadow-xl hover:shadow-blue-500/5 transition-all cursor-pointer flex items-center gap-4"
                        >
                            <div class="w-12 h-12 rounded-xl bg-slate-50 flex items-center justify-center text-slate-400 group-hover:bg-blue-50 group-hover:text-blue-600 transition-all">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 15l-3-3m0 0l3-3m-3 3h8M3 12a9 9 0 1118 0 9 9 0 01-18 0z" /></svg>
                            </div>
                            <span class="font-bold text-slate-600 group-hover:text-blue-600">"Go Back"</span>
                        </div>
                    </Show>

                    <For
                        each=move || items.get()
                        key=|item| format!("{}-{:?}", item.name, item.path_type)
                        children=move |item| {
                            let item_clone = item.clone();
                            let is_dir = item.path_type == PathType::Dir;
                            let name = item.name.clone();
                            
                            let media_url = if !is_dir {
                                let path = current_path.get();
                                format!("/media/{}{}", path, name)
                            } else {
                                String::new()
                            };

                            view! {
                                <div 
                                    on:click=move |_| {
                                        if is_dir {
                                            navigate(item_clone.name.clone(), true);
                                        } else {
                                            let _ = web_sys::window().unwrap().open_with_url_and_target(&media_url, "_blank");
                                        }
                                    }
                                    class="group bg-white p-4 rounded-2xl border border-slate-200 hover:border-blue-300 hover:shadow-xl hover:shadow-blue-500/5 transition-all cursor-pointer flex flex-col gap-3"
                                >
                                    <div class="flex items-start justify-between">
                                        <div class=format!("w-12 h-12 rounded-xl flex items-center justify-center transition-all {}", 
                                            if is_dir { "bg-blue-50 text-blue-600" } else { "bg-slate-50 text-slate-400 group-hover:bg-blue-50 group-hover:text-blue-600" }
                                        )>
                                            {if is_dir {
                                                view! { <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24"><path d="M10 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-2z"/></svg> }.into_any()
                                            } else {
                                                view! { <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z"/></svg> }.into_any()
                                            }}
                                        </div>
                                        
                                        <div class="opacity-0 group-hover:opacity-100 transition-opacity">
                                            <svg class="w-5 h-5 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" /></svg>
                                        </div>
                                    </div>

                                    <div class="flex flex-col">
                                        <span class="font-bold text-slate-700 truncate group-hover:text-blue-600 transition-colors">{name}</span>
                                        <span class="text-[10px] text-slate-400 font-bold uppercase tracking-wider">
                                            {if is_dir { "Folder".to_string() } else { 
                                                format!("{:.2} MB", item.size.unwrap_or(0) as f64 / 1024.0 / 1024.0)
                                            }}
                                        </span>
                                    </div>
                                </div>
                            }
                        }
                    />
                </div>

                // Loading state
                <Show when=move || loading.get()>
                    <div class="fixed bottom-8 left-1/2 -translate-x-1/2 bg-white/80 backdrop-blur-md px-6 py-3 rounded-full shadow-2xl border border-slate-200 flex items-center gap-3 animate-bounce">
                        <div class="w-2 h-2 bg-blue-600 rounded-full animate-pulse"></div>
                        <span class="text-xs font-black uppercase tracking-widest text-slate-600">"Syncing with R2..."</span>
                    </div>
                </Show>
            </main>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
