<script>
  import FormError from "./FormError.svelte";
  import {
    isDateRule,
    isRequiredRule,
    isFormValid,
    setError,
  } from "../validations";
  import { onMount } from "svelte";
  import API_URL from "../settings";
  import { createEventDispatcher } from "svelte";

  export let company_id = "";
  let errors = {};
  let modal;
  const dispatch = createEventDispatcher();

  onMount(async () => {
    modal = new bootstrap.Modal("#dosimeterModal");
  });

  export function show(){
    modal.show();
  }

  export function hide(){
    modal.hide();
  }

  const validateForm = (data) => {
    let errors = {};

    setError(errors, data, "brand", isRequiredRule, "Este dato es obligatorio");

    setError(errors, data, "model", isRequiredRule, "Este dato es obligatorio");

    setError(errors, data, "sn", isRequiredRule, "Este dato es obligatorio");

    setError(
      errors,
      data,
      "last_calibration_date",
      isRequiredRule,
      "Este dato es obligatorio"
    );
    setError(
      errors,
      data,
      "last_calibration_date",
      isDateRule,
      "La fecha ingresada no es válida"
    );

    return errors;
  };

  const onSubmit = (e) => {
    let formData = new FormData(e.target);
    const data = {};

    for (let field of formData) {
      const [key, value] = field;
      data[key] = value;
    }

    if (data._id === "") delete data._id;

    errors = {};
    errors = validateForm(data);

    if (isFormValid(errors)) {
      modal.hide();
      data.last_calibration_date = new Date(
        data.last_calibration_date
      ).toJSON();
      doPost(data);
    } else {
      console.clear();
      console.log("Invalid form", errors);
    }
  };

  const doDelete = async (id) => {
    const myHeaders = new Headers({
      "Content-Type": "application/json",
    });

    const fetchConfig = {
      method: "DELETE",
      headers: myHeaders,
      cache: "no-cache",
    };

    console.log("delete", id);
    const res = await fetch(`${API_URL}/dosimeter/${id}`, fetchConfig);
    console.log(res);
    modal.hide();
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

    const res = await fetch(`${API_URL}/dosimeter`, fetchConfig);
    const json = await res.json();

    updated(JSON.stringify(json))
  };

  const updated = (dosimeter) => {
    // -- hace el dispatch
    console.log("dispatch", dosimeter)
    dispatch("updated", {
      dosimeter: dosimeter,
    });
  };

</script>

<div
class="modal fade"
id="dosimeterModal"
tabindex="-1"
aria-labelledby="dosimeterModalForm"
aria-hidden="true"
>
<div class="modal-dialog">
  <div class="modal-content">
    <div class="modal-header">
      <h5 class="modal-title" id="dosimeterModalForm">Nuevo Dosimeter</h5>
      <button
        type="button"
        class="btn-close"
        data-bs-dismiss="modal"
        aria-label="Close"
      />
    </div>
    <form
      id="formDosimeter"
      class=""
      novalidate
      on:submit|preventDefault={onSubmit}
    >
      <input
        type="text"
        id="_id"
        name="_id"
        value=""
        style="display: none"
      />
      <input
        type="text"
        id="company_id"
        name="company_id"
        value="{company_id}"
        style="display: none"
      />
      <div class="modal-body">
        <div class="mb-3">
          <label for="brand" class="form-label">Marca</label>
          <input
            type="text"
            class="form-control"
            name="brand"
            id="brand"
            aria-describedby="brandHelp"
          />
          <div id="brandHelp" class="form-text">Marca del dosímetro</div>
          <FormError err={errors.brand} />
        </div>
        <div class="mb-3">
          <label for="model" class="form-label">Modelo</label>
          <input
            type="text"
            class="form-control"
            name="model"
            id="model"
            aria-describedby="modelHelp"
          />
          <div id="modelHelp" class="form-text">Modelo del dosímetro</div>
          <FormError err={errors.model} />
        </div>

        <div class="mb-3">
          <label for="sn" class="form-label">Número de serie</label>
          <input
            type="text"
            class="form-control"
            name="sn"
            id="sn"
            aria-describedby="snlHelp"
          />
          <div id="snlHelp" class="form-text">
            Número de serie que el fabricante le asigna al dosímetro
          </div>
          <FormError err={errors.sn} />
        </div>

        <div class="mb-3">
          <label for="last_calibration_date" class="form-label"
            >Fecha de última calibración</label
          >
          <input
            type="date"
            class="form-control"
            id="last_calibration_date"
            name="last_calibration_date"
            aria-describedby="last_calibration_dateHelp"
          />
          <div id="last_calibration_dateHelp" class="form-text">
            fecha de la última calibración hecha al dosímetro
          </div>
          <FormError err={errors.last_calibration_date} />
        </div>

        <div class="mb-3">
          <label for="staus" class="form-label">Estado</label>
          <select
            id="status"
            name="status"
            class="form-select"
            aria-label="Default select example"
          >
            <option value="Enabled" selected>Habilitado</option>
            <option value="Disabled">Desahabilitado</option>
          </select>
          <div id="statusHelp" class="form-text">
            Si el dosíemtro se encuentra en servicio
          </div>
        </div>
      </div>
      <div class="modal-footer">
        <button
          type="button"
          class="btn btn-secondary"
          data-bs-dismiss="modal">Cerrar</button
        >
        <button type="submit" id="modal-save" class="btn btn-primary"
          >Guardar</button
        >
        <button
          type="button"
          on:click={() => doDelete(document.getElementById("_id").value)}
          id="modal-delete"
          class="btn btn-danger">Eliminar</button
        >
      </div>
    </form>
  </div>
</div>
</div>