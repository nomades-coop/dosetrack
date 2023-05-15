<script>
  import { onMount } from "svelte";
  import Section from "../components/Section.svelte";
  import TableOperatorsDasboard from "../components/TableOperatorsDasboard.svelte";
  import PeriodViewer from "../components/PeriodViewer.svelte";

  import API_URL from "../settings";
  import { UserStore } from "../store";
  import { Decimal } from "decimal.js";
  import { push } from "svelte-spa-router";

  // creates an array with 1.7 decimal value 33 times
  let doses = Array.from({ length: 33 }, () => new Decimal(1.7));

  let totalDoses = doses
    .reduce((acc, dose) => new Decimal(acc).add(dose), 0)
    .toFixed();

  let dosetrack_user = {};
  let auth0_user = {};
  auth0_user = {};
  UserStore.subscribe((data) => {
    dosetrack_user = data.Dosetrack;
    auth0_user = data.Auth0;
  });

  let tableData;

  // Get company id from logged user
  let company_id = null;

  try {
    company_id = dosetrack_user.company_id.$oid; //"abca535261245abfcd4da31a";
  } catch {
    push("/dose/registration");
  }

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

<Section title="Operadores en situación crítica!" showToolbar="">
  <div class="alert alert-success d-flex justify-content-between" role="alert">
    <span class="dosis-op">Todo OK</span>
    <span class="dosis"> <span style="font-size: 12pt;" /> </span>
  </div>

  <!-- <div class="alert alert-danger d-flex justify-content-between" role="alert">
    <span class="dosis-op">Lopez, Juan</span>
    <span class="dosis">200 <span style="font-size: 12pt;" /> </span>
  </div>

  <div class="alert alert-danger d-flex justify-content-between" role="alert">
    <span class="dosis-op">Mansoi, Juana</span>
    <span class="dosis">200 <span style="font-size: 12pt;">mSv</span> </span>
  </div>

  <div class="alert alert-danger d-flex justify-content-between" role="alert">
    <span class="dosis-op">Perez, Andres</span>
    <span class="dosis">200 <span style="font-size: 12pt;">mSv</span> </span>
  </div> -->
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
