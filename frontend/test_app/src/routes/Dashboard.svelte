<script>
  import { onMount } from "svelte";
  import Section from "../components/Section.svelte";
  import TableOperators from "../components/TableOperators.svelte";
  import API_URL from "../settings";

  let tableData;
  let company_id = "abca535261245abfcd4da31a";

  let promise = fetchOperators();

  onMount(async () => {});

  const content = (list) => {
    return {
      headers: [
        { _id: { title: "", type: "_id" } },
        { company_id: { title: "", type: "_id" } },
        { name: { title: "Nombre", type: "str" } },
        { dni: { title: "DNI", type: "str" } },
        { licence: { title: "Licencia", type: "str" } },
        { dosimeter_id: { title: "Dosímetro", type: "_id" } },
        { status: { title: "Status", type: "str" } },
      ],
      rows: tableData,
    };
  };

  async function fetchOperators() {
    const myHeaders = new Headers({
      "Content-Type": "application/json",
    });

    const fetchConfig = {
      method: "GET",
      headers: myHeaders,
      cache: "no-cache",
    };

    const res = await fetch(`${API_URL}/operators/${company_id}`);
    tableData = await res.json();

    if (res.ok) {
      return tableData;
    } else {
      throw new Error("No se pudo obtener la lista de dosimetros");
    }
  }
</script>

<Section title="Operadores en situación crítica" showToolbar="">...</Section>

<Section title="Radiación absorbida total desde el 1.ero de Enero">
  {#await promise}
    Esperando...
  {:then lista}
    <TableOperators content={content(lista)} />
  {:catch error}
    {error.message}
    no andó
  {/await}
</Section>
