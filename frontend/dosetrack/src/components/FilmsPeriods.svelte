<script>
  import { afterUpdate, onMount } from "svelte";
  import FilmSelector from "../components/FilmSelector.svelte";
  import IMask from "imask"; //from https://imask.js.org/
  import FaSave from "svelte-icons-pack/fa/FaSave";
  import Icon from "svelte-icons-pack";
  import { getFilmsByCompany, saveFilmDose } from "../services/films";
  import { UserStore } from "../store";
  import API_URL from "../settings";
  import PeriodInput from "./PeriodInput.svelte";
  import { periods_by_company } from "../services/periods";
  import { toast } from "@zerodevx/svelte-toast";
  import ImCheckmark from "svelte-icons-pack/im/ImCheckmark";
  import ImCross from "svelte-icons-pack/im/ImCross";

  export let company_id = null;
  export let period = null;
  export let operators = [];

  let films = [];
  let masked_control = [];
  let months = Array(12)
    .fill()
    .map((_, idx) => 1 + idx);

  onMount(async () => {
    console.log("Mounted FilmsPeriods");
  });

  films = async () => {
    return await getFilmsByCompany(company_id, period.period);
  };

  afterUpdate(async () => {
    let numbers = [...document.getElementsByClassName("number")];
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
  async function savePeriodDoses(period_id = null) {
    let show_toast_error = false;
    let dose_cards = document.querySelectorAll(
      `.period[period_id="${period_id}"] .film-dose`
    );

    dose_cards.forEach((card) => (card.style = "background-color:#6084b9"));

    // convierto la nodelist a array
    // por cada elemento selecciono los valores de dosis y film
    let film_doses = Array.prototype.slice.call(dose_cards).map((d) => {
      return {
        company_id: company_id,
        period_id: period_id,
        operator_id: d.attributes.operator_id.value,
        film_id: d.querySelector("#operator_film").value,
        dose: parseInt(d.querySelector("#operator_dose").value),
      };
    });

    film_doses.forEach(async (film_dose) => {
      let result = await saveFilmDose(film_dose);

      if (result.error) {
        toast.push("Corrija las tarjetas con error");
        let error_dose_cards = document.querySelectorAll(
          `.film-dose[period_id="${period_id}"][operator_id="${film_dose.operator_id}"]`
        );
        error_dose_cards.forEach(
          (card) => (card.style = "background-color:#ff5656")
        );
      }
    });
  }
</script>

<div class="period" period_id={period._id.$oid}>
  <h2>
    Período {period.period.slice(0, 4)}-{period.period.slice(-2)}
    <div class="btn-container">
      <button
        on:click={savePeriodDoses(period._id.$oid)}
        class="btn btn-success btn-lg"
        ><Icon src={ImCheckmark} color="red" /></button
      >
      <!-- <button class="btn btn-danger btn"
        ><Icon src={ImCross} color="gray" /></button
      > -->
    </div>
  </h2>

  <div class="mb-3 mt-3" />

  {#await films()}
    <h1>espera</h1>
  {:then films}
    <div class="flex-container longhand">
      {#each operators as operator (operator._id.$oid)}
        <div
          class="film-dose"
          operator_id={operator._id.$oid}
          period_id={period._id.$oid}
        >
          <h3>{operator.name}</h3>
          <h5>DNI {operator.dni}</h5>

          <div class="input-group mb-3">
            <span class="input-group-text" id="basic-addon1">🪪</span>
            <FilmSelector
              id="operator_film"
              minimal={true}
              custom_class="bolder-film-selector"
              {films}
            />
          </div>

          <div class="input-group mb-3">
            <span class="input-group-text" id="basic-addon1">☢️</span>
            <input
              id="operator_dose"
              type="number"
              value=""
              class="form-control dosis"
              placeholder="Dosis en mSv"
              aria-label="Username"
              aria-describedby="basic-addon1"
            />
          </div>
        </div>
      {/each}
    </div>
  {/await}
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

  .film-dose {
    background: rgb(96, 132, 185);
    padding: 13px;
    border-radius: 9px;
    margin: 10px;
    max-width: 18em;
    /* line-height: 100px; */
    color: white;
    font-weight: bold;
    font-size: 1.5em;
    text-align: center;
  }

  .btn-container {
    float: right;
  }

  .dosis {
    font-size: 20pt;
    text-align: center;
    font-weight: bolder;
    padding: 0;
    color: #5a5353;
  }

  .period {
    margin-top: 15px;
  }
</style>
