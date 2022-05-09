<script>
  export let err = null;
</script>

{#if err}
  <div id="calibrationDateErrors" class="form-text text-danger">
    <ul>
      {#each err.error as error}
        <li>{error}</li>
      {/each}
    </ul>
  </div>
{/if}
