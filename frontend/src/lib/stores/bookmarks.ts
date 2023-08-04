import {writable} from 'svelte/store';
// import {environment as env} from '$app/environment';
export const bookmarks = writable([]);

async function fetchBookmarks() {
  const res = await fetch('http://localhost:8080/bookmarks');
  bookmarks.set(await res.json());
}

fetchBookmarks();
