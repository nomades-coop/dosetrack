<script>
  import FormError from "./FormError.svelte";
  import API_URL from "../settings";
  import { onMount } from "svelte";
  import { useAuth0 } from "../services/auth0";
  import { newRegistration } from "../services/registration";
  import { toast } from "@zerodevx/svelte-toast";
  import { push, replace } from "svelte-spa-router";

  const { user, isAuthenticated } = useAuth0;
  import {
    isDateRule,
    isRequiredRule,
    isFormValid,
    setError,
  } from "../validations";

  let errors = {};

  onMount(async () => {});

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

    const res = await fetch(`${API_URL}/registration`, fetchConfig);

    if (res.ok) {
      errors = {};
      const json = await res.json();
      let result = JSON.stringify(json);
      window.location.assign("/");
      console.log("POST result:", result);
    } else {
      errors = {};
      if (res.status === 409) {
        errors["company_name"] = { error: [] };
        errors["company_cuit"] = { error: [] };
        errors["company_name"]["error"].push([
          "La empresa ya ha sido registrada",
        ]);
        errors["company_cuit"]["error"].push([
          "La empresa ya ha sido registrada",
        ]);
      }
      res.text().then((text) => {
        console.log("POST Error:", text);
      });
    }

    return errors;
  };

  const onSubmit = async (e) => {
    let formData = new FormData(e.target);
    const data = {};

    for (let field of formData) {
      const [key, value] = field;
      data[key] = value;
    }

    if (data._id === "") delete data._id;
    if (data.dosimeter_id === "") delete data.dosimeter_id;

    errors = {};
    errors = validateForm(data);
    console.log("form data", data);

    if (isFormValid(errors)) {
      errors = await doPost(data);
    } else {
      console.log("Invalid form", errors);
    }

    console.log("form errors", errors);
  };

  const validateForm = (data) => {
    console.log(data);
    let errors = {};

    setError(
      errors,
      data,
      "company_name",
      isRequiredRule,
      "Este dato es obligatorio"
    );
    setError(
      errors,
      data,
      "company_cuit",
      isRequiredRule,
      "Este dato es obligatorio"
    );
    setError(
      errors,
      data,
      "company_status",
      isRequiredRule,
      "Este dato es obligatorio"
    );
    setError(
      errors,
      data,
      "user_name",
      isRequiredRule,
      "Este dato es obligatorio"
    );
    setError(
      errors,
      data,
      "user_last_name",
      isRequiredRule,
      "Este dato es obligatorio"
    );
    setError(
      errors,
      data,
      "user_status",
      isRequiredRule,
      "Este dato es obligatorio"
    );

    return errors;
  };
</script>

<form id="formOperator" class="" novalidate on:submit|preventDefault={onSubmit}>
  <h4>Datos de la empresa</h4>
  <input type="text" id="_id" name="_id" value="" style="display: none" />

  <input
    type="hidden"
    id="company_status"
    name="company_status"
    value="Enabled"
  />

  <div class="mb-3">
    <label for="company_name" class="form-label">Razón Social</label>
    <input
      type="text"
      class="form-control"
      name="company_name"
      id="company_name"
      aria-describedby="nameHelp"
    />
    <div id="company-nameHelp" class="form-text">
      Nombre comercial de la empresa.
    </div>
    <FormError err={errors.company_name} />
  </div>
  <div class="mb-3">
    <label for="company_cuit" class="form-label">CUIT</label>
    <input
      type="text"
      class="form-control"
      name="company_cuit"
      id="company_cuit"
      maxlength="11"
      aria-describedby="company_cuitHelp"
    />
    <div id="company_cuitHelp" class="form-text">
      Identificador ante el ente impositivo
    </div>
    <FormError err={errors.company_cuit} />
  </div>

  <h4>Datos del usuario administrador</h4>

  <input type="hidden" id="user_status" name="user_status" value="Enabled" />

  <input type="hidden" id="user_role" name="user_role" value="Enabled" />

  <div class="mb-3">
    <label for="dni" class="form-label">Nombre</label>
    <input
      type="text"
      class="form-control"
      name="user_name"
      id="user_name"
      value={$user.given_name ?? ""}
      maxlength="100"
      aria-describedby="user_nameHelp"
    />
    <div id="user_nameHelp" class="form-text">Su nombre</div>
    <FormError err={errors.user_name} />
  </div>

  <div class="mb-3">
    <label for="user_last_name" class="form-label">Apellido</label>
    <input
      type="text"
      class="form-control"
      name="user_last_name"
      id="user_last_name"
      value={$user.family_name ?? ""}
      maxlength="100"
      aria-describedby="user_last_nameHelp"
    />
    <div id="user-lastnameHelp" class="form-text">Su apellido</div>
    <FormError err={errors.user_last_name} />
  </div>

  <div class="mb-3">
    <label for="user_email" class="form-label">E-Mail</label>
    <input
      type="email"
      class="form-control"
      name="user_email"
      id="user_email"
      value={$user.email}
      maxlength="100"
      aria-describedby="user_emailHelp"
      readonly
    />
    <div id="user-lastnameHelp" class="form-text">Su e-mail</div>
    <FormError err={errors.user_email} />
  </div>

  <div class="text-center">
    <button type="submit" id="modal-save" class="btn btn-primary btn-lg"
      >Registrarme y crear empresa a mi cargo</button
    >
  </div>

  <!-- <div class="modal-footer">

  <button type="submit" id="modal-save" class="btn btn-primary"
    >Registarme y Crear empresa a mi cargo</button
  >

</div> -->
</form>

<style>
  /* .error {
    animation: shake 0.2s ease-in-out 0s 2;
    box-shadow: 0em 0em 2em 1em red;
  } */

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
