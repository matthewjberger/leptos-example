use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="py-8 bg-gray-900 dark:bg-black border-t border-gray-800">
            <div class="max-w-6xl mx-auto px-4 text-center">
                <p class="text-gray-400">
                    "Built with "
                    <a
                        href="https://leptos.dev"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="text-blue-400 hover:text-blue-300"
                    >
                        "Leptos"
                    </a>
                    " and "
                    <a
                        href="https://tailwindcss.com"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="text-blue-400 hover:text-blue-300"
                    >
                        "Tailwind CSS"
                    </a>
                </p>
            </div>
        </footer>
    }
}
