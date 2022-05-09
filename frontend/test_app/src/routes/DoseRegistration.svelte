<script>
  import Photo from "../components/Photo.svelte";
  import Section from "../components/Section.svelte";
  import API_URL from "../settings";

  let fotito;

  let operator = JSON.parse(window.localStorage.getItem("operator"));

  console.log(operator);

  const setOperator = async () => {
    let operator_id = document.getElementById("token").value;
    console.log("setOperator", operator_id);

    const res = await fetch(`${API_URL}/operator/${operator_id}`);
    operator = await res.json();

    window.localStorage.setItem("operator", JSON.stringify(operator));
  };

  const takePicture = (event) => {
    fotito = event.detail.picture;
  };

  const sendData = (data) => {
    console.log("fotito", fotito);
  };

  const doPost = async (data) => {
    const myHeaders = new Headers({
      "Content-Type": "application/json",
    });

    const fetchConfig = {
      method: "POST",
      headers: myHeaders,
      cache: "no-cache",
      body: JSON.stringify(data),
    };

    const res = await fetch(`${API_URL}/dose`, fetchConfig);

    const json = await res.json();
    result = JSON.stringify(json);
    console.log(result);
  };
</script>

<Section title="Bienvenido a Dosetrack">
  {#if !operator}
    <div class="mb-3">
      <label for="name" class="form-label"
        >Por favor ingrese su token de indentificación</label
      >
      <input
        type="text"
        class="form-control"
        name="token"
        id="token"
        aria-describedby="nameHelp"
      />
      <div id="nameHelp" class="form-text">
        El token se lo debe proporcionar su empleador.
      </div>
    </div>

    <div class="text-center">
      <button on:click={setOperator} class="btn btn-primary btn-lg"
        >Ingresar</button
      >
    </div>
  {:else}
    <div class="mb-3">
      <label for="dose" class="form-label">Por favor ingrese dosis diaria</label
      >
      <div class="input-group input-group-lg">
        <input
          name="dose"
          id="dose"
          type="number"
          pattern="999.999"
          class="form-control"
          aria-label="Sizing example input"
          aria-describedby="inputGroup-sizing-lg"
        />
        <span
          class="input-group-text"
          id="inputGroup-sizing-lg"
          style="background-color: rgb(106 158 255);">mSv</span
        >
      </div>
    </div>

    <Photo on:newPhoto={takePicture} />
    <div class="text-center mt-3">
      <button on:click={sendData} class="btn btn-primary btn-lg"
        >Subir información</button
      >
    </div>
  {/if}
</Section>

<style>
</style>
