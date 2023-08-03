<script>
  import { onMount } from "svelte";

  import { UserStore } from "../store";
  import FilmsPeriods from "../components/FilmsPeriods.svelte";

  import * as OperatorsService from "../services/operators";

  let dosetrack_user = {};
  let auth0_user = {};
  auth0_user = {};

  // get the list of operators operators_by_company asyn function
  let company_id;
  let periods_data = [];
  let periods = [];

  UserStore.subscribe((data) => {
    dosetrack_user = data.Dosetrack;
    auth0_user = data.Auth0;
    company_id = dosetrack_user.company_id.$oid;
  });

  onMount(async () => {
    if (company_id) {
      periods_data = await OperatorsService.getPeriodsData(company_id, null);
      periods = periods_data;
      console.log(periods);
    }
  });
</script>

{#await periods_data}
  <div class="spinner-border" role="status">
    <span class="visually-hidden">Loading...</span>
  </div>
{:then periods}
  {#each periods as period}
    <FilmsPeriods {company_id} {period} />
  {/each}
{/await}
