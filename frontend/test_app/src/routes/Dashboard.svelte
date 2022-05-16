<script>
  import { onMount } from "svelte";
  import Section from "../components/Section.svelte";
  import TableOperators from "../components/TableOperators.svelte";
  import TableOperatorsDasboard from "../components/TableOperatorsDasboard.svelte";
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
        { dosimeter_id: { title: "Dosímetro", type: "_id" } },
        { Dosis: { title: "Dosis Acumulada", type: "dosis", align: "center" } },
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

<Section title="Operadores en situación crítica" showToolbar="">
  <div class="alert alert-danger d-flex justify-content-between" role="alert">
    <span class="dosis-op">Centurión, Pablo</span>
    <span class="dosis">456 <span style="font-size: 12pt;">mSv</span> </span>
  </div>
</Section>

<Section title="Radiación acumulada anual">
  {#await promise}
    Esperando...
  {:then lista}
    <TableOperatorsDasboard content={content(lista)} />
  {:catch error}
    {error.message}
    no andó
  {/await}
</Section>
<button class="btn btn-primary">Decargar CSV</button>

<style>
  .dosis {
    float: right;
    font-size: 18pt;
    font-weight: 700;
  }
  .dosis-op {
    font-size: 18pt;
    font-weight: 700;
  }
</style>
