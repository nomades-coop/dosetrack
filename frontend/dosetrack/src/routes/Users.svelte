<script>
  import FormError from "../components/FormError.svelte";
  import Section from "../components/Section.svelte";
  import TableUsers from "../components/TableUsers.svelte";
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
  
  let dosetrack_user = {};
  let auth0_user = {};
  auth0_user = {};
  UserStore.subscribe((data)=>{
    dosetrack_user = data.Dosetrack;
    auth0_user = data.Auth0;
  })

  let list = [];
  let company_id = dosetrack_user.company_id.$oid;

  let promise = fetchUsers(company_id);

  let errors = {};
  let result;
  let modal;
  let dosimeters = [];



  async function fetchUsers(company_id) {
    const res = await fetch(`${API_URL}/users/${company_id}`);
    
    if (res.ok) {
      let list = await res.json();
      return list;
    } else {
      throw new Error("No se pudo obtener la lista de usuarios");
    }
  }

  async function fetchOperators() {
    const res = await fetch(`${API_URL}/operators/${company_id}`);
    
    if (res.ok) {
      list = await res.json();
      return list;
    } else {
      throw new Error("No se pudo obtener la lista de operadores");
    }
  }
  
  const content = (list) => {
    return {
      headers: [
        { _id: { title: "", type: "_id" } },
        { company_id: { title: "", type: "_id" } },
        { last_name: { title: "Apellido", type: "str" } },
        { name: { title: "Nombre", type: "str" } },
        { email: { title: "E-Mail", type: "str" } },
        { role: { title: "Rol", type: "str" } },
        { status: { title: "Status", type: "str" } },
      ],
      rows: list,
    };
  };

  const removeUser = (event) => {
    errors = {};
    let user = event.detail.user;

    //TODO: Ver si es necesario eliminar el usuario de la base de datos

    console.log(user)
    
  };

  const editUser = (event) => {
    errors = {};
    let user = event.detail.user;
    console.log(user);
  };

</script>

<Section title="Usuarios">
  {#await promise}
    Esperando...
  {:then lista}
    <TableUsers
      on:edit={editUser}
      on:remove={removeUser}
      content={content(lista)}
    />
  {:catch error}
    <!-- {error.message} -->
  {/await}
  <!-- Modal -->
    
</Section>

<style>

</style>
