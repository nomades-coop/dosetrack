<svelte:options accessors={true} />

<script>
  import { onMount } from "svelte";

  export let id = "film_id";
  export let name = "film_id";
  export let films = [];
  export let selected = null;
  export let minimal = false;
  export let custom_class = "";

  let hide;
  if (minimal) hide = "hide";

  function onchange(e) {
    selected = films.find(
      (op) => op._id.$oid === e.target.selectedOptions[0].value
    );
  }
</script>

<label for="staus" class="form-label {hide}">Film</label>
<select
  {id}
  {name}
  class="form-select form-select-lg {custom_class}"
  aria-label="Selector de operadores"
  on:change={onchange}
>
  <option value="">Seleccione un film</option>
  {#each films as film}
    <option
      value={film._id.$oid}
      selected={film._id.$oid === selected ? "selected" : ""}
      >{film.company_code}</option
    >
  {/each}
</select>
<div id="operatorHelp" class="form-text {hide}">Seleccione un film</div>

<style>
  .hide {
    visibility: hidden;
    display: none;
  }
</style>
