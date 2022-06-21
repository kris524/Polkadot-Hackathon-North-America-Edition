<script>
  // This is to make it easier to tell if the app has not yet been hot-reloaded during dev.
  setTimeout(function() {
    console.log("loaded");
  }, 3000);

  import MainMenu from "./components/MainMenu.svelte";
  import SwapBox from "./components/SwapBox.svelte";
  import PoolList from "./components/PoolList.svelte";
  import MessageOverlay from "./components/MessageOverlay.svelte";

  const MODE_SWAP_BOX = 1;
  const MODE_POOL_LIST = 2;
  
  let mode = MODE_SWAP_BOX;
</script>

<div class="flex flex-col min-h-screen">

  <section>
    <MainMenu/>
  </section>

  <div class="grow relative flex flex-col">

    <section class="flex flex-col justify-center items-center grow">
      {#if mode == MODE_SWAP_BOX}
        <SwapBox/>
      {:else if mode == MODE_POOL_LIST}
        <PoolList/>
      {:else}
        <span>Unexpected.</span>
      {/if}
    </section>

    <div class="absolute top-0 right-0">
      <MessageOverlay/>
    </div>

  </div>
</div>

<style global>
  /* Initialize Tailwind CSS */
  @tailwind base;
  @tailwind components;
  @tailwind utilities;
</style>
