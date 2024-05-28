import { Accessor, Setter, createSignal } from "solid-js";
import { CardProps } from "~/routes";

export default function Adder(props: { cards: Accessor<CardProps[]>, setCards: Setter<CardProps[]> }) {

    let url_input: HTMLInputElement;
    let title_input: HTMLInputElement;
    let description_input: HTMLInputElement;

    const [isAdding, setIsAdding] = createSignal(false);

    const fetchServer = async (url: string, title: string, description: string) => {
        "use server"
        let response = await fetch(`http://${process.env.BACK_HOST}:3001/api/card`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                uuid: "", // the backend will take care of this
                url,
                title,
                description
            })
        });
        return await response.text();
    }

    const postCard = async (url: string, title: string, description: string) => {
        // Optimistically update the cards

        props.setCards([...props.cards(), {
            uuid: "", // the backend will take care of this
            url,
            title,
            description
        }]);

        setIsAdding(false);

        // Update the server
        let uuid = await fetchServer(url, title, description);

        console.log(uuid)

        props.setCards(
            props.cards().map(card => {
                if (card.uuid === "") {
                    return { ...card, uuid };
                }
                return card;
            })
        )

        console.log(props.cards())
    }

    const queryAndSend = async () => {
        let url = url_input.value;
        let title = title_input.value;
        let description = description_input.value;

        if (url === "" || title === "" || description === "") {
            alert("Por favor, llena todos los campos");
            return;
        }

        postCard(url, title, description);
    }




    return (
        <>
            <div class="absolute right-5 bottom-5 z-30">
                <button onclick={() => setIsAdding(!isAdding())} class="h-10 w-10 rounded-full bg-black shadow text-center my-auto text-white text-xl font-black hover:scale-105 transition-transform">

                </button>
            </div>
            <div class={`absolute left-0 top-0 w-screen h-screen bg-black bg-opacity-50 ${isAdding() ? "" : "hidden"} z-20`}>

            </div>
            {isAdding() && (
                <div class="fixed inset-0 flex items-center justify-center z-30">
                    <div class="bg-white p-6 rounded shadow-lg grid grid-cols-2 gap-2">
                        {/* Aquí puedes poner los campos de entrada de datos */}
                        {/* @ts-ignore */}
                        <input ref={url_input} type="text" placeholder="URL" class="border p-2 mb-4 w-full col-span-2" />
                        {/* @ts-ignore */}
                        <input ref={title_input} type="text" placeholder="Titulo" class="border p-2 mb-4 w-full" />
                        {/* @ts-ignore */}
                        <input ref={description_input} type="text" placeholder="Desc" class="border p-2 mb-4 w-full" />
                        <button onClick={() => setIsAdding(false)} class="bg-black text-white px-4 py-2 rounded drop-shadow">
                            Cerrar
                        </button>
                        <button onClick={() => queryAndSend()} class="bg-white text-black px-4 py-2 rounded drop-shadow border border-black">
                            Guardar
                        </button>
                    </div>
                </div>
            )}
        </>
    )
}