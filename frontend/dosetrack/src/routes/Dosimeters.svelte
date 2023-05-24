<script>
  import Section from "../components/Section.svelte";
  import TableDosimeters from "../components/TableDosimeters.svelte";
  import TableFilms from "../components/TableFilms.svelte";
  import DosimeterModal from "../components/DosimeterModal.svelte";
  import API_URL from "../settings";
  import { UserStore } from "../store";
  import FilmModal from "../components/FilmModal.svelte";
  import TablePeriods from "../components/TablePeriods.svelte";
  import PeriodModal from "../components/PeriodModal.svelte";

  let dosetrack_user = {};
  let auth0_user = {};
  auth0_user = {};
  UserStore.subscribe((data) => {
    dosetrack_user = data.Dosetrack;
    auth0_user = data.Auth0;
  });
  let company_id = dosetrack_user.company_id.$oid;

  let list = [];
  let dosimeter_modal;
  let film_modal;
  let period_modal;

  let promise = fetchDosimeters();

  async function fetchDosimeters() {
    const myHeaders = new Headers({
      "Content-Type": "application/json",
    });

    const fetchConfig = {
      method: "GET",
      headers: myHeaders,
      cache: "no-cache",
    };

    const resDosimeters = await fetch(
      `${API_URL}/dosimeters/${company_id}`,
      fetchConfig
    );
    const resFilms = await fetch(
      `${API_URL}/film/by_company/${company_id}`,
      fetchConfig
    );
    const resPeriods = await fetch(
      `${API_URL}/period/by_company/${company_id}`,
      fetchConfig
    );

    if (resDosimeters.ok && resFilms.ok && resPeriods.ok) {
      let dosimeters = await resDosimeters.json();
      let films = await resFilms.json();
      let periods = await resPeriods.json();
      list = { dosimeters, films, periods };
      return list;
    } else {
      throw new Error("No se pudo obtener la lista de dosimetros");
    }
  }

  const newDosimeter = () => {
    // errors = {};
    document.getElementById("modal-delete").style.display = "none";
    document.getElementById("modal-save").style.display = "inline-block";
    document.getElementById("dosimeterModalForm").innerHTML = "Nuevo Dosímetro";
    document.getElementById("formDosimeter").reset();
    document.getElementById("company_id").value = company_id;
    dosimeter_modal.show();
  };

  const newFilm = () => {
    // errors = {};
    film_modal.reset();
    film_modal.show();
  };

  const newPeriod = () => {
    period_modal.reset();
    period_modal.show();
  };

  const removeDosimeter = (event) => {
    errors = {};
    let dosimeter = event.detail.dosimeter;

    document.getElementById("modal-delete").style.display = "inline-block";
    document.getElementById("modal-save").style.display = "none";

    document.getElementById("dosimeterModalForm").innerHTML =
      "Delete dosimeter";

    document.getElementById("_id").value = dosimeter._id.$oid;
    document.getElementById("brand").value = dosimeter.brand;
    document.getElementById("model").value = dosimeter.model;
    document.getElementById("sn").value = dosimeter.sn;
    document.getElementById("last_calibration_date").value = new Date(
      dosimeter.last_calibration_date
    )
      .toISOString()
      .substring(0, 10);
    document.getElementById("status").value = dosimeter.status;

    dosimeter_modal.show();
  };

  const editDosimeter = (event) => {
    let dosimeter = event.detail.dosimeter;

    document.getElementById("modal-delete").style.display = "none";
    document.getElementById("modal-save").style.display = "inline-block";

    document.getElementById("dosimeterModalForm").innerHTML =
      "Modificar Dosímetro";

    document.getElementById("_id").value = dosimeter._id.$oid;
    document.getElementById("brand").value = dosimeter.brand;
    document.getElementById("model").value = dosimeter.model;
    document.getElementById("sn").value = dosimeter.sn;
    document.getElementById("last_calibration_date").value = new Date(
      dosimeter.last_calibration_date
    )
      .toISOString()
      .substring(0, 10);
    document.getElementById("status").value = dosimeter.status;

    dosimeter_modal.show();
  };

  const editPeriod = (event) => {
    let period = event.detail.period;
    period_modal.reset();
    document.getElementById("periodModalForm").innerHTML = "Modificar Período";
    document.getElementById("period_id").value = period._id.$oid;
    document.getElementById("period").value = period.period;
    document.getElementById("period_from").value = new Date(
      parseInt(period.start_date.$date.$numberLong)
    )
      .toISOString()
      .substring(0, 10);
    document.getElementById("period_to").value = new Date(
      parseInt(period.end_date.$date.$numberLong)
    )
      .toISOString()
      .substring(0, 10);
    period_modal.show();
  };

  const removePeriod = () => {};

  const removeFilm = () => {};

  const editFilm = (event) => {
    let film = event.detail.film;

    film_modal.set("Modificar Film", film);
    film_modal.show();
  };

  const updateDosimetersList = (dosimeter) => {
    console.log(dosimeter);
    promise = fetchDosimeters();
  };
</script>

<Section title="Dosímetros">
  {#await promise}
    Esperando...
  {:then lista}
    <TableDosimeters
      on:edit={editDosimeter}
      on:remove={removeDosimeter}
      content={lista.dosimeters}
    />
  {:catch error}
    {error.message}
    Server offline
  {/await}
  <!-- Button trigger modal -->
  <button
    type="button"
    class="btn btn-success"
    data-bs-toggle="modal_"
    data-bs-target="#dosimeterModal"
    on:click={newDosimeter}
  >
    Nuevo dosímetro
  </button>

  <DosimeterModal
    on:updated={updateDosimetersList}
    {company_id}
    bind:this={dosimeter_modal}
  />
</Section>

<Section title="Períodos">
  {#await promise}
    Esperando...
  {:then lista}
    <TablePeriods
      on:edit={editPeriod}
      on:remove={removePeriod}
      content={lista.periods}
    />
  {:catch error}
    {error.message}
    Server offline
  {/await}

  <button
    type="button"
    class="btn btn-success"
    data-bs-toggle="modal_"
    data-bs-target="#dosimeterModal"
    on:click={newPeriod}
  >
    Nuevo Período
  </button>
  <PeriodModal
    on:updated={updateDosimetersList}
    {company_id}
    bind:this={period_modal}
  />
</Section>

<Section title="Films">
  {#await promise}
    Esperando...
  {:then lista}
    <TableFilms
      on:edit={editFilm}
      on:remove={removeFilm}
      content={lista.films}
    />
  {:catch error}
    {error.message}
    Server offline
  {/await}
  <!-- Button trigger modal -->
  <button
    type="button"
    class="btn btn-success"
    data-bs-toggle="modal_"
    data-bs-target="#dosimeterModal"
    on:click={newFilm}
  >
    Nuevo film
  </button>

  <FilmModal
    on:updated={updateDosimetersList}
    {company_id}
    bind:this={film_modal}
  />
</Section>

<style>
</style>
