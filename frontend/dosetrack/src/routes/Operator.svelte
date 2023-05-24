<script>
  import { onMount } from "svelte";
  import Section from "../components/Section.svelte";
  import { Chart, registerables } from "chart.js/dist/chart.esm";
  import TableOperators from "../components/TableOperators.svelte";
  import API_URL from "../settings";
  import TableDoses from "../components/TableDoses.svelte";
  import moment from "moment";
  export let params = {};

  let tableData;
  let operator;
  let promise = fetchDoses();

  onMount(async () => {
    Chart.register(...registerables);

    console.log("mount", tableData);
  });

  const showChart = (labels, data) => {
    const ctx = document.getElementById("myChart").getContext("2d");
    const myChart = new Chart(ctx, {
      type: "line",
      data: {
        labels: labels,
        datasets: [
          {
            data: data,
            lineTension: 0,
            backgroundColor: "transparent",
            borderColor: "#007bff",
            borderWidth: 3,
            pointBackgroundColor: "#007bff",
          },
        ],
      },
      options: {
        responsive: true,
        plugins: {
          legend: {
            position: "none",
          },
          title: {
            display: false,
            text: "Chart.js Line Chart",
          },
        },
      },
    });
  };

  const content = (list) => {
    return {
      headers: [
        { operator_id: { title: "", type: "_id" } },
        { dosimeter_id: { title: "", type: "_id" } },
        { when: { title: "Fecha", type: "date" } },
        { dosimeter: { title: "Dosímetro", type: "obj", field: "brand" } },

        { dose: { title: "Dosis", type: "str", align: "center" } },
      ],
      rows: tableData,
    };
  };

  async function fetchDoses() {
    console.log("params", params);

    const myHeaders = new Headers({
      "Content-Type": "application/json",
    });

    const fetchConfig = {
      method: "GET",
      headers: myHeaders,
      cache: "no-cache",
    };

    const res = await fetch(
      `${API_URL}/dose/${params.company_id}/${params.operator_id}`
    );
    tableData = await res.json();

    let labels = tableData.map((d) =>
      moment(parseInt(d.when.$date.$numberLong)).format("DD-MM-YYYY")
    );
    let data = tableData.map((d) => d.dose);
    showChart(labels, data);

    if (res.ok) {
      if (tableData.length > 0) {
        operator = tableData[0].operator;
      } else {
        operator = { name: "No hay dosis para el operador" };
      }
      console.log("operator", operator);
      return tableData;
    } else {
      throw new Error("No se pudo obtener la lista de dosimetros");
    }
  }
</script>

{#await promise}
  Esperando...
{:then _}
  <Section
    title={operator.name}
    subtitle="Trend de radiación absorbida total"
    note="Ultima actualización: 18/08/2022"
    showToolbar=""
    slot="hidden"
  />
{/await}

<div class="row m-1 m-md-5">
  <canvas id="myChart" class="ratio ratio-16x9 img-fluid" />
</div>

<Section title="Dosis registradas">
  {#await promise}
    Esperando...
  {:then lista}
    <TableDoses content={content(lista)} />
  {:catch error}
    {error.message}
    Server offline
  {/await}
</Section>
