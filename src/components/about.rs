use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section id="about" class="py-20 bg-white dark:bg-gray-900">
            <div class="max-w-6xl mx-auto px-4">
                <h2 class="text-4xl font-bold text-center mb-12 text-gray-900 dark:text-white">
                    "About Me"
                </h2>
                <div class="max-w-3xl mx-auto space-y-8">
                    <p class="text-gray-700 dark:text-gray-300 leading-relaxed text-lg text-center">
                        "A passionate entrepreneur revolutionizing data compression in Silicon Valley 🦀💾"
                    </p>
                    <p class="text-gray-700 dark:text-gray-300 leading-relaxed text-lg text-center">
                        "Founder and CEO of Pied Piper, creator of the revolutionary middle-out compression algorithm with an unprecedented 5.2 Weissman score. Building the new decentralized internet, one node at a time."
                    </p>
                </div>
            </div>
        </section>
    }
}
