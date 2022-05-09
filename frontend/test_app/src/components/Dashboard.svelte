<script>
  import { onMount } from "svelte";
  import Section from "./Section.svelte";
  import { Chart, registerables } from "chart.js/dist/chart.esm";
  import TableOperators from "./TableOperators.svelte";

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

  let person1 = {
    picture: "img/foto.jpg",
    name: "Pablo C",
    dose: 0.54,
  };

  let person2 = {
    picture: "img/algo.jpg",
    name: "Santiago S",
    dose: 0.23,
  };

  let data = {
    headers: [{ title: "" }, { title: "Name" }, { title: "Dose" }],
    rows: [person1, person2],
  };
</script>

<Section
  title="Dashboard!"
  subtitle="Trend de radiación absorbida total"
  note="Ultima actualización: 18/08/2022"
  showToolbar=""
>
  <canvas id="myChart" width="300" height="60%" />
</Section>

<Section title="Operadores con mayor dosis">
  <TableOperators {data} />
</Section>
