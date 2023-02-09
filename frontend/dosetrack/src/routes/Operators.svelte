<script>
  import FormError from "../components/FormError.svelte";
  import Section from "../components/Section.svelte";
  import TableOperators from "../components/TableOperators.svelte";
  import { onMount } from "svelte";
  import { 
    isDateRule,
    isRequiredRule,
    setError,
    isFormValid,
    isCommaSeparated,
  } from "../validations";
  import API_URL from "../settings";
  import {UserStore} from "../store";
  import {operators_by_company} from "../services/operators";
  import {getDosimetersByCompany} from "../services/dosimeter";	

  import { toast } from '@zerodevx/svelte-toast'

  let dosetrack_user = {};
  let auth0_user = {};
  auth0_user = {};
  UserStore.subscribe((data)=>{
    dosetrack_user = data.Dosetrack;
    auth0_user = data.Auth0;
  })

  let list = [];
  let company_id = dosetrack_user.company_id.$oid;

  let promise = fetchOperators();

  let errors = {};
  let result;
  let modal;
  let dosimeters = [];
  let token;
  let tokenInput;

  onMount(async () => {
    modal = new bootstrap.Modal("#operatorModal");
    dosimeters = await fetchDosimeters(company_id);
    

  });

  async function fetchDosimeters(company_id) {
    return await getDosimetersByCompany(company_id);
  }

  async function fetchOperators() {
    return await operators_by_company(company_id);
  }

  const onSubmit = (e) => {
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
    console.log("form errors", errors);

    if (isFormValid(errors)) {
      modal.hide();
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
    const res = await fetch(`${API_URL}/operator/${id}`, fetchConfig);
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

    const res = await fetch(`${API_URL}/operator`, fetchConfig);

    const json = await res.json();
    result = JSON.stringify(json);
    console.log(result);
    promise = fetchOperators();
  };

  const validateForm = (data) => {
    errors = {};

    setError(errors, data, "name", isRequiredRule, "Es obligatorio");
    setError(
      errors,
      data,
      "name",
      isCommaSeparated,
      "El apellido debe estar separado del nombre con una coma [,]"
    );
    setError(errors, data, "dni", isRequiredRule, "Es obligatorio");
    setError(errors, data, "licence", isRequiredRule, "Es obligatorio");

    return errors;
  };

  const content = (list) => {
    return {
      headers: [
        { _id: { title: "", type: "_id" } },
        { company_id: { title: "", type: "_id" } },
        { name: { title: "Nombre", type: "str" } },
        { dni: { title: "DNI", type: "str" } },
        { licence: { title: "Licencia", type: "str" } },
        { dosimeter_id: { title: "Dosímetro", type: "_id" } },
        { status: { title: "Status", type: "str" } },
      ],
      rows: list,
    };
  };

  const newOperator = () => {
    errors = {};
    document.getElementById("token-container").classList.add("d-none");
    document.getElementById("token").value = "";

    document.getElementById("modal-delete").style.display = "none";
    document.getElementById("modal-save").style.display = "inline-block";
    document.getElementById("operatorModalForm").innerHTML = "Nuevo Operador";
    document.getElementById("formOperator").reset();
    document.getElementById("company_id").value = company_id;
    modal.show();
  };

  const removeOperator = (event) => {
    errors = {};
    let operator = event.detail.operator;

    console.log("remove", operator);
    document.getElementById("modal-delete").style.display = "inline-block";
    document.getElementById("modal-save").style.display = "none";

    document.getElementById("operatorModalForm").innerHTML = "Delete operator";
    document.getElementById("token-container").classList.remove("d-none");
    document.getElementById("token").value = operator._id.$oid;

    document.getElementById("_id").value = operator._id.$oid;
    document.getElementById("company_id").value = operator.company_id.$oid;
    document.getElementById("name").value = operator.name;
    document.getElementById("dni").value = operator.dni;
    document.getElementById("licence").value = operator.licence;
    if (operator.dosimeter_id) {
      document.getElementById("dosimeter_id").value =
        operator.dosimeter_id.$oid;
    }
    document.getElementById("status").value = operator.status;

    modal.show();
  };

  const editOperator = (event) => {
    errors = {};
    let operator = event.detail.operator;

    document.getElementById("modal-delete").style.display = "none";
    document.getElementById("modal-save").style.display = "inline-block";

    document.getElementById("operatorModalForm").innerHTML =
      "Modificar Operador";

    document.getElementById("_id").value = operator._id.$oid;
    document.getElementById("token-container").classList.remove("d-none");
    document.getElementById("token").value = operator._id.$oid;
    document.getElementById("company_id").value = operator.company_id.$oid;
    document.getElementById("name").value = operator.name;
    document.getElementById("dni").value = operator.dni;
    document.getElementById("licence").value = operator.licence;
    if (operator.dosimeter_id) {
      document.getElementById("dosimeter_id").value =
        operator.dosimeter_id.$oid;
    } else {
      document.getElementById("dosimeter_id").value = "";
    }
    document.getElementById("status").value = operator.status;
    modal.show();
  };

  const copyToken = () => {
    const token = document.getElementById("token").value;
    
    if (!navigator.clipboard) {
      token.select()
      token.setSelectionRange(0, 99999);
      document.execCommand("copy");
    } else {
      navigator.clipboard.writeText(token).then(() => {
          toast.push('¡Token copiado al portapapeles!');
          console.log("Async: Copying to clipboard was successful!");
        },
        (err) => {
          console.error("Async: Could not copy text: ", err);
        }
      );
    }

  };

</script>

<Section title="Operadores">
  {#await promise}
    Esperando...
  {:then lista}
    <TableOperators
      on:edit={editOperator}
      on:remove={removeOperator}
      content={content(lista)}
    />
  {:catch error}
    <!-- {error.message} -->
  {/await}
  <!-- Button trigger modal -->
  <button
    type="button"
    class="btn btn-success"
    data-bs-toggle="modal_"
    data-bs-target="#operatorModal"
    on:click={newOperator}
  >
    Crear Operador
  </button>

  <!-- Modal -->
  <div
    class="modal fade"
    id="operatorModal"
    tabindex="-1"
    aria-labelledby="operatorModalForm"
    aria-hidden="true"
  >
    <div class="modal-dialog">
      <div class="modal-content">
        <div class="modal-header">
          <h5 class="modal-title" id="operatorModalForm">New Operator</h5>
          <button
            type="button"
            class="btn-close"
            data-bs-dismiss="modal"
            aria-label="Close"
          />
        </div>
        <form
          id="formOperator"
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
              <label for="name" class="form-label">Apellido y Nombre</label>
              <input
                type="text"
                class="form-control"
                name="name"
                id="name"
                aria-describedby="nameHelp"
              />
              <div id="nameHelp" class="form-text">
                Nombre y Apellido del operador
              </div>
              <div id="nameHelp" class="invalid-feedback">Dosimeter's name</div>
              <FormError err={errors.name} />
            </div>
            <div class="mb-3">
              <label for="dni" class="form-label">DNI</label>
              <input
                type="text"
                class="form-control"
                name="dni"
                id="dni"
                maxlength="11"
                aria-describedby="dniHelp"
              />
              <div id="dniHelp" class="form-text">DNI del operador</div>
              <FormError err={errors.dni} />
            </div>

            <div class="mb-3">
              <label for="licence" class="form-label">Licencia</label>
              <input
                type="text"
                class="form-control"
                name="licence"
                id="licence"
                aria-describedby="licencelHelp"
              />
              <div id="licencelHelp" class="form-text">
                Licencia del Operador
              </div>
              <FormError err={errors.licence} />
            </div>

            <div class="mb-3">
              <label for="dosimeter_id" class="form-label">Dosímetro</label>

              <select
                class="form-select"
                id="dosimeter_id"
                name="dosimeter_id"
                aria-describedby="dosimeter_idHelp"
                aria-label="Default select example"
              >
                <option value="" selected>Sin dosímetro</option>
                {#each dosimeters as dosimeter}
                  <option value={dosimeter._id.$oid}
                    >{dosimeter.brand} ({dosimeter.model}) - {dosimeter.sn}</option
                  >
                {/each}
              </select>

              <div id="dosimeter_idHelp" class="form-text">
                Dosimetro asignado
              </div>
              <FormError err={errors.dosimeter_id} />
            </div>

            <div class="mb-3">
              <label for="staus" class="form-label">Status</label>
              <select
                id="status"
                name="status"
                class="form-select"
                aria-label="Default select example"
              >
                <option value="Enabled" selected>Habilitado</option>
                <option value="Disabled">Deshabilitado</option>
              </select>
              <div id="statusHelp" class="form-text">Dosimeter status</div>
            </div>

            <div id="token-container" class="mb-3 d-none">
              <label for="_id" class="form-label">Token</label>
              <input
                type="text"
                class="form-control"
                id="token"
                bind:value={token}
                bind:this={tokenInput}
                on:click={copyToken}
                readonly
                aria-describedby="licencelHelp"
                
                style="text-align: center;font-weight: bolder;
                font-size: 1.5em;"
              />
              <div id="licencelHelp" class="form-text">
                Identificador del operador (Click para copiar)
              </div>
              <FormError err={errors.licence} />
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
              class="btn btn-danger">Delete</button
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

  #token:focus {
      color: #292521;
      background-color: #43ec61;
      border-color: #ff8777;
      outline: 0;
      box-shadow: 0 0 0 0.25rem #0dfd3163;
  }

  input[readonly], #token{
    color: #000000;
    background-color: #d1ffd0;
    cursor: copy;
    transition: border-color .15s ease-in-out,box-shadow .15s ease-in-out;
  }
</style>
