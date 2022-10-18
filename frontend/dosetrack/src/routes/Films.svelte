<script>
  import { each, validate_each_argument } from "svelte/internal";
  import EditableRow from "../components/editable_row.svelte";

  import { onMount } from "svelte";

  let watchTatget
  let outputTarget
  let observer

  let months = Array(12 - 1 + 1).fill().map((_, idx) => 1 + idx)

	onMount(() => {

    observer = new MutationObserver((mutations)=>{
    mutations.forEach(mutation => {
      if (mutation.type=='characterData'){
        outputTarget.innerText = mutation.target.textContent;
      }
    });
  })

  // debugger
  observer.observe(watchTatget, {attributes: true, characterData: true, subtree: true})

	});

  let columns = [
    {
      class: '',
      style: '',
      value: 1,
      input: {
        type: 'select',
        option_list: [
          {
            option: "Pablo",
            value: "1"
          },
          {
            option: "Matias",
            value: 2
          }
        ]
      }      
    }
  ]

</script>

<h1 id='valores' contentEditable="true" bind:this={outputTarget}>2022</h1>

<div class="table-responsive">
  <table class="table table-striped table-hover table-sm">
    <thead>
      <tr>
        <th style="display:none;"></th>
        <th>Operador</th>
        <th>Código</th>
        <th>FILM</th>

        {#each months as month }
          <th>{month}</th>    
        {/each}
        <th>Acumulado</th>
      </tr>
    </thead>
    <tbody>

      <tr>
        <td style="display:none;">algo aqui</td>
        <td> <input type="text" value="Pablo Centurion"  class="inputs"></td>
        <td>FIGMA90</td>
        <td>
          <select name="" id=""  class="input-select">
            <option value="">Nombre dle FIlm</option>

          </select>
        </td>
        {#each months as month }
          <td> <input type="text" value="0" class="inputs"></td>    
        {/each}

        <td><input type="text" value="0"  class="inputs"></td>
      </tr>      

      <tr>
        <td>Pablo Centurion</td>
        <td>FIGMA90</td>
        <td>Nombre dle FIlm</td>
        <td bind:this={watchTatget} contentEditable="true">0,1</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>1,2</td>
        <td>0</td>
        <td>1,3</td>
      </tr>

      <tr>
        <td>Ariel M</td>
        <td>FIGMA90</td>
        <td>Nombre dle FIlm</td>
        <td bind:this={watchTatget} contentEditable="true">0,1</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>1,2</td>
        <td>0</td>
        <td>1,3</td>
      </tr>

      <EditableRow {columns}/>


    </tbody>
  </table>
</div>

<style>
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

</style>