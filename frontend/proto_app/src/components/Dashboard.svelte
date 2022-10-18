<script>
  import Section from "./Section.svelte";
  import Chart from "./Chart.svelte";
  import TableOperators from "./TableOperators.svelte";

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
  <Chart />
</Section>

<Section title="Operadores con mayor dosis">
  <TableOperators {data} />
</Section>
