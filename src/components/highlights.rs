use leptos::prelude::*;

#[derive(Clone)]
struct Highlight {
    title: &'static str,
    description: &'static str,
    link: &'static str,
    image: Option<&'static str>,
}

#[component]
pub fn Highlights() -> impl IntoView {
    let highlights = vec![
        Highlight {
            title: "Middle-Out Compression",
            description: "Revolutionary lossless compression algorithm achieving an unprecedented 5.2 Weissman score. The core technology behind Pied Piper's success.",
            link: "https://github.com",
            image: Some(
                "https://images.unsplash.com/photo-1558494949-ef010cbdcc31?w=400&h=300&fit=crop",
            ),
        },
        Highlight {
            title: "PiperNet",
            description: "Decentralized peer-to-peer network that makes the traditional internet obsolete. Built for privacy, speed, and true decentralization.",
            link: "https://github.com",
            image: Some(
                "https://images.unsplash.com/photo-1451187580459-43490279c0fa?w=400&h=300&fit=crop",
            ),
        },
    ];

    view! {
        <section
            id="highlights"
            class="py-20 bg-gradient-to-br from-blue-50 to-indigo-50 dark:from-gray-900 dark:to-gray-800"
        >
            <div class="max-w-6xl mx-auto px-4">
                <div class="text-center mb-12">
                    <h2 class="text-4xl font-bold text-gray-900 dark:text-white">"Highlights"</h2>
                </div>
                <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                    {highlights
                        .into_iter()
                        .map(|highlight| {
                            view! {
                                <a
                                    href=highlight.link
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="block bg-white dark:bg-gray-900 rounded-lg shadow-lg overflow-hidden border border-gray-300 dark:border-gray-700 hover:shadow-xl transition-all hover:scale-105 cursor-pointer flex flex-col"
                                >
                                    {highlight
                                        .image
                                        .map(|img| {
                                            view! {
                                                <img
                                                    src=img
                                                    alt=highlight.title
                                                    class="w-full h-48 object-cover"
                                                />
                                            }
                                        })}
                                    <div class="p-6 flex-1">
                                        <h3 class="text-xl font-bold mb-3 text-gray-900 dark:text-white">
                                            {highlight.title}
                                        </h3>
                                        <p class="text-gray-700 dark:text-gray-300 leading-relaxed">
                                            {highlight.description}
                                        </p>
                                    </div>
                                </a>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>
    }
}
