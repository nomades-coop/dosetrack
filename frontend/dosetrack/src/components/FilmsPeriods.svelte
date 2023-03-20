<script>
  import { afterUpdate, onMount } from "svelte";
  import FilmSelector from "../components/FilmSelector.svelte";
  import IMask from "imask"; //from https://imask.js.org/
  import FaSave from "svelte-icons-pack/fa/FaSave";
  import Icon from "svelte-icons-pack";
  import { getFilmsByCompany } from "../services/films";
  import { UserStore } from "../store";
  import API_URL from "../settings";
  import PeriodInput from "./PeriodInput.svelte";

  export let company = null;
  export let year = new Date().getFullYear();
  export let operators = [];

  let masked_control = [];
  let months = Array(12)
    .fill()
    .map((_, idx) => 1 + idx);

  onMount(async () => {
    console.log("Mounted FilmsPeriods");
    console.log({ operators });
  });

  afterUpdate(async () => {
    console.log("updated...");
    console.log("Doc", document.getElementById("62eeb7ec0c11652e03b5e6b7"));
    let numbers = [...document.getElementsByClassName("number")];
    console.log({ numbers });

    numbers.forEach((x) => {
      masked_control.push(
        IMask(x, {
          mask: Number,
          min: 0.0,
          max: 9.99,
          scale: 2,
          thousandsSeparator: " ",
          padFractionalZeros: true,
          thousandsSeparator: "",
          normalizeZeros: true,
          radix: ".",
          mapToRadix: [","],
          overwrite: true,
        })
      );
    });
  });

  // send table row as json to server
  async function saveFilm(film) {
    console.log(film);
    let response = await fetch(`${API_URL}/film_dose`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(film),
    });
    let result = await response.json();
    console.log(result);
  }
</script>

<h2>
  Período 2023:01
  <div class="btn-container">
    <button class="btn btn-success btn-sm">✔️</button>
    <button class="btn btn-danger btn-sm">✖️</button>
  </div>
</h2>

<div class="row">
  <div class="col">
    Desde
    <input
      type="date"
      class="form-control"
      id="fecha_desde"
      name="fecha_desde"
    />
  </div>
  <div class="col">
    Hasta
    <input
      type="date"
      class="form-control"
      id="fecha_hasta"
      name="fecha_hasta"
    />
  </div>
</div>
<div class="mb-3 mt-3" />

<div class="flex-container longhand">
  <div class="flex-item">
    <h3>Juan C.</h3>
    <h5>DNI 22999777</h5>

    <div class="input-group mb-3">
      <span class="input-group-text" id="basic-addon1">🪪</span>
      <select class="form-select" aria-label="Default select example">
        <option selected>Open this select menu</option>
        <option value="1">One</option>
        <option value="2">Two</option>
        <option value="3">Three</option>
      </select>
    </div>

    <div class="input-group mb-3">
      <span class="input-group-text" id="basic-addon1">🪪</span>
      <input
        type="text"
        class="form-control"
        placeholder="Código Film"
        aria-label="Username"
        aria-describedby="basic-addon1"
      />
    </div>

    <div class="input-group mb-3">
      <span class="input-group-text" id="basic-addon1">☢️</span>
      <input
        type="text"
        class="form-control"
        placeholder="Dosis en mSrv"
        aria-label="Username"
        aria-describedby="basic-addon1"
      />
    </div>
  </div>

  <div class="flex-item">2</div>
  <div class="flex-item">3</div>
  <div class="flex-item">4</div>
</div>

<hr />
<div class="text-center">
  <button class="btn btn-primary">Nuevo Período</button>
</div>

<h1 class="pepe" id="valores">{year}</h1>

<div class="table-responsive">
  <table class="table table-striped table-hover table-sm">
    <thead>
      <tr>
        <th style="display:none;" />
        <th>Operador</th>
        <th>FILM</th>

        {#each months as month}
          <th style="text-align:center">{month}</th>
        {/each}
        <th>Acumulado</th>
      </tr>
    </thead>
    <tbody>
      {#each operators as operator (operator._id.$oid)}
        <tr id={operator._id.$oid}>
          <td
            ><h3><span class="badge bg-primary">{operator.name}</span></h3>
          </td>
          <!-- <td style="min-width:150px"><FilmSelector {films} minimal={true}/></td> -->

          <td style="min-width:150px"
            ><input
              id="{company}{operator._id.$oid}"
              type="text"
              class="form-control"
              value="AKM997"
            /></td
          >
          <td style="min-width:5em">
            <input
              id="coso"
              type="text"
              class="form-control number"
              value="0.00"
            />
          </td>
          <td style="min-width:5em">
            <input type="text" class="form-control number" value="0.00" />
          </td>
          <td style="min-width:5em">
            <input type="text" class="form-control number" value="0.00" />
          </td>
          <td style="min-width:5em">
            <input type="text" class="form-control number" value="0.00" />
          </td>
          <td style="min-width:5em">
            <input type="text" class="form-control number" value="0.00" />
          </td>
          <td style="min-width:5em">
            <input type="text" class="form-control number" value="0.00" />
          </td>
          <td style="min-width:5em">
            <input type="text" class="form-control number" value="0.00" />
          </td>
          <td style="min-width:5em">
            <input type="text" class="form-control number" value="0.00" />
          </td>
          <td style="min-width:5em">
            <input type="text" class="form-control number" value="0.00" />
          </td>
          <td style="min-width:5em">
            <input type="text" class="form-control number" value="0.00" />
          </td>
          <td style="min-width:5em">
            <input type="text" class="form-control number" value="0.00" />
          </td>
          <td style="min-width:5em">
            <input type="text" class="form-control number" value="0.00" />
          </td>
          <td style="min-width:5em">
            <input type="text" class="form-control number" value="0.00" />
          </td>
          <td>
            <button class="btn btn-success icon" on:click={saveFilm}>
              <Icon size="20px" src={FaSave} />
            </button>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  .flex-container {
    padding: 0;
    margin: 0;
    list-style: none;

    -ms-box-orient: horizontal;
    display: -webkit-box;
    display: -moz-box;
    display: -ms-flexbox;
    display: -moz-flex;
    display: -webkit-flex;
    display: flex;
  }

  h1 {
    padding-left: 0.5em;
  }

  .longhand {
    -webkit-flex-flow: wrap row;
    flex-flow: wrap row;
  }

  .flex-item {
    background: rgb(96, 132, 185);
    padding: 5px;
    border-radius: 9px;
    margin: 10px;

    /* line-height: 100px; */
    color: white;
    font-weight: bold;
    font-size: 1.5em;
    text-align: center;
  }

  .number {
    text-align: right;
    min-width: 5em;
  }

  .icon {
    padding-top: 0.2em;
    padding-bottom: 0.5em;
  }

  .btn-container {
    float: right;
  }
</style>
