<script>
  import { createEventDispatcher } from "svelte";
  import moment from "moment";
  export let content = {
    headers: [],
    rows: [],
  };

  const dispatch = createEventDispatcher();
  let modal;
  let imgInModal;
  let captionText;

  console.log(content);

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

  const showModal = (event) => {
    imgInModal.src = event.target.src;
    imgInModal.alt = event.target.alt;
    modal.style.display = "block";
    captionText.innerHTML = imgInModal.alt;
  };

  const closeModal = () => {
    modal.style.display = "none";
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
                  {:else if content.headers[i].type === "pic"}
                    <img
                      src={column[1] ?? "https://via.placeholder.com/50"}
                      alt={"Dosis registrada: " + row[4][1] * 1000 + "μSv"}
                      style="width: 50px; height: 50px; border-radius: 50%;"
                      class="dose-img"
                      on:click={(event) => showModal(event)}
                      on:keydown={(event) => showModal(event)}
                    />
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

<div bind:this={modal} id="imgModal" class="modal">
  <!-- Modal Content (The Image) -->
  <img
    bind:this={imgInModal}
    on:click={() => closeModal()}
    on:keydown={() => closeModal()}
    class="modal-picture-content"
    id="img01"
    alt="Big Picure"
  />

  <!-- Modal Caption (Image Text) -->
  <div bind:this={captionText} id="caption" />
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

  /* Modal */
  .dose-img:hover {
    opacity: 0.7;
  }

  /* The Modal (background) */
  .modal {
    display: none; /* Hidden by default */
    position: fixed; /* Stay in place */
    /* z-index: 1; Sit on top */
    /* padding-top: 100px; Location of the box */
    left: 0;
    top: 0;
    width: 100%; /* Full width */
    height: 100%; /* Full height */
    overflow: auto; /* Enable scroll if needed */
    background-color: rgb(0, 0, 0); /* Fallback color */
    background-color: rgba(0, 0, 0, 0.9); /* Black w/ opacity */
  }

  /* Modal Content (Image) */
  .modal-picture-content {
    margin: auto;
    display: block;
    height: 80%;
    max-width: 700px;
  }

  /* Caption of Modal Image (Image Text) - Same Width as the Image */
  #caption {
    margin: auto;
    display: block;
    width: 80%;
    max-width: 700px;
    text-align: center;
    color: #ccc;
    padding: 10px 0;
    height: 150px;
  }

  /* Add Animation - Zoom in the Modal */
  .modal-picture-content,
  #caption {
    animation-name: zoom;
    animation-duration: 0.6s;
  }

  @keyframes zoom {
    from {
      transform: scale(0);
    }
    to {
      transform: scale(1);
    }
  }

  /* 100% Image Width on Smaller Screens
  @media only screen and (max-width: 700px) {
    .modal-content {
      width: 100%;
    }
  } */
</style>
