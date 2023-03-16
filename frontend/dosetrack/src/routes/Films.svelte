<script>
  import { beforeUpdate, onMount } from "svelte";
  import FaSave from "svelte-icons-pack/fa/FaSave";
  import Icon from "svelte-icons-pack";
  import {operators_by_company} from "../services/operators";
  import {periods_by_company} from "../services/periods";
  import {getFilmsByCompany} from "../services/films";
  import {UserStore} from "../store";
  import FilmsPeriods from "../components/FilmsPeriods.svelte";

  let dosetrack_user = {};
  let auth0_user = {};
  auth0_user = {};
  UserStore.subscribe((data)=>{
    dosetrack_user = data.Dosetrack;
    auth0_user = data.Auth0;
  })
  
  let company_id = dosetrack_user.company_id.$oid;

  // get the list of operators operators_by_company asyn function 
  let operators = [];
  let periods = []
  let films = [];

  beforeUpdate(async () => {
    console.log("beforeUpdate...");
    console.log({operators});
    console.log({periods});
    console.log({films});
  });
  
  onMount(async () => {
    operators = await operators_by_company(company_id);
    periods = await periods_by_company(company_id);
    films = await getFilmsByCompany(company_id);
    // console.log({operators, periods, films})
  });
</script>

<FilmsPeriods company={company_id} {operators} year="2050"/>