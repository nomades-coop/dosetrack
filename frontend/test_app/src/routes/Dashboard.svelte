<script>
  import { onMount } from "svelte";
  import Section from "../components/Section.svelte";
  import { Chart, registerables } from "chart.js/dist/chart.esm";
  import TableOperators from "../components/TableOperators.svelte";
  import operators_data from "../operators";

  const data = operators_data;

  onMount(async () => {
    Chart.register(...registerables);

    console.log(data);

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

<Section title="Operadores con mayor dosis">
  <TableOperators {data} />
</Section>
