<script>
  import Icon from "svelte-icons-pack/Icon.svelte";
  import BiSolidCamera from "svelte-icons-pack/bi/BiSolidCamera";
  import BiSolidCameraOff from "svelte-icons-pack/bi/BiSolidCameraOff";
  import IoCameraReverse from "svelte-icons-pack/io/IoCameraReverse";
  import BiReset from "svelte-icons-pack/bi/BiReset";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  let fotito;
  let width = 640; // We will scale the photo width to this
  let height = 0; // This will be computed based on the input stream

  let videoSource = null;
  let loading = false;
  let image_data_url;
  let showIcon = true;
  let cameraStart;
  let cameraShot;
  let selectedCamera = "environment";

  let cameraIcon = BiSolidCameraOff;

  const obtenerVideoCamara = async (camera) => {
    try {
      loading = true;
      const stream = await navigator.mediaDevices.getUserMedia({
        video: {
          facingMode: camera,
        },
        audio: false,
      });

      videoSource.srcObject = stream;
      videoSource.play();
      loading = false;
      showIcon = true;
      cameraShot.classList.remove("d-none");
      cameraStart.classList.add("d-none");
      videoSource.classList.remove("d-none");
    } catch (error) {
      console.log(error);
    }
  };

  const changeIcon = () => {
    showIcon = !showIcon;
  };

  const changeCamera = () => {
    if (selectedCamera === "environment") {
      selectedCamera = "user";
    } else {
      selectedCamera = "environment";
    }

    obtenerVideoCamara(selectedCamera);
  };

  const takePicture = () => {
    height = videoSource.videoHeight / (videoSource.videoWidth / width);

    videoSource.setAttribute("width", width);
    videoSource.setAttribute("height", height);
    canvas.setAttribute("width", width);
    canvas.setAttribute("height", height);

    const context = canvas.getContext("2d");
    context.drawImage(videoSource, 0, 0, canvas.width, canvas.height);
    image_data_url = canvas.toDataURL("image/jpeg", 0.8);
    fotito.src = image_data_url;
    fotito.style.zIndex = 1;
    fotito.classList.remove("d-none");
    context.clearRect(0, 0, canvas.width, canvas.height);

    newPhoto(image_data_url);
    // console.log(image_data_url);
  };

  const reset = () => {
    fotito.style.zIndex = -1;
    fotito.classList.add("d-none");
    fotito.src = "";
    image_data_url = "";
  };

  const newPhoto = (imgData) => {
    // -- hace el dispatch
    dispatch("newPhoto", {
      picture: imgData,
    });
    // reset();
  };
</script>

{#if loading}
  <div data-none={changeIcon()} />
{/if}
<!-- svelte-ignore a11y-media-has-caption -->

<div class="d-flex justify-content-center">
  <video bind:this={videoSource} class="img-fluid align-self-center d-none" />

  <img
    bind:this={fotito}
    class="img-fluid align-self-center d-none"
    src=""
    alt=""
  />
</div>
<canvas
  id="canvas"
  class="align-self-center"
  style="position: absolute; top:0; left:0; z-index: -10;"
/>

<div data-desc="button-bar" class="d-flex justify-content-center mt-3 mb-2">
  <button
    bind:this={cameraStart}
    class="btn btn-danger btn-lg"
    on:click={() => obtenerVideoCamara("environment")}
  >
    {#if showIcon}
      <Icon src={cameraIcon} color="#fff" />
    {:else}
      <div class="spinner-border spinner-border-md" role="status">
        <span class="visually-hidden">Loading...</span>
      </div>
    {/if}
  </button>
  <div bind:this={cameraShot} class="d-none d-flex">
    <button class="btn btn-primary btn-lg fs-3" on:click={takePicture}
      ><Icon src={BiSolidCamera} color="#fff" /></button
    >
    <button
      class="btn btn-primary btn-lg fs-3"
      style="margin: 0 1em 0 1em;"
      on:click={changeCamera}
      ><Icon src={IoCameraReverse} color="#fff" /></button
    >
    <button id="photo.reset" class="btn btn-danger btn-lg fs-3" on:click={reset}
      ><Icon src={BiReset} color="#fff" /></button
    >
  </div>
</div>

<style>
  video {
    border-radius: 15px;
    max-width: 80%;
  }

  img {
    z-index: -1;
    position: absolute;
    border-radius: 15px;
    border: solid 5px lawngreen;
    max-width: 80%;
  }

  :global(.custom-icon) {
    position: relative;
    top: -3px;
  }
</style>
