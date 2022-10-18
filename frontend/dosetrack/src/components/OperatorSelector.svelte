<svelte:options accessors={true}/>
<script>
  export let id="operator_id";
  export let name="operator_id";
  export let operators=[];
  export let selected=null;


  function onchange(e) {
    selected = operators.find( (op) => op._id.$oid === e.target.selectedOptions[0].value );
  }
  
</script>

<label for="staus" class="form-label">Operador</label>
<select
  id="{id}"
  name="{name}"
  class="form-select"
  aria-label="Selector de operadores"
  on:change="{onchange}"
>
<option value="" selected>Seleccione un operador</option>
  {#each operators as operator} 
    <option value="{operator._id.$oid}">{operator.name}</option>
  {/each}
</select>
<div id="operatorHelp" class="form-text">
  Seleccione el operador asignado al film
</div>