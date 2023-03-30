<script>
  import PeriodDoses from "../components/PeriodDoses.svelte";

  let startDate = new Date("2021-01-28 00:00:00.000");
  let endDate = new Date("2021-03-1 00:00:00.000");

  const doses = [];
  while (startDate <= endDate) {
    doses.push({
      date: new Date(startDate),
      dose: Math.round(Math.random() * 1000) / 100,
    });
    startDate.setDate(startDate.getDate() + 1);
  }
</script>

<br />
<div class="row" style="max-width: 135px; margin-left:auto;margin-right: auto;">
  <select
    class="form-select form-select-lg mb-3"
    aria-label=".form-select-lg example"
  >
    <option selected>2023-05</option>
    <option>2023-04</option>
    <option>2023-03</option>
    <option>2023-02</option>
    <option>2023-01</option>
  </select>
</div>

<PeriodDoses title="Cosme, Fulanito" filmPeriodTotal="200" {doses} />

<PeriodDoses title="Lopez, Juan" filmPeriodTotal="200" {doses} />

<PeriodDoses title="Manso, Juana" filmPeriodTotal="200" {doses} />

<PeriodDoses title="Perez, Andres" filmPeriodTotal="200" {doses} />
