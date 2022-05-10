<script>
  import { onMount } from "svelte";
  import Section from "../components/Section.svelte";
  import { Chart, registerables } from "chart.js/dist/chart.esm";
  import TableOperators from "../components/TableOperators.svelte";
  import API_URL from "../settings";

  let tableData;
  let company_id = "abca535261245abfcd4da31a";

  let promise = fetchOperators();

  onMount(async () => {
    Chart.register(...registerables);

    const ctx = await document.getElementById("myChart").getContext("2d");
    const myChart = new Chart(ctx, {
      type: "line",
      data: {
        labels: [...Array(32).keys()],
        datasets: [
          {
            label: "None",
            data: [52, 39, 23, 25, 22, 33],
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
  });

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

<Section
  title="Dashboard!"
  subtitle="Trend de radiación absorbida total"
  note="Ultima actualización: 18/08/2022"
  showToolbar=""
>
  <div class="row">
    <canvas id="myChart" class="ratio ratio-16x9 img-fluid" />
  </div>
</Section>

<Section title="Operadores">
  {#await promise}
    Esperando...
  {:then lista}
    <TableOperators content={content(lista)} />
  {:catch error}
    {error.message}
    no andó
  {/await}
</Section>
