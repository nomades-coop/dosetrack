<script>
  import { debug } from "svelte/internal";


  export let columns = [{
    class: '',
    style: '',
    value: 'value',
    input: {
      type: 'input',
      option_list: []
    }
  }]

  let edit_mode = true
  let edit_row = 3

  const editRow = (e) => {
    let tr = e.target.closest('tr');

    [...tr.children].forEach( (td, i)=>{ console.log(i)} );


  }

  const valueType = (col) =>{
    col["value"]

  }
  
</script>

<tr>
  {#each columns as col, i }
    <td id="{i}" class='{col["class"]}' style='{col["style"]}'>{valueType(col)}</td>
  {/each}
  <td>
    <button on:click={editRow}>E</button>
    <button>OK</button>
  </td>
</tr>  

<style>
  
</style>