use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum SortOrder {
    Alphabetical,
    ReverseAlphabetical,
}

#[derive(Clone)]
struct Project {
    title: &'static str,
    description: &'static str,
    technologies: Vec<&'static str>,
    link: &'static str,
}

#[component]
pub fn Projects() -> impl IntoView {
    let (sort_order, set_sort_order) = signal(SortOrder::Alphabetical);

    let projects = vec![
        Project {
            title: "middle-out",
            description: "The core compression algorithm that started it all. Achieves a 5.2 Weissman score through revolutionary middle-out approach.",
            technologies: vec!["Rust", "C++", "Algorithms"],
            link: "https://github.com",
        },
        Project {
            title: "piperchat",
            description: "End-to-end encrypted video chat with superior compression. Real-time communication without compromising quality.",
            technologies: vec!["WebRTC", "Rust", "TypeScript"],
            link: "https://github.com",
        },
        Project {
            title: "pipercoin",
            description: "Cryptocurrency built on top of the decentralized Pied Piper network. Secure, fast, and truly decentralized.",
            technologies: vec!["Blockchain", "Cryptography", "Rust"],
            link: "https://github.com",
        },
        Project {
            title: "pipernet",
            description: "Decentralized peer-to-peer network protocol for the new internet. Making centralized servers obsolete.",
            technologies: vec!["Distributed Systems", "Networking", "Rust"],
            link: "https://github.com",
        },
        Project {
            title: "smartfridge-sdk",
            description: "IoT compression solution for smart refrigerator data. Because even your fridge deserves good compression.",
            technologies: vec!["IoT", "Embedded", "C"],
            link: "https://github.com",
        },
        Project {
            title: "tabs-not-spaces",
            description: "Static analysis tool for enforcing tabs (the objectively correct choice). Saving bytes one indentation at a time.",
            technologies: vec!["Tooling", "Rust", "CLI"],
            link: "https://github.com",
        },
    ];

    let sorted_projects = move || {
        let mut projects_clone = projects.clone();
        match sort_order.get() {
            SortOrder::Alphabetical => {
                projects_clone.sort_by(|a, b| a.title.cmp(b.title));
            }
            SortOrder::ReverseAlphabetical => {
                projects_clone.sort_by(|a, b| b.title.cmp(a.title));
            }
        }
        projects_clone
    };

    view! {
        <section id="projects" class="py-20 bg-gray-100 dark:bg-gray-800">
            <div class="max-w-6xl mx-auto px-4">
                <div class="flex justify-between items-center mb-12">
                    <h2 class="text-4xl font-bold text-gray-900 dark:text-white">
                        "Projects"
                    </h2>
                    <div class="flex gap-2">
                        <button
                            on:click=move |_| set_sort_order.set(SortOrder::Alphabetical)
                            class=move || {
                                let base = "px-4 py-2 rounded-lg transition-colors";
                                if sort_order.get() == SortOrder::Alphabetical {
                                    format!("{} bg-blue-500 text-white", base)
                                } else {
                                    format!("{} bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600", base)
                                }
                            }
                        >
                            "A-Z"
                        </button>
                        <button
                            on:click=move |_| set_sort_order.set(SortOrder::ReverseAlphabetical)
                            class=move || {
                                let base = "px-4 py-2 rounded-lg transition-colors";
                                if sort_order.get() == SortOrder::ReverseAlphabetical {
                                    format!("{} bg-blue-500 text-white", base)
                                } else {
                                    format!("{} bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600", base)
                                }
                            }
                        >
                            "Z-A"
                        </button>
                    </div>
                </div>
                <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                    {move || sorted_projects().into_iter().map(|project| {
                        view! {
                            <a
                                href=project.link
                                target="_blank"
                                rel="noopener noreferrer"
                                class="block bg-white dark:bg-gray-900 rounded-lg shadow-lg overflow-hidden hover:shadow-xl transition-all border border-gray-300 dark:border-gray-700 hover:scale-105 cursor-pointer"
                            >
                                <div class="p-6">
                                    <h3 class="text-xl font-bold mb-3 text-gray-900 dark:text-white">{project.title}</h3>
                                    <p class="text-gray-700 dark:text-gray-300 mb-4 leading-relaxed">{project.description}</p>
                                    <div class="flex flex-wrap gap-2">
                                        {project.technologies.into_iter().map(|tech| {
                                            view! {
                                                <span class="px-3 py-1 bg-gray-100 dark:bg-gray-800 text-gray-900 dark:text-gray-300 rounded text-sm border border-gray-300 dark:border-gray-700">
                                                    {tech}
                                                </span>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>
                            </a>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}
