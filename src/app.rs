use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="flex flex-row bg-gradient-to-r from-stone-100 from-10% via-gray-300 via-30% to-zinc-100 to-90%">
            <div class=" h-screen w-1/4">

            </div>
            <div class="h-screen w-1/2">
                <div class="h-screen flex items-center justify-center">
                    <div class="text-center">
                        <h1 class="text-4xl mb-4">Hello, World!</h1>
                        <p class="text-lg mb-8">This is fp-space.</p>


                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
                            <a class="bg-gray-100 hover:bg-yellow-100 p-4 rounded-lg shadow-lg" href="https://github.com/fp-space/HappyDayReminder">
                                <h2 class="text-xl font-semibold mb-2">HappyDayReminder</h2>
                                <p class="text-sm">生日提醒</p>
                            </a>
                            <a class="bg-gray-100 hover:bg-cyan-100 p-4 rounded-lg shadow-lg" href="https://github.com/fp-space/MiniIM">
                                <h2 class="text-xl font-semibold mb-2">MiniIM</h2>
                                <p class="text-sm">IM 即时通讯</p>
                            </a>
                            <a class="bg-gray-100 hover:bg-violet-100 p-4 rounded-lg shadow-lg" href="https://github.com/fp-space/document">
                                <h2 class="text-xl font-semibold mb-2">document</h2>
                                <p class="text-sm">资料</p>
                            </a>
                        </div>
                    </div>
                </div>
            </div>
            <div class="h-screen w-1/4">

            </div>
        </div>
    }
}
