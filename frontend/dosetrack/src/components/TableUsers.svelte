<script>
  import Icon from "svelte-icons-pack/Icon.svelte";
  import BsPencilFill from "svelte-icons-pack/bs/BsPencilFill";
  import BsTrash2Fill from "svelte-icons-pack/bs/BsTrash2Fill";
  import IoNuclear from "svelte-icons-pack/io/IoNuclear";
  import { createEventDispatcher } from "svelte";
  import { push, pop, replace } from "svelte-spa-router";

  export let content = {
    headers: [],
    rows: [],
  };

  const dispatch = createEventDispatcher();

  let empty =
    Object.keys(content).length === 0 && content.constructor === Object;

  if (empty) {
    content = {
      headers: [],
      rows: [],
    };
  } else {
    let fields = content.headers.map((h) => Object.keys(h)[0]);

    content.headers = content.headers
      .map((k, v) => Object.entries([k][0]))
      .map((e) => e[0][1]);

    content.rows = content.rows
      .map((r) => fields.reduce((o, k) => ({ ...o, [k]: r[k] }), {}))
      .map((r) => Object.entries(r));
  }

  const toObject = (data) => {
    // -- convierte el array en un obj
    let keys = data.map((d) => d[0]);
    let values = data.map((d) => d[1]);

    let obj = {};
    keys.forEach((key, i) => (obj[key] = values[i]));

    return obj;
  };

  const edit = (column, user) => {
    // TODO: Permitir editar un usuario (Solo rol y status, lo demas viene de auth0)
    console.log(column, user);
    dispatch("edit", {
      user: toObject(user),
    });
  };

  const remove = (user) => {
    
    dispatch("remove", {
      user: toObject(user),
    });
  };

</script>

<div class="row">
  <!-- Operator-->
  <div class="table-responsive">
    <!-- <div class="table-wrapper-scroll-y my-custom-scrollbar"> -->
    <table class="table table-striped table-hover table-sm">
      <thead>
        <tr>
          {#each content.headers as head}
            {#if head.type != "_id"}
              <th scope="col">{head.title}</th>
            {/if}
          {/each}
          <th />
        </tr>
      </thead>
      <tbody>
        {#each content.rows as row}
          <tr class="align-middle" data-user-id={row[0][1].$oid}>
            {#each row as column, i}
              {#if content.headers[i].type != "_id"}
                <td on:click={edit(i, row)}>
                  {#if content.headers[i].type === "img"}
                    {column[1]}
                    <img src="" alt="" />
                  {:else if content.headers[i].type === "date"}
                    {new Date(column[1]).toISOString().split("T")[0]}
                  {:else}
                    {column[1]}
                  {/if}
                </td>
              {/if}
            {/each}

            <td nowrap style="text-align: end">
              <div class="flex-column">
                <span class="btn btn-outline-danger" on:click={remove(row)}
                  ><Icon src={BsTrash2Fill} /></span
                >
              </div>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
    <!-- </div> -->
  </div>
</div>

<style>
  /* .my-custom-scrollbar {
    position: relative;
    height: 270px;
    overflow: auto;
  }
  .table-wrapper-scroll-y {
    display: block;
  } */

  /* width */
  ::-webkit-scrollbar {
    width: 15px;
  }

  /* Track */
  ::-webkit-scrollbar-track {
    box-shadow: inset 0 0 5px grey;
    border-radius: 5px;
  }

  /* Handle */
  ::-webkit-scrollbar-thumb {
    background: rgb(220, 215, 215);
    border-radius: 5px;
  }

  /* Handle on hover */
  ::-webkit-scrollbar-thumb:hover {
    background: #525050;
  }
</style>
