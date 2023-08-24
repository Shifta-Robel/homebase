<script lang="ts">
  import {bookmarks} from '../lib/stores/bookmarks.ts';
  let bookmarkData = [];
  const unsubscribe = bookmarks.subscribe(value => {
    bookmarkData = value;
  })
</script>
<h1>Welcome to SvelteKit</h1>
<p class="bg-red-500">Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
<ul>
  {#each bookmarkData as bookmark}
    <li>{bookmark.title}</li>
  {/each}
</ul>
