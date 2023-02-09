<script>
  import { createEventDispatcher } from 'svelte';
  import { onMount } from 'svelte';
  import IMask from "imask"; //from https://imask.js.org/

  const dispatch = createEventDispatcher();

  export let editable = false
  export let col = {}
  
  let value = col['value'] || {value: ''}
  let input = col['input'] || {type: ''}
  let readonly = col['readonly'] || false

  let editableClass = []
  let masked_control = []

  editableClass[true] = "EditableOn"
  editableClass[false] = "EditableOff"


  onMount(async () => {

    [...document.getElementsByClassName('number')].forEach( (x)=> {
      masked_control.push( IMask(
      x,
      {
        mask: Number,
        min: 0.0,
        max: 9.99,
        scale: 2,
        thousandsSeparator: ' ',
        padFractionalZeros: true,
      }))
      
    })


  })

  const on_change = (e) => {

    value = e.target.value

    dispatch('columnChanged', value)

  }

</script>

  <td class="{editableClass[editable]}">
    {#if input['type']==='number'}
      <input id="" type="text" value="{value}" class="inputs number" on:change={on_change} {readonly}>

    {:else if input['type']==='select' }
      <select name="" id="" class="input-select" on:change={on_change}>
        {#each input['options'] as option}
            <option value={option['value']}>{option['text']}</option>
        {/each}
      </select>

    {:else}
      <input id="" type="text" value="{value}" class="inputs" on:change={on_change}>
    {/if}
  </td>

  <td class="{editableClass[!editable]}">
    {#if input['type']==='number'}
      {value}
    {:else if input['type']==='select' }
      {input['options'].find(o => o['value'] == value)['text'] }
    {:else}
      {value}
    {/if}
  </td>


<style>
  .EditableOn {
    display: table-cell;
  }

  .EditableOff {
    display: none;
  }

  .input-select {
    height: 22px;
    border-radius: 7px;
    border: 1px solid #bdbdbd;
  }

  .inputs:not(select) {
    width: 45px;
    height: 22px;
    border-radius: 7px;
    border: 1px solid #bdbdbd;
  }

  .number {
    text-align: right;
  }
</style>

