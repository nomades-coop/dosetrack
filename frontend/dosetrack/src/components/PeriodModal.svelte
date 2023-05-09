<script>
  import FormError from "./FormError.svelte";
  import {
    isDateRule,
    isRequiredRule,
    isFormValid,
    setError,
    isLowerThan,
    isGreaterThan,
  } from "../validations";
  import { onMount } from "svelte";
  import API_URL from "../settings";
  import { createEventDispatcher } from "svelte";
  import IMask from "imask"; //from https://imask.js.org/
  import * as OperatorsService from "../services/operators";
  import * as PeriodsService from "../services/periods";
  import OperatorSelector from "./OperatorSelector.svelte";
  import PeriodSelector from "./PeriodSelector.svelte";
  import Operators from "../routes/Operators.svelte";

  export let company_id = "";

  let errors = {};
  let modal;
  let maskOptions;
  let mask;
  let operators = [];
  let periods = [];
  let operatorSelector;
  let periodSelector;

  const dispatch = createEventDispatcher();

  onMount(async () => {
    // console.log(company_id);
    modal = new bootstrap.Modal("#periodModal");
    maskOptions = {
      mask: "YYYY-MM",
      lazy: false,
      blocks: {
        YYYY: {
          mask: "0000",
        },

        MM: {
          mask: IMask.MaskedRange,
          from: 1,
          to: 12,
          maxLength: 2,
        },
      },
    };
    mask = IMask(document.getElementById("period"), maskOptions);

    operators = await OperatorsService.getByCompany(company_id, true);
    operators = operators.sort((a, b) => (a.name > b.name ? 1 : -1));

    periods = await PeriodsService.getByCompany(company_id);
  });

  export function show() {
    mask = IMask(document.getElementById("period"), maskOptions);
    modal.show();
  }

  export function hide() {
    modal.hide();
  }

  export function reset() {
    errors = {};
    document.getElementById("modal-delete").style.display = "none";
    document.getElementById("modal-save").style.display = "inline-block";
    document.getElementById("periodModalForm").innerHTML = "Nuevo Período";
    document.getElementById("formPeriod").reset();
    mask = IMask(document.getElementById("period"), maskOptions);
  }

  export function set(title = null, film) {
    document.getElementById("modal-delete").style.display = "none";
    document.getElementById("modal-save").style.display = "inline-block";

    document.getElementById("periodModalForm").innerHTML =
      title || "Nuevo Período";

    document.getElementById("film_id").value = film._id.$oid;
    document.getElementById("operator_id").value = film.operator._id.$oid;
    document.getElementById("company_code").value = film.company_code;
    mask.value = film.period;

    document.getElementById("status").value = film.status;
  }

  const validateForm = (data) => {
    let errors = {};

    setError(
      errors,
      data,
      "period",
      isRequiredRule,
      "Este dato es obligatorio"
    );
    setError(
      errors,
      data,
      "period_from",
      isRequiredRule,
      "Este dato es obligatorio"
    );
    setError(
      errors,
      data,
      "period_to",
      isRequiredRule,
      "Este dato es obligatorio"
    );
    setError(errors, data, "period_from", isDateRule, "No es una fecha válida");
    setError(errors, data, "period_to", isDateRule, "No es una fecha válida");
    setError(
      errors,
      data,
      ["period_from", "period_to"],
      isLowerThan,
      "La fecha debe ser menor al fin del período."
    );
    setError(
      errors,
      data,
      ["period_to", "period_from"],
      isGreaterThan,
      "La fecha debe ser mayor al inicio del período."
    );

    setError(
      errors,
      data,
      "period_from",
      haveSameYear,
      "El año debe coincidir con el período."
    );
    setError(
      errors,
      data,
      "period_to",
      haveSameYear,
      "El año debe coincidir con el período."
    );

    return errors;
  };

  const haveSameYear = (valueA) => {
    let año_periodo = parseInt(
      document.getElementById("period").value.slice(0, 4)
    );
    return año_periodo === new Date(valueA).getFullYear();
  };

  const onSubmit = async (e) => {
    let formData = new FormData(e.target);
    const data = {};

    for (let field of formData) {
      const [key, value] = field;
      data[key] = value;
    }

    if (data._id === "") delete data._id;

    if (data.period_id === "") {
      delete data.period_id;
    } else {
      data._id = data.period_id;
      delete data.period_id;
    }

    if (data.operator_id === "") delete data.operator_id;
    data.period = mask.unmaskedValue;

    data.company_id = company_id;

    errors = {};
    errors = validateForm(data);

    if (isFormValid(errors)) {
      data.start_date = { $date: new Date(data.period_from).toISOString() };
      data.end_date = { $date: new Date(data.period_to).toISOString() };
      delete data.period_from;
      delete data.period_to;

      errors = await doPost(data);

      if (errors) {
        return errors;
      }

      modal.hide();
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
    const res = await fetch(`${API_URL}/period/${id}`, fetchConfig);
    console.log(res);
    modal.hide();
  };

  const doPost = async (data) => {
    // let errors = {};

    const myHeaders = new Headers({
      "Content-Type": "application/json",
    });

    const fetchConfig = {
      method: "POST",
      headers: myHeaders,
      cache: "no-cache",
      body: JSON.stringify(data),
    };

    const res = await fetch(`${API_URL}/period`, fetchConfig);
    const json = await res.json();

    if (res.ok) {
      updated(JSON.stringify(json));
    } else {
      if (json.error === 902) {
        errors["period"] = { error: [] };
        errors["period"]["error"].push(["¡Este período ya existe!"]);
        return errors;
      }
    }
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
  id="periodModal"
  tabindex="-1"
  aria-labelledby="periodModalForm"
  aria-hidden="true"
>
  <div class="modal-dialog">
    <div class="modal-content">
      <div class="modal-header">
        <h5 class="modal-title" id="periodModalForm">Nuevo Período</h5>
        <button
          type="button"
          class="btn-close"
          data-bs-dismiss="modal"
          aria-label="Close"
        />
      </div>
      <form
        id="formPeriod"
        class=""
        novalidate
        on:submit|preventDefault={onSubmit}
      >
        <input
          type="text"
          id="period_id"
          name="period_id"
          value=""
          style="display: none"
        />

        <div class="modal-body">
          <div class="mb-3">
            <label for="period" class="form-label">Período</label>
            <input
              type="text"
              class="form-control"
              name="period"
              id="period"
              aria-describedby="periodlHelp"
            />
            <div id="periodlHelp" class="form-text">
              Período asignado al film en formato AAAA-MM ({new Date().getFullYear()}-{(
                "0" +
                (new Date().getMonth() + 1)
              ).slice(-2)})
            </div>
            <FormError err={errors.period} />
          </div>

          <div class="mb-3">
            <label for="period_from" class="form-label"
              >Inicio del Período</label
            >
            <input
              type="date"
              class="form-control"
              id="period_from"
              name="period_from"
              aria-describedby="period_fromHelp"
            />
            <div id="period_fromHelp" class="form-text">
              Fecha en la que incia el período.
            </div>
            <FormError err={errors.period_from} />
          </div>

          <div class="mb-3">
            <label for="period_to" class="form-label">Fin del Período</label>
            <input
              type="date"
              class="form-control"
              id="period_to"
              name="period_to"
              aria-describedby="period_toHelp"
            />
            <div id="period_toHelp" class="form-text">
              Fecha en la que finaliza el período.
            </div>
            <FormError err={errors.period_to} />
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
