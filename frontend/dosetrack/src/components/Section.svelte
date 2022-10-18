<script>
  export let title = "Section Name";
  export let subtitle = "";
  export let note = "";
  export let showToolbar = false;
  export let slot = "visible";
</script>

<div
  class="section"
>
  <div class="d-flex flex-column">
    <h3>{title}</h3>
    {#if subtitle}<h6>{subtitle}</h6>{/if}
    {#if note}<p>{note}</p>{/if}
  </div>
  {#if showToolbar}
    <div class="btn-toolbar mb-2 mb-md-0">
      <div class="btn-group me-2">
        <button type="button" class="btn btn-sm btn-outline-secondary"
          >Share</button
        >
        <button type="button" class="btn btn-sm btn-outline-secondary"
          >Export</button
        >
      </div>
      <button
        type="button"
        class="btn btn-sm btn-outline-secondary dropdown-toggle"
      >
        <span data-feather="calendar" />
        This week
      </button>
    </div>
  {/if}
  
  {#if slot === "visible"}
  <slot>
    <p>No hay contenido para mostrar aquí</p>
  </slot>
  {/if}
  
</div>

<style>
  /* d-flex justify-content-between flex-wrap flex-md-nowrap align-items-center pt-3 pb-2 mb-3 border-bottom */
  .section {
    border: solid 1px rgb(6, 90, 27);
    border-radius: 5px;
    padding: 8px;
    margin-bottom: 5px;
    margin-top: 23px;
  }
</style>