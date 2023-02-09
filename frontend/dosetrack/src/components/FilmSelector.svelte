<svelte:options accessors={true}/>
<script>
  import { onMount } from "svelte";

  export let id="operator_id";
  export let name="operator_id";
  export let films=[];
  export let selected=null;
  export let minimal=false;
  
  let hide;
  if (minimal) hide = 'hide';

  function onchange(e) {
    selected = films.find( (op) => op._id.$oid === e.target.selectedOptions[0].value );
  }
  
</script>

<label for="staus" class="form-label {hide}">Film</label>
<select
  id="{id}"
  name="{name}"
  class="form-select"
  aria-label="Selector de operadores"
  on:change="{onchange}"
>
<option value="" selected>Seleccione un film</option>
  {#each films as film} 
    <option value="{film._id.$oid}">{film.company_code} {film.period}</option>
  {/each}
</select>
<div id="operatorHelp" class="form-text {hide}">
  Seleccione un film
</div>

<style>
.hide {
  visibility: hidden;
  display: none;
}
</style>