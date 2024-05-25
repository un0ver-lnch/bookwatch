import { For, createSignal, onMount } from "solid-js";
import Card from "~/components/card";

type CardProps = {
  url: string;
  title: string;
  description: string;
};

export default function Home() {

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
  })


  return (
    <main class="mx-auto my-5 grid grid-cols-1 container w-11/12 gap-4">
      <For each={cards()}>
        {
          (card) => (
            <Card isLoading={isLoading} URL={card.url} title={card.title} description={card.description} />
          )
        }
      </For>
    </main>
  );
}
