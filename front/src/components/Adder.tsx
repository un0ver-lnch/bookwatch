import { Accessor, Setter, createSignal } from "solid-js";
import { CardProps } from "~/routes";

export default function Adder(props: { cards: Accessor<CardProps[]>, setCards: Setter<CardProps[]> }) {

    const [isAdding, setIsAdding] = createSignal(false);

    const fetchServer = async (url: string, title: string, description: string) => {
        "use server"
        await fetch("http://localhost:3001/api/cards", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                url,
                title,
                description
            })
        });
    }

    const postCard = async (url: string, title: string, description: string) => {
        // Optimistically update the cards

        props.setCards([...props.cards(), {
            url,
            title,
            description
        }]);

        setIsAdding(false);

        // Update the server
        await fetchServer(url, title, description);
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
                        <input type="text" placeholder="URL" class="border p-2 mb-4 w-full col-span-2" />
                        <input type="text" placeholder="Titulo" class="border p-2 mb-4 w-full" />
                        <input type="text" placeholder="Desc" class="border p-2 mb-4 w-full" />
                        <button onClick={() => setIsAdding(false)} class="bg-black text-white px-4 py-2 rounded drop-shadow">
                            Cerrar
                        </button>
                        <button onClick={() => postCard("", "", "")} class="bg-white text-black px-4 py-2 rounded drop-shadow border border-black">
                            Guardar
                        </button>
                    </div>
                </div>
            )}
        </>
    )
}