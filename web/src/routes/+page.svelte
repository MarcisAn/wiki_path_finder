<script lang="ts" type="module">
  import "../index.css";
  import { onMount } from "svelte";
  import init, { run } from "../pkg/wiki_graph.js";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Button } from "$lib/components/ui/button";
  import { result } from "../lib/wiki_graph";
  import { Jumper } from "svelte-loading-spinners";
  let a: string = "";
  let b: string = "";

  onMount(async () => {
    init();
  });

  async function get_path() {
    run(a, b);
  }
</script>

<h1>Atrast īsāko ceļu starp diviem Vikipēdijas rakstiem</h1>

<div class="textinputs">
  <div class="flex flex-col">
    <div>
      <Label for="a">Sākuma raksts</Label>
      <Input id="a" class="w-100" bind:value={a} />
    </div>
    <div>
      <Label for="a">Mērķa raksts</Label>
      <Input id="b" class="w-100" bind:value={b} />
    </div>
  </div>
</div>
<div class="mt-3 flex flex-row justify-center">
  <Button variant="destructive" on:click={get_path}>Atrast ceļu</Button>
</div>

<div class="flex flex-row justify-center">
  <div class="result">
    {#each $result as node, i}
      <p>{node}</p>
      {#if i != $result.length - 1}
        <p> ---> </p>
      {/if}
    {/each}
  </div>
</div>
