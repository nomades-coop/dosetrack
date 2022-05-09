<script>
  import { onMount } from "svelte";
  import { add_flush_callback } from "svelte/internal";

  export let message = "Seguro?";
  export let title = "Confirmar";

  export let response = 0;

  let confirmModal;
  let selectedYes = false;

  onMount(async () => {
    confirmModal = new bootstrap.Modal("#confirmModal");
    //confirmModal.show();
  });

  export const ask = (t, m) => {
    title = t;
    message = m;
    let mod = new bootstrap.Modal("#confirmModal");
    mod.show();
    return mod.response;
  };

  export const show = () => {
    confirmModal.show();
  };
  export const hide = () => {
    confirmModal.hide();
  };

  const selectNo = () => {
    console.log("No!");
    selectedYes = false;
    response = false;
    confirmModal.hide();
  };
  const selectYes = () => {
    console.log("Si!");
    selectedYes = true;
    response = true;
    confirmModal.hide();
  };
</script>

<!-- Modal Yes No -->
<div
  class="modal fade"
  id="confirmModal"
  tabindex="-1"
  aria-labelledby="confirmModalLabel"
  aria-hidden="true"
>
  <div class="modal-dialog">
    <div class="modal-content">
      <div class="modal-header">
        <h5 class="modal-title" id="confirmModalLabel">{title}</h5>
        <button
          type="button"
          class="btn-close"
          data-bs-dismiss="modal"
          aria-label="Close"
        />
      </div>
      <div class="modal-body">
        {message}
      </div>
      <div class="modal-footer">
        <button
          type="button"
          class="btn btn-default"
          on:click={selectNo}
          id="modal-btn-no">No</button
        >
        <button
          type="button"
          class="btn btn-primary"
          on:click={selectYes}
          id="modal-btn-si">Si</button
        >
      </div>
    </div>
  </div>
</div>
