import { readable, writable, get } from 'svelte/store';

export const UserStore = writable({ "Auth0": {}, "Dosetrack": {} });
