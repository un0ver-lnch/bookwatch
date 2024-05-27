import { createSignal, onMount, onCleanup, Index } from "solid-js";
import Adder from "~/components/Adder";
import Card from "~/components/Card";

type CardProps = {
  uuid: string;
  url: string;
  title: string;
  description: string;
};

export default function Home() {

  const [fetcher, setFetcher] = createSignal<NodeJS.Timeout>();

  const [cards, setCards] = createSignal<CardProps[]>(Array(3).fill({
    url: "",
    title: "",
    description: ""
  }));

  const [isLoading, setIsLoading] = createSignal(true);

  const fetchCards = async () => {
    "use server"
    const response = await fetch("http://localhost:3001/api/cards");
    const data = await response.json();
    return data;
  }

  onMount(async () => {
    const data = await fetchCards();
    setCards(data);
    setIsLoading(false);
    let interval = setInterval(async () => {
      const data = await fetchCards();
      if (data.length !== cards().length) {
        setCards(data);
        setIsLoading(false);
      };
    }, 1000);
    setFetcher(interval)
  })

  onCleanup(() => {
    clearInterval(fetcher());
  })


  return (
    <main class="mx-auto my-5 grid grid-cols-1 container w-11/12 gap-4">
      <Index each={cards()} fallback={<div class="text-center text-3xl">Nothing here...</div>}>
        {
          (card) =>
            <Card isLoading={isLoading} card={card} cards={cards} setCards={setCards} />
        }
      </Index>
      <Adder cards={cards} setCards={setCards} />
    </main>
  );
}


export type {
  CardProps
}