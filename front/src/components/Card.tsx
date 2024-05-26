import { Accessor } from "solid-js";

export default function Card(props: { isLoading: Accessor<boolean>, URL: string, title: string, description: string }) {
    return (
        <div class={`flex items-center rounded shadow h-14 bg-stone-200 border ${props.isLoading() ? "animate-pulse" : ""}`}>
            <div class="flex grow mx-2 md:mx-4 lg:mx-8">
                <div class="grow">
                    <h1 class="font-bold text-sm">{props.title}</h1>
                    <h2 class="font-light text-sm">
                        {props.description}
                    </h2>
                </div>
                <a href={props.URL} class="grow-0 bg-black rounded h-6 w-6 my-auto text-center">
                    {props.isLoading() ? "⌛" : "🔗"}
                </a>
            </div>
        </div>
    )
}