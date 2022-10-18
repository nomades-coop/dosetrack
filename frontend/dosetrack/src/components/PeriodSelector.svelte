<svelte:options accessors={true}/>
<script>
  export let id="period_id";
  export let name="period_id";
  export let periods=[];
  export let selected=null;


  function onchange(e) {
    selected = periods.find( (op) => op._id.$oid === e.target.selectedOptions[0].value );
  }
  
</script>

<label for="staus" class="form-label">Período</label>
<select
  id="{id}"
  name="{name}"
  class="form-select"
  aria-label="Selector de períodos"
  on:change="{onchange}"
>
<option value="" selected>Seleccione un período</option>
  {#each periods as period} 
    <option value="{period._id.$oid}">{period.period}</option>
  {/each}
</select>
<div id="periodHelp" class="form-text">
  Seleccione el período
</div>