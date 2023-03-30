<script>
  import ImZoomIn from "svelte-icons-pack/im/ImZoomIn";
  import ImZoomOut from "svelte-icons-pack/im/ImZoomOut";
  import ImLoop2 from "svelte-icons-pack/im/ImLoop2";
  import Icon from "svelte-icons-pack";

  export let title = "";
  export let doses = [];
  export let filmPeriodTotal = 0;
  export let zoom = "100%";

  let months = {
    1: "Ene",
    2: "Feb",
    3: "Mar",
    4: "Abr",
    5: "May",
    6: "Jun",
    7: "Jul",
    8: "Ago",
    9: "Sep",
    10: "Oct",
    11: "Nov",
    12: "Dic",
  };

  let totalDosimetersDose = doses.reduce((acc, dose) => acc + dose.dose, 0);

  function zoomIn() {
    zoom = parseInt(zoom) * 1.1 + "%";
    console.log(zoom);
  }

  function zoomOut() {
    zoom = parseInt(zoom) / 1.1 + "%";
  }

  function zoomReset() {
    zoom = "100%";
  }
</script>

<div class="contenedor">
  <h2 class="title">
    {title}
    <span style="float: right;margin-top: -0.5em;">
      <button class="btn btn-ligth btn-sm" on:click={() => zoomIn()}
        ><Icon src={ImZoomIn} /></button
      >
      <button class="btn btn-ligth btn-sm" on:click={() => zoomOut()}
        ><Icon src={ImZoomOut} /></button
      >
      <button class="btn btn-ligth btn-sm" on:click={() => zoomReset()}
        ><Icon src={ImLoop2} /></button
      >
    </span>
  </h2>

  <div class="container" style="zoom: {zoom}">
    <table class="data-table">
      <tr>
        <td class="dosimeter-total" colspan={doses.length}
          >{Math.round(totalDosimetersDose * 100) / 100}</td
        >
      </tr>
      <tr>
        {#each doses as dose}
          <td class="dosimeter-item date"
            >{Math.round(dose.dose * 100) / 100}</td
          >
        {/each}
      </tr>
      <tr>
        {#each doses as dose}
          <td
            class="card-title snap {(dose.date.getMonth() + 1) % 2 === 0
              ? 'month-odd'
              : 'month-even'} date"
          >
            <span class="month">{months[dose.date.getMonth() + 1]}</span><br />
            {dose.date.getDate()}
          </td>
        {/each}
      </tr>
      <tr
        ><td class="films-total" colspan={doses.length}>{filmPeriodTotal}</td
        ></tr
      >
    </table>
  </div>
</div>

<style>
  :root {
    --red: #ef233c;
    --darkred: #c00424;
    --platinum: #e5e5e5;
    --black: #2b2d42;
    --white: #fff;
    --thumb: #edf2f4;
    --dark: #000;
  }

  .container {
    overflow-x: auto;
    scroll-snap-type: x mandatory;
    flex-wrap: wrap;
    border: solid 1px black;
    max-width: fit-content;
    border-radius: 4px;
    background-color: #d6d6d6;
  }

  .snap {
    scroll-snap-align: start;
    transition: all 0.5s;
  }

  td {
    padding: 2px;
    position: relative;
  }

  td:hover {
    color: var(--dark);
    background: #00ff5a;
    cursor: pointer;
    transition: none;
  }

  .card-title {
    font-family: "Wallpoet", sans-serif;
    font-weight: bold;
    text-align: center;
  }

  .dosimeter-item {
    background-color: #f3f0c2;
    text-align: center;
  }

  .dosimeter-total {
    background-color: #f3f0c2;
    text-align: center;
    font-weight: bold;
    font-size: 20px;
  }

  .films-total {
    background-color: #cfa6fb;
    text-align: center;
    font-weight: bold;
    font-size: 20px;
  }

  .month {
    font-size: 10pt;
    color: gray;
  }

  .month-odd {
    background-color: #c7eeff;
  }

  .month-even {
    background-color: #fff;
  }

  /* button {
    font-size: 8pt;
    font-weight: 900;
    color: #949494;
    background-color: #e8e8e8;
    border-radius: 7px;
    width: 30px;
    height: 30px;
    vertical-align: middle;
  } */

  h2 {
    font-size: 32px;
    margin-bottom: 0.29em;
  }

  .container::-webkit-scrollbar {
    height: 12px;
  }

  .container::-webkit-scrollbar-thumb,
  .container::-webkit-scrollbar-track {
    border-radius: 3px;
  }

  .container::-webkit-scrollbar-thumb {
    /*   background: var(--darkred); */
    background: #0095ff;
  }

  .container::-webkit-scrollbar-track {
    background: var(--thumb);
  }
  .contenedor {
    margin-top: 15px;
    margin-bottom: 30px;
    border: 1px solid;
    border-radius: 5px;
    padding: 5px;
    background-color: #ecf4fb;
    padding-bottom: 15px;
  }

  .data-table {
    border-radius: 10px;
  }
</style>
