use leptos::prelude::*;

#[derive(Clone)]
struct Job {
    title: &'static str,
    company: &'static str,
    period: &'static str,
    achievements: Vec<&'static str>,
}

#[component]
pub fn Experience() -> impl IntoView {
    let jobs = vec![
        Job {
            title: "CEO & Founder",
            company: "Pied Piper",
            period: "Sep 2014 - Present",
            achievements: vec![
                "Developed revolutionary middle-out compression algorithm achieving unprecedented 5.2 Weissman score",
                "Led company through TechCrunch Disrupt victory, securing initial funding and industry recognition",
                "Pivoted company strategy multiple times to stay ahead of Hooli's competitive threats",
                "Built decentralized peer-to-peer internet infrastructure capable of replacing traditional web",
                "Grew engineering team from 4 to 50+ while maintaining startup culture and technical excellence",
                "Negotiated partnerships with major tech companies while protecting core IP",
            ],
        },
        Job {
            title: "Software Engineer",
            company: "Hooli",
            period: "Jun 2012 - Aug 2014",
            achievements: vec![
                "Worked on Nucleus project before identifying fundamental architectural limitations",
                "Developed initial compression concepts that would later become Pied Piper's core technology",
                "Collaborated with cross-functional teams on large-scale distributed systems",
                "Left to pursue independent venture after recognizing potential of middle-out approach",
            ],
        },
        Job {
            title: "Software Engineering Intern",
            company: "Various Silicon Valley Startups",
            period: "May 2010 - May 2012",
            achievements: vec![
                "Gained hands-on experience in Silicon Valley startup culture and rapid development cycles",
                "Learned importance of intellectual property protection and technical due diligence",
                "Built foundational skills in systems programming and algorithm optimization",
            ],
        },
    ];

    let total_jobs = jobs.len();
    let (current_index, set_current_index) = signal(total_jobs - 1);
    let (show_all, set_show_all) = signal(true);
    let jobs_stored = StoredValue::new(jobs);

    view! {
        <section id="experience" class="py-20 bg-white dark:bg-gray-900">
            <div class="max-w-4xl mx-auto px-4">
                <div class="flex justify-between items-center mb-12">
                    <h2 class="text-4xl font-bold text-gray-900 dark:text-white">
                        "Experience"
                    </h2>
                    <button
                        on:click=move |_| set_show_all.update(|v| *v = !*v)
                        class="px-4 py-2 bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white rounded-lg hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors"
                    >
                        {move || if show_all.get() { "Show Roles" } else { "Show Timeline" }}
                    </button>
                </div>
                <Show
                    when=move || show_all.get()
                    fallback=move || view! {
                        <div class="space-y-8">
                            <div class="flex justify-between items-center mb-8">
                                <button
                                    on:click=move |_| set_current_index.update(|i| *i = i.saturating_sub(1))
                                    class="px-4 py-2 bg-blue-500 dark:bg-blue-500 text-white rounded-lg hover:bg-blue-600 dark:hover:bg-blue-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                                    disabled=move || current_index.get() == 0
                                >
                                    "← Older"
                                </button>
                                <span class="text-gray-700 dark:text-gray-300">
                                    {move || format!("{} of {}", current_index.get() + 1, total_jobs)}
                                </span>
                                <button
                                    on:click=move |_| set_current_index.update(|i| *i = (*i + 1).min(total_jobs - 1))
                                    class="px-4 py-2 bg-blue-500 dark:bg-blue-500 text-white rounded-lg hover:bg-blue-600 dark:hover:bg-blue-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                                    disabled={move || current_index.get() >= total_jobs - 1}
                                >
                                    "Newer →"
                                </button>
                            </div>
                            <div style="min-height: 500px;">
                                {move || {
                                    let index = current_index.get();
                                    let reversed_index = total_jobs - 1 - index;
                                    jobs_stored.with_value(|jobs| {
                                        let job = &jobs[reversed_index];
                                        view! {
                                        <div class="border-l-4 border-blue-500 dark:border-blue-500 pl-6">
                                            <h3 class="text-2xl font-bold text-gray-900 dark:text-white">{job.title}</h3>
                                            <p class="text-lg text-gray-700 dark:text-gray-300 mb-2">{job.company}</p>
                                            <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">{job.period}</p>
                                            <ul class="space-y-2">
                                                {job.achievements.iter().map(|achievement| {
                                                    view! {
                                                        <li class="flex items-start">
                                                            <span class="text-blue-500 dark:text-blue-400 mr-2">"•"</span>
                                                            <span class="text-gray-700 dark:text-gray-300">{*achievement}</span>
                                                        </li>
                                                    }
                                                }).collect_view()}
                                            </ul>
                                        </div>
                                    }
                                    })
                                }}
                            </div>
                        </div>
                    }
                >
                    <div class="space-y-8">
                        {jobs_stored.with_value(|jobs| {
                            jobs.iter().map(|job| {
                                view! {
                                    <div class="border-l-4 border-blue-500 dark:border-blue-500 pl-6">
                                        <h3 class="text-2xl font-bold text-gray-900 dark:text-white">{job.title}</h3>
                                        <p class="text-lg text-gray-700 dark:text-gray-300 mb-2">{job.company}</p>
                                        <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">{job.period}</p>
                                        <ul class="space-y-2">
                                            {job.achievements.iter().map(|achievement| {
                                                view! {
                                                    <li class="flex items-start">
                                                        <span class="text-blue-500 dark:text-blue-400 mr-2">"•"</span>
                                                        <span class="text-gray-700 dark:text-gray-300">{*achievement}</span>
                                                    </li>
                                                }
                                            }).collect_view()}
                                        </ul>
                                    </div>
                                }
                            }).collect_view()
                        })}
                    </div>
                </Show>
            </div>
        </section>
    }
}
