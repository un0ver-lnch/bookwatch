import { Accessor, Setter } from "solid-js";
import { CardProps } from "~/routes";

export default function Card(props: { isLoading: Accessor<boolean>, card: Accessor<CardProps>, cards: Accessor<CardProps[]>, setCards: Setter<CardProps[]> }) {

    const removeCardServer = async (uuid: string) => {
        "use server"
        await fetch(`http://${process.env.BACK_HOST}:3001/api/card/${uuid}`, {
            method: "DELETE",
        });
    }


    const removeCard = async (uuid: string) => {
        // Optimistically update the cards
        props.setCards(props.cards().filter(card => card.uuid !== uuid));
        // Update the server
        await removeCardServer(uuid);
    }

    return (
        <div class={`flex items-center rounded shadow h-14 bg-stone-200 border ${props.isLoading() ? "animate-pulse" : ""}`}>
            <div class="flex grow mx-2 md:mx-4 lg:mx-8">
                <div class="grow">
                    <h1 class="font-bold text-sm">{props.card().title}</h1>
                    <h2 class="font-light text-sm">
                        {props.card().description}
                    </h2>
                </div>
                <a href={props.card().url} target="_blank" class="grow-0 bg-black rounded h-6 w-6 my-auto text-center">
                    {props.isLoading() ? "⌛" : "🔗"}
                </a>
                <button onclick={() => removeCard(props.card().uuid)} disabled={props.card().uuid === ""} class={`grow-0 border border-red-500 transition-colors rounded h-6 w-6 my-auto text-center ml-2 ${props.card().uuid === "" ? "disabled:opacity-75 disabled:bg-stone-500" : "hover:bg-red-500"}`}>
                    {props.isLoading() ? "⌛" : "🗑️"}
                </button>
            </div>
        </div>
    )
}