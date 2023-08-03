<script>
  import Photo from "../components/Photo.svelte";
  import Section from "../components/Section.svelte";
  import API_URL from "../settings";
  import { onMount } from "svelte";
  import { toast } from "@zerodevx/svelte-toast";

  // import {UserStore} from "../store";

  // let dosetrack_user = {};
  // let auth0_user = {};
  // auth0_user = {};
  // UserStore.subscribe((data)=>{
  //   dosetrack_user = data.Dosetrack;
  //   auth0_user = data.Auth0;
  // })
  // let company_id = dosetrack_user.company_id.$oid;

  let fotito;
  let doseDiv;
  let doseInput;

  let operator = JSON.parse(window.localStorage.getItem("operator"));

  console.log("local", operator);

  onMount(async () => {
    if (operator) {
      getOperator(operator);

      doseDiv = document.getElementById("error");
      doseInput = document.getElementById("dose");
      doseDiv.classList.remove("error");
    }
  });

  const getOperator = async (op) => {
    //const res = await fetch(`${API_URL}/operator/62681c48726191810ba5ae6a`);
    const res = await fetch(`${API_URL}/operator/${op._id.$oid}`);
    op = await res.json();

    if (op.details === undefined) {
      window.localStorage.setItem("operator", JSON.stringify(op));
      operator = op;
      console.log("remote", op);
    } else {
      console.log("else", op);
      let no_op = {
        id: { $oid: "000000000000000000000000" },
      };

      operator = op;

      window.localStorage.setItem("operator", JSON.stringify(no_op));
    }
  };

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

  const sendData = () => {
    let doseMsv = parseInt(doseInput.value);

    if (!doseMsv) {
      doseDiv.classList.add("error");
      doseInput.focus();
      toast.push("La dosis es inválida.");
      return;
    }

    let dose = {
      company_id: operator.company_id.$oid,
      operator_id: operator._id.$oid,
      dosimeter_id: operator.dosimeter_id.$oid,
      dose: doseMsv,
      picture: fotito,
      when: { $date: new Date().toISOString() },
    };

    doPost(dose);
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

    if (res.ok) {
      const json = await res.json();
      let result = JSON.stringify(json);
      doseDiv.classList.remove("error");
      toast.push("Dosis registrada exitosamente");
      doseInput.value = "";
      // reset photo
      fotito = null;
      document.getElementById("photo.reset").click();
      // console.log(result);
    } else {
      toast.push("No se pudo registrar la dosis");
    }
  };
</script>

<Section title={operator ? operator.name : ""}>
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
  {:else if operator.dosimeter_id}
    <div id="error" class="align-items-center justify-content-center error">
      <div class="mb-3">
        <!-- <label for="dose" class="form-label"
          >Por favor ingrese dosis diaria</label
        > -->
        <div
          class="input-group input-group-lg"
          style="padding: 1.rem!important;"
        >
          <input
            name="dose"
            id="dose"
            type="number"
            pattern="999.999"
            onkeypress="return event.charCode >= 48 && event.charCode <= 57"
            class="form-control"
            aria-label="Sizing example input"
            aria-describedby="inputGroup-sizing-lg"
          />
          <span
            class="input-group-text"
            id="inputGroup-sizing-lg"
            style="background-color: rgb(106 158 255);"><b>μSv</b></span
          >
        </div>
      </div>
    </div>

    <Photo on:newPhoto={takePicture} />
    <div class="text-center mt-3">
      <button on:click={sendData} class="btn btn-primary btn-lg"
        >Subir información</button
      >
    </div>
  {:else}
    <div class="alert alert-danger" role="alert">
      No tiene asignado un dosímetro. Por favor consulte a su supervisor.
    </div>
  {/if}
</Section>

<style>
  .error {
    animation: shake 0.2s ease-in-out 0s 2;
    box-shadow: 0em 0em 2em 1em red;
  }

  @keyframes shake {
    0% {
      margin-left: 0rem;
      margin-right: 0rem;
    }
    25% {
      margin-left: 0.5rem;
      margin-right: 0.5rem;
    }
    75% {
      margin-left: -0.5rem;
      margin-right: -0.5rem;
    }
    100% {
      margin-left: 0rem;
      margin-right: 0rem;
    }
  }
</style>
