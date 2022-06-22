<script>
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  function onSwapClick() {
    dispatch("navigateToSwap", {});
  }

  function onPoolsClick() {
    dispatch("navigateToPools", {});
  }
</script>

<div class="flex
            items-center
            p-4 gap-16
            bg-gradient-to-br from-stone-100 to-stone-200
            outline outline-2 outline-stone-200
            ring ring-3 ring-stone-50
            shadow-xl shadow-red-400/30">
  <div class="pl-2 text-2xl font-bold tracking-wider">
    ⇄SWP⇄
  </div>

  <menu class="flex grow gap-4
               text-lg font-medium">
    <li><a on:click={onSwapClick} href="#/swap">Swap</a>
    <li><a on:click={onPoolsClick} href="#/pools">Pools</a>
  </menu>

  <div>
    System status.
  </div>

</div>
