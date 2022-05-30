<script>
  import FormError from "../components/FormError.svelte";
  import Section from "../components/Section.svelte";
  import TableDosimeters from "../components/TableDosimeters.svelte";
  import API_URL from "../settings";
  import { onMount } from "svelte";

  import {
    isDateRule,
    isRequiredRule,
    isFormValid,
    setError,
  } from "../validations";

  let list = [];

  let promise = fetchDosimeters();

  let errors = {};
  let result;
  let modal;

  onMount(async () => {
    modal = new bootstrap.Modal("#dosimeterModal");
  });

  async function fetchDosimeters() {
    const myHeaders = new Headers({
      "Content-Type": "application/json",
    });

    const fetchConfig = {
      method: "GET",
      headers: myHeaders,
      cache: "no-cache",
    };

    const res = await fetch(`${API_URL}/dosimeters`, fetchConfig);
    list = await res.json();

    if (res.ok) {
      return list;
    } else {
      throw new Error("No se pudo obtener la lista de dosimetros");
    }
  }

  const onSubmit = (e) => {
    let formData = new FormData(e.target);
    const data = {};

    for (let field of formData) {
      const [key, value] = field;
      data[key] = value;
    }

    if (data._id === "") delete data._id;
    console.log(data);

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
    result = JSON.stringify(json);
    console.log(result);
    promise = fetchDosimeters();
  };

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

  const content = (list) => {
    return {
      headers: [
        { _id: { title: "", type: "_id" } },
        { brand: { title: "Marca", type: "str" } },
        { model: { title: "Modelo", type: "str" } },
        { sn: { title: "SN", type: "str" } },
        { last_calibration_date: { title: "Calibración", type: "date" } },
        { status: { title: "Status", type: "str" } },
      ],
      rows: list,
    };
  };

  const newDosimeter = () => {
    errors = {};
    document.getElementById("modal-delete").style.display = "none";
    document.getElementById("modal-save").style.display = "inline-block";
    document.getElementById("dosimeterModalForm").innerHTML = "Nuevo Dosímetro";
    document.getElementById("formDosimeter").reset();
    document.getElementById("company_id").value = "6272d4752a9df7aeb4aaab90";
    modal.show();
  };

  const removeDosimeter = (event) => {
    errors = {};
    let dosimeter = event.detail.dosimeter;

    document.getElementById("modal-delete").style.display = "inline-block";
    document.getElementById("modal-save").style.display = "none";

    document.getElementById("dosimeterModalForm").innerHTML =
      "Delete dosimeter";

    document.getElementById("_id").value = dosimeter._id.$oid;
    document.getElementById("brand").value = dosimeter.brand;
    document.getElementById("model").value = dosimeter.model;
    document.getElementById("sn").value = dosimeter.sn;
    document.getElementById("last_calibration_date").value = new Date(
      dosimeter.last_calibration_date
    )
      .toISOString()
      .substring(0, 10);
    document.getElementById("status").value = dosimeter.status;

    modal.show();
  };

  const editDosimeter = (event) => {
    errors = {};
    let dosimeter = event.detail.dosimeter;

    document.getElementById("modal-delete").style.display = "none";
    document.getElementById("modal-save").style.display = "inline-block";

    document.getElementById("dosimeterModalForm").innerHTML =
      "Modificar Dosímetro";

    document.getElementById("_id").value = dosimeter._id.$oid;
    document.getElementById("brand").value = dosimeter.brand;
    document.getElementById("model").value = dosimeter.model;
    document.getElementById("sn").value = dosimeter.sn;
    document.getElementById("last_calibration_date").value = new Date(
      dosimeter.last_calibration_date
    )
      .toISOString()
      .substring(0, 10);
    document.getElementById("status").value = dosimeter.status;

    modal.show();
  };
</script>

<Section title="Dosímetros">
  {#await promise}
    Esperando...
  {:then lista}
    <TableDosimeters
      on:edit={editDosimeter}
      on:remove={removeDosimeter}
      content={content(lista)}
    />
  {:catch error}
    {error.message}
    no andó
  {/await}
  <!-- Button trigger modal -->
  <button
    type="button"
    class="btn btn-success"
    data-bs-toggle="modal_"
    data-bs-target="#dosimeterModal"
    on:click={newDosimeter}
  >
    Nuevo dosímetro
  </button>

  <!-- Modal -->
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
            value="6272d4752a9df7aeb4aaab90"
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
</Section>

<style>
  .form-label {
    margin-bottom: 0.5rem;
    font-weight: bold;
  }
</style>
