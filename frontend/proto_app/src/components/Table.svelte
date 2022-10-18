<script>
  export let data = {
    headers: [],
    rows: [{}],
  };

  //data.rows = Object.keys(data.rows[key]).map((key2) => data.rows[key][key2]);
  data.rows = Object.keys(data.rows).map((key) =>
    Object.keys(data.rows[key]).map((key2) => data.rows[key][key2])
  );
  console.log(data.rows);
</script>

<div class="table-responsive">
  <table class="table table-striped table-hover table-sm">
    <thead>
      <tr>
        {#each data.headers as head}
          <th scope="col">{head.title}</th>
        {/each}
      </tr>
    </thead>
    <tbody>
      {#each data.rows as row}
        <tr>
          {#each row as column}
            <td>{column}</td>
          {/each}
        </tr>
      {/each}
    </tbody>
  </table>
</div>
