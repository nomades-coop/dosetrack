<script>
  import FormError from "./FormError.svelte";
  import { isRequiredRule, isFormValid, setError } from "../validations";
  import { onMount } from "svelte";
  import API_URL from "../settings";
  import { createEventDispatcher } from "svelte";

  import * as OperatorsService from "../services/operators";
  import * as PeriodsService from "../services/periods";

  import PeriodSelector from "./PeriodSelector.svelte";

  export let company_id = "";
  let errors = {};
  let modal;

  let operators = [];
  let periods = [];

  let periodSelector;

  const dispatch = createEventDispatcher();

  onMount(async () => {
    // console.log(company_id);
    modal = new bootstrap.Modal("#filmModal");

    operators = await OperatorsService.getByCompany(company_id, true);
    operators = operators.sort((a, b) => (a.name > b.name ? 1 : -1));

    periods = await PeriodsService.getByCompany(company_id);
  });

  export function show() {
    modal.show();
  }

  export function hide() {
    modal.hide();
  }

  export function reset() {
    document.getElementById("modal-delete").style.display = "none";
    document.getElementById("modal-save").style.display = "inline-block";
    document.getElementById("company_id").value = company_id;
    document.getElementById("filmModalForm").innerHTML = "Nuevo Film";
    document.getElementById("formFilm").reset();
  }

  export function set(title = null, film) {
    document.getElementById("modal-delete").style.display = "none";
    document.getElementById("modal-save").style.display = "inline-block";

    document.getElementById("filmModalForm").innerHTML = title || "Nuevo Film";

    document.getElementById("film_id").value = film._id.$oid;
    // document.getElementById("operator_id").value = film.operator._id.$oid;
    document.getElementById("company_code").value = film.company_code;
    document.getElementById("film_period_id").value = film.period._id.$oid;

    document.getElementById("status").value = film.status;
  }

  const validateForm = (data) => {
    let errors = {};

    setError(
      errors,
      data,
      "company_code",
      isRequiredRule,
      "Este dato es obligatorio"
    );
    // setError(
    //   errors,
    //   data,
    //   "operator_id",
    //   isRequiredRule,
    //   "Este dato es obligatorio"
    // );
    setError(
      errors,
      data,
      "period_id",
      isRequiredRule,
      "Este dato es obligatorio"
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

    if (data.film_id === "") {
      delete data.film_id;
    } else {
      data._id = data.film_id;
      delete data.film_id;
    }
    // if (data.operator_id === "") delete data.operator_id;

    data.company_id = company_id;
    console.log({ data });

    errors = {};
    errors = validateForm(data);

    if (isFormValid(errors)) {
      modal.hide();
      data.period = data.period_id;
      delete data.period_id;

      doPost(data);
    } else {
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
    const res = await fetch(`${API_URL}/film/${id}`, fetchConfig);
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

    const res = await fetch(`${API_URL}/film`, fetchConfig);
    const json = await res.json();

    updated(JSON.stringify(json));
  };

  const updated = (dosimeter) => {
    // -- hace el dispatch
    console.log("dispatch", dosimeter);
    dispatch("updated", {
      dosimeter: dosimeter,
    });
  };
</script>

<div
  class="modal fade"
  id="filmModal"
  tabindex="-1"
  aria-labelledby="filmModalForm"
  aria-hidden="true"
>
  <div class="modal-dialog">
    <div class="modal-content">
      <div class="modal-header">
        <h5 class="modal-title" id="filmModalForm">Nuevo Film</h5>
        <button
          type="button"
          class="btn-close"
          data-bs-dismiss="modal"
          aria-label="Close"
        />
      </div>
      <form
        id="formFilm"
        class=""
        novalidate
        on:submit|preventDefault={onSubmit}
      >
        <input
          type="text"
          id="film_id"
          name="film_id"
          value=""
          style="display: none"
        />
        <input
          type="text"
          id="company_id"
          name="company_id"
          value={company_id}
          style="display: none"
        />
        <div class="modal-body">
          <!-- <div class="mb-3">
            <OperatorSelector bind:this={operatorSelector} {operators} />
            <FormError err={errors.operator_id} />
          </div> -->
          <div class="mb-3">
            <label for="company_code" class="form-label">Código</label>
            <input
              type="text"
              class="form-control"
              name="company_code"
              id="company_code"
              aria-describedby="company_codeHelp"
            />
            <div id="company_codeHelp" class="form-text">
              Código asignado por la empresa
            </div>
            <FormError err={errors.company_code} />
          </div>

          <div class="mb-3">
            <PeriodSelector
              id="film_period_id"
              bind:this={periodSelector}
              {periods}
            />
            <FormError err={errors.period_id} />
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
