use components::{checkbox::Checkbox, gleaderboard::Gleaderboard, conditionalview::{CondSig, CondSure}};
use leptos::prelude::*;
use shitmap::ShitMap;
use std::{collections::HashMap, time::Duration};

mod components;
mod datasource;
mod paths;
mod shitmap;

fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App />});
}

#[component]
pub fn App() -> impl IntoView {
    let (show_sidebar, set_show_sidebar) = signal(false);
    let show_terrs = RwSignal::new(true);
    let show_conns = RwSignal::new(true);
    let show_res = RwSignal::new(true);
    let show_timers = RwSignal::new(true);
    let show_guild_leaderboard = RwSignal::new(true);

    let (tupd, set_tupd) = signal(());

    let tiles = LocalResource::new(async move || datasource::load_map_tiles().await.unwrap());
    let extradata =
        LocalResource::new(async move || datasource::get_extra_terr_info().await.unwrap());
    let terrs = LocalResource::new(move || {
        tupd.track();
        let e = async || datasource::get_wynntils_terrs().await.unwrap();
        e()
    });

    // auto update the map every 30 seconds
    set_interval(
        move || {
            set_tupd.notify();
        },
        Duration::from_secs(10),
    );

    let tiles = move || tiles.get().map(|t| t.take()).unwrap_or(Vec::new());
    let extradata = move || extradata.get().map(|t| t.take()).unwrap_or(HashMap::new());
    let terrs = Memo::new(move |_| terrs.get().map(|t| t.take()).unwrap_or(HashMap::new()));

    let conn_path = move || paths::create_route_paths(terrs.get(), extradata());

    view! {
        <ShitMap>
            // map tiles
            <div>
            {move || tiles().iter().map(|tile| {
                let url = tile.url.as_ref().to_string();
                let width = tile.width();
                let height = tile.height();
                let left = tile.left_side();
                let top = tile.top_side();

                view! {
                    <img
                        src={url}
                        class="shitmap-tile"
                        style:width={move || format!("{}px", width + 1.0)}
                        style:height={move || format!("{}px", height + 1.0)}
                        style:transform={move || format!("translate3D({}px, {}px, 0)", left, top)}
                    />
                }
            }).collect_view()}
            </div>

            // conns
            <svg style="position: absolute;overflow: visible;contain: layout;" class:hidden={move || !show_conns.get()}>
                <path
                    id="connpath"
                    d={move || conn_path()}
                    style="fill:none;"
                    stroke-linecap="round"
                />
                <g inner_html=
                {
                    "
                    <use
                        href=\"#connpath\"
                        style=\"stroke:black;stroke-width:4;\"
                    />
                    <use
                        href=\"#connpath\"
                        style=\"stroke:white;stroke-width:2;\"
                    />
                    "
                }/>
            </svg>

            // territories
            <CondSig cond=show_terrs> <div>
                <For
                    each=move || terrs.get().into_iter()
                    key=|(k, v)| (k.clone(), v.guild.clone())
                    children=move |(k, v)| {
                        let width = v.location.width();
                        let height = v.location.height();
                        let left = v.location.left_side();
                        let top = v.location.top_side();
                        let col = v.guild.get_color();
                        let col = format!("{}, {}, {}", col.0, col.1, col.2);
                        let col2 = col.clone();

                        let now = chrono::Utc::now();
                        let time = now.signed_duration_since(v.acquired).num_seconds();

                        let (time, set_time) = signal(time);

                        set_interval(move || {
                            let now = chrono::Utc::now();

                            let time = now.signed_duration_since(v.acquired).num_seconds();

                            set_time.set(time);
                        }, Duration::from_millis(1000));

                        let timestr = move || {
                            let time = time.get();

                            let days = time / 86400;
                            let hours = (time % 86400) / 3600;
                            let minutes = (time % 3600) / 60;
                            let seconds = time % 60;

                            if days > 0 {
                                format!("{}d {}h", days, hours)
                            } else if hours > 0 {
                                format!("{}h {}m", hours, minutes)
                            } else if minutes > 0 {
                                format!("{}m {}s", minutes, seconds)
                            } else {
                                format!("{}s", seconds)
                            }
                        };

                        let color = move || {
                            let time = time.get();

                            // times based on treasury
                            if time < 3600 {
                                "background-color: oklch(0.723 0.219 149.579 / .6)"
                            } else if time < (3600 * 24) {
                                "background-color: oklch(0.768 0.233 130.85 / .6);"
                            } else if time < (3600 * 24 * 5) {
                                "background-color: oklch(0.795 0.184 86.047 / .6);"
                            } else if time < (3600 * 24 * 12) {
                                "background-color: oklch(0.705 0.213 47.604 / .6);"
                            } else {
                                "background-color: oklch(0.637 0.237 25.331 / .6);"
                            }
                        };

                        let tkey = k.clone();
                        let extra = Memo::new(move |_| extradata().get(&tkey).cloned());

                        let res = Memo::new(move |_| {
                            if let Some(e) = extra.get() {
                                e.resources.has_res()
                            } else {
                                (false, false, false, false, false)
                            }
                        });

                        view! {
                            <div class="shitmap-item guildterr"
                                style:width={move || format!("{}px", width)}
                                style:height={move || format!("{}px", height)}
                                style:transform={move || format!("translate3D({}px, {}px, 0)", left, top)}
                                style:background-color={move || format!("rgba({}, 0.35)", col)}
                                style:border-color={move || format!("rgb({})", col2)}
                            >
                                <h3 class="font-bold text-3xl text-white textshadow">{v.guild.prefix.clone()}</h3>
                                <CondSig cond=true>
                                    <div class="flex pb-1">
                                        // this is here so that tailwinds cli realizes that this class is used
                                        // class="hidden"
                                        <CondSure cond={move || res.get().0}> <div class="icon-emerald"/> </CondSure>
                                        <CondSure cond=res.get().1> <div class="icon-crops"/> </CondSure>
                                        <CondSure cond=res.get().2> <div class="icon-fish"/> </CondSure>
                                        <CondSure cond=res.get().3> <div class="icon-ores"/> </CondSure>
                                        <CondSure cond=res.get().4> <div class="icon-wood"/> </CondSure>
                                    </div>
                                </CondSig>
                                <CondSig cond=show_timers>
                                    <h4 class="px-2 rounded-2xl text-sm text-center whitespace-nowrap" style={move || color}>{timestr}</h4>
                                </CondSig>
                            </div>
                        }
                    }
                />
            </div></CondSig>
        </ShitMap>

        // sidebar open button
        <div on:click={move |_| set_show_sidebar.set(!show_sidebar.get())} class="fixed top-0 left-0 p-2 cursor-pointer z-50 bg-neutral-900 rounded-e-full mt-2 p-2">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-8 text-white">
                <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
            </svg>
        </div>
        // class="-translate-x-full"
        <div class="flex flex-col bg-neutral-900 w-full max-w-full h-screen z-50 absolute top-0 md:max-w-sm transition-transform text-white" class:-translate-x-full={move || !show_sidebar.get()}>
            // top text
            <div>
                <div class="flex justify-between p-2 items-center">
                    <h1 class="text-4xl">Wynnmap</h1>

                    // close button
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-8 cursor-pointer" on:click={move |_| set_show_sidebar.set(!show_sidebar.get())}>
                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
                    </svg>
                </div>
                <hr class="border-neutral-600" />
            </div>

            // checkboxes
            <div class="flex-1 flex flex-col p-2 text-lg">
                <Checkbox id="terrs" checked={show_terrs}>"Territories"</Checkbox>
                <Checkbox id="conns" checked={show_conns}>"Connections"</Checkbox>
                <Checkbox id="resico" checked={show_res}>"Resource icons"</Checkbox>
                <Checkbox id="timers" checked={show_timers}>"Timers"</Checkbox>
            </div>

            // guild leaderboard
            <div class="flex flex-col min-h-0">
                <hr class="border-neutral-600" />
                <div class="flex justify-between items-center text-xl p-2 py-1" on:click={move |_| show_guild_leaderboard.set(!show_guild_leaderboard.get())}>
                    <h2>"Guild leaderboard"</h2>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6 cursor-pointer" >
                        <path stroke-linecap="round" stroke-linejoin="round" d="m4.5 15.75 7.5-7.5 7.5 7.5" class:hidden={move || show_guild_leaderboard.get()} />
                        <path stroke-linecap="round" stroke-linejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5" class:hidden={move || !show_guild_leaderboard.get()} />
                    </svg>
                </div>
                <hr class="border-neutral-600" class:hidden={move || !show_guild_leaderboard.get()} />
                <div class="overflow-scroll shrink min-h-0">
                    <Gleaderboard terrs={terrs} class="w-full" class:hidden={move || !show_guild_leaderboard.get()} />
                </div>
            </div>

            // bottom text
            <div>
                <hr class="border-neutral-600" />
                <h2 class="text-neutral-500 p-1 px-2"><a class="underline" href="https://github.com/Zatzou/wynnmap" target="_blank">"Wynnmap"</a>" "{env!("CARGO_PKG_VERSION")}</h2>
            </div>
        </div>
    }
}
