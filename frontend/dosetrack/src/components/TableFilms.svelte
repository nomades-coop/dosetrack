<script>
  import Icon from "svelte-icons-pack/Icon.svelte";
  import BsPencilFill from "svelte-icons-pack/bs/BsPencilFill";
  import BsTrash2Fill from "svelte-icons-pack/bs/BsTrash2Fill";
  import { createEventDispatcher } from "svelte";

  export let content = {
    headers: [],
    rows: [],
  };

  const tableContent = (list) => {
    return {
      headers: [
        { _id: { title: "", type: "_id" } },
        { operator: { title: "Operador", type: "obj", accesor: "name" } },
        { company_code: { title: "Código", type: "str" } },
        { period: { title: "Periodo", type: "obj", accesor: "period" } },
        { status: { title: "Status", type: "str" } },
      ],
      rows: list,
    };
  };

  content = tableContent(content);

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

    // debugger;
  }

  const toObject = (data) => {
    // -- convierte el array en un obj
    let keys = data.map((d) => d[0]);
    let values = data.map((d) => d[1]);

    let obj = {};
    keys.forEach((key, i) => (obj[key] = values[i]));

    return obj;
  };

  const edit = (dosimeter) => {
    // -- hace el dispatch
    dispatch("edit", {
      film: toObject(dosimeter),
    });
  };

  const remove = (dosimeter) => {
    // -- hace el dispatch
    dispatch("remove", {
      film: toObject(dosimeter),
    });
  };
</script>

<div class="row">
  <!-- Operator-->
  <div class="table-responsive">
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
          <tr class="align-middle" data-dosimeter-id={row[0][1].$oid}>
            {#each row as column, i}
              {#if content.headers[i].type != "_id"}
                <td>
                  {#if content.headers[i].type === "img"}
                    {column[1]}
                    <img src="" alt="" />
                  {:else if content.headers[i].type === "date"}
                    {new Date(column[1]).toISOString().split("T")[0]}
                  {:else if content.headers[i].type === "obj"}
                    {column[1] ? column[1][content.headers[i].accesor] : "N/A"}
                  {:else}
                    {column[1] || "N/A"}
                  {/if}
                </td>
              {/if}
            {/each}
            <td style="text-align: end">
              <div class="flex-column">
                <span class="btn btn-outline-primary" on:click={edit(row)}
                  ><Icon src={BsPencilFill} /></span
                >
                <span class="btn btn-outline-danger" on:click={remove(row)}
                  ><Icon src={BsTrash2Fill} /></span
                >
              </div>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>
