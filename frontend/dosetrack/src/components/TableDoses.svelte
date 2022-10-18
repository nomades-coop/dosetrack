<script>
  import { createEventDispatcher } from "svelte";
  import moment from "moment";
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
    console.log("content", content);
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
</script>

<div class="row">
  <!-- Operator-->
  <div class="table-responsive">
    <!-- <div class="table-wrapper-scroll-y my-custom-scrollbar"> -->
    <table class="table table-striped table-hover">
      <thead>
        <tr>
          {#each content.headers as head}
            {#if head.type != "_id"}
              <th style="text-align: {head.align || 'inherit'}" scope="col"
                >{head.title}</th
              >
            {/if}
          {/each}
        </tr>
      </thead>
      <tbody>
        {#each content.rows as row}
          <tr class="align-middle" data-operator-id={row[0][1].$oid}>
            {#each row as column, i}
              {#if content.headers[i].type != "_id"}
                <td style="text-align: {content.headers[i].align || 'inherit'}">
                  {#if content.headers[i].type === "img"}
                    {column[1]}
                    <img src="" alt="" />
                  {:else if content.headers[i].type === "date"}
                    {moment(parseInt(column[1]["$date"]["$numberLong"])).format(
                      "DD/MM/YYYY HH:mm:ss"
                    )}
                  {:else if content.headers[i].type === "obj"}
                    {column[1][content.headers[i].field]}
                  {:else}
                    {column[1]}
                  {/if}
                </td>
              {/if}
            {/each}
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
