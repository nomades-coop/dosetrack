<script>
  import { debug } from "svelte/internal";
  import EditableColumn from "./editable_column.svelte";

  export let columns = [{
    class: '',
    style: '',
    value: 'value',
    input: {
      type: 'input',
      option_list: []
    }
  }]

  let editable = false

  const editRow = (e) => {
    let tr = e.target.closest('tr');

    //[...tr.children].forEach( (td, i)=>{ console.log(i)} );

    editable = !editable

  }
  
  const column_changed = (e) => {
    alert(e.detail)
  }

</script>

<tr>

  {#each columns as col, i }
    <EditableColumn col={col} editable={editable} on:columnChanged={column_changed}/>
  {/each}
  <td>
    <button class="btn btn-xxs btn-danger hidden-button" on:click={editRow}>E</button>
  </td>

</tr>  

<style>

  .btn-xxs {
      width: 24px;
      height: 24px;
      font-size: 10px;
      padding: 0;
  }

  .hidden-button {
      opacity:0;
  }

  tr:hover .hidden-button {
      opacity:1;
  }
</style>