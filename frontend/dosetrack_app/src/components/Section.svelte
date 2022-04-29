<script>
  export let title = "Section Name";
  export let subtitle = "";
  export let note = "";
  export let showToolbar = false;
</script>

<div
  class="d-flex justify-content-between flex-wrap flex-md-nowrap align-items-center pt-3 pb-2 mb-3 border-bottom"
>
  <div class="d-flex flex-column">
    <h2>{title}</h2>
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
</div>

<slot>
  <p>No hay contenido para mostrar aquí</p>
</slot>
