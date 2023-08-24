import {writable} from 'svelte/store';
import {environment as env} from '$app/environment';

export const bookmarks = writable([]);

async function fetchBookmarks() {
  // const res = await fetch('/api/bookmarks');
  const res = await fetch('/api/bookmarks');
  bookmarks.set(await res.json());
}

fetchBookmarks();
