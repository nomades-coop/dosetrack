<script >
  import { afterUpdate, onMount } from "svelte";
  import FilmSelector from "../components/FilmSelector.svelte";
  import IMask from "imask"; //from https://imask.js.org/
  import FaSave from "svelte-icons-pack/fa/FaSave";
  import Icon from "svelte-icons-pack";
  import { getFilmsByCompany } from "../services/films";
  import {UserStore} from "../store";
  import API_URL from "../settings";

  export let company = null;
  export let year = new Date().getFullYear();
  export let operators = [];

  let masked_control=[];
  let months = Array(12).fill().map((_, idx) => 1 + idx);
  

  onMount(async () => {

      console.log("Mounted FilmsPeriods");
      console.log({operators});

  });


  afterUpdate(async () => {
    console.log("updated...");
    console.log('Doc',document.getElementById('62eeb7ec0c11652e03b5e6b7'));
    let numbers = [...document.getElementsByClassName('number')];
    console.log({numbers});

    numbers.forEach( x => {
      masked_control.push( IMask(
      x,
      {
        mask: Number,
        min: 0.0,
        max: 9.99,
        scale: 2,
        thousandsSeparator: ' ',
        padFractionalZeros: true,
        thousandsSeparator: '',
        normalizeZeros: true,
        radix: '.',
        mapToRadix: [','],
        overwrite: true,
      }))
      
    })

	});


  // send table row as json to server
  async function saveFilm(film) {
    console.log(film);
    let response = await fetch(`${API_URL}/film_dose`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(film)
    });
    let result = await response.json();
    console.log(result);
  }

</script>

<h1 class="pepe" id='valores'>{year}</h1>

<div class="table-responsive">
  <table class="table table-striped table-hover table-sm">
    <thead>
      <tr>
        <th style="display:none;"></th>
        <th>Operador</th>
        <th>FILM</th>

        {#each months as month }
          <th style="text-align:center">{month}</th>    
        {/each}
        <th>Acumulado</th>
      </tr>
    </thead>
    <tbody>

      {#each operators as operator (operator._id.$oid)}
      <tr id={operator._id.$oid}>
        <td><h3><span class="badge bg-primary">{operator.name}</span></h3> </td>
        <!-- <td style="min-width:150px"><FilmSelector {films} minimal={true}/></td> -->

        <td style="min-width:150px"><input id="{company}{operator._id.$oid}" type="text" class="form-control" value="AKM997"></td>
        <td style="min-width:5em"> <input id="coso" type="text" class="form-control number" value="0.00"> </td>
        <td style="min-width:5em"> <input type="text" class="form-control number" value="0.00"> </td>
        <td style="min-width:5em"> <input type="text" class="form-control number" value="0.00"> </td>
        <td style="min-width:5em"> <input type="text" class="form-control number" value="0.00"> </td>
        <td style="min-width:5em"> <input type="text" class="form-control number" value="0.00"> </td>
        <td style="min-width:5em"> <input type="text" class="form-control number" value="0.00"> </td>
        <td style="min-width:5em"> <input type="text" class="form-control number" value="0.00"> </td>
        <td style="min-width:5em"> <input type="text" class="form-control number" value="0.00"> </td>
        <td style="min-width:5em"> <input type="text" class="form-control number" value="0.00"> </td>
        <td style="min-width:5em"> <input type="text" class="form-control number" value="0.00"> </td>
        <td style="min-width:5em"> <input type="text" class="form-control number" value="0.00"> </td>
        <td style="min-width:5em"> <input type="text" class="form-control number" value="0.00"> </td>
        <td style="min-width:5em"> <input type="text" class="form-control number" value="0.00"> </td>
        <td> <button class="btn btn-success icon" on:click={saveFilm}> <Icon size="20px" src={FaSave} /> </button>  </td>
      </tr>
      {/each}

    </tbody>
  </table>
</div>

<style>
  .number {
    text-align: right;
    min-width: 5em;
  }

  .icon {
    padding-top: 0.2em;
    padding-bottom: 0.5em;
  }
</style>